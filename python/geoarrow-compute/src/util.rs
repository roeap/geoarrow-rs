use std::sync::Arc;

use geoarrow::NativeArray;
use geoarrow::array::NativeArrayDyn;
use geoarrow::chunked_array::ChunkedNativeArray;
use geoarrow::error::GeoArrowError;
use pyo3::prelude::*;
use pyo3_arrow::{PyArray, PyChunkedArray, PyTable};
use pyo3_geoarrow::{PyChunkedNativeArray, PyGeoArrowResult, PyNativeArray};

pub(crate) fn table_to_pytable(table: geoarrow::table::Table) -> PyTable {
    let (batches, schema) = table.into_inner();
    PyTable::try_new(batches, schema).unwrap()
}

pub(crate) fn pytable_to_table(table: PyTable) -> Result<geoarrow::table::Table, GeoArrowError> {
    let (batches, schema) = table.into_inner();
    geoarrow::table::Table::try_new(batches, schema)
}

pub(crate) fn return_geometry_array(
    py: Python,
    arr: Arc<dyn NativeArray>,
) -> PyGeoArrowResult<PyObject> {
    Ok(PyNativeArray::new(NativeArrayDyn::new(arr))
        .to_geoarrow(py)?
        .unbind())
}

pub(crate) fn return_chunked_geometry_array(
    py: Python,
    arr: Arc<dyn ChunkedNativeArray>,
) -> PyGeoArrowResult<PyObject> {
    Ok(PyChunkedNativeArray::new(arr).to_geoarrow(py)?.unbind())
}

pub(crate) fn return_array(py: Python, arr: PyArray) -> PyGeoArrowResult<PyObject> {
    Ok(arr.to_arro3(py)?.unbind())
}

pub(crate) fn return_chunked_array(py: Python, arr: PyChunkedArray) -> PyGeoArrowResult<PyObject> {
    Ok(arr.to_arro3(py)?.unbind())
}
