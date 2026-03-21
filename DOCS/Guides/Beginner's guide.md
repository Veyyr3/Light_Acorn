# Introduction

Hi!
 
**Before we started** find info about: Zone, Location, REACORN and ACORN ways etc. in `How Acorn works.md`.

So... Let's start.

# The Philosophy

> Any program is simple loop until we say: "Stop!".

And Light Acorn is simple loop where you create program by put there your ideas (of course you can change code's order in runtime).

**Light Acorn has several interface files:**
* `acorn_gsetup.rs`: Here you define Global States (like player score or 3D assets).
* `acorn_zsetup.rs`: This is where you define the order of execution.
* `acorn_settings.rs`: General configuration for your engine

**These files allow to YOU control code and code's order in loop.**

In Light Acorn Main loop is `acorn_loop` where You put parts of your code.

> **The Minimal part of code is Function.**

# The First Code

Now I'll show **how to write code in the REACORN-way.**

### 1 Step

The First file which you should visit is `acorn_zsetup.rs`. It means: *acorn zone setup*. There you create functions and add to Zones, Locations.

**Create function by this template:**
```rust
fn name(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // your_logic
}
```

**Remember:** any function in Zones should have 3 arguments: `world`, `zones`, `context` but it is not required to use them!

That's why in `acorn_heart`: 
```rust
pub type AcornFunction = fn(&mut World, &mut AcornZoneContext, &mut AcornGlobalContext);
```

**For tutorial I'll create own function:**
```rust
fn acorn_example_greeting(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    print!("Hello, Light Acorn!");
}
```

Where Arguments:
* **world** - for ECS Queries. This is necessary in order to process thousands of objects.
* **zones** - for Lord-Functions. This is necessary for control Minor-Locations.
* **context** - for Global States. Example: player score, 3d assets.

### 2 Step

Next, **you should decide where your Function will run**: 
* **Zone before 2d render**: here contains handle input, ECS logic, events, 3D rendering.
* **Zone after 2d render**: here contains UI drawing and other functions.

**Why?**

**Because:**
* In `acorn_loop` there is `set_default_camera` function in the middle of loop which turn on 2D rendering. **If you add 3D draw into Zone after 2d render**, this can lead to z-buffer errors (when your character is actually above the ground, but the ground is drawn above him anyway).

My `acorn_example_greeting` function doesn't care about where run because it print only greeting `^_^`. But anyway you should understand the problem.

For example I'll add `acorn_example_greeting` into before_2d_zone inside `acorn_zone_setup` function (`acorn_zsetup.rs` file):
```rust
let before_2d_zone = Zone::default()
    .with_locations(vec![
        Location::from_fn_vec(vec![
            acorn_example_greeting,
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);
```

And... **It's ALL!** Your function will execute every frame.

# Global Setup

For example you need to store player's score.

1. Open `acorn_settings.rs`
2. Add your new field `pub score: u8` inside struct `AcornGlobalContext`:

```rust
AcornGlobalContext { 
    pub score: u8,
}
```

3. Open `acorn_gsetup.rs` and write inside `acorn_global_setup` function:

```rust
AcornGlobalContext { 
    score: 64
}
```

4. Use new field in any functions through `context.score`:

```rust
fn print_score(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    context: &mut AcornGlobalContext
) {
    println!("{}", context.score);
}
```

Yes, it's very simple!

**Advise:** If you have many fields (score, health, ammo), you can group them into sub-structs inside `AcornGlobalContext` to keep your code clean.

Like this:

```rust
struct PlayerStats {
    pub score: u8,
    pub health: u8,
    pub ammo: u8, // if minigun use u16 :)
}

pub struct AcornGlobalContext {
    pub player_stats: PlayerStats,
}

// and use in functions: context.player_stats.score
```