use kiss3d::nalgebra::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Untitled Game");
    let path = std::env::current_dir().unwrap();
    while window.render() {
        
    }



}