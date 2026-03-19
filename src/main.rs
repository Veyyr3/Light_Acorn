// (c) 2026 Lord of the Light Acorn: Veyyr3.
// This file is part of Light Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/main.rs
mod acorn_kernel;
use acorn_kernel::{
    acorn_render::acorn_loop, // import acorn_loop
    acorn_heart::{Zone, Location, AcornECS}, // import Zone, Location, AcornECS
    acorn_settings::AcornContext, // struct AcornContext 
    // game suggestions
    acorn_tools::acorn_game_tools::agt_heart::{
        Acorn3DAssetDatabase, 
        Entity3DTransform, 
        Entity3DModel
    }, 
    acorn_tools::acorn_game_tools::agt_obj_parsers::load_obj_with_materials_to_mesh,
    acorn_tools::acorn_game_tools::agt_functions::{
        acorn_generate_transform,
        acorn_game_draw_3d,
        acorn_game_draw_3d_instanced,
    },
};
use macroquad::prelude::*;
use bevy_ecs::prelude::*;
use macroquad::miniquad as mq;

use crate::acorn_kernel::acorn_tools::acorn_game_tools::mq_experimental::{AcornMeshInstanceDB, acorn_draw_instanced, acorn_register_for_instancing};
use crate::acorn_kernel::acorn_tools::acorn_game_tools::shaders::*;


// use crate::acorn_kernel::acorn_settings::AcornRenderBuffer;

/*
Hi!

This main.rs file is the example which you may try and search.

======================
Right now you are using tempelate REACORN-way (when you can reoder functions in runtime).
BUT IF YOU DON'T WANT MUTABLE CODE IN RUNTIME: use ACORN WAY template in "TEMPLATES" folder.
======================

See other templates of projects in "TEMPLATES" folder.

======================
Memorise: Zone is when, Location is where, Function is time-marker.
======================
*/

/// Create here your Zones and Locations. 
/// It's your interface. (sorry, code doesn't let me use GUI here)
/// Add function to Location, Location to Zone.
/// Warning: If you want to add new Zone then you should add new cycle "for" in acorn_render (read in docs about this).
fn acorn_setup() -> AcornContext {
    /* 
    Here is an example. 
    
    ======================
    Locations don't need variables! But you can use variables if you want.
    Warning: Variables of Locations should exists before variables of Zones in acorn_setup.
    ======================

    ======================
    Memorise: read code from top to down. Locations, Zones will run by chain.
    ======================
    Warning Memorise: Functions will run from down to top (see reason in acorn_render.rs)
    ======================
    */

    /*
    Also I offer to you Lord-Minor achitecture to full control life of functions.

    Lord-Location: here are Lord-Functions which can change other functions order in Minor-Locations.
    For each Lord-Functions in Lord-Location you should create own Minor-Location.

    Minor-Location: here are functions which obey to Lord-Function. They listen him and die, move or born by his orders.

    But YOU are not required to use this architecture. You are Lord of your ideas.
    */

    // before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
    let before_2d_zone = Zone::default()
    .with_locations(vec![
        // Lord-Location.
        Location::from_fn_vec(vec![
            // Deleter of functions.
            acorn_example_delete_function, // (press TAB to delete functions in Minor-Location)
        ]),
        // Minor-Location
        Location::from_fn_vec(vec![
            // game
            acorn_draw_instanced,
            // acorn_game_draw_3d_instanced,
            acorn_generate_transform,
            // acorn_game_draw_3d,
            // acorn_game_draw_3d_assets, // to draw yours 3d models

            acorn_example_game_rotate_acorn,
            acorn_example_game_draw_grid,
            acorn_example_game_camera,
            handle_input,
            add_acorns,
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

    // after_2d_zone (Ex: UI draw and other Locations)
    let after_2d_zone = Zone::default()
    .with_locations(vec![
        Location::from_fn_vec(vec![
            draw_statistics,
            draw_sight,
            
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

    // ---------------------------- Game setup ----------------------------
    // Keep 3d models in assets database.
    let mut assets_3d = Acorn3DAssetDatabase {meshes: Vec::new()};

    // Add your .obj files with push.
    // PLEASE, remember index of your 3d models when you add news.
    // It so, because for perfomance. 
    // BUT I leave it to you for organize logic assets keeping.
    let acorn_mesh = load_obj_with_materials_to_mesh("src/acorn_kernel/acorn_tools/acorn_game_tools/objs/acorn_engine.obj");
assets_3d.meshes.push(acorn_mesh); // Теперь у желудя индекс 0

// 2. Получаем доступ к контексту для работы с GPU
let mut internal_gl = unsafe { get_internal_gl() };
let ctx = internal_gl.quad_context;

// 3. Создаем GPU-базу и регистрируем в ней наш желудь
let mut instance_assets = AcornMeshInstanceDB { gpu_meshes: Vec::new() };
// 4. Создаем 1x1 белую текстуру программно
let white_texture = Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]);
let default_tex_id = white_texture.raw_miniquad_id();

// Важно: индекс в instance_assets.gpu_meshes ДОЛЖЕН совпадать 
// с индексом в assets_3d.meshes (в нашем случае это 0)
let prepared_acorn = acorn_register_for_instancing(ctx, &assets_3d.meshes[0], default_tex_id);
instance_assets.gpu_meshes.push(prepared_acorn);

    // подсчет количества желудей
    let acorns_x = 1;
    let acorns_y = 1;
    let acorns_count = acorns_x*acorns_y;

    // 1. Получаем доступ к "железу"
let mut internal_gl = unsafe { get_internal_gl() };
let ctx = internal_gl.quad_context;

// 2. Компилируем шейдер (обязательно!)
let shader = ctx.new_shader(
    mq::ShaderSource::Glsl {
        vertex: VERTEX_SHADER_SRC,
        fragment: FRAGMENT_SHADER_SRC,
    },
    mq::ShaderMeta {
        uniforms: mq::UniformBlockLayout {
            uniforms: vec![mq::UniformDesc::new("uViewProjection", mq::UniformType::Mat4)],
        },
        images: vec!["uTexture".to_string()],
    },
).expect("Shader compilation failed");

// 3. Создаем Pipeline
let instancing_pipeline = ctx.new_pipeline(
    &[
        mq::BufferLayout::default(), 
        mq::BufferLayout {
            step_func: mq::VertexStep::PerInstance,
            ..Default::default()
        },
    ],
    &[
        mq::VertexAttribute::with_buffer("aPos", mq::VertexFormat::Float3, 0),
        mq::VertexAttribute::with_buffer("aTex", mq::VertexFormat::Float2, 0),
        mq::VertexAttribute::with_buffer("aCol", mq::VertexFormat::Byte4, 0),
        mq::VertexAttribute::with_buffer("aNormal", mq::VertexFormat::Float4, 0),
        mq::VertexAttribute::with_buffer("aModelMat", mq::VertexFormat::Mat4, 1),
    ],
    shader,
    mq::PipelineParams {
        depth_test: mq::Comparison::LessOrEqual,
        depth_write: true,
        ..Default::default()
    },
);

// 4. Создаем BufferId для матриц (резервируем место под 10 000 штук)
let gpu_instance_buffer = ctx.new_buffer(
    mq::BufferType::VertexBuffer,
    mq::BufferUsage::Stream,
    mq::BufferSource::empty::<Mat4>(10_000),
);

    // Return AcornContext for Main function
    AcornContext { 
        before_2d_zone, 
        after_2d_zone,
        // suggestion for game
        assets_3d,
        // camera attributes
        pos: vec3(5.0, 30.0, 5.0), // camera position
        yaw: 1.1,
        pitch: 0.0,
        // acorns stats
        acorns_x,
        acorns_y,
        acorns_count,
        // batching
        matrix_buffer: Vec::new(),
        // instacing
        instancing_pipeline,
        gpu_instance_buffer,
        instance_assets,
    }
}

/* ======================
Here are examples of functions.

Create functions by this template:
fn name(world: &mut World, context: &mut AcornContext) {
    // your_logic
}

Advise: Create functions in other files and import here.
====================== */

// ---------------------------- Example Lord-Functions ----------------------------
// Add this function into Lord-Location
fn acorn_example_delete_function(_world: &mut World, context: &mut AcornContext) {
    // KILL ANY FUNCTION IN FIRST ZONE, SECOND LOCATION!
    // PRESS TAB!
    // of course you have right to write if/else checking to get rid of 101 error in runtime:
    // if !context.before_2d_zone.locations[1].functions.is_empty()
    // but I leave this to understand REACORN-way for you
    if is_key_pressed(KeyCode::Tab) { 
        context.before_2d_zone.locations[1].functions.remove(0);
        println!("I've killed function! Message from: acorn_example_delete_function");
    }
}

// ---------------------------- Example Game Functuions ----------------------------
// Use this example ZST in ECS Query to replace if/else branching.
// In acorn_example_game_rotate_acorn ECS function rotating only entities with IsAcorn.
#[derive(Component)]
struct IsAcorn;

// spawner 3d model of acorn.
fn acorn_game_spawn_acorn(world: &mut World, context: &mut AcornContext) {
    let count_x: u16 = context.acorns_x;
    let count_z: u16 = context.acorns_y;
    let spacing = 10.0;

    let offset_x = (count_x as f32 * spacing) / 2.0;
    let offset_z = (count_z as f32 * spacing) / 2.0;

    for x in 0..count_x {
        for z in 0..count_z {
            let pos_x = (x as f32 * spacing) - offset_x;
            let pos_z = (z as f32 * spacing) - offset_z;

            world.spawn((
                Entity3DTransform {
                    position: vec3(pos_x, 0.0, pos_z),
                    rotation: 0.0,
                    scale: vec3(1.0, 1.0, 1.0),
                },
                Entity3DModel {
                    mesh_id: 0 
                },
                IsAcorn,
            ));
        }
    }
    
    println!("Acorns are {}!", count_x * count_z);
}

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn acorn_example_game_camera(_world: &mut World, context: &mut AcornContext) {
    // spawn camera
    set_camera(&Camera3D {
            position: context.pos,
            up: vec3(0.0, 1.0, 0.0),
            target: context.pos + get_look_dir(context.yaw, context.pitch),
            ..Default::default()
        });
}

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn acorn_example_game_rotate_acorn(world: &mut World, _context: &mut AcornContext) {
    // Take all acorns and change rotations
    let mut query = 
        world
        .query_filtered::<&mut Entity3DTransform, With<IsAcorn>>();

    for mut i in query.iter_mut(world) {
        i.rotation += 0.1;
    }
}

// Add to before 2d zone (in after 2d zone it may work incorrect)
fn acorn_example_game_draw_grid(_world: &mut World, _context: &mut AcornContext) {
    draw_grid(1000, 2.0, WHITE, GRAY);
}

// ---------------------------- MY FUNCTIONS ----------------------------
fn handle_input(_world: &mut World, context: &mut AcornContext) {
    let look_speed = 1.0;
    let move_speed = 0.2;

    // 1. rotate
    let mouse_delta = mouse_delta_position();
    context.yaw -= mouse_delta.x * look_speed; 
    context.pitch += mouse_delta.y * look_speed;
    context.pitch = context.pitch.clamp(-1.5, 1.5);

    let look_dir = get_look_dir(context.yaw, context.pitch);

    // 2. move
    if is_key_down(KeyCode::W) { context.pos += look_dir * move_speed; }
    if is_key_down(KeyCode::S) { context.pos -= look_dir * move_speed; }

    let right = look_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();
    if is_key_down(KeyCode::D) { context.pos += right * move_speed; }
    if is_key_down(KeyCode::A) { context.pos -= right * move_speed; }

    // 3. control mouse cursor
    if is_key_pressed(KeyCode::C) { set_cursor_grab(false); show_mouse(true); }
    if is_key_pressed(KeyCode::Z) { set_cursor_grab(true); show_mouse(false); }
}

fn get_look_dir(yaw: f32, pitch: f32) -> Vec3 {
    vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos()
    ).normalize()
}

fn draw_sight(_world: &mut World, _context: &mut AcornContext) {
    draw_circle(
        screen_width()/2.0, 
        screen_height()/2.0, 
        5.0, 
        YELLOW
    )
}

fn add_acorns(world: &mut World, context: &mut AcornContext) {
    let mut changed = false;

    if is_key_pressed(KeyCode::Q) {
        context.acorns_x += 1;
        changed = true;
    }
    if is_key_pressed(KeyCode::E) {
        context.acorns_y += 1;
        changed = true;
    }
    // delete acorns
    if is_key_pressed(KeyCode::R) && context.acorns_x > 1 {
        context.acorns_x -= 1;
        changed = true;
    }

    if changed {
        context.acorns_count = context.acorns_x*context.acorns_y;
        // despawn old acorns
        let mut query = world.query_filtered::<Entity, With<IsAcorn>>();
        let entities: Vec<Entity> = query.iter(world).collect();
        for entity in entities {
            world.despawn(entity);
        }

        acorn_game_spawn_acorn(world, context);
        
        println!("Grid resized to: {}x{}", context.acorns_x, context.acorns_y);
    }
}

fn draw_statistics(_world: &mut World, context: &mut AcornContext) {
    draw_text(
        &format!("FPS: {}", get_fps()),
        40.0, 
        20.0, 
        20.0, 
        YELLOW
    );
    draw_text(
        &format!("Acorns: {}", context.acorns_count),
        40.0, 
        40.0, 
        20.0, 
        YELLOW
    );
}

#[macroquad::main("Light Acorn test")]
async fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Global variable of Zones. Hand over to acorn_loop.
    let mut acorn_context = acorn_setup();

    // Create entities here (or in runtime by your logic)
    // acorn_example_spawn_entity(&mut acorn_ecs.world, &mut acorn_context);
    acorn_game_spawn_acorn(&mut acorn_ecs.world, &mut acorn_context);

    // main loop
    acorn_loop(acorn_context, acorn_ecs).await;
}
