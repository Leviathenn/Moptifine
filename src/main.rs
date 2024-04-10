/**
 * @author Leviathenn
 */

mod window;
use window::WindowOptions;

fn main() {
    let options = window::WindowOptions { title: "My Window" ,render: render};
    
    let mut window = window::create_window(options);
    
    fn render(){
        println!("Window Has been rendered!");
    }

}
