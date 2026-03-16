// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/main.rs
mod acorn_kernel;
use acorn_kernel::{
    acorn_render::acorn_loop, // import acorn_loop
    acorn_heart::AcornECS, // import Zone, Location, AcornECS
    acorn_setup::{ // import functions from acorn_setup for use in Main
        acorn_game_spawn_acorn, 
        acorn_setup, 
        acorn_example_spawn_entity
    }
};


/*
Hi!

This main.rs file.

Examples are in acorn_setup.rs which you may try and search.

======================
Right now you are using tempelate REACORN-way (when you can reoder functions in runtime).
BUT IF YOU DON'T WANT MUTABLE CODE IN RUNTIME: use ACORN WAY template in "TEMPLATES" folder.
======================

See other templates of projects in "TEMPLATES" folder.

======================
Memorise: Zone is when, Location is where, Function is time-marker.
======================
*/

#[macroquad::main("Light Acorn test")]
async fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Global variable of Zones. Hand over to acorn_loop.
    let mut acorn_context = acorn_setup();

    // Create entities here (or in runtime by your logic)
    acorn_example_spawn_entity(&mut acorn_ecs.world, &mut acorn_context);
    acorn_game_spawn_acorn(&mut acorn_ecs.world, &mut acorn_context);

    // main loop
    acorn_loop(acorn_context, acorn_ecs).await;
}
