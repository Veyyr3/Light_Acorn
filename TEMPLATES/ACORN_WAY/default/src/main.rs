// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/main.rs
mod acorn_kernel;
mod acorn_settings;
mod acorn_gsetup;
mod acorn_zsetup;
use acorn_kernel::{
    acorn_render::acorn_loop, // import acorn_loop
    acorn_heart::AcornECS // import Zone, Location, AcornECS
};
use crate::{
    // Global setup
    acorn_gsetup::acorn_global_setup, 
    // Zone setup
    acorn_zsetup::{
        acorn_setup,
        // example function
        acorn_example_spawn_entity, 
    }, 
};

/*
Hi!

This main.rs file is the example which you may try and search.

======================
Right now you are using tempelate ACORN-way 
(When you put function in strict order. Functions order is not mutable in runtime).
======================

See other templates of projects in "TEMPLATES" folder.

======================
See acorn_zsetup.rs file to start write your functions.
======================

======================
Memorise: Zone is when, Location is where, Function is atom.
======================
*/

#[macroquad::main("Light Acorn test")]
async fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Global variable of Zones. Hand over to acorn_loop.
    let (before, after) = acorn_setup();
    // Global states. Hand over to acorn_loop.
    let mut acorn_global_context = acorn_global_setup();

    // Create entities here (or in runtime by your logic)
    acorn_example_spawn_entity(&mut acorn_ecs.world, &mut acorn_global_context);

    // main loop
    acorn_loop(before, after, acorn_ecs, acorn_global_context).await;
}
