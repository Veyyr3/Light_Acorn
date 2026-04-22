// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

/*
Create here your Zones, Locations. 
It's your interface. (sorry, code doesn't let me use GUI here)

Below, there are examples of Acorn functions.

======================
Warning: If you want to add new Zone then you should add new loop "for" in acorn_render (read in docs about this).
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
    /* 
    Here is an example. 
    

    Locations don't need variables! But you can use variables if you want.
    Warning: Variables of Locations should exists before variables of Zones in acorn_setup.

    ======================
    Memorise: read code from top to down. Locations, Zones will run by chain.
    ======================
    Warning Memorise: Functions will run from down to top (see reason in acorn_render.rs)
    ======================
    */

    /*
    ======================
    Also I offer to you Lord-Minor achitecture to full control life of functions.
    ======================

    Lord-Location: here are Lord-Functions which can change other functions order in Minor-Locations.
    For each Lord-Functions in Lord-Location you should create own Minor-Location.

    Minor-Location: here are functions which obey to Lord-Function. They listen him and die, move or born by his orders.

    ======================
    Example: 
    if there are 3 Lord-Functions in Lord-Location then 3 Minor-Locations for each Lord-Functions.
    ======================
    OR Just Memorise: One Lord-Function = One his Minor-Location.
    ======================
    But YOU are not required to use this architecture. You are Lord of your ideas.
    ======================
    */

    // ui_input_zone (Ex: handle input, events: victory, failure etc.)
    let ui_input_zone = zone! {
        // Lord-Location.
        location! {
            acorn_example_add_circle_function, // add blue circle (press left mouse button)
            acorn_example_runtime_spawner, // add new entity (press Space and see result in console)
            acorn_example_delete_function, // (press TAB to delete functions in Minor-Location)
            // add own functions through comma
        }
        // add own locations through comma
    };

    // before_2d_zone (Ex: ECS Queries, 3D Mesh drawing and other Locations)
    let before_2d_zone = zone! {
        // Minor-Location
        location! {
            // ECS
            acorn_example_query_ecs, // print Oaks result
            // simple function
            acorn_example_greeting,
            // ECS
            acorn_example_update_oaks, // update ECS state
            // game
            acorn_game_draw_3d_assets, // to draw yours 3d models
            acorn_example_game_rotate_acorn,
            acorn_example_game_draw_grid,
            acorn_example_game_camera,
            // add own functions through comma
        }
        // add own locations through comma
    };   

    // after_2d_zone (Ex: UI draw and other Locations)
    let after_2d_zone = zone! {
        location! {
            acorn_debug_inspector
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

// ---------------------------- Example simple functions ----------------------------
// All simple functions should have World argument but shouldn't use it.
fn acorn_example_greeting(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    print!("Hello, Light Acorn!");
}

fn acorn_example_draw_circle(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    draw_circle(
        screen_width()/2.0, 
        screen_height()/2.0, 
        60.0, 
        BLUE
    )
}

// ---------------------------- Example ECS functions ----------------------------
// All ECS functions should have World argument.

// example component
#[derive(Component)]
struct Oaks {x: u64}

// Use spawn entities in fn main
pub fn acorn_example_spawn_entity(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    world.spawn((
        Oaks { x: 100 },
    ));
    println!("Entity spawned!");
}

// Add this function into location
fn acorn_example_query_ecs(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // create query
    let mut query = world.query::<&Oaks>();
    
    // loop for all entities
    for oaks in query.iter(world) {
        println!("Entity has: {} oaks", oaks.x);
    }
}

// Add this function into location
fn acorn_example_update_oaks(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // create query
    let mut query = world.query::<&mut Oaks>();

    // loop for all entities. 
    // Spoiler: game will be over when oaks reach 18 446 744 073 709 551 615 :)
    for mut oaks in query.iter_mut(world) {
        oaks.x += 1; 
    }
}

// Add this function into location
fn acorn_example_runtime_spawner(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // create new entity. Press Space!
    if is_key_pressed(KeyCode::Space) {
        world.spawn((
            Oaks { x: 0 }, 
        ));
        println!("Runtime Spawn!");
    }
}

// ---------------------------- Example Lord-Functions ----------------------------
// Add this function into Lord-Location in Ui input zone
fn acorn_example_delete_function(
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
        println!("I've killed function! Message from: acorn_example_delete_function");
    }
}

// Add this function into Lord-Location
fn acorn_example_add_circle_function(
    _world: &mut World, 
    zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // press left mouse button to draw your circle!
    if is_mouse_button_pressed(MouseButton::Left) { 
        zones.after_2d_zone.locations[0].functions.push(acorn_example_draw_circle);
        println!("I've created function! Message from: acorn_example_add_circle_function");
    }
}

// ---------------------------- Example Game Functuions ----------------------------
// Use this example ZST in ECS Query to replace if/else branching.
// In acorn_example_game_rotate_acorn ECS function rotating only entities with IsAcorn.
#[derive(Component)]
struct IsAcorn;

// spawner 3d model of acorn.
pub fn acorn_game_spawn_acorn(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    world.spawn((
       Entity3DTransform {
            position: vec3(0.0, 1.0, 0.0),
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0)
       }, 
       Entity3DModel {
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
       IsAcorn // component-marker
    ));
    println!("Entity spawned!");
}

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn acorn_example_game_camera(
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
fn acorn_example_game_rotate_acorn(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // Take all acorns and change rotations
    let mut query = 
        world
        .query_filtered::<&mut Entity3DTransform, With<IsAcorn>>();

    for mut i in query.iter_mut(world) {
        i.rotation += 0.1;
    }
}

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn acorn_example_game_draw_grid(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    draw_grid(20, 1.0, WHITE, GRAY);
}