// src/main.rs
mod acorn_kernel;
use acorn_kernel::{
    acorn_render::acorn_loop, // import acorn_loop
    acorn_heart::{Zone, Location} // import Zone, Location
};
use macroquad::prelude::*;

/// Create here your Zones and Locations. 
/// Add function to Location, Location to Zone.
/// Warning: If you want to add new Zone then you should add new cycle "for" in acorn_render (read in docs about this).
fn acorn_setup() -> (Zone, Zone) {
    /* 
    Here is example. 
    
    Locations don't need variables! But you can use variables if you want.
    Variables of Locations should exists before variables of Zones.

    Memorise: read code from top to down. Functions, Locations, Zones will run by chain.
    */

    // before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
    let before_2d_zone = Zone::default()
    .with_locations(vec![
        // test location
        Location::from_fn_vec(vec![
            acorn_example_greeting,
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

// Example functions. Advise: Create functions in other files.
fn acorn_example_greeting() {
    print!("Hello, Light Acorn!")
}

fn acorn_example_draw_circle() {
    draw_circle(
        screen_width()/2.0, 
        screen_height()/2.0, 
        60.0, 
        BLUE
    )
}

#[macroquad::main("Light Acorn test")]
async fn main() {
    // Global variable of Zones. Hand over to acorn_loop.
    let (before, after) = acorn_setup();

    // main loop
    acorn_loop(before, after).await;
}
