/**
 * @author Leviathenn
 */

extern crate kiss3d;

 use kiss3d::window::Window;

    pub struct WindowOptions{
        pub(crate) title: &'static str,
        pub(crate) render: fn(),
    }
    pub fn create_window(options: WindowOptions) -> Window{
        let mut nwindow = Window::new(options.title);
        window_loop_create(options, &mut nwindow);
        return nwindow;
    }
    fn window_loop_create(option:WindowOptions, nwindow: &mut Window){
        while nwindow.render(){
            (option.render)();
        }
    }
  