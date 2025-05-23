use crate::NativeArray;
use crate::array::*;
use crate::chunked_array::{ChunkedGeometryArray, ChunkedNativeArray, ChunkedPointArray};
use crate::datatypes::NativeType;
use crate::error::Result;
use crate::trait_::ArrayAccessor;
use geo::algorithm::centroid::Centroid as GeoCentroid;
use geoarrow_schema::Dimension;

/// Calculation of the centroid.
///
/// The centroid is the arithmetic mean position of all points in the shape.
/// Informally, it is the point at which a cutout of the shape could be perfectly
/// balanced on the tip of a pin.
/// The geometric centroid of a convex object always lies in the object.
/// A non-convex object might have a centroid that _is outside the object itself_.
///
/// # Examples
///
/// ```
/// use geoarrow::algorithm::geo::Centroid;
/// use geoarrow::array::PolygonArray;
/// use geoarrow::trait_::ArrayAccessor;
/// use geoarrow_schema::Dimension;
/// use geo::{point, polygon};
///
/// // rhombus shaped polygon
/// let polygon = polygon![
///     (x: -2., y: 1.),
///     (x: 1., y: 3.),
///     (x: 4., y: 1.),
///     (x: 1., y: -1.),
///     (x: -2., y: 1.),
/// ];
/// let polygon_array: PolygonArray = (vec![polygon].as_slice(), Dimension::XY).into();
///
/// assert_eq!(
///     Some(point!(x: 1., y: 1.)),
///     polygon_array.centroid().get_as_geo(0),
/// );
/// ```
pub trait Centroid {
    type Output;

    /// See: <https://en.wikipedia.org/wiki/Centroid>
    ///
    /// # Examples
    ///
    /// ```
    /// use geoarrow::algorithm::geo::Centroid;
    /// use geoarrow::array::LineStringArray;
    /// use geoarrow::trait_::ArrayAccessor;
    /// use geoarrow_schema::Dimension;
    /// use geo::{line_string, point};
    ///
    /// let line_string = line_string![
    ///     (x: 40.02f64, y: 116.34),
    ///     (x: 40.02f64, y: 118.23),
    /// ];
    /// let line_string_array: LineStringArray = (vec![line_string].as_slice(), Dimension::XY).into();
    ///
    /// assert_eq!(
    ///     Some(point!(x: 40.02, y: 117.285)),
    ///     line_string_array.centroid().get_as_geo(0),
    /// );
    /// ```
    fn centroid(&self) -> Self::Output;
}

impl Centroid for PointArray {
    type Output = PointArray;

    fn centroid(&self) -> Self::Output {
        self.clone()
    }
}

impl Centroid for RectArray {
    type Output = PointArray;

    fn centroid(&self) -> Self::Output {
        let mut output_array = PointBuilder::with_capacity_and_options(
            Dimension::XY,
            self.len(),
            self.coord_type(),
            self.metadata().clone(),
        );
        self.iter_geo()
            .for_each(|maybe_g| output_array.push_point(maybe_g.map(|g| g.centroid()).as_ref()));
        output_array.into()
    }
}

/// Implementation that iterates over geo objects
macro_rules! iter_geo_impl {
    ($type:ty) => {
        impl Centroid for $type {
            type Output = PointArray;

            fn centroid(&self) -> Self::Output {
                let mut output_array = PointBuilder::with_capacity_and_options(
                    Dimension::XY,
                    self.len(),
                    self.coord_type(),
                    self.metadata().clone(),
                );
                self.iter_geo().for_each(|maybe_g| {
                    output_array.push_point(maybe_g.and_then(|g| g.centroid()).as_ref())
                });
                output_array.into()
            }
        }
    };
}

iter_geo_impl!(LineStringArray);
iter_geo_impl!(PolygonArray);
iter_geo_impl!(MultiPointArray);
iter_geo_impl!(MultiLineStringArray);
iter_geo_impl!(MultiPolygonArray);
iter_geo_impl!(MixedGeometryArray);
iter_geo_impl!(GeometryCollectionArray);
iter_geo_impl!(GeometryArray);

impl Centroid for &dyn NativeArray {
    type Output = Result<PointArray>;

    fn centroid(&self) -> Self::Output {
        use NativeType::*;

        let result = match self.data_type() {
            Point(_) => self.as_point().centroid(),
            LineString(_) => self.as_line_string().centroid(),
            Polygon(_) => self.as_polygon().centroid(),
            MultiPoint(_) => self.as_multi_point().centroid(),
            MultiLineString(_) => self.as_multi_line_string().centroid(),
            MultiPolygon(_) => self.as_multi_polygon().centroid(),
            GeometryCollection(_) => self.as_geometry_collection().centroid(),
            Rect(_) => self.as_rect().centroid(),
            Geometry(_) => self.as_geometry().centroid(),
        };
        Ok(result)
    }
}

impl<G: NativeArray> Centroid for ChunkedGeometryArray<G> {
    type Output = Result<ChunkedPointArray>;

    fn centroid(&self) -> Self::Output {
        self.try_map(|chunk| chunk.as_ref().centroid())?.try_into()
    }
}

impl Centroid for &dyn ChunkedNativeArray {
    type Output = Result<ChunkedPointArray>;

    fn centroid(&self) -> Self::Output {
        use NativeType::*;

        match self.data_type() {
            Point(_) => self.as_point().centroid(),
            LineString(_) => self.as_line_string().centroid(),
            Polygon(_) => self.as_polygon().centroid(),
            MultiPoint(_) => self.as_multi_point().centroid(),
            MultiLineString(_) => self.as_multi_line_string().centroid(),
            MultiPolygon(_) => self.as_multi_polygon().centroid(),
            GeometryCollection(_) => self.as_geometry_collection().centroid(),
            Rect(_) => self.as_rect().centroid(),
            Geometry(_) => self.as_geometry().centroid(),
        }
    }
}
