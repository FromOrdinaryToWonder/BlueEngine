/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use blue_engine::{header::{Engine, ObjectSettings, WindowDescriptor}, objects::two_dimensions::triangle};

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    let _ = triangle(ObjectSettings::default(), &mut engine).unwrap();

    engine
        .update_loop(move |_, _, _, _, _| {})
        .expect("Error during update loop");
}
