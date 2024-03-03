use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyTuple};
use pyo3::wrap_pyfunction;

use crate::nrlmsise;
use crate::AstroTime;

use super::PyAstroTime;
use super::PyITRFCoord;

///
/// NRL MSISE-00 Density Model
///
/// Inputs:
///
///   ITRFCoord:   coord at which to compute density & temperature
///        Time:   Optional satkit.time object
///                representing instant at which to compute
///                density & temperature using space weather
///                data file as input to model
///
/// 2nd option for inputs:
///
///    altitude:   altitude in meters,
///    Latitude:   Optional latitude in radians
///   Longitude:   Optional longitude in radians
///        Time:   Optional satkit.time object
///                representing instant at which to compute
///                density & temperature using space weather
///                data file as input to model
///
/// Outputs:
///    
///    Tuple:   (rho, T) where
///   
///       rho: Atmosphere mass density in kg / m^3
///         T: Temperature in Kelvin
///
#[pyfunction(name = "nrlmsise")]
#[pyo3(signature=(*args))]
fn pynrlmsise(args: &PyTuple) -> PyResult<(f64, f64)> {
    if args.len() == 0 {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "Invalid number of arguments",
        ));
    }
    let time: Option<AstroTime> = {
        if args[args.len() - 1].is_instance_of::<PyAstroTime>() {
            Some(args[args.len() - 1].extract::<PyAstroTime>().unwrap().inner)
        } else {
            None
        }
    };
    if args[0].is_instance_of::<PyITRFCoord>() {
        let itrf = args[0].extract::<PyITRFCoord>().unwrap().inner;
        Ok(nrlmsise::nrlmsise(
            itrf.hae() / 1.0e3,
            Some(itrf.latitude_rad()),
            Some(itrf.longitude_rad()),
            time,
            true,
        ))
    } else if args[0].is_instance_of::<PyFloat>() {
        let altitude = args[0].extract::<f64>().unwrap();
        let latitude: Option<f64> = {
            if args.len() > 1 && args[1].is_instance_of::<PyFloat>() {
                Some(args[1].extract::<f64>().unwrap())
            } else {
                None
            }
        };
        let longitude: Option<f64> = {
            if args.len() > 2 && args[2].is_instance_of::<PyFloat>() {
                Some(args[2].extract::<f64>().unwrap())
            } else {
                None
            }
        };
        Ok(nrlmsise::nrlmsise(
            altitude / 1.0e3,
            latitude,
            longitude,
            time,
            true,
        ))
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err("Invalid arguments"));
    }
}

#[pymodule]
pub fn density(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pynrlmsise, m)?).unwrap();
    Ok(())
}
