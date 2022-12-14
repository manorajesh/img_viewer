extern crate kiss3d;

use kiss3d::nalgebra::{Translation2, UnitComplex, UnitQuaternion, Vector3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use std::path::Path;
fn main() {
    let mut window = Window::new("Kiss3d: texturing");

    let mut c = window.add_cube(1.0, 1.0, 1.0);
    c.set_color(1.0, 0.0, 0.0);
    c.set_texture_from_file(Path::new("orange_v7.png"), "kitten");

    let mut r = window.add_rectangle(100.0, 100.0);
    r.append_translation(&Translation2::new(-100.0, -100.0));
    r.set_color(0.0, 0.0, 1.0);
    r.set_texture_from_memory(include_bytes!("../orange_v7.png"), "kitten_mem");

    window.set_light(Light::StickToCamera);

    let rot3d = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let rot2d = UnitComplex::new(0.01);

    while window.render() {
        c.append_rotation(&rot3d);
        r.prepend_to_local_rotation(&rot2d)
    }
}