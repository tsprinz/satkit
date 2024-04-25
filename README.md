# Satellite Toolkit with Rust

An accurate, high-performance satellite orbital kinematics toolkit, written in Rust with a sensible interface.
<br/>
Also includes python bindings for *all* functions via via [pyo3](https://pyo3.rs/)

### Github

![Build Passing?](https://github.com/ssmichael1/satkit/actions/workflows/build.yml/badge.svg)
![Wheel Passing?](https://github.com/ssmichael1/satkit/actions/workflows/wheels.yml/badge.svg)
![GitHub License](https://img.shields.io/github/license/ssmichael1/satkit)

### Crates.io

![Crates.io Version](https://img.shields.io/crates/v/satkit)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/satkit)

### PyPi

![PyPI - Version](https://img.shields.io/pypi/v/satkit)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/satkit)
![PyPI - Implementation](https://img.shields.io/pypi/implementation/satkit)
![PyPI - Status](https://img.shields.io/pypi/status/satkit)


![Read the Docs](https://img.shields.io/readthedocs/satellite-toolkit)

## Language Bindings

- **Native Rust** bindings
- **Python bindings** for compiled rust code ... speed of Rust with convenience of Python<br/>
  Install with `pip install satkit`<br/>
  PyPi includes binary packages for windows, macos (Intel & ARM), and linux.  Python documentation is at: <https://satellite-toolkit.readthedocs.io/en/latest/>

## Features

- High-precision coordinate transforms between:
  - International Terrestrial Reference Frame (ITRF)
  - Geocentric Celestial Reference Frame (GCRF) using IAU-2000 reduction
  - True-Equinox Mean Equator (TEME) frame used in SGP4 propagation of TLEs
  - Celestial Intermediate Reference Frame (CIRF)
  - Terrestrial Intermediate Reference Frame (TIRF)
  - Terrestrial Geodetic frame (latitude, longitude)
- Geodesic distances
- SGP4, and Keplerian orbit propagation
- JPL high-precision planetary ephemerides
- High-order gravity models
- High-precision, high-speed numerical satellite orbit propagation with high-order efficient Runga-Kutta solvers, ability to solve for state transition matrix, and inclusion following forces:
  - High-order Earth gravity with multiple models
  - Solar gravity
  - Lunar gravity
  - Drag (NRL MISE-00 density model)
  - Radiation pressure

## ODE Solvers

The high-precision numerical satellite orbit propagation makes use of standard Runga-Kutta methods for integration of ordinary differential equations. The ODE solver is included as part of the library.

The methods use Runga-Kutta pairs for ODE integration and error estimation generated by Jim Verner: [https://www.sfu.ca/~jverner/](https://www.sfu.ca/~jverner/)

## References, Models, and External Software.

### The equations and many of the unit tests underlying this work are drawn from the following sources:

- **"Fundamentals of Astrodynamics and Applications, Fourth Edition"**, D. Vallado, Microcosm Press and Springer, 2013.<br>
  [https://celestrak.org/software/vallado-sw.php](https://celestrak.org/software/vallado-sw.php)
- **"Satellite Orbits: Models, Methods, Applications"**, O. Montenbruck and E. Gill, Springer, 2000.<br>
  [https://doi.org/10.1007/978-3-642-58351-3](https://doi.org/10.1007/978-3-642-58351-3)

### This code makes reference to and relies on models generated by the following:

- **SGP4 Orbit Propagator** - https://celestrak.org/software/tskelso-sw.php<br/>
  NORAD / SGP4 orbit propagator used to generate position and velocity states from orbital ephemerides described by Two-Line Element Sets (TLEs). This code base includes a pure-rust translation of the SGP4 orbit propagator
- **NRL MSISE-00 Density Model** - https://ccmc.gsfc.nasa.gov/models/NRLMSIS~00/<br/>
  NRL model of air density, including density at high altitudes, used in to compute satellite drag
- **Gravity Models** - http://icgem.gfz-potsdam.de/home<br/>
  International Center for Global Earth Models (ICEGM), collection and archive in a common format of all existing global gravity field models
- **Space Weather** - https://celestrak.org/SpaceData/<br/>
  Space weather used to modulate the air density used in drag calculations
- **Earth Orientation Parameters** - https://celestrak.org/SpaceData/<br/>
  Time-varying Earth orientation parameters used for time epoch conversions and high-precision rotations between the inertial and Earth-fixed coordinate frames
- **IERS Conventions** - https://www.iers.org/IERS/EN/Publications/TechnicalNotes/tn36.html<br/>
  International Earth Rotation and Reference Systems Service Technical Note 36 for rotation between inertial and Earth-fixed coordinate systems.

## Verification

The code includes rust test modules and python test modules for verification of nearly calculations, including but not limited to:

- **JPL Ephemeris** - Via JPL-provided test vectors for Chebychev polynomial calculation
- **SGP4** - Via SGP4 test vectors provided with original C<sup>++</sup> distribution

## Author

Steven Michael (ssmichael@gmail.com)

Please reach out of you find errors in code or calculations, are interested in contributing to this repository, or have suggestions for improvements to the API.
