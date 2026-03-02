// src/main.rs
mod acorn_kernel;
use acorn_kernel::{
    acorn_render::acorn_loop, // import acorn_loop
    acorn_heart::{Zone, Location, AcornECS} // import Zone, Location, AcornECS
};
use macroquad::prelude::*;
use bevy_ecs::prelude::*;

/*
Hi!

This main.rs file is the example which you may try and search.

See templates of projects in "TEMPLATES" folder.

Memorise: Zone is when, Location is where, Function is atom.
*/

/// Create here your Zones and Locations. 
/// Add function to Location, Location to Zone.
/// Warning: If you want to add new Zone then you should add new cycle "for" in acorn_render (read in docs about this).
fn acorn_setup() -> (Zone, Zone) {
    /* 
    Here is an example. 
    
    Locations don't need variables! But you can use variables if you want:
    Ex: create variables of Locations for REACORN (when you change functions order in runtime). Read in docs about this.

    Warning: Variables of Locations should exists before variables of Zones in acorn_setup.

    Memorise: read code from top to down. Functions, Locations, Zones will run by chain.
    */

    // before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
    let before_2d_zone = Zone::default()
    .with_locations(vec![
        // test location
        Location::from_fn_vec(vec![
            // simple function
            acorn_example_greeting,
            // ECS
            acorn_example_runtime_spawner, // add new entity
            acorn_example_update_oaks, // update ECS state
            acorn_example_query_ecs, // print result
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

    // after_2d_zone (Ex: UI draw and other Locations)
    let after_2d_zone = Zone::default()
    .with_locations(vec![
        // test location
        Location::from_fn_vec(vec![
            acorn_example_draw_circle,
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

    // Return tuple of Zones for Main function
    (before_2d_zone, after_2d_zone) 
}

// ---------------------------- Example simple functions ----------------------------
// Advise: Create functions in other files and import here.
// All simple functions should have World argument but shouldn't use it.
fn acorn_example_greeting(_world: &mut World) {
    print!("Hello, Light Acorn!")
}

fn acorn_example_draw_circle(_world: &mut World) {
    draw_circle(
        screen_width()/2.0, 
        screen_height()/2.0, 
        60.0, 
        BLUE
    )
}

// ---------------------------- Example ECS functions ----------------------------
// Advise: Create functions in other files and import here.
// All ECS functions should have World argument.

// create component
#[derive(Component)]
struct Oaks {x: u64}

// Use spawn entities in fn main
fn acorn_example_spawn_entity(world: &mut World) {
    world.spawn((
        Oaks { x: 100 },
    ));
    println!("Entity spawned!");
}

// Add this function into location
fn acorn_example_query_ecs(world: &mut World) {
    // create query
    let mut query = world.query::<&Oaks>();
    
    // cycle for all entities
    for oaks in query.iter(world) {
        println!("Entity has: {} oaks", oaks.x);
    }
}

// Add this function into location
fn acorn_example_update_oaks(world: &mut World) {
    // create query
    let mut query = world.query::<&mut Oaks>();

    // cycle for all entities. 
    // Spoiler: game will be over when oaks reach 18 446 744 073 709 551 615 :)
    for mut oaks in query.iter_mut(world) {
        oaks.x += 1; 
    }
}

// Add this function into location
fn acorn_example_runtime_spawner(world: &mut World) {
    // create new entity. Press Space!
    if is_key_pressed(KeyCode::Space) {
        world.spawn((
            Oaks { x: 0 }, 
        ));
        println!("Runtime Spawn!");
    }
}

#[macroquad::main("Light Acorn test")]
async fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Global variable of Zones. Hand over to acorn_loop.
    let (before, after) = acorn_setup();

    // Create entities here (or in runtime by your logic)
    acorn_example_spawn_entity(&mut acorn_ecs.world);

    // main loop
    acorn_loop(before, after, acorn_ecs).await;
}
