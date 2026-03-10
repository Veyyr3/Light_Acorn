# The Light Acorn Framework

<img title="" src="/DOCS/imgs/logo_Light_Acorn.png" alt="" width="168" data-align="center">

Hi! This was done on an old 2013 X550CC laptop and antiX.

# Features

- **Light Acorn:** A lightweight Zone & Location-based framework for creating games for old PC, powered by Macroquad and Bevy ECS. 
  

- **Light Acorn is Functional Data-Oriented Framework by design.**
  

- **This framework is not crate. It is template for your projects.**
  

- **Minimum entry threshold:** you don't need to lifetime fighting 'a and 'b, to know complex macros and smart pointers for begin creating your games.
  
  

- **Acorn Architecture:** The entire framework skeleton is **built exclusively on vectors and loops**. Not a single unsafe, not a single smart pointer, not a single complex macro in the core. (Only 1 unsafe for GL contex in tools and vec! macros).
  
  

- **Bevy ECS include but optional.**
  

- **Zone & Location is unique concept:** The grouping of functions and their order is the basis of the engine. Developer control code's order, grouping functions by Zone (group of Locations) and Location (group of functions). Developer can create own Zones or Locations in Kernel: custom Zones with custom execution order.
  

- **Developer is owner his code:** You can **change functions order** in Locations. You can change Location's order in Zones. And **YOU CAN do it all in runtime** **WITHOUT**: unsafe blocks, smart pointers, macros and you shouldn't linking your code with Python, Lua (REACORN-way). PS: Only vectors and syntactic sugar macros.
  

- **Developer has choice of architecture Light Acorn:** predicatable monolith **(ACORN)** OR flexible change of the order of execution of functions **(REACORN)**
  

- **Game built-in tools for create apps:** There are functions, structs, variables etc. which developer can use for creating. **OR DELETE** this tools **and** **create own**. Light Acorn doesn't stop developer to choose libraries or tools.

# Stack

- Rust.

- Macroquad (main render).

- Bevy ECS.

- Tobj (to parse .obj files).

# Quick Start

1. Light Acorn is a **template**. To start, clone the repository:

```bash
# For Linux & Windows (Git Bash / PowerShell)
git clone https://github.com/Veyyr3/Light_Acorn.git
cd Light_Acorn
```

2. And run the demo:

```bash
cargo run
```

3. See result!

<img src="/DOCS/imgs/play.png" title="" alt="" data-align="center">

# Code's examples

Create Simple function:

```rust
fn acorn_example_greeting(_world: &mut World, _context: &mut AcornContext) {
    print!("Hello, Light Acorn!")
}
```

Add function to Zone and Location:

```rust
// before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
let before_2d_zone = Zone::default()
.with_locations(vec![
	Location::from_fn_vec(vec![
		// simple function
		acorn_example_greeting,
		// add own functions through comma 
	]),
	// add own locations through comma 
]);
```

**All code examples are already in the template!**

# The Light Acorn Future

- Add GPU instansing.

- Integrated UI tools.

**And you can help to improve Light Acorn!**

# Acknowledgments

**I express my gratitude to the authors:**

1. **Rust:** For the difficulty of learning.

2. **Macroquad:** For ease of learning. 

3. **Bevy ECS:** For ease of learning (15 times easier than OOP) and Data-Oriented.

4. **tobj:** For a simple and reliable way to bring 3D models from Blender into the Acorn.

5. **Blender:** For the opportunity to create.

6. **antiX:** For giving the opportunity to continue working on old laptop.

7. **AI:** For the opportunity to do the impossible.

# License

This project is dual-licensed under the **MIT License** and the **Mozilla Public License 2.0**
