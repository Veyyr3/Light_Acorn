// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

/*
Create here your Zones, Locations. 
It's your interface. (sorry, code doesn't let me use GUI here)

Below, there are examples of Acorn functions.

======================
Warning: If you want to add new Zone then you should add new zone_run! in acorn_render (read in docs about this).
======================
*/


// src/acorn_zsetup.rs

use crate::acorn_settings::{
    AcornZoneContext,
    AcornGlobalContext,
};
// sugar macros
use crate::{zone, location};
// game suggestions
use crate::acorn_tools::acorn_game_tools::prelude::*;
use macroquad::prelude::*;
use bevy_ecs::prelude::*;

// for game
use crate::game_functions::*;

pub fn acorn_zone_setup() -> AcornZoneContext {
    // ------------- ui_input_zone (Ex: handle input, events: victory, failure and etc.) -------------
    let ui_input_zone = zone! {
        // Location for UI input
        location! {
            agt_3d_camera_common_control,
            agt_player_fps_speed_control,
        }
    };

    // ------------- before_2d_zone (Ex: ECS Queries, 3D Mesh drawing and other Locations) -------------
    let before_2d_zone = zone! {
        // Minor-Location
        location! {
            agt_3d_camera, // camera should be here first!
            rotate_coin,
            agt_draw_3d_assets, // to draw yours 3d models
            example_game_draw_grid,
        },
        location! {
            agt_gravity_no_under_ground,
        },
        AGT_SLIDE_COLLISION, // <- A Functions Set
        location! {
            agt_3d_camera_link_to_player,
        }
    };   

    // ------------- after_2d_zone (Ex: UI draw and other Locations) -------------
    let after_2d_zone = zone! {
        // Minor-Location
        location! {
            write_game_statistics,
            // add own functions through comma 
        }
    };

    // Return AcornZoneContext for Main function
    AcornZoneContext { 
        ui_input_zone,
        before_2d_zone, 
        after_2d_zone,
    }
}

/* ======================
Here are examples of functions.

Create functions by this template:
fn name(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // your_logic
}

Arguments:
* world - for ECS Queries. This is necessary in order to process thousands of objects.
* zones - for Lord-Functions. This is necessary for control Minor-Locations.
* context - for Global States. Example: player score, 3d assets.

Advise: Create functions in other files and import here.
====================== */

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn example_game_draw_grid(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    draw_grid(100, 1.0, WHITE, GRAY);
}