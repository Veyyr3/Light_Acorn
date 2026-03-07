// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/acorn_kernel/acorn_render.rs
use macroquad::prelude::*;
use crate::acorn_kernel::acorn_heart::{Zone, AcornECS};

/// Main loop of Light Acorn.
/// You shouldn't touch this. 
/// Warning: If you want to add new Zones you should touch this (read in docs about this).
pub async fn acorn_loop(before_2d_zone: Zone, after_2d_zone: Zone, mut ecs: AcornECS) {
    loop {
        clear_background(BLACK);

        // Run Schedule
        ecs.schedule.run(&mut ecs.world);

        // before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
        for location in &before_2d_zone.locations {
            for function in &location.functions {
                function(&mut ecs.world); // Call function in strict order
            }
        }

        // ---------------------------- Turn on 2D render ----------------------------
        set_default_camera(); 

        // after_2d_zone (Ex: UI draw and other Locations)
        for location in &after_2d_zone.locations {
            for function in &location.functions {
                function(&mut ecs.world); // Call function in strict order
            }
        }

        // ---------------------------- Next_frame ----------------------------
        next_frame().await;
    }
}