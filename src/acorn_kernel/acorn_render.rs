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
use crate::acorn_settings::{AcornZoneContext, AcornGlobalContext};

/// Main loop of Light Acorn.
/// You shouldn't touch this. 
/// 
/// Warning: If you want to add new Zones you should touch this (read in docs about this).
/// Warning: If you want ot exit loop you should touch this. See code below about Escape button.
pub async fn acorn_loop( 
    mut acorn_ecs: AcornECS,
    mut acorn_zone_context: AcornZoneContext, 
    mut acorn_global_context:AcornGlobalContext
) {
    loop {
        // background color. You can change to your favorite color.
        clear_background(BLACK);


        /* 
        ============================ 
        ui_input_zone (Ex: handle input, events: victory, failure etc.)
        ============================
        */ 
        let len_ui_input_zone = acorn_zone_context.ui_input_zone.locations.len();
        // locations go by order
        for location_index in 0..len_ui_input_zone {
            let fn_count = acorn_zone_context
                .ui_input_zone
                .locations[location_index]
                .functions.len();

            // Reverse cycle for protect from panic (101 errors) in runtime
            // Functions go by reverse order
            // Warning: You should add new functions from down to top
            for fn_index in (0..fn_count).rev() {
                let function = 
                acorn_zone_context.ui_input_zone
                    .locations[location_index]
                    .functions[fn_index];
                    
                // Call function in strict order
                function(&mut acorn_ecs.world, &mut acorn_zone_context, &mut acorn_global_context);
            }
        }


        /* 
        ============================ 
        // Run Bevy ECS Schedule. You can add here your ECS Systems. 
        // It is necessary for multithreading (but you pay some overhead).
        ============================
        */ 
        // acorn_ecs.schedule.run(&mut acorn_ecs.world);


        /* 
        ============================ 
        before_2d_zone (Ex: ECS Queries, 3D Mesh drawing and other Locations)
        ============================
        */ 
        let len_before_2d_zone = acorn_zone_context.before_2d_zone.locations.len();
        // locations go by order
        for location_index in 0..len_before_2d_zone {
            let fn_count = acorn_zone_context
                .before_2d_zone
                .locations[location_index]
                .functions.len();

            // Reverse cycle for protect from panic (101 errors) in runtime
            // Functions go by reverse order
            // Warning: You should add new functions from down to top
            for fn_index in (0..fn_count).rev() {
                let function = 
                acorn_zone_context.before_2d_zone
                    .locations[location_index]
                    .functions[fn_index];
                    
                // Call function in strict order
                function(&mut acorn_ecs.world, &mut acorn_zone_context, &mut acorn_global_context);
            }
        }


        // ---------------------------- Turn on 2D render ----------------------------
        set_default_camera(); 


        /* 
        ============================ 
        after_2d_zone (Ex: UI draw and other Locations) 
        ============================
        */ 
        let len_after_2d_zone = acorn_zone_context.after_2d_zone.locations.len();
        // locations go by order
        for location_index in 0..len_after_2d_zone {
            let fn_count = acorn_zone_context
                .after_2d_zone
                .locations[location_index]
                .functions.len();

            // Reverse cycle for protect from panic (101 errors) in runtime
            // Functions go by reverse order
            // Warning: You should add new functions from down to top
            for fn_index in (0..fn_count).rev() {
                let function = 
                acorn_zone_context.after_2d_zone
                    .locations[location_index]
                    .functions[fn_index];
                    
                // Call function in strict order
                function(&mut acorn_ecs.world, &mut acorn_zone_context, &mut acorn_global_context);
            }
        }


        // Example to break loop
        // press Escape to exit.
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // ---------------------------- Next_frame ----------------------------
        next_frame().await;
    }
}