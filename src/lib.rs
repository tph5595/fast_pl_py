use pyo3::prelude::*;
use rpls;

#[pyfunction]
fn pairs_to_landscape(bd_pairs: Vec<(f32, f32)>, k: usize, debug: bool) -> PyResult<Vec<Vec<(f32, f32)>>> {
    let bd_pairs = bd_pairs.into_iter().map(|pair| rpls::birthdeath::BirthDeath{birth: pair.0, death: pair.1}).collect();
    let landscapes = rpls::rpls::pairs_to_landscape(bd_pairs, k, debug);

    // Convert landscape to standard types
    Ok(landscapes
        .unwrap()
        .into_iter()
        .map(|landscape|
             landscape
             .into_iter()
             .map(|point| (point.x.0, point.y.0))
             .collect::<Vec<(f32, f32)>>()
             )
        .collect::<Vec<Vec<(f32, f32)>>>())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rpls_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pairs_to_landscape, m)?)?;
    Ok(())
}
