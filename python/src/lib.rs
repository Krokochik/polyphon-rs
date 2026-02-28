use pyo3::prelude::*;
use polyphon_core::encode as en;

#[pyfunction]
fn encode(s: &str) -> PyResult<String> {
    Ok(en(s))
}

#[pymodule]
fn polyphon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    Ok(())
}