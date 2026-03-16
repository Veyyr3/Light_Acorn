// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Kernel
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_heart.rs
use bevy_ecs::prelude::*;
use crate::acorn_settings::AcornContext;

// ---------------------------- Heart of Light Acorn ----------------------------

/// Alias of function (alias is pseudonym of data type)
/// All functions should have World argument but not required use it
pub type AcornFunction = fn(&mut World, &mut AcornContext); 

/// Location is group of functions
pub struct Location {
    pub functions: Vec<AcornFunction>,
}

/// Zone is group of Locations
pub struct Zone {
    pub locations: Vec<Location>,
}

/// Acorn ECS based on Bevy ECS. 
/// Use this in fn main to create ECS.
pub struct AcornECS {
    pub world: World,
    // pub schedule: Schedule
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

    /// Creating Location from vector of functions 
    pub fn from_fn_vec(functions: Vec<AcornFunction>) -> Self {
        Self { functions }
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

    /// Creating Zone from vector of Locations
    pub fn with_locations(mut self, locations: Vec<Location>) -> Self {
        self.locations = locations;
        self
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

/// Default behavior for AcornECS
impl Default for AcornECS {
    fn default() -> Self {
        Self {
            world: World::new(),
            //schedule: Schedule::default(),
        }
    }
}