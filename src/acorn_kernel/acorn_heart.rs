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

// ---------------------------- Implementations ----------------------------

impl Location {
    /// Create Location
    pub fn new() -> Self {
        Self {
            functions: vec![],
        }
    }

    /// Add function into Location
    pub fn add(mut self, function: AcornFunction) -> Self {
        self.functions.push(function);
        self
    }
}

impl Zone {
    /// Create Zone
    pub fn new() -> Self {
        Self {
            locations: vec![],
        }
    }

    /// Add Location into Zone
    pub fn add(&mut self, location: Location) {
        self.locations.push(location);
    }
}

// ---------------------------- Default behaviors ----------------------------

/// Default behavior for Location
impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

/// Default behavior for Zone
impl Default for Zone {
    fn default() -> Self {
        Self::new()
    }
}