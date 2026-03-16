// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Kernel
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_render.rs
use macroquad::prelude::*;
use crate::acorn_kernel::{
    acorn_heart::AcornECS, 
};
use crate::acorn_settings::AcornContext;

/// Main loop of Light Acorn.
/// You shouldn't touch this. 
/// 
/// Warning: If you want to add new Zones you should touch this (read in docs about this).
pub async fn acorn_loop(mut acorn_context: AcornContext, mut acorn_ecs: AcornECS) {
    loop {
        clear_background(BLACK);

        // Run Schedule
        //acorn_ecs.schedule.run(&mut acorn_ecs.world);

        // before_2d_zone_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
        let len_before_2d_zone = acorn_context.before_2d_zone.locations.len();

        // locations go by order
        for location_index in 0..len_before_2d_zone {
            let fn_count = acorn_context
                .before_2d_zone
                .locations[location_index]
                .functions.len();

            // Reverse cycle for protect from panic (101 errors) in runtime
            // Functions go by reverse order
            // Warning: You should add new functions from down to top
            for fn_index in (0..fn_count).rev() {
                let function = 
                acorn_context.before_2d_zone
                    .locations[location_index]
                    .functions[fn_index];
                    
                // Call function in strict order
                function(&mut acorn_ecs.world, &mut acorn_context);
            }
        }

        // ---------------------------- Turn on 2D render ----------------------------
        set_default_camera(); 

        // after_2d_zone (Ex: UI draw and other Locations)
        let len_after_2d_zone = acorn_context.after_2d_zone.locations.len();

        // locations go by order
        for location_index in 0..len_after_2d_zone {
            let fn_count = acorn_context
                .after_2d_zone
                .locations[location_index]
                .functions.len();

            // Reverse cycle for protect from panic (101 errors) in runtime
            // Functions go by reverse order
            // Warning: You should add new functions from down to top
            for fn_index in (0..fn_count).rev() {
                let function = 
                acorn_context.after_2d_zone
                    .locations[location_index]
                    .functions[fn_index];
                    
                // Call function in strict order
                function(&mut acorn_ecs.world, &mut acorn_context);
            }
        }

        // ---------------------------- Next_frame ----------------------------
        next_frame().await;
    }
}