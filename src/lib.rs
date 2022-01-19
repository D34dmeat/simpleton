//! # Simpleton
//! The simpletons tui
//! 
//! A simple tui with no actual features
//! 

pub mod display;
use display::*;




#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        use crate::display::*;
        let my_page = Page::new_with_title("hello",Action::default());
        let mut my_display = Display::default();
        let my_page_index = my_display.add_page(my_page);
                
        assert_eq!(my_display.get_page(my_page_index).title, "hello".to_string());
    }
}
