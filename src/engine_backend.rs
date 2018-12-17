/*
 * Copyright (c) 2018. Anthony Callahan.
 */


/// Everything in this file is backend work such as a mesh importer
/// Three-rs custom functions
/// Terrain support
/// Custom Materials
/// Custom material pickers
/// debugging functions

extern crate three;
use three::Object;
use std::env;

pub fn make_square() -> three::Geometry {
    /// Creates a plane that is 1 unit wide and 1 unit tall.
    three::Geometry::plane(1.0, 1.0)
}

pub fn load_Obj () {
    let mut args = env::args();
    let obj_path = concat!(env!("CARGO_MANIFEST_DIR"), "/meshes/MaleLow.obj");
    let path = args.nth(1).unwrap_or(obj_path.into());
}