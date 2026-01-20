mod polynomial;
mod roots;
mod tests;
use pyo3::prelude::*;

#[pymodule]
fn root_finder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(roots::companion, m)?)?;
    // m.add_function(wrap_pyfunction!(roots::newton_raphson, m)?)?;
    Ok(())
}
