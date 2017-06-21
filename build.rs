extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Registry, Profile, Fallbacks, GlobalGenerator};

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let path = Path::new(&dest).join("gl_bindings.rs");
    let mut file = File::create(&path).unwrap();

    let ext = [
        
    ];

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, ext)
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}