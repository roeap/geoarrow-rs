use crate::ArrayBase;
use crate::io::geozero::ToGeometryArray;
use crate::io::geozero::scalar::geometry_collection::process_geometry_collection;
use crate::io::geozero::scalar::linestring::process_line_string;
use crate::io::geozero::scalar::multilinestring::process_multi_line_string;
use crate::io::geozero::scalar::multipoint::process_multi_point;
use crate::io::geozero::scalar::multipolygon::process_multi_polygon;
use crate::io::geozero::scalar::point::process_point;
use crate::io::geozero::scalar::polygon::process_polygon;
use crate::scalar::{Geometry, OwnedGeometry};
use crate::trait_::ArrayAccessor;
use arrow_array::OffsetSizeTrait;
use geo_traits::{GeometryTrait, GeometryType};
use geozero::{GeomProcessor, GeozeroGeometry};

pub(crate) fn process_geometry<P: GeomProcessor>(
    geom: &impl GeometryTrait<T = f64>,
    geom_idx: usize,
    processor: &mut P,
) -> geozero::error::Result<()> {
    use GeometryType::*;

    match geom.as_type() {
        Point(g) => process_point(g, geom_idx, processor)?,
        LineString(g) => process_line_string(g, geom_idx, processor)?,
        Polygon(g) => process_polygon(g, true, geom_idx, processor)?,
        MultiPoint(g) => process_multi_point(g, geom_idx, processor)?,
        MultiLineString(g) => process_multi_line_string(g, geom_idx, processor)?,
        MultiPolygon(g) => process_multi_polygon(g, geom_idx, processor)?,
        GeometryCollection(g) => process_geometry_collection(g, geom_idx, processor)?,
        Rect(_g) => todo!(),
        Triangle(_) | Line(_) => todo!(),
    };

    Ok(())
}

impl GeozeroGeometry for Geometry<'_> {
    fn process_geom<P: GeomProcessor>(&self, processor: &mut P) -> geozero::error::Result<()>
    where
        Self: Sized,
    {
        process_geometry(&self, 0, processor)
    }
}

/// Convert a geozero scalar data source to an [OwnedGeometry].
pub trait ToGeometry<O: OffsetSizeTrait> {
    /// Convert a geozero scalar data source to an [OwnedGeometry].
    fn to_geometry(&self) -> geozero::error::Result<OwnedGeometry>;
}

impl<T: GeozeroGeometry, O: OffsetSizeTrait> ToGeometry<O> for T {
    fn to_geometry(&self) -> geozero::error::Result<OwnedGeometry> {
        let arr = self.to_geometry_array()?;
        assert_eq!(arr.len(), 1);
        Ok(OwnedGeometry::from(arr.value(0)))
    }
}
