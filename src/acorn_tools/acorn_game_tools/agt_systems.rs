// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use crate::acorn_tools::acorn_game_tools::agt_heart::{AcornDynamicSpatialHash, AcornSimpleAABB, CollisionPush, Entity3DTransform};

pub fn acorn_system_populate_spatial_hash(
    query: Query<(Entity, &Entity3DTransform, &AcornSimpleAABB)>,
    mut spatial_hash: ResMut<AcornDynamicSpatialHash>,
) {
    spatial_hash.grid.clear(); // Очищаем старые данные кадра

    for (entity, transform, aabb) in query.iter() {
        // Вычисляем, какие целые клетки занимает AABB сущности
        let min_grid = (transform.position - aabb.half_extents).round();
        let max_grid = (transform.position + aabb.half_extents).round();

        // Пробегаем по клеткам, которые занимает конкретно ЭТА сущность 
        // (это микро-цикл, он зависит от размера монстра, а не от количества врагов)
        for x in (min_grid.x as i32)..=(max_grid.x as i32) {
            for y in (min_grid.y as i32)..=(max_grid.y as i32) {
                for z in (min_grid.z as i32)..=(max_grid.z as i32) {
                    let cell = IVec3::new(x, y, z);
                    spatial_hash.grid.entry(cell).or_insert_with(Vec::new).push(entity);
                }
            }
        }
    }
}

pub fn acorn_system_find_collisions(
    mut commands: Commands,
    // Читаем позиции всех сущностей без мутаций
    query: Query<(Entity, &Entity3DTransform, &AcornSimpleAABB)>,
    spatial_hash: Res<AcornDynamicSpatialHash>,
) {
    for (entity, transform, aabb) in query.iter() {
        let center_cell = IVec3::new(
            transform.position.x.round() as i32,
            transform.position.y.round() as i32,
            transform.position.z.round() as i32,
        );

        if let Some(other_entities) = spatial_hash.grid.get(&center_cell) {
            for &other in other_entities {
                if other == entity { continue; }

                // Спокойно читаем вторую сущность из этого же query за O(1)
                if let Ok((_, other_transform, other_aabb)) = query.get(other) {
                    let delta = transform.position - other_transform.position;
                    let sum_half_extents = aabb.half_extents + other_aabb.half_extents;

                    let overlap_x = sum_half_extents.x - delta.x.abs();
                    let overlap_y = sum_half_extents.y - delta.y.abs();
                    let overlap_z = sum_half_extents.z - delta.z.abs();

                    if overlap_x > 0.0 && overlap_y > 0.0 && overlap_z > 0.0 {
                        let mut push_vector = Vec3::ZERO;

                        if overlap_x < overlap_y && overlap_x < overlap_z {
                            let sign = if delta.x >= 0.0 { 1.0 } else { -1.0 };
                            push_vector.x = overlap_x * sign;
                        } else if overlap_y < overlap_x && overlap_y < overlap_z {
                            let sign = if delta.y >= 0.0 { 1.0 } else { -1.0 };
                            push_vector.y = overlap_y * sign;
                        } else {
                            let sign = if delta.z >= 0.0 { 1.0 } else { -1.0 };
                            push_vector.z = overlap_z * sign;
                        }

                        // Вместо мгновенного изменения — откладываем команду на сдвиг
                        commands.entity(entity).insert(CollisionPush { vector: push_vector });
                    }
                }
            }
        }
    }
}

pub fn acorn_system_apply_push(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Entity3DTransform, &CollisionPush)>,
) {
    for (entity, mut transform, push) in query.iter_mut() {
        transform.position += push.vector;
        
        // Удаляем маркер, чтобы на следующем кадре не выталкивать по новой
        commands.entity(entity).remove::<CollisionPush>();
    }
}