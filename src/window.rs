/**
 * @author Leviathenn
 */

extern crate kiss3d;

 use kiss3d::window::Window;

    pub struct WindowOptions{
        pub(crate) title: &'static str,
    }
    pub fn create_window(options: WindowOptions) -> Window{
        return Window::new(options.title);
    }
  