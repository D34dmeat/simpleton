extern crate simpleton;
use crate::simpleton::display::DefaultAction;
use simpleton::display::{Display, Response, Page, Action};


fn main(){
    let mut my_display = Display::new();
    let home = my_display.add_page(Page::new_with_title("This is the home page", Action::default()));
    build_pages( &mut my_display);
    loop {
        
        match my_display.show() {
            Response::Alt(x) => {let t = Page::build_page(&format!("this is page {}",x));my_display.add_page(t);},
            Response::Exit => break,
            Response::Back => my_display.back(),
            Response::Page(x)=>my_display.set_page(x),
            Response::Home => my_display.set_page(home),
            Response::Commands(x) if x[0] == "Page" => if let Ok(num) = x[1].parse::<usize>(){my_display.set_page(num)},
            Response::Commands(x) => {println!("commands {:?}",x);break},
            _=>(),
        }
    }    
    
}

fn build_pages(disp:&mut Display){
    let my_page = disp.new_page("testing title", &["option 1", "option 2"], &["this is some info"], "query", Box::new(|disp,res|{res}));
    let my_page2 = disp.new_page("testing title2", &["option 1", "option 2", "option 3"], &["this is some info on the subject","in several lines"], "answer", Box::new(|disp,res| -> Response {
        match res {
            Response::Alt(x) =>{
                // do some actions here based on this page's options
                match x {
                    0 => {println!("0");res}
                    _=>res
                }
            },
            _=>res
        }
    }));
    
}