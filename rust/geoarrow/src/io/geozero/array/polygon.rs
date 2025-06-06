use crate::ArrayBase;
use crate::array::polygon::PolygonCapacity;
use crate::array::{PolygonArray, PolygonBuilder};
use crate::io::geozero::scalar::process_polygon;
use crate::trait_::ArrayAccessor;
use geoarrow_schema::Dimension;
use geozero::{GeomProcessor, GeozeroGeometry};

impl GeozeroGeometry for PolygonArray {
    fn process_geom<P: GeomProcessor>(&self, processor: &mut P) -> geozero::error::Result<()>
    where
        Self: Sized,
    {
        let num_geometries = self.len();
        processor.geometrycollection_begin(num_geometries, 0)?;

        for geom_idx in 0..num_geometries {
            process_polygon(&self.value(geom_idx), true, geom_idx, processor)?;
        }

        processor.geometrycollection_end(num_geometries - 1)?;
        Ok(())
    }
}

/// GeoZero trait to convert to GeoArrow PolygonArray.
pub trait ToPolygonArray {
    /// Convert to GeoArrow PolygonArray
    fn to_line_string_array(&self, dim: Dimension) -> geozero::error::Result<PolygonArray>;

    /// Convert to a GeoArrow PolygonBuilder
    fn to_line_string_builder(&self, dim: Dimension) -> geozero::error::Result<PolygonBuilder>;
}

impl<T: GeozeroGeometry> ToPolygonArray for T {
    fn to_line_string_array(&self, dim: Dimension) -> geozero::error::Result<PolygonArray> {
        Ok(self.to_line_string_builder(dim)?.into())
    }

    fn to_line_string_builder(&self, dim: Dimension) -> geozero::error::Result<PolygonBuilder> {
        let mut mutable_array = PolygonBuilder::new(dim);
        self.process_geom(&mut mutable_array)?;
        Ok(mutable_array)
    }
}

#[allow(unused_variables)]
impl GeomProcessor for PolygonBuilder {
    fn geometrycollection_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        // reserve `size` geometries
        let capacity = PolygonCapacity::new(0, 0, size);
        self.reserve(capacity);
        Ok(())
    }

    fn geometrycollection_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        // self.shrink_to_fit()
        Ok(())
    }

    fn xy(&mut self, x: f64, y: f64, idx: usize) -> geozero::error::Result<()> {
        // # Safety:
        // This upholds invariants because we call try_push_length in multipoint_begin to ensure
        // offset arrays are correct.
        unsafe { self.push_coord(&geo::Coord { x, y }).unwrap() }
        Ok(())
    }

    // Here, size is the number of rings in the polygon
    fn polygon_begin(
        &mut self,
        tagged: bool,
        size: usize,
        idx: usize,
    ) -> geozero::error::Result<()> {
        // reserve `size` rings
        let capacity = PolygonCapacity::new(0, size, 0);
        self.reserve(capacity);

        // # Safety:
        // This upholds invariants because we separately update the ring offsets in
        // linestring_begin
        unsafe { self.try_push_geom_offset(size).unwrap() }
        Ok(())
    }

    fn linestring_begin(
        &mut self,
        tagged: bool,
        size: usize,
        idx: usize,
    ) -> geozero::error::Result<()> {
        // reserve `size` coordinates
        let capacity = PolygonCapacity::new(size, 0, 0);
        self.reserve(capacity);

        // # Safety:
        // This upholds invariants because we separately update the geometry offsets in
        // polygon_begin
        unsafe { self.try_push_ring_offset(size).unwrap() }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::polygon::{p0, p1};
    use crate::trait_::ArrayAccessor;
    use geo::Geometry;
    use geozero::ToWkt;
    use geozero::error::Result;

    #[test]
    fn geozero_process_geom() -> geozero::error::Result<()> {
        let arr: PolygonArray = (vec![p0(), p1()].as_slice(), Dimension::XY).into();
        let wkt = arr.to_wkt()?;
        let expected = "GEOMETRYCOLLECTION(POLYGON((-111 45,-111 41,-104 41,-104 45,-111 45)),POLYGON((-111 45,-111 41,-104 41,-104 45,-111 45),(-110 44,-110 42,-105 42,-105 44,-110 44)))";
        assert_eq!(wkt, expected);
        Ok(())
    }

    #[test]
    fn from_geozero() -> Result<()> {
        let geo = Geometry::GeometryCollection(
            vec![p0(), p1()]
                .into_iter()
                .map(Geometry::Polygon)
                .collect(),
        );
        let multi_point_array = geo.to_line_string_array(Dimension::XY).unwrap();
        assert_eq!(multi_point_array.value_as_geo(0), p0());
        assert_eq!(multi_point_array.value_as_geo(1), p1());
        Ok(())
    }
}
