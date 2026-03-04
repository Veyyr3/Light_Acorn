// src/main.rs
mod acorn_kernel;
use acorn_kernel::{
    acorn_render::acorn_loop, // import acorn_loop
    acorn_heart::{Zone, Location, AcornECS}, // import Zone, Location, AcornECS
    acorn_settings::AcornContext // struct AcornContext 
};
use macroquad::prelude::*;
use bevy_ecs::prelude::*;

/*
Hi!

This main.rs file is the example which you may try and search.

Right now you are using tempelate REACORN-way (when you can reoder functions in runtime).
BUT IF YOU DON'T WANT MUTABLE CODE IN RUNTIME: use ACORN template in TEMPLATES" folder.

See other templates of projects in "TEMPLATES" folder.

Memorise: Zone is when, Location is where, Function is time-marker.
*/

/// Create here your Zones and Locations. 
/// It's your interface. (sorry, code doesn't let me use GUI here)
/// Add function to Location, Location to Zone.
/// Warning: If you want to add new Zone then you should add new cycle "for" in acorn_render (read in docs about this).
fn acorn_setup() -> AcornContext {
    /* 
    Here is an example. 
    
    Locations don't need variables! But you can use variables if you want.

    Warning: Variables of Locations should exists before variables of Zones in acorn_setup.

    Memorise: read code from top to down. Locations, Zones will run by chain.
    Warning Memorise: Functions will run from down to top (see reason in acorn_render.rs)
    */

    /*
    Also I offer to you Lord-Minor achitecture to full control life of functions.

    Lord-Location: here are Lord-Functions which can change other functions order in Minor-Locations.
    For each Lord-Functions in Lord-Location you should create own Minor-Location.

    Minor-Location: here are functions which obey to Lord-Function. They listen him and die, move or born by his orders.
    */

    // before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
    let before_2d_zone = Zone::default()
    .with_locations(vec![
        // Lord-Location.
        Location::from_fn_vec(vec![
            acorn_example_delete_function, // Deleter of functions.
        ]),
        // Minor-Location
        Location::from_fn_vec(vec![
            // ECS
            acorn_example_query_ecs, // print Oaks result (press TAB to delete this function)
            // simple function
            acorn_example_greeting,
            // ECS
            acorn_example_runtime_spawner, // add new entity
            acorn_example_update_oaks, // update ECS state
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

    // after_2d_zone (Ex: UI draw and other Locations)
    let after_2d_zone = Zone::default()
    .with_locations(vec![
        // Lord-Location
        Location::from_fn_vec(vec![
            acorn_example_add_circle_function
        ]),
        // Minor-Location
        Location::from_fn_vec(vec![
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

    // Return AcornContext for Main function
    AcornContext { before_2d_zone, after_2d_zone }
}

// ---------------------------- Example simple functions ----------------------------
// Advise: Create functions in other files and import here.
// All simple functions should have World argument but shouldn't use it.
fn acorn_example_greeting(_world: &mut World, _context: &mut AcornContext) {
    print!("Hello, Light Acorn!")
}

fn acorn_example_draw_circle(_world: &mut World, _context: &mut AcornContext) {
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
fn acorn_example_spawn_entity(world: &mut World, _context: &mut AcornContext) {
    world.spawn((
        Oaks { x: 100 },
    ));
    println!("Entity spawned!");
}

// Add this function into location
fn acorn_example_query_ecs(world: &mut World, _context: &mut AcornContext) {
    // create query
    let mut query = world.query::<&Oaks>();
    
    // cycle for all entities
    for oaks in query.iter(world) {
        println!("Entity has: {} oaks", oaks.x);
    }
}

// Add this function into location
fn acorn_example_update_oaks(world: &mut World, _context: &mut AcornContext) {
    // create query
    let mut query = world.query::<&mut Oaks>();

    // cycle for all entities. 
    // Spoiler: game will be over when oaks reach 18 446 744 073 709 551 615 :)
    for mut oaks in query.iter_mut(world) {
        oaks.x += 1; 
    }
}

// Add this function into location
fn acorn_example_runtime_spawner(world: &mut World, _context: &mut AcornContext) {
    // create new entity. Press Space!
    if is_key_pressed(KeyCode::Space) {
        world.spawn((
            Oaks { x: 0 }, 
        ));
        println!("Runtime Spawn!");
    }
}

// ---------------------------- Example Lord-Functions ----------------------------
// Add this function into Lord-Location
fn acorn_example_delete_function(_world: &mut World, contex: &mut AcornContext) {
    // KILL ANY FUNCTION IN FIRST ZONE, SECOND LOCATION!
    // PRESS TAB!
    if is_key_pressed(KeyCode::Tab) { 
        contex.before_2d_zone.locations[1].functions.remove(0);
        println!("I've killed function! Message from: acorn_example_delete_function");
    }
}

// Add this function into Lord-Location
fn acorn_example_add_circle_function(_world: &mut World, contex: &mut AcornContext) {
    // press left mouse button to draw your circle!
    if is_mouse_button_pressed(MouseButton::Left) { 
        contex.after_2d_zone.locations[1].functions.push(acorn_example_draw_circle);
        println!("I've gave birth function! Message from: acorn_example_add_circle_function");
    }
}

#[macroquad::main("Light Acorn test")]
async fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Global variable of Zones. Hand over to acorn_loop.
    let mut acorn_context = acorn_setup();

    // Create entities here (or in runtime by your logic)
    acorn_example_spawn_entity(&mut acorn_ecs.world, &mut acorn_context);

    // main loop
    acorn_loop(acorn_context, acorn_ecs).await;
}
