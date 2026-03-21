# How Acorn works

## The Acorn Architecture

<img src="/DOCS/imgs/how_it_works/zone_location.png" title="" alt="" data-align="center">

**The Acorn Architecture** consists of hierarchy: **functions** are the minimum units. **Location** is the container of functions. **Zone** is the container of Locations. 

> **In timeline functions execute by order**.

**In code:** Zone, Locations are vectors. Yes, it's simple.

In **REACORN-way** developers use indices to access the context to change functions order. So, developer should create protection to check array boundaries.

Also in **REACORN-way** functions execute vice versa (to protect 101 error in runtime).

**Here an example of Acorn Function (REACORN-way):**

```rust
fn name(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // your_logic
}
```

* `world` - for ECS Queries. This is necessary in order to process thousands of objects.
* `zones` - for Lord-Functions. This is necessary for control Minor-Locations.
* `context` - for Global States. Example: player score, 3d assets.

## The Acorn Pipeline

<img src="/DOCS/imgs/how_it_works/acorn_pipeline.png" title="" alt="" data-align="center">

In `acorn_zsetup` (acorn zone setup) developer create Zones, Locations and add here his Functions. 

In `acorn_gsetup` (acorn global setup) developer create Global States which Functions use and change in runtime.

`main` function use the data from `acorn_zsetup`, `acorn_gsetup` and hands over to `acorn_loop`.

## The Light Acorn Loop

<img src="/DOCS/imgs/how_it_works/how_to_use.png" title="" alt="" data-align="center">

**Light Acorn Loop** based on **Macroquad async function**. Developer uses in the Light Acorn API 2 zones: **before** and **after** turning on 2d render. Zone is container of own developer's locations. Location is container of own developer's functions. 

**Function should be** `AcornFunction` to run in `acorn_loop`. But it **can contents anything**: Macroquad functions, bevy_ecs Queries, function of other library or custom function of developer. Of course, developer can add own Zones (see in other doc).

> **Actually Light Acorn is Macroquad with architecture.**

**Features:** Developer can change function's order in locations. Developer can change location's order in zones. Developer can change order of zones. In ACORN-way developer can't reorder functions in runtime. In REACORN-way developer can reorder functions in runtime.

## The Acorn ECS

<img src="/DOCS/imgs/how_it_works/acorn_ecs.png" title="" alt="" data-align="center">

**There are Acorn rules:**

1. All functions are time-markers.
2. All functions can change data only through ECS-window or Global Context.

Acorn ECS based on Bevy ECS. Actually ECS here is data base. 

> **ECS-Queries are here like SQL-Queries in SQLite.**

**Close-rooms architecture:** Functions don't know about each other, don't know who changed data because they aren't linked by data.

I said that All functions go by order and so:

> **In Acorn in one moment executing only 1 function**. 

This removes the data race problem, but nobody can't stop developer: **you can put several parallel ECS-Queries in 1 function** *(by use std::threads or other libraries)*. After this, function ended and acorn loop goes on.

**Here is an example ECS-function:**

```rust
fn acorn_example_update_oaks(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // create query
    let mut query = world.query::<&mut Oaks>();

    // loop for all entities. 
    // Spoiler: game will be over when oaks reach 18 446 744 073 709 551 615 :)
    for mut oaks in query.iter_mut(world) {
        oaks.x += 1; 
    }
}
```

## What are ACORN-way and REACORN-way?

**It's simple:**

- **ACORN-way:** you can't reorder functions in runtime. But you are absolutely sure that functions will execute by your order. Also compilator can make inlining functions to get more perfomance.

- **REACORN-way:** you can reorder funtions in runtime because all functions contain in vectors. *Yes, it is simple way to get Metaprogramming in Rust*. But **you should be disciplined** to avoid 101 erorrs in runtime. 

And so... next **I offer to you new architecture: Lord-Minor to be disciplined.**

## The Lord-Minor Architecture

<img src="/DOCS/imgs/how_it_works/lord_minor.png" title="" alt="" data-align="center">

**It is only for REACORN-way.** It is feudalism of functions to protect discipline.

**Lord-Location:** here live **Lord-Functions** which can change other functions order in **Minor-Locations**.

> **For each Lord-Functions you should create own Minor-Location.**

**Minor-Location:** here are functions which obey to Lord-Function. They listen him and die, move or born by his orders.

**Example:** if there are 3 Lord-Functions in Lord-Location then 3 Minor-Locations for each Lord-Functions.

> **OR Just Memorise:** ***One Lord-Function = One his Minor-Location.***

But **YOU are not required to use this architecture. You are Lord of your ideas.**
