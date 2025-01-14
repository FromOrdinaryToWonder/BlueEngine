/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

//! <img src="https://raw.githubusercontent.com/AryanpurTech/BlueEngineDocs/master/resources/logo_3d.gif" loop=infinite width="100%" />
//!
//! # Blue Engine
//!
//! Blue Engine is an easy to use, portable, and extendable/customizable graphics engine. Here
//! lives the documentation for the engine.
//!
//! ## Setup
//!
//! The setup and installation details live in the project's [guide](https://aryanpurtech.github.io/BlueEngineDocs/).
//! A basic program in Blue Engine is as follow:
//!
//! ## Example
//!
//! ```rust
//! use blue_engine::{
//!     header::{ Engine, ObjectSettings },
//!     primitive_shapes::triangle
//! };
//!
//! fn main() {
//!     // initialize the engine
//!     let mut engine = Engine::new().expect("engine couldn't be initialized");
//!
//!     // create a triangle
//!     triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects).unwrap();
//!
//!    // run the engine
//!    engine
//!        .update_loop(move |_, _, _, _, _, _| {})
//!        .expect("Error during update loop");
//! }
//! ```
//!
//! ## Utilities
//!
//! This crate is the core of the engine, but there is also [utilities crate](https://github.com/AryanpurTech/BlueEngineUtilities)
//! which have a lot of utilities for the engine such as lighting, physics, etc.
//
// ## Guide for code navigation
//
// The code of the engine is organized in a rather different manner than traditional in the
// language. There are inspirations from other languages to make it easier to navigate the
// project.
//
// * `header` - contains all the declarations such as structs, exports, enums, ...
//
// The other files contain definitions for each part.
//
// * `objects` - contains the definition for Object type, which is a type that make it easier to manage data for rendering.
// * `primitive_shapes` - contains definition for some 2D and 3D shapes. They are basic shapes and
// can be used as examples of how to create your own content.
// * `render` - contains definition for rendering part of the engine.
// * `utils` - Utilities for the engine (soon moving to it's own
// [crate](https://github.com/AryanpurTech/BlueEngineUtilities)).
// * `window` - contains definition for creation of window and instance creation.

pub(crate) mod definition;
pub mod header;
pub mod objects;
pub mod primitive_shapes;
pub mod render;
pub mod utils;
pub mod window;
pub use crate::header::*;
