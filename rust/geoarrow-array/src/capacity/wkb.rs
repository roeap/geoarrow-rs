use std::ops::Add;

use arrow_array::OffsetSizeTrait;

use geo_traits::{
    GeometryCollectionTrait, GeometryTrait, LineStringTrait, MultiLineStringTrait, MultiPointTrait,
    MultiPolygonTrait, PointTrait, PolygonTrait,
};
use wkb::writer::{
    geometry_collection_wkb_size, geometry_wkb_size, line_string_wkb_size,
    multi_line_string_wkb_size, multi_point_wkb_size, multi_polygon_wkb_size, point_wkb_size,
    polygon_wkb_size,
};

/// A counter for the buffer sizes of a [`WkbArray`][crate::array::WkbArray].
///
/// This can be used to reduce allocations by allocating once for exactly the array size you need.
#[derive(Debug, Clone, Copy)]
pub struct WkbCapacity {
    pub(crate) buffer_capacity: usize,
    pub(crate) offsets_capacity: usize,
}

impl WkbCapacity {
    /// Create a new capacity with known sizes.
    pub fn new(buffer_capacity: usize, offsets_capacity: usize) -> Self {
        Self {
            buffer_capacity,
            offsets_capacity,
        }
    }

    /// Create a new empty capacity.
    pub fn new_empty() -> Self {
        Self::new(0, 0)
    }

    /// Return `true` if the capacity is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer_capacity == 0 && self.offsets_capacity == 0
    }

    /// The capacity of the underlying data buffer
    pub fn buffer_capacity(&self) -> usize {
        self.buffer_capacity
    }

    /// The capacity of the underlying offsets buffer
    pub fn offsets_capacity(&self) -> usize {
        self.offsets_capacity
    }

    /// Add a Point to this capacity counter.
    #[inline]
    pub fn add_point<'a>(&mut self, point: Option<&'a (impl PointTrait + 'a)>) {
        if let Some(point) = point {
            self.buffer_capacity += point_wkb_size(point.dim());
        }
        self.offsets_capacity += 1;
    }

    /// Add a LineString to this capacity counter.
    #[inline]
    pub fn add_line_string<'a>(
        &mut self,
        line_string: Option<&'a (impl LineStringTrait<T = f64> + 'a)>,
    ) {
        if let Some(line_string) = line_string {
            self.buffer_capacity += line_string_wkb_size(line_string);
        }
        self.offsets_capacity += 1;
    }

    /// Add a Polygon to this capacity counter.
    #[inline]
    pub fn add_polygon<'a>(&mut self, polygon: Option<&'a (impl PolygonTrait<T = f64> + 'a)>) {
        if let Some(polygon) = polygon {
            self.buffer_capacity += polygon_wkb_size(polygon);
        }
        self.offsets_capacity += 1;
    }

    /// Add a MultiPoint to this capacity counter.
    #[inline]
    pub fn add_multi_point<'a>(
        &mut self,
        multi_point: Option<&'a (impl MultiPointTrait<T = f64> + 'a)>,
    ) {
        if let Some(multi_point) = multi_point {
            self.buffer_capacity += multi_point_wkb_size(multi_point);
        }
        self.offsets_capacity += 1;
    }

    /// Add a MultiLineString to this capacity counter.
    #[inline]
    pub fn add_multi_line_string<'a>(
        &mut self,
        multi_line_string: Option<&'a (impl MultiLineStringTrait<T = f64> + 'a)>,
    ) {
        if let Some(multi_line_string) = multi_line_string {
            self.buffer_capacity += multi_line_string_wkb_size(multi_line_string);
        }
        self.offsets_capacity += 1;
    }

    /// Add a MultiPolygon to this capacity counter.
    #[inline]
    pub fn add_multi_polygon<'a>(
        &mut self,
        multi_polygon: Option<&'a (impl MultiPolygonTrait<T = f64> + 'a)>,
    ) {
        if let Some(multi_polygon) = multi_polygon {
            self.buffer_capacity += multi_polygon_wkb_size(multi_polygon);
        }
        self.offsets_capacity += 1;
    }

    /// Add a Geometry to this capacity counter.
    #[inline]
    pub fn add_geometry<'a>(&mut self, geom: Option<&'a (impl GeometryTrait<T = f64> + 'a)>) {
        if let Some(geom) = geom {
            self.buffer_capacity += geometry_wkb_size(geom);
        }
        self.offsets_capacity += 1;
    }

    /// Add a GeometryCollection to this capacity counter.
    #[inline]
    pub fn add_geometry_collection<'a>(
        &mut self,
        geometry_collection: Option<&'a (impl GeometryCollectionTrait<T = f64> + 'a)>,
    ) {
        if let Some(geometry_collection) = geometry_collection {
            self.buffer_capacity += geometry_collection_wkb_size(geometry_collection);
        }
        self.offsets_capacity += 1;
    }

    /// Create a capacity counter from an iterator of Points.
    pub fn from_points<'a>(
        geoms: impl Iterator<Item = Option<&'a (impl PointTrait<T = f64> + 'a)>>,
    ) -> Self {
        let mut counter = Self::new_empty();
        for maybe_geom in geoms.into_iter() {
            counter.add_point(maybe_geom);
        }
        counter
    }

    /// Create a capacity counter from an iterator of LineStrings.
    pub fn from_line_strings<'a>(
        geoms: impl Iterator<Item = Option<&'a (impl LineStringTrait<T = f64> + 'a)>>,
    ) -> Self {
        let mut counter = Self::new_empty();
        for maybe_geom in geoms.into_iter() {
            counter.add_line_string(maybe_geom);
        }
        counter
    }

    /// Create a capacity counter from an iterator of Polygons.
    pub fn from_polygons<'a>(
        geoms: impl Iterator<Item = Option<&'a (impl PolygonTrait<T = f64> + 'a)>>,
    ) -> Self {
        let mut counter = Self::new_empty();
        for maybe_geom in geoms.into_iter() {
            counter.add_polygon(maybe_geom);
        }
        counter
    }

    /// Create a capacity counter from an iterator of MultiPoints.
    pub fn from_multi_points<'a>(
        geoms: impl Iterator<Item = Option<&'a (impl MultiPointTrait<T = f64> + 'a)>>,
    ) -> Self {
        let mut counter = Self::new_empty();
        for maybe_geom in geoms.into_iter() {
            counter.add_multi_point(maybe_geom);
        }
        counter
    }

    /// Create a capacity counter from an iterator of MultiLineStrings.
    pub fn from_multi_line_strings<'a>(
        geoms: impl Iterator<Item = Option<&'a (impl MultiLineStringTrait<T = f64> + 'a)>>,
    ) -> Self {
        let mut counter = Self::new_empty();
        for maybe_geom in geoms.into_iter() {
            counter.add_multi_line_string(maybe_geom);
        }
        counter
    }

    /// Create a capacity counter from an iterator of MultiPolygons.
    pub fn from_multi_polygons<'a>(
        geoms: impl Iterator<Item = Option<&'a (impl MultiPolygonTrait<T = f64> + 'a)>>,
    ) -> Self {
        let mut counter = Self::new_empty();
        for maybe_geom in geoms.into_iter() {
            counter.add_multi_polygon(maybe_geom);
        }
        counter
    }

    /// Create a capacity counter from an iterator of Geometries.
    pub fn from_geometries<'a>(
        geoms: impl Iterator<Item = Option<&'a (impl GeometryTrait<T = f64> + 'a)>>,
    ) -> Self {
        let mut counter = Self::new_empty();
        for maybe_geom in geoms.into_iter() {
            counter.add_geometry(maybe_geom);
        }
        counter
    }

    /// The number of bytes an array with this capacity would occupy.
    pub fn num_bytes<O: OffsetSizeTrait>(&self) -> usize {
        let offsets_byte_width = if O::IS_LARGE { 8 } else { 4 };
        let num_offsets = self.offsets_capacity;
        (offsets_byte_width * num_offsets) + self.buffer_capacity
    }
}

impl Default for WkbCapacity {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl Add for WkbCapacity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let buffer_capacity = self.buffer_capacity + rhs.buffer_capacity;
        let offsets_capacity = self.offsets_capacity + rhs.offsets_capacity;

        Self::new(buffer_capacity, offsets_capacity)
    }
}
