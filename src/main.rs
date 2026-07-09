// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/main.rs

mod acorn_kernel; // Zone, Location, AcornFunction
mod acorn_zsetup; // zones, locations, functions setup
mod acorn_gsetup; // global setup
mod acorn_settings; // to setup your global statements, zone and etc.
mod acorn_esetup; // to setup multithread systems through Bevy
// tools
mod acorn_tools; // game tools
// for game
mod game_functions;
mod game_components;
mod game_settings;

use acorn_kernel::prelude::*; // acorn loop, acorn ECS
use acorn_zsetup::{ // import functions from acorn_setup for use in Main
    acorn_zone_setup,
};
use acorn_gsetup::acorn_global_setup;
use acorn_esetup::acorn_ecs_setup;

use crate::{
    acorn_tools::acorn_game_tools::agt_player::agt_spawn_player, 
    // for game
    game_functions::*
};

/*
Hi!

This main.rs file.

Examples are in acorn_setup.rs which you may try and search.

======================
Right now you are using template REACORN-way (when you can reoder functions in runtime).
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
    let mut acorn_ecs = acorn_ecs_setup();

    // WARNING: if you don't need Bevy multithreading then use that:
    // let mut acorn_ecs = AcornECS::default();

    // Contex of Zones. Hand over to acorn_loop.
    let mut acorn_zone_context = acorn_zone_setup();

    // Global states. Hand over to acorn_loop.
    let mut acorn_global_context = acorn_global_setup();

    // Create entities here (or in runtime by your logic)
    spawn_acorns_lvl1(
        &mut acorn_ecs.world, 
        &mut acorn_zone_context, 
        &mut acorn_global_context
    );
    spawn_finish(
        &mut acorn_ecs.world, 
        &mut acorn_zone_context, 
        &mut acorn_global_context
    );
    // spawn_coin_lvl1(
    //     &mut acorn_ecs.world, 
    //     &mut acorn_zone_context, 
    //     &mut acorn_global_context
    // );
    agt_spawn_player( 
        &mut acorn_ecs.world, 
        &mut acorn_zone_context, 
        &mut acorn_global_context
    );

    // main loop
    acorn_loop(acorn_ecs, acorn_zone_context, acorn_global_context).await;
}