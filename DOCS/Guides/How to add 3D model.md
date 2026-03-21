## **How to add your 3D model from Blender to Light Acorn.**

#### 1 Step: left-click to select your model (make sure that Object Mode is on).

<img title="" src="/DOCS/imgs/how_to_add_3D_model/choose_your_model_by_left_click.png" alt="" data-align="center">

#### 2 Step: File --> Export --> Wavefront (.obj)

<img title="" src="/DOCS/imgs/how_to_add_3D_model/export_to_obj.png" alt="" data-align="center">

**Description:** tobj in Light Acorn works only with .obj files. It simple but you can't add animations to your model.

#### 3 Step: check the box "Triangulated Mesh" --> name your model --> export.

<img title="" src="/DOCS/imgs/how_to_add_3D_model/exporting.png" alt="" data-align="center">

#### 4 Step: find your .obj with .mtl files and copy to root Light Acorn.

<img title="" src="/DOCS/imgs/how_to_add_3D_model/create_folder.png" alt="" data-align="center">

#### 5 Step: add your model in code inside `acorn_gsetup.rs`.

```rust
// ---------------------------- Game setup ----------------------------
// Keep 3d models in assets database.
let mut assets_3d = Acorn3DAssetDatabase {meshes: Vec::new()};

// Add your .obj files with push.
// PLEASE, remember index of your 3d models when you add news.
// It so, because for perfomance. 
// BUT I leave it to you for organize logic assets keeping.
assets_3d.meshes.push(
    load_obj_with_materials_to_mesh("src/acorn_tools/acorn_game_tools/objs/acorn_engine.obj")
);

AcornGlobalContext { 
    // suggestion for game
    assets_3d
}
```

Add your model with `load_obj_with_materials_to_mesh` function through pushing into `Acorn3DAssetDatabase`.

**PLEASE**, remember index of your 3d models when you add news OR use a trick below in the code. **Right now index of 3D model is 0.**

#### 6 Step: create function to spawn your 3D entity.

```rust
fn spawn(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    world.spawn((
       Entity3DTransform {
            position: vec3(0.0, 1.0, 0.0),
            rotation: 0.0,
            scale: vec3(1.0, 1.0, 1.0)
       }, 
       Entity3DModel {
            // WARNING: you should remember index of your 3d model
            mesh_id: 0 

            /*
            But you can use a trick:

            // src/game_assets.rs
            pub const ACORN_MODEL: usize = 0;
            pub const TREE_MODEL: usize = 1;
            pub const ROCK_MODEL: usize = 2;

            AND write like that:
            mesh_id: ACORN_MODEL
            */
       },
    ));
    println!("Entity spawned!");
}
```

#### 7 Step: add your `spawn` function 

**Into `main` like this:**

```rust
#[macroquad::main("Light Acorn test")]
async fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Contex of Zones. Hand over to acorn_loop.
    let mut acorn_zone_context = acorn_zone_setup();
    // Global states. Hand over to acorn_loop.
    let mut acorn_global_context = acorn_global_setup();

    // Create entities here (or in runtime by your logic)
    spawn( // <-- YOUR FUNCTION
        &mut acorn_ecs.world, 
        &mut acorn_zone_context, 
        &mut acorn_global_context
    );

    // main loop
    acorn_loop(acorn_ecs, acorn_zone_context, acorn_global_context).await;
}
```
**In `acorn_zone_setup` inside `acorn_zsetup.rs` (if you want to runtime spawn by events):**

```rust
let before_2d_zone = Zone::default()
    .with_locations(vec![
        Location::from_fn_vec(vec![
            spawn, // <-- your function
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);
```

#### 8 Step: see result!

<img title="" src="/DOCS/imgs/how_to_add_3D_model/result.png" alt="" data-align="center">
