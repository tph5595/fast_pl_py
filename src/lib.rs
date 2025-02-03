#![warn(
     clippy::all,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use pyo3::prelude::*;
use fast_pl;

fn convert_bd_pairs(bd_pairs: Vec<(f64, f64)>) -> Vec<fast_pl::birthdeath::BirthDeath>{
    bd_pairs.into_iter().map(|pair| fast_pl::birthdeath::BirthDeath{birth: pair.0, death: pair.1}).collect()
}

// fn convert_landscape(landscapes: Vec<Vec<fast_pl::persistencelandscape::PointOrd>>) -> Vec<Vec<(f64, f64)>>{
//     landscapes
//         .into_iter()
//         .map(|landscape|
//              landscape
//              .into_iter()
//              .map(|point| (point.x.0, point.y.0))
//              .collect::<Vec<(f64, f64)>>()
//              )
//         .collect::<Vec<Vec<(f64, f64)>>>()
// }

#[pyfunction]
fn pairs_to_landscape(bd_pairs: Vec<(f64, f64)>, k: usize, debug: bool, disable_filter: bool) -> PyResult<Vec<Vec<(f64, f64)>>> {
    let bd_pairs = convert_bd_pairs(bd_pairs);
    let landscapes = fast_pl::rpls::pairs_to_landscape(bd_pairs, k, debug, disable_filter).unwrap();

    Ok(landscapes)

}

#[pyfunction]
fn pairs_to_l2_norm(bd_pairs: Vec<(f64, f64)>, k: usize, debug: bool, disable_filter: bool) -> PyResult<f64> {
    let bd_pairs = convert_bd_pairs(bd_pairs);
    Ok(fast_pl::rpls::pairs_to_l2_norm(bd_pairs, k, debug, disable_filter).unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rpls_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pairs_to_landscape, m)?)?;
    m.add_function(wrap_pyfunction!(pairs_to_l2_norm, m)?)?;
    Ok(())
}
