// Constants
// Potentially, shift to its own crate if no other crate exists.

/// Standard atmospheric pressure. 101,325 Pascals (newtons per square metre).
pub const STANDARD_ATMOSPHERE_PRESSURE: f32 = 101_325.;

/// Standard (room) temperature. [NIST](https://www.nist.gov/pml/owm/si-units-temperature)
pub const STANDARD_TEMPERATURE_NIST: f32 = 293.15;

/// Ideal gas constant. [LibreText](https://chem.libretexts.org/Bookshelves/Physical_and_Theoretical_Chemistry_Textbook_Maps/Supplemental_Modules_(Physical_and_Theoretical_Chemistry)/Physical_Properties_of_Matter/States_of_Matter/Properties_of_Gases/Gas_Laws/The_Ideal_Gas_Law)
pub const IDEAL_GAS_CONSTANT: f32 = 8.3145;
