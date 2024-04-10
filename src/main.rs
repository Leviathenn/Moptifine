/**
 * @author Leviathenn
 */

mod window;
use window::WindowOptions;
fn main() {
    let options = window::WindowOptions { title: "My Window" };
    let window = window::create_window(options);
}