// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Kernel
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_render.rs
use macroquad::prelude::*;
use crate::{
    acorn_kernel::acorn_heart::{
        AcornECS, 
        Zone
    }, 
    acorn_settings::AcornGlobalContext
};

/// Main loop of Light Acorn.
/// You shouldn't touch this. 
/// Warning: If you want to add new Zones you should touch this (read in docs about this).
pub async fn acorn_loop(
    before_2d_zone: Zone, 
    after_2d_zone: Zone, 
    mut ecs: AcornECS, 
    mut context: AcornGlobalContext
) {
    loop {
        clear_background(BLACK);

        // Run Schedule
        //ecs.schedule.run(&mut ecs.world);

        // before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
        for location in &before_2d_zone.locations {
            for function in &location.functions {
                function(&mut ecs.world, &mut context); // Call function in strict order
            }
        }

        // ---------------------------- Turn on 2D render ----------------------------
        set_default_camera(); 

        // after_2d_zone (Ex: UI draw and other Locations)
        for location in &after_2d_zone.locations {
            for function in &location.functions {
                function(&mut ecs.world, &mut context); // Call function in strict order
            }
        }

        // ---------------------------- Next_frame ----------------------------
        next_frame().await;
    }
}