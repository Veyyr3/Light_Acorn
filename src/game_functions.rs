// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

use crate::acorn_settings::{
    AcornZoneContext,
    AcornGlobalContext,
};
use crate::acorn_tools::acorn_game_tools::prelude::*;
use crate::game_components;
use macroquad::prelude::*;
use bevy_ecs::prelude::*;

// Add this function into Lord-Location in Ui input zone
// fn example_delete_function(
//     _world: &mut World, 
//     zones: &mut AcornZoneContext, 
//     _context: &mut AcornGlobalContext
// ) {
//     // KILL ANY FUNCTION IN SECOND ZONE!
//     // PRESS TAB!
//     // of course you have right to write if/else checking to get rid of 101 error in runtime:
//     // if !zones.before_2d_zone.locations[0].functions.is_empty()
//     // but I leave this to understand REACORN-way for you
//     if is_key_pressed(KeyCode::Tab) { 
//         zones.before_2d_zone.locations[0].functions.remove(0);
//         println!("I've killed function! Message from: example_delete_function");
//     }
// }

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

pub fn spawn_acorns_lvl1(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    let coordinates = vec![
        vec3(10.0, 1.5, 10.0),
        vec3(15.0, 5.0, 10.0),
        vec3(20.0, 10.0, 10.0),
        vec3(25.0, 10.0, 10.0),
        vec3(30.0, 15.0, 10.0),
        vec3(35.0, 20.0, 10.0), // high
        vec3(40.0, 10.0, 10.0),
        vec3(45.0, 10.0, 10.0),
    ];

    for coor in coordinates {
        world.spawn((
            AcornEntity3DTransform {
                position: coor,
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
        ));
    }
    
}

// pub fn rotate_coin(
//     world: &mut World, 
//     _zones: &mut AcornZoneContext, 
//     _context: &mut AcornGlobalContext
// ) {
//     // Take all acorns and change rotations
//     let mut query = 
//         world
//         .query_filtered::<&mut AcornEntity3DTransform, With<IsAcorn>>();

//     for mut i in query.iter_mut(world) {
//         i.rotation += 0.1;
//     }
// }