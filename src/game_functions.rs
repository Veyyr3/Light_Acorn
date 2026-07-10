// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

use crate::acorn_settings::{
    AcornZoneContext,
    AcornGlobalContext,
};
use crate::acorn_tools::acorn_game_tools::prelude::*;
use crate::game_components::*;
use crate::game_settings::*;
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

// ---------------------------- Spawn ----------------------------

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
                mesh_id: ACORN_MODEL 
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

pub fn spawn_finish(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    world.spawn((
        AcornEntity3DTransform {
            position: context.finish.position,
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0)
        }, 
        AcornEntity3DModel {
            mesh_id: FINISH_MODEL 
        },
        context.finish.aabb,
        Acorn3DSpeed {
            speed_value: Vec3::ZERO
        },
        AcornIsTrigger {
            is_trigger: true
        },
        AcornIsCollided {
            is_collided: false
        },
        IsFinish,
    ));
}

pub fn spawn_coin_lvl1(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    world.spawn((
        AcornEntity3DTransform {
            position: vec3(35.0, 24.0, 10.0),
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0)
        }, 
        AcornEntity3DModel {
            mesh_id: COIN_MODEL 
        },
        AcornAABB {
            min: vec3(-1.0, -0.5, -1.0),
            max: vec3(1.0, 0.5, 1.0)
        },
        Acorn3DSpeed {
            speed_value: Vec3::ZERO
        },
        AcornIsTrigger {
            is_trigger: true
        },
        AcornIsCollided {
            is_collided: false
        },
        IsCoin,
    ));
}

// ---------------------------- Logic ----------------------------

pub fn rotate_coin(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // Take all acorns and change rotations
    let mut query = 
        world
        .query_filtered::<&mut AcornEntity3DTransform, With<IsCoin>>();

    for mut i in query.iter_mut(world) {
        i.rotation += 0.1;
    }
}

pub fn update_lvl(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // get context
    let finish = &context.finish;
    let player = &context.game_base_preset.player;

    let is_collide = agt_is_predictive_collide(
        &player.aabb,
        player.position,
        &player.speed,
        &finish.aabb, 
        finish.position
    );
    println!("player {}", player.position);
    println!("player speed {:?}", player.speed);
    println!("finish {}", finish.position);

    if is_collide {
        context.level += 1;
        context.is_new_level = true;
        println!("collide!");
    }
}

// ---------------------------- UI ----------------------------

pub fn write_game_statistics(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    // get context
    let score = context.score;
    let level = context.level;

    // variables for text
    let y_start = 20.0;
    let x_start = 20.0;
    let font_size = 20.0;
    
    draw_text(&format!("LEVEL: {}", level), x_start, y_start, font_size, YELLOW);
    draw_text(&format!("SCORE: {}", score), x_start, y_start + 15.0, font_size, YELLOW);
}