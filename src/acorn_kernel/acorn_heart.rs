// src/acorn_kernel/acorn_heart.rs

// ---------------------------- Heart of Light Acorn ----------------------------

/// Alias of function (alias is pseudonym of data type)
pub type AcornFunction = fn(); 

/// Location is group of functions
pub struct Location {
    pub functions: Vec<AcornFunction>,
}

/// Zone is group of Locations
pub struct Zone {
    pub locations: Vec<Location>,
}


// ---------------------------- Default behaviors ----------------------------

/// Create empty vector for Location
impl Default for Location {
    fn default() -> Self {
        Self { functions: vec![] }
    }
}

/// Create empty vector for Location
impl Default for Zone {
    fn default() -> Self {
        Self { locations: vec![] }
    }
}