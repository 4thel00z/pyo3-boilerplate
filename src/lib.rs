use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn boilerplate_fun(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn boilerplate(py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_wrapped(wrap_pyfunction!(boilerplate_fun))?;
    Ok(())
}
