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

/// Create here your Zones and Locations. 
/// Add function to Location, Location to Zone.
/// Warning: If you want to add new Zone then you should add new loop "for" in acorn_render (read in docs about this).
pub fn acorn_zone_setup() -> AcornZoneContext {
    // ------------- ui_input_zone (Ex: handle input, events: victory, failure and etc.) -------------
    let ui_input_zone = zone! {
        // Lord-Location.
        location! {
            example_speed_acorn, // move acorn
            example_delete_function, // (press TAB to delete functions in Minor-Location)
            // add own functions through comma
        },
        // Location for UI input
        location! {
            agt_3d_camera_common_control, // update camera position and look
            agt_player_fps_speed_control,
        }
        // add own locations through comma
    };

    // ------------- before_2d_zone (Ex: ECS Queries, 3D Mesh drawing and other Locations) -------------
    let before_2d_zone = zone! {
        // Minor-Location
        location! {
            agt_3d_camera, // camera should be here first!
            // ECS
            // example_query_ecs, // print Oaks result
            // simple function
            // example_greeting, // print 'Hello, Light Acorn!'
            // example_update_oaks, // update ECS state. But this function also is in acorn_esetup.rs. 
            // game
            agt_draw_3d_assets, // to draw yours 3d models
            example_game_rotate_acorn, // ECS
            example_game_draw_grid, // press TAB and this function will be deleted first
            // add own functions through comma
        },
        location! {
            agt_gravity_no_under_ground,
        },
        AGT_DEBUG_SLIDE_COLLISION, // <- A Functions Set
        location! {
            agt_3d_camera_link_to_player,
        }
    };   

    // ------------- after_2d_zone (Ex: UI draw and other Locations) -------------
    let after_2d_zone = zone! {
        // Minor-Location
        location! {
            // add own functions through comma 
        }
        // add own locations through comma 
    };

    // Return AcornZoneContext for Main function
    AcornZoneContext { 
        ui_input_zone,
        before_2d_zone, 
        after_2d_zone,
        // your Zone through comma if you have
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

// ---------------------------- Example Lord-Functions ----------------------------
// Add this function into Lord-Location in Ui input zone
fn example_delete_function(
    _world: &mut World, 
    zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // KILL ANY FUNCTION IN SECOND ZONE!
    // PRESS TAB!
    // of course you have right to write if/else checking to get rid of 101 error in runtime:
    // if !zones.before_2d_zone.locations[0].functions.is_empty()
    // but I leave this to understand REACORN-way for you
    if is_key_pressed(KeyCode::Tab) { 
        zones.before_2d_zone.locations[0].functions.remove(0);
        println!("I've killed function! Message from: example_delete_function");
    }
}

// // Add this function into Lord-Location
// fn example_add_circle_function(
//     _world: &mut World, 
//     zones: &mut AcornZoneContext, 
//     _context: &mut AcornGlobalContext
// ) {
//     // press left mouse button to draw your circle!
//     if is_mouse_button_pressed(MouseButton::Left) { 
//         zones.after_2d_zone.locations[0].functions.push(example_draw_circle);
//         println!("I've created function! Message from: example_add_circle_function");
//     }
// }

// ---------------------------- Example Game Functuions ----------------------------
// Use this example ZST in ECS Query to replace if/else branching.
// In example_game_rotate_acorn ECS function rotating only entities with IsAcorn.
#[derive(Component)]
struct IsAcorn;

// new
#[derive(Component)]
struct CanAcornMove;

// spawner 3d model of acorn.
pub fn example_spawn_acorn(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    world.spawn((
        AcornEntity3DTransform {
            position: vec3(0.0, 10.0, 0.0),
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0)
        }, 
        AcornEntity3DModel {
            // WARNING: you should remember index of your 3d model
            mesh_id: 0 

            /*
            But you can use a trick:

            // src/game_assets.rs
            pub const ACORN_MODEL: usize = 0;
            pub const TREE_MODEL: usize = 1;
            pub const ROCK_MODEL: usize = 2;

            AND write like that:
            mesh_id: ACORN_MODEL
            */
        },
        AcornAABB {
            min: vec3(-1.0, -1.0, -1.0),
            max: vec3(1.0, 1.0, 1.0)
        },
        Acorn3DSpeed {
            speed_value: Vec3::ZERO
        },
        IsAcorn, // component-marker
        AcornHasGravity
    ));
    println!("Entity spawned!");
}

// new
pub fn example_spawn_acorn_move(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {

    world.spawn((
        AcornEntity3DTransform {
            position: vec3(5.0, 1.0, 0.0),
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0)
        }, 
        AcornEntity3DModel {
                // WARNING: you should remember index of your 3d model
                mesh_id: 0 

                /*
                But you can use a trick:

                // src/game_assets.rs
                pub const ACORN_MODEL: usize = 0;
                pub const TREE_MODEL: usize = 1;
                pub const ROCK_MODEL: usize = 2;

                AND write like that:
                mesh_id: ACORN_MODEL
                */
        },
        AcornAABB {
            min: vec3(-1.0, -1.0, -1.0),
            max: vec3(1.0, 1.0, 1.0)
        },
        Acorn3DSpeed {
            speed_value: Vec3::ZERO
        },
        CanAcornMove, // component-marker
        AcornHasGravity,
    ));
    println!("Entity spawned!");

}

// new
fn example_speed_acorn(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    let mut query = world.query_filtered::<&mut Acorn3DSpeed, With<CanAcornMove>>();
    let dt = context.frame_delta;

    for mut i in query.iter_mut(world) {
        if is_key_down(KeyCode::Right){
            i.speed_value.x = -2.0 * dt;
        } else {
            i.speed_value.x = 0.0;
        }

        if is_key_down(KeyCode::Left) {
            i.speed_value.z = 2.0 * dt;
        } else {
            i.speed_value.z = 0.0;
        }
    }
}

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn example_game_simple_camera(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // spawn camera
    set_camera(&Camera3D {
        position: vec3(5.0, 5.0, 5.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.5, 0.0),
        ..Default::default()
    });
}

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn example_game_rotate_acorn(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // Take all acorns and change rotations
    let mut query = 
        world
        .query_filtered::<&mut AcornEntity3DTransform, With<IsAcorn>>();

    for mut i in query.iter_mut(world) {
        i.rotation += 0.1;
    }
}

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn example_game_draw_grid(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    draw_grid(20, 1.0, WHITE, GRAY);
}