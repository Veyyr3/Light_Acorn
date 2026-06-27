// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/acorn_esetup.rs

// necessary imports
use bevy_ecs::prelude::*;
use crate::acorn_kernel::prelude::AcornECS;
use crate::acorn_tools::acorn_game_tools::prelude::*;
// only for example
use crate::acorn_zsetup::Oaks;

/*
Use this file to add Bevy systems for multithreading.

If you don't want then it's optional. 

=======

Some advise how to use Bevy resource from AcornGlobalContext:
acorn_ecs.world.insert_resource(acorn_global_context.clone());
*/

/// Add here your Bevy Systems
pub fn acorn_ecs_setup() -> AcornECS {
    let mut acorn_ecs = AcornECS::default();

    // Resources
    /*
    acorn_ecs.world.insert_resource(GameSettings {
        max_oaks: 18_446_744_073_709_551_615, 
    });
    */

    // Systems
    acorn_ecs.schedule.add_systems((
        example_bevy_system,

        // add systems here
    ));

    acorn_ecs
}

/// An Example of Bevy System
fn example_bevy_system(mut query: Query<&mut Oaks>) {
    // loop for all entities with Oaks. 
    // Spoiler: game will be over when oaks reach 18 446 744 073 709 551 615 :)
    for mut oaks in &mut query {
        oaks.x += 1; 
    }
}