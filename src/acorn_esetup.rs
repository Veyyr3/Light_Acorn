// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/acorn_esetup.rs

use std::collections::HashSet;

// necessary imports
use bevy_ecs::prelude::*;
use crate::acorn_kernel::prelude::AcornECS;
// only for example
use crate::acorn_zsetup::Oaks;

use crate::acorn_tools::acorn_game_tools::agt_heart::{AcornAABB, AcornPhysicsWorld, CollisionFlags, Entity3DTransform};
use macroquad::prelude::*;

/*
Use this file to add Bevy systems for multithreading.

If you don't want then it's optional. 
*/

/// Add here your Bevy Systems
pub fn acorn_ecs_setup() -> AcornECS {
    let mut acorn_ecs = AcornECS::default();

    let mut physics_world = AcornPhysicsWorld {
        solid_blocks: HashSet::new(),
    };

    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                // Создаем координату для текущего блока
                let block_pos = IVec3::new(x, y, z);
                
                // Добавляем её в нашу хэш-карту
                physics_world.solid_blocks.insert(block_pos);
            }
        }
    }

    // Resources
    acorn_ecs.world.insert_resource(physics_world);

    /*
    acorn_ecs.world.insert_resource(GameSettings {
        max_oaks: 18_446_744_073_709_551_615, 
    });
    */

    // Systems
    acorn_ecs.schedule.add_systems((
        example_bevy_system,
        acorn_system_flat_collision_flags
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

pub fn acorn_system_flat_collision_flags(
    // Только ОДИН запрос: бежим по динамическим сущностям
    mut query: Query<(&Entity3DTransform, &AcornAABB, &mut CollisionFlags)>,
    // Быстрый доступ к миру за O(1)
    world: Res<AcornPhysicsWorld>, 
) {
    const OFFSET: f32 = 0.1; // На сколько "щупаем" пространство перед собой

    for (transform, aabb, mut flags) in query.iter_mut() {
        let pos = transform.position;
        let radius = aabb.radius_size;

        // Проверяем направления напрямую через хэш-карту мира за O(1)
        // Если там стена — флаг становится false. Всё!
        
        flags.can_move_pos_x = !world.is_solid(pos + Vec3::X * (radius.x + OFFSET));
        flags.can_move_neg_x = !world.is_solid(pos - Vec3::X * (radius.x + OFFSET));
        
        flags.can_move_pos_y = !world.is_solid(pos + Vec3::Y * (radius.y + OFFSET));
        flags.can_move_neg_y = !world.is_solid(pos - Vec3::Y * (radius.y + OFFSET)) && (pos.y - radius.y > 0.0);
        
        flags.can_move_pos_z = !world.is_solid(pos + Vec3::Z * (radius.z + OFFSET));
        flags.can_move_neg_z = !world.is_solid(pos - Vec3::Z * (radius.z + OFFSET));
    }
}


// some advise how to use Bevy resource from AcornGlobalContext
// acorn_ecs.world.insert_resource(acorn_global_context.clone());