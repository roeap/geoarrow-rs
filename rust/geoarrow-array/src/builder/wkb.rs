use arrow_array::OffsetSizeTrait;
use arrow_array::builder::GenericBinaryBuilder;
use geo_traits::{
    GeometryCollectionTrait, GeometryTrait, LineStringTrait, MultiLineStringTrait, MultiPointTrait,
    MultiPolygonTrait, PointTrait, PolygonTrait,
};
use geoarrow_schema::WkbType;
use wkb::Endianness;
use wkb::writer::{
    write_geometry, write_geometry_collection, write_line_string, write_multi_line_string,
    write_multi_point, write_multi_polygon, write_point, write_polygon,
};

use crate::array::WkbArray;
use crate::capacity::WkbCapacity;

/// The GeoArrow equivalent to `Vec<Option<Wkb>>`: a mutable collection of Wkb buffers.
///
/// Converting a [`WkbBuilder`] into a [`WkbArray`] is `O(1)`.
#[derive(Debug)]
pub struct WkbBuilder<O: OffsetSizeTrait>(GenericBinaryBuilder<O>, WkbType);

impl<O: OffsetSizeTrait> WkbBuilder<O> {
    /// Creates a new empty [`WkbBuilder`].
    pub fn new(typ: WkbType) -> Self {
        Self::with_capacity(typ, Default::default())
    }

    /// Initializes a new [`WkbBuilder`] with a pre-allocated capacity of slots and values.
    pub fn with_capacity(typ: WkbType, capacity: WkbCapacity) -> Self {
        Self(
            GenericBinaryBuilder::with_capacity(
                capacity.offsets_capacity,
                capacity.buffer_capacity,
            ),
            typ,
        )
    }

    /// Creates a new empty [`WkbBuilder`] with a capacity inferred by the provided geometry
    /// iterator.
    pub fn with_capacity_from_iter<'a>(
        geoms: impl Iterator<Item = Option<&'a (impl GeometryTrait<T = f64> + 'a)>>,
        typ: WkbType,
    ) -> Self {
        let counter = WkbCapacity::from_geometries(geoms);
        Self::with_capacity(typ, counter)
    }

    // Upstream APIs don't exist for this yet. To implement this without upstream changes, we could
    // change to using manual `Vec`'s ourselves
    // pub fn reserve(&mut self, capacity: WkbCapacity) {
    // }

    /// Push a Point onto the end of this builder
    #[inline]
    pub fn push_point(&mut self, geom: Option<&impl PointTrait<T = f64>>) {
        if let Some(geom) = geom {
            write_point(&mut self.0, geom, Endianness::LittleEndian).unwrap();
            self.0.append_value("")
        } else {
            self.0.append_null();
        }
    }

    /// Push a LineString onto the end of this builder
    #[inline]
    pub fn push_line_string(&mut self, geom: Option<&impl LineStringTrait<T = f64>>) {
        if let Some(geom) = geom {
            write_line_string(&mut self.0, geom, Endianness::LittleEndian).unwrap();
            self.0.append_value("")
        } else {
            self.0.append_null()
        }
    }

    /// Push a Polygon onto the end of this builder
    #[inline]
    pub fn push_polygon(&mut self, geom: Option<&impl PolygonTrait<T = f64>>) {
        if let Some(geom) = geom {
            write_polygon(&mut self.0, geom, Endianness::LittleEndian).unwrap();
            self.0.append_value("")
        } else {
            self.0.append_null()
        }
    }

    /// Push a MultiPoint onto the end of this builder
    #[inline]
    pub fn push_multi_point(&mut self, geom: Option<&impl MultiPointTrait<T = f64>>) {
        if let Some(geom) = geom {
            write_multi_point(&mut self.0, geom, Endianness::LittleEndian).unwrap();
            self.0.append_value("")
        } else {
            self.0.append_null()
        }
    }

    /// Push a MultiLineString onto the end of this builder
    #[inline]
    pub fn push_multi_line_string(&mut self, geom: Option<&impl MultiLineStringTrait<T = f64>>) {
        if let Some(geom) = geom {
            write_multi_line_string(&mut self.0, geom, Endianness::LittleEndian).unwrap();
            self.0.append_value("")
        } else {
            self.0.append_null()
        }
    }

    /// Push a MultiPolygon onto the end of this builder
    #[inline]
    pub fn push_multi_polygon(&mut self, geom: Option<&impl MultiPolygonTrait<T = f64>>) {
        if let Some(geom) = geom {
            write_multi_polygon(&mut self.0, geom, Endianness::LittleEndian).unwrap();
            self.0.append_value("")
        } else {
            self.0.append_null()
        }
    }

    /// Push a Geometry onto the end of this builder
    #[inline]
    pub fn push_geometry(&mut self, geom: Option<&impl GeometryTrait<T = f64>>) {
        if let Some(geom) = geom {
            write_geometry(&mut self.0, geom, Endianness::LittleEndian).unwrap();
            self.0.append_value("")
        } else {
            self.0.append_null()
        }
    }

    /// Push a GeometryCollection onto the end of this builder
    #[inline]
    pub fn push_geometry_collection(
        &mut self,
        geom: Option<&impl GeometryCollectionTrait<T = f64>>,
    ) {
        if let Some(geom) = geom {
            write_geometry_collection(&mut self.0, geom, Endianness::LittleEndian).unwrap();
            self.0.append_value("")
        } else {
            self.0.append_null()
        }
    }

    /// Extend this builder from an iterator of Geometries.
    pub fn extend_from_iter<'a>(
        &mut self,
        geoms: impl Iterator<Item = Option<&'a (impl GeometryTrait<T = f64> + 'a)>>,
    ) {
        geoms
            .into_iter()
            .for_each(|maybe_geom| self.push_geometry(maybe_geom));
    }

    /// Create this builder from a slice of Geometries.
    pub fn from_geometries(geoms: &[impl GeometryTrait<T = f64>], typ: WkbType) -> Self {
        let mut array = Self::with_capacity_from_iter(geoms.iter().map(Some), typ);
        array.extend_from_iter(geoms.iter().map(Some));
        array
    }

    /// Create this builder from a slice of nullable Geometries.
    pub fn from_nullable_geometries(
        geoms: &[Option<impl GeometryTrait<T = f64>>],
        typ: WkbType,
    ) -> Self {
        let mut array = Self::with_capacity_from_iter(geoms.iter().map(|x| x.as_ref()), typ);
        array.extend_from_iter(geoms.iter().map(|x| x.as_ref()));
        array
    }

    /// Consume this builder and convert to a [WkbArray].
    ///
    /// This is `O(1)`.
    pub fn finish(mut self) -> WkbArray<O> {
        WkbArray::new(self.0.finish(), self.1.metadata().clone())
    }
}
