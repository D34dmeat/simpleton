extern crate simpleton;
use simpleton::display::{Display, Response};


fn main(){
    let mut my_display = Display::new();

    build_pages( &mut my_display);
    loop {
        
        match my_display.show() {
            Response::Page(x) => my_display.set_page(x),
            Response::Alt(x) => {println!("Response Alt is {} from page {} ",x, my_display.get_page_index());break},
            Response::Exit => break,
            Response::Back => (),
            Response::New(_, _, _, _, _) => todo!(),
            Response::NewInfo(_, _, _, _) => todo!(),
            Response::Home => (),
            Response::Commands(x) if x[0] == "Page" => if let Ok(num) = x[1].parse::<usize>(){my_display.set_page(num)},
            Response::Commands(x) => {println!("commands {:?}",x);break},
            Response::Menu => {
                let res = my_display.show_menu_page();()
    
                
            },
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