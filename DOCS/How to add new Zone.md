# How to add new Zone

## REACORN-way

**1 Step: open acorn_settings.rs and add Zone**

```rust
// src/acorn_kernel/acorn_settings.rs
use crate::acorn_kernel::{
    acorn_heart::Zone, 
    // suggestions
    acorn_tools::acorn_game_tools::agt_heart::Acorn3DAssetDatabase
};

/// Contain here your Zones and global statements 
/// 
/// Advise: better keep global statements in other struct.
pub struct AcornContext {
    pub before_2d_zone: Zone,
    pub after_2d_zone: Zone,
    // add here your Zone trough comma
    // from game tools
    pub assets_3d: Acorn3DAssetDatabase,
}
```

**2 Step: add new loop in acorn_render.rs into acorn_loop function by this template:**

```rust
let len_your_zone = acorn_context.your_zone.locations.len();

// locations go by order
for location_index in 0..len_your_zone {
    let fn_count = acorn_context
        .your_zone
        .locations[location_index]
        .functions.len();

    // Reverse cycle for protect from panic (101 errors) in runtime
    // Functions go by reverse order
    // Warning: You should add new functions from down to top
    for fn_index in (0..fn_count).rev() {
        let function = 
        acorn_context.your_zone
            .locations[location_index]
            .functions[fn_index];
            
        // Call function in strict order
        function(&mut acorn_ecs.world, &mut acorn_context);
    }
}
```

**3 Step: in acorn_setup add your Zone**

```rust
let your_zone = Zone::default()
    .with_locations(vec![
        // Lord-Location
        Location::from_fn_vec(vec![
            acorn_example_add_circle_function
        ]),
        // Minor-Location
        Location::from_fn_vec(vec![
            acorn_debug_inspector
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

// ***
// in the end of acorn_setup
// ***

// Return AcornContext for Main function
    AcornContext { 
        before_2d_zone, 
        after_2d_zone,
        your_zone,
    }
```

**And it's all!**

## ACORN-way

**1 Step: open acorn_render.rs and add new loop in acorn_render.rs into acorn_loop function by this template:**

```rust
for location in &your_zone.locations {
    for function in &location.functions {
        function(&mut ecs.world); // Call function in strict order
    }
}
```

**2 Step: update acorn_loop**

```rust
pub async fn acorn_loop(before_2d_zone: Zone, after_2d_zone: Zone, your_zone: Zone, mut ecs: AcornECS)
```

**3 Step: update acorn_setup**

```rust
fn acorn_setup() -> (Zone, Zone, Zone)
```

**4 Step: in acorn_setup add your Zone**

```rust
let your_zone = Zone::default()
    .with_locations(vec![
        // test location
        Location::from_fn_vec(vec![
            acorn_example_draw_circle,
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

// ***
// in the end of acorn_setup
// ***

// Return AcornContext for Main function
// Return tuple of Zones for Main function
(before_2d_zone, after_2d_zone, your_zone) 
```

**5 Step: update main function**

```rust
#[macroquad::main("Light Acorn test")]
async fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Global variable of Zones. Hand over to acorn_loop.
    let (before, after, your) = acorn_setup();

    // main loop
    acorn_loop(before, after, your, acorn_ecs).await;
}
```

**And it's all!**


