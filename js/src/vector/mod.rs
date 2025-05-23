use wasm_bindgen::prelude::*;

macro_rules! impl_vector {
    (
        $(#[$($attrss:meta)*])*
        pub struct $struct_name:ident(pub(crate) $geoarrow_arr:ty);
    ) => {
        $(#[$($attrss)*])*
        #[wasm_bindgen]
        pub struct $struct_name(pub(crate) $geoarrow_arr);

        impl From<$geoarrow_arr> for $struct_name {
            fn from(value: $geoarrow_arr) -> Self {
                Self(value)
            }
        }

        impl From<$struct_name> for $geoarrow_arr {
            fn from(value: $struct_name) -> Self {
                value.0
            }
        }
    };
}

impl_vector! {
    /// An immutable chunked array of Point geometries in WebAssembly memory using GeoArrow's
    /// in-memory representation.
    pub struct PointVector(pub(crate) geoarrow::chunked_array::ChunkedPointArray);
}
impl_vector! {
    /// An immutable chunked array of LineString geometries in WebAssembly memory using GeoArrow's
    /// in-memory representation.
    pub struct LineStringVector(pub(crate) geoarrow::chunked_array::ChunkedLineStringArray);
}
impl_vector! {
    /// An immutable chunked array of Polygon geometries in WebAssembly memory using GeoArrow's
    /// in-memory representation.
    pub struct PolygonVector(pub(crate) geoarrow::chunked_array::ChunkedPolygonArray);
}
impl_vector! {
    /// An immutable chunked array of MultiPoint geometries in WebAssembly memory using GeoArrow's
    /// in-memory representation.
    pub struct MultiPointVector(pub(crate) geoarrow::chunked_array::ChunkedMultiPointArray);
}
impl_vector! {
    /// An immutable chunked array of MultiLineString geometries in WebAssembly memory using
    /// GeoArrow's in-memory representation.
    pub struct MultiLineStringVector(pub(crate) geoarrow::chunked_array::ChunkedMultiLineStringArray);
}
impl_vector! {
    /// An immutable chunked array of MultiPolygon geometries in WebAssembly memory using
    /// GeoArrow's in-memory representation.
    pub struct MultiPolygonVector(pub(crate) geoarrow::chunked_array::ChunkedMultiPolygonArray);
}
impl_vector! {
    /// An immutable chunked array of GeometryCollection geometries in WebAssembly memory using
    /// GeoArrow's in-memory representation.
    pub struct GeometryCollectionVector(pub(crate) geoarrow::chunked_array::ChunkedGeometryCollectionArray);
}
impl_vector! {
    /// An immutable chunked array of WKB-encoded geometries in WebAssembly memory using GeoArrow's
    /// in-memory representation.
    pub struct WKBVector(pub(crate) geoarrow::chunked_array::ChunkedWKBArray<i32>);
}
impl_vector! {
    /// An immutable chunked array of Rect geometries in WebAssembly memory using GeoArrow's
    /// in-memory representation.
    pub struct RectVector(pub(crate) geoarrow::chunked_array::ChunkedRectArray);
}
