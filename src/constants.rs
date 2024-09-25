// Constants
// Potentially, shift to its own crate if no other crate exists.

/// Standard atmospheric pressure. 101,325 Pascals (newtons per square metre).
pub const STANDARD_ATMOSPHERE_PRESSURE: f32 = 101_325.;

/// Standard (room) temperature.
///
/// Source: [NIST](https://www.nist.gov/pml/owm/si-units-temperature)
pub const STANDARD_TEMPERATURE_NIST: f32 = 293.15;

/// Ideal gas constant.
///
/// Source: [LibreText](https://chem.libretexts.org/Bookshelves/Physical_and_Theoretical_Chemistry_Textbook_Maps/Supplemental_Modules_(Physical_and_Theoretical_Chemistry)/Physical_Properties_of_Matter/States_of_Matter/Properties_of_Gases/Gas_Laws/The_Ideal_Gas_Law)
pub const IDEAL_GAS_CONSTANT: f32 = 8.3145;

// Light
/// Speed of light in a vacuum.
///
/// In metres per second.
///
/// Source: [The Physics Hypertext](https://physics.info/velocity/)
pub const SPEED_OF_LIGHT_VACUUM: f32 = 299_792_458.0; // May change it to i32. i32 converts to f32 easily.

// Mass
/// Standard kilogram.
pub const STANDARD_KILOGRAM: f32 = 1.0;

// Gravity
/// Gravitational Force constant of Earth.[NSSDC](https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html)
pub const STANDARD_GRAVITY_EARTH: f32 = 9.80665; //Change to a negative value if more useful.
