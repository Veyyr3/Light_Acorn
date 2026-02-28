// src/acorn_kernel/acorn_render.rs

use macroquad::prelude::*;
use crate::acorn_kernel::acorn_heart::Zone;
// use bevy_ecs::prelude::World;

/// Main loop of Light Acorn
pub async fn acorn_loop() {
    // Create here your Zones (not in loop)
    let before_2d_zone = Zone::default();
    let after_2d_zone = Zone::default();

    loop {
        clear_background(BLACK);

        // before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
        for location in &before_2d_zone.locations {
            for function in &location.functions {
                function(); // Call function in strict order
            }
        }

        // ---------------------------- Turn on 2D settings ----------------------------
        set_default_camera(); 

        // after_2d_zone (Ex: UI draw and other Locations)
        for location in &after_2d_zone.locations {
            for function in &location.functions {
                function(); // Call function in strict order
            }
        }

        // ---------------------------- Next_frame ----------------------------
        next_frame().await;
    }
}