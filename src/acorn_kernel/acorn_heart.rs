// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Kernel
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_heart.rs
use bevy_ecs::prelude::*;
use crate::acorn_settings::{AcornZoneContext, AcornGlobalContext};

// ---------------------------- Heart of Light Acorn ----------------------------

/// Alias of function (alias is pseudonym of data type)
/// All functions should have World argument but not required use it
pub type AcornFunction = fn(&mut World, &mut AcornZoneContext, &mut AcornGlobalContext); 

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
    pub schedule: Schedule
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
            schedule: Schedule::default(),
        }
    }
}

// ---------------------------- Sugar macros ----------------------------

#[macro_export]
/// Example: 
/// ```
/// location! { 
///     function1, 
///     function2,
/// }
/// ```
/// This sugar macro is the same as:
/// ```
/// let mut temp_vec: Vec<AcornFunction> = vec![
///     function1 as AcornFunction,
///     function2 as AcornFunction,
/// ];
/// temp_vec.reverse();
/// Location::from_fn_vec(temp_vec)
/// ```
macro_rules! location {
    ($($func:expr),* $(,)?) => {
        {
            let mut temp_vec: Vec<$crate::acorn_kernel::acorn_heart::AcornFunction> = vec![ 
                $($func as $crate::acorn_kernel::acorn_heart::AcornFunction),* ];
            temp_vec.reverse();
            $crate::acorn_kernel::acorn_heart::Location::from_fn_vec(temp_vec)
        }
    };
}

#[macro_export]
/// Example: 
/// ```
/// zone! { 
///     location! {
///         function1, 
///         function2,
///     }, 
///     location! {
///         function1, 
///         function2,
///     },
/// }
/// ```
/// This sugar macro same like this:
/// ```
/// Zone::default()
/// .with_locations(vec![
///     Location::from_fn_vec(vec![
///         // functions
///     ]),
///     Location::from_fn_vec(vec![
///         // add own functions through comma 
///     ]),
///     // add own locations through comma 
/// ]);
/// ```
macro_rules! zone {
    ($($loc:expr),* $(,)?) => {
        $crate::acorn_kernel::acorn_heart::Zone::default()
            .with_locations(vec![
                $($loc),*
            ])
    };
}

#[macro_export]
/// Run a Zone in the `acorn_render.rs`. 
/// 
/// Example:
/// 
/// ```zone_run!(acorn_zone_context, your_zone_name, acorn_ecs, acorn_global_context);```
/// 
/// This sugar macro same like this:
/// 
/// ```
/// let len_before_2d_zone = acorn_zone_context.before_2d_zone.locations.len();
/// // locations go by order
/// for location_index in 0..len_before_2d_zone {
///     let fn_count = acorn_zone_context
///         .before_2d_zone
///         .locations[location_index]
///         .functions.len();
///     // Reverse cycle for protect from panic (101 errors) in runtime
///     // Functions go by reverse order
///     // Warning: You should add new functions from down to top
///     for fn_index in (0..fn_count).rev() {
///         let function = 
///         acorn_zone_context.before_2d_zone
///             .locations[location_index]
///             .functions[fn_index];        
///         /// Call function in strict order
///         function(&mut acorn_ecs.world, &mut acorn_zone_context, &mut acorn_global_context);
///     }
/// }
/// ```
macro_rules! zone_run {
    ($context:ident, $zone_field:ident, $ecs:ident, $global:ident) => {
        let __loc_len = $context.$zone_field.locations.len();
        
        for __l_idx in 0..__loc_len {
            let __fn_len = $context.$zone_field.locations[__l_idx].functions.len();
            
            for __f_idx in (0..__fn_len).rev() {
                let function = $context.$zone_field.locations[__l_idx].functions[__f_idx];
                
                function(
                    &mut $ecs.world, 
                    &mut $context, 
                    &mut $global
                );
            }
        }
    };
}