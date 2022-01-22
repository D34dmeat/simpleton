//! # Simpleton
//! The simpletons tui
//! 
//! A simple tui with no actual features
//! 

pub mod display;
pub use display::{Display,Page,Response,Action,DefaultAction};




#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        use crate::display::*;
        let (width,height) = (90,20);
        let my_page = Page::new_with_title("hello",Action::default());
        let mut my_display = Display::default();
        let my_page_index = my_display.add_page(my_page);
        my_display.set_width(width);
        my_display.set_height(height);
                
        assert_eq!(my_display.get_page(my_page_index).get_title(), "hello".to_string());
    }
}
