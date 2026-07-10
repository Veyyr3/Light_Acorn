// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Light Acorn Framework: Game Tools
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_tools/acorn_game_tools/agt_function_sets.rs

use crate::acorn_kernel::prelude::*;
use crate::acorn_tools::acorn_game_tools::prelude::*;

/*
Here are Functions Sets. 

Functions Set is group of functions. Put them direct into Zone.
*/

// ---------------------------- Before 2D Zone Acorn Functions Sets ----------------------------

#[allow(dead_code)]
/// ## Description
/// Add simple collision between entities. 
/// 
/// This Functions Set resets the speed (XYZ) for an entity if its future position collides with another entity.
/// 
/// ## Functions:
/// * `agt_xz_grid_create` – Create grid collision.
/// * `agt_xz_grid_do_simple_collision` – Do simple collision in cells where >1 entities.
/// * `agt_xz_grid_clear` – Clear grid collision.
/// * `agt_do_entities_move` – Apply speed of entities to coordinates.
/// 
/// ## Example:
/// ```
/// let before_2d_zone = zone! {
///     location! {
///         function,
///     },
///     AGT_SIMPLE_COLLISION,
/// ```
pub const AGT_SIMPLE_COLLISION: [AcornFunction; 4] = 
    [
        agt_xz_grid_create,
        agt_xz_grid_do_simple_collision,
        agt_xz_grid_clear,
        agt_do_entities_move,
    ];

#[allow(dead_code)]
/// ## Description
/// Add simple collision between entities. If an entity is trigger then others can move through the entity.
/// You can use this Function Set to implement objects like finish, bonus, button and etc. in your game.
/// 
/// This Functions Set resets the speed (XYZ) for an entity if its future position collides with another entity.
/// 
/// ## Functions:
/// * `agt_clear_collision_triggers` – Set bool flag to false of component [`AcornIsCollided`].
/// * `agt_xz_grid_create` – Create grid collision.
/// * `agt_xz_grid_do_simple_collision_include_triggers` – Do simple collision in cells where >1 entities, include triggers.
/// * `agt_xz_grid_clear` – Clear grid collision.
/// * `agt_do_entities_move` – Apply speed of entities to coordinates.
/// 
/// ## Example:
/// ```
/// let before_2d_zone = zone! {
///     location! {
///         function,
///     },
///     AGT_SIMPLE_COLLISION,
/// ```
pub const AGT_SIMPLE_COLLISION_INCLUDE_TRIGGERS: [AcornFunction; 5] = 
    [
        agt_clear_collision_triggers,
        agt_xz_grid_create,
        agt_xz_grid_do_simple_collision_include_triggers,
        agt_xz_grid_clear,
        agt_do_entities_move,
    ];

#[allow(dead_code)]
/// ## Description
/// Add slide collision between entities. 
/// 
/// This Functions Set adds wall sliding. If an entity encounters an obstacle on the X-axis, all of its X-axis velocity is transferred to the Z-axis. And vice versa.
/// 
/// ## Functions:
/// * `agt_xz_grid_create` – Create grid collision.
/// * `agt_xz_grid_do_slide_collision` – Do slide collision in cells where >1 entities.
/// * `agt_xz_grid_clear` – Clear grid collision.
/// * `agt_do_entities_move` – Apply speed of entities to coordinates.
/// 
/// ## Example:
/// ```
/// let before_2d_zone = zone! {
///     location! {
///         function,
///     },
///     AGT_SLIDE_COLLISION,
/// ```
pub const AGT_SLIDE_COLLISION: [AcornFunction; 4] = 
    [
        agt_xz_grid_create,
        agt_xz_grid_do_slide_collision,
        agt_xz_grid_clear,
        agt_do_entities_move,
    ];

#[allow(dead_code)]
/// ## Description
/// Add slide collision between entities. 
/// 
/// This Functions Set adds wall sliding. If an entity encounters an obstacle on the X-axis, all of its X-axis velocity is transferred to the Z-axis. And vice versa.
/// 
/// ## Functions:
/// * `agt_clear_collision_triggers` – Set bool flag to false of component [`AcornIsCollided`].
/// * `agt_xz_grid_create` – Create grid collision.
/// * `agt_xz_grid_do_slide_collision_include_triggers` – Do slide collision in cells where >1 entities.
/// * `agt_xz_grid_clear` – Clear grid collision.
/// * `agt_do_entities_move` – Apply speed of entities to coordinates.
/// 
/// ## Example:
/// ```
/// let before_2d_zone = zone! {
///     location! {
///         function,
///     },
///     AGT_SLIDE_COLLISION,
/// ```
pub const AGT_SLIDE_COLLISION_INCLUDE_TRIGGERS: [AcornFunction; 5] = 
    [
        agt_clear_collision_triggers,
        agt_xz_grid_create,
        agt_xz_grid_do_slide_collision_include_triggers,
        agt_xz_grid_clear,
        agt_do_entities_move,
    ];

#[allow(dead_code)]
/// ## Description
/// Add simple collision between entities + debug functions: draw grid, draw AABB of entities. 
/// 
/// This Functions Set resets the speed (XYZ) for an entity if its future position collides with another entity.
/// 
/// ## Functions:
/// * `agt_xz_grid_create` – Create grid collision.
/// * `agt_xz_grid_debug_draw` – Draw grid.
/// * `agt_debug_entities_aabb_draw` – Draw AABB of entities.
/// * `agt_xz_grid_do_simple_collision` – Do simple collision in cells where >1 entities.
/// * `agt_xz_grid_clear` – Clear grid collision.
/// * `agt_do_entities_move` – Apply speed of entities to coordinates.
/// 
/// ## Example:
/// ```
/// let before_2d_zone = zone! {
///     location! {
///         function,
///     },
///     AGT_SIMPLE_COLLISION,
/// ```
pub const AGT_DEBUG_SIMPLE_COLLISION: [AcornFunction; 6] = 
    [
        agt_xz_grid_create,
        agt_xz_grid_debug_draw,
        agt_debug_entities_aabb_draw,
        agt_xz_grid_do_simple_collision,
        agt_xz_grid_clear,
        agt_do_entities_move,
    ];

#[allow(dead_code)]
/// ## Description
/// Add slide collision between entities + debug functions: draw grid, draw AABB of entities. 
/// 
/// This Functions Set adds wall sliding. If an entity encounters an obstacle on the X-axis, all of its X-axis velocity is transferred to the Z-axis. And vice versa.
/// 
/// ## Functions:
/// * `agt_xz_grid_create` – Create grid collision.
/// * `agt_xz_grid_debug_draw` – Draw grid.
/// * `agt_debug_entities_aabb_draw` – Draw AABB of entities.
/// * `agt_xz_grid_do_slide_collision` – Do slide collision in cells where >1 entities.
/// * `agt_xz_grid_clear` – Clear grid collision.
/// * `agt_do_entities_move` – Apply speed of entities to coordinates.
/// 
/// ## Example:
/// ```
/// let before_2d_zone = zone! {
///     location! {
///         function,
///     },
///     AGT_SLIDE_COLLISION,
/// ```
pub const AGT_DEBUG_SLIDE_COLLISION: [AcornFunction; 6] = 
    [
        agt_xz_grid_create,
        agt_xz_grid_debug_draw,
        agt_debug_entities_aabb_draw,
        agt_xz_grid_do_slide_collision,
        agt_xz_grid_clear,
        agt_do_entities_move,
    ];