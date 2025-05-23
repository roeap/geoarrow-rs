use crate::io::geos::scalar::geometrycollection::to_geos_geometry_collection;
use crate::io::geos::scalar::linestring::to_geos_line_string;
use crate::io::geos::scalar::multilinestring::to_geos_multi_line_string;
use crate::io::geos::scalar::multipoint::to_geos_multi_point;
use crate::io::geos::scalar::multipolygon::to_geos_multi_polygon;
use crate::io::geos::scalar::point::to_geos_point;
use crate::io::geos::scalar::polygon::to_geos_polygon;
use crate::io::geos::scalar::{
    GEOSGeometryCollection, GEOSLineString, GEOSMultiLineString, GEOSMultiPoint, GEOSMultiPolygon,
    GEOSPoint, GEOSPolygon,
};
use crate::scalar::Geometry;
use geo_traits::{
    GeometryCollectionTrait, GeometryTrait, LineStringTrait, MultiLineStringTrait, MultiPointTrait,
    MultiPolygonTrait, PointTrait, PolygonTrait, UnimplementedLine, UnimplementedRect,
    UnimplementedTriangle,
};
use geos::Geom;

impl<'a> TryFrom<&'a Geometry<'_>> for geos::Geometry {
    type Error = geos::Error;

    fn try_from(value: &'a Geometry<'_>) -> std::result::Result<geos::Geometry, geos::Error> {
        match value {
            Geometry::Point(g) => g.try_into(),
            Geometry::LineString(g) => g.try_into(),
            Geometry::Polygon(g) => g.try_into(),
            Geometry::MultiPoint(g) => g.try_into(),
            Geometry::MultiLineString(g) => g.try_into(),
            Geometry::MultiPolygon(g) => g.try_into(),
            Geometry::GeometryCollection(g) => g.try_into(),
            Geometry::Rect(_g) => todo!(),
        }
    }
}

pub(crate) fn to_geos_geometry(
    geometry: &impl GeometryTrait<T = f64>,
) -> std::result::Result<geos::Geometry, geos::Error> {
    use geo_traits::GeometryType::*;

    match geometry.as_type() {
        Point(g) => to_geos_point(g),
        LineString(g) => to_geos_line_string(g),
        Polygon(g) => to_geos_polygon(g),
        MultiPoint(g) => to_geos_multi_point(g),
        MultiLineString(g) => to_geos_multi_line_string(g),
        MultiPolygon(g) => to_geos_multi_polygon(g),
        GeometryCollection(g) => to_geos_geometry_collection(g),
        Rect(_) => panic!("Unsupported rect in conversion to GEOS"),
        Triangle(_) => panic!("Unsupported triangle in conversion to GEOS"),
        Line(_) => panic!("Unsupported Line in conversion to GEOS"),
    }
}

#[derive(Clone)]
pub enum GEOSGeometry {
    Point(GEOSPoint),
    LineString(GEOSLineString),
    Polygon(GEOSPolygon),
    MultiPoint(GEOSMultiPoint),
    MultiLineString(GEOSMultiLineString),
    MultiPolygon(GEOSMultiPolygon),
    GeometryCollection(GEOSGeometryCollection),
}

impl GEOSGeometry {
    pub fn new(geom: geos::Geometry) -> Self {
        match geom.geometry_type() {
            geos::GeometryTypes::Point => Self::Point(GEOSPoint::new_unchecked(geom)),
            geos::GeometryTypes::LineString => {
                Self::LineString(GEOSLineString::new_unchecked(geom))
            }
            geos::GeometryTypes::Polygon => Self::Polygon(GEOSPolygon::new_unchecked(geom)),
            geos::GeometryTypes::MultiPoint => {
                Self::MultiPoint(GEOSMultiPoint::new_unchecked(geom))
            }
            geos::GeometryTypes::MultiLineString => {
                Self::MultiLineString(GEOSMultiLineString::new_unchecked(geom))
            }
            geos::GeometryTypes::MultiPolygon => {
                Self::MultiPolygon(GEOSMultiPolygon::new_unchecked(geom))
            }
            geos::GeometryTypes::GeometryCollection => {
                Self::GeometryCollection(GEOSGeometryCollection::new_unchecked(geom))
            }
            geos::GeometryTypes::LinearRing => panic!("GEOS Linear ring not supported"),
            geos::GeometryTypes::__Unknown(x) => panic!("Unknown geometry type {x}"),
        }
    }
}

impl GeometryTrait for GEOSGeometry {
    type T = f64;
    type PointType<'a> = GEOSPoint;
    type LineStringType<'a> = GEOSLineString;
    type PolygonType<'a> = GEOSPolygon;
    type MultiPointType<'a> = GEOSMultiPoint;
    type MultiLineStringType<'a> = GEOSMultiLineString;
    type MultiPolygonType<'a> = GEOSMultiPolygon;
    type GeometryCollectionType<'a> = GEOSGeometryCollection;
    type RectType<'a> = UnimplementedRect<f64>;
    type LineType<'a> = UnimplementedLine<f64>;
    type TriangleType<'a> = UnimplementedTriangle<f64>;

    fn dim(&self) -> geo_traits::Dimensions {
        match self {
            Self::Point(g) => g.dim(),
            Self::LineString(g) => g.dim(),
            Self::Polygon(g) => g.dim(),
            Self::MultiPoint(g) => g.dim(),
            Self::MultiLineString(g) => g.dim(),
            Self::MultiPolygon(g) => g.dim(),
            Self::GeometryCollection(g) => g.dim(),
        }
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        GEOSPoint,
        GEOSLineString,
        GEOSPolygon,
        GEOSMultiPoint,
        GEOSMultiLineString,
        GEOSMultiPolygon,
        GEOSGeometryCollection,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        match self {
            Self::Point(g) => geo_traits::GeometryType::Point(g),
            Self::LineString(g) => geo_traits::GeometryType::LineString(g),
            Self::Polygon(g) => geo_traits::GeometryType::Polygon(g),
            Self::MultiPoint(g) => geo_traits::GeometryType::MultiPoint(g),
            Self::MultiLineString(g) => geo_traits::GeometryType::MultiLineString(g),
            Self::MultiPolygon(g) => geo_traits::GeometryType::MultiPolygon(g),
            Self::GeometryCollection(g) => geo_traits::GeometryType::GeometryCollection(g),
        }
    }
}
