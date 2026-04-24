// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// necessary imports
use bevy_ecs::prelude::*;
use crate::acorn_kernel::prelude::AcornECS;
// only for example
use crate::acorn_zsetup::Oaks;

pub fn acorn_ecs_setup() -> AcornECS {
    let mut acorn_ecs = AcornECS::default();

    acorn_ecs.schedule.add_systems((
        acorn_example_bevy_system,
        // add systems here
    ));

    acorn_ecs
}

/// An Example of Bevy ECS system
fn acorn_example_bevy_system(mut query: Query<&mut Oaks>) {
    // loop for all entities with Oaks. 
    // Spoiler: game will be over when oaks reach 18 446 744 073 709 551 615 :)
    for mut oaks in &mut query {
        oaks.x += 1; 
    }
}


// some advise how to use Bevy resource from AcornGlobalContext
// acorn_ecs.world.insert_resource(acorn_global_context.clone());