// src/main.rs
mod acorn_kernel;
use acorn_kernel::{
    acorn_render::acorn_loop, // import acorn_loop
    acorn_heart::{Zone, Location} // import Zone, Location
};
use macroquad::prelude::*;

/// Create here your Zones and Locations. 
/// Add function to Location, Location to Zone.
fn acorn_setup() -> (Zone, Zone) {
    // ---------------------------- Zones ----------------------------

    // before_2d_zone_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
    let mut before_2d_zone = Zone::default();
    // after_2d_zone_zone (Ex: UI draw and other Locations)
    let mut after_2d_zone = Zone::default();

    // ---------------------------- Add Locations to Zones ----------------------------
    
    // Here is example. Locations don't need variables!
    before_2d_zone.add(
        // Test Location
        Location::default()
            .add(acorn_example_greeting)
    );

    after_2d_zone.add(
        // Test Location
        Location::default()
            .add(acorn_example_draw_circle)
    );

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
