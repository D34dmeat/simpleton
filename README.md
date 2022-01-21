# Simpleton - The simpletons tui
 
 ![](https://github.com/d34dmeat/simpleton/workflows/simpleton%20Build/badge.svg) ![](https://github.com/d34dmeat/simpleton/workflows/simpleton%20Examples/badge.svg) ![Simpleton DocTest](https://github.com/d34dmeat/simpleton/workflows/docTest/badge.svg)

 Simpleton is a simple, no bells and whistles tui
 and to be perfectly clear... it's not in a usable state, so don't use it!

## Example
 ```rust ignore
extern crate simpleton;
use simpleton::*;

fn main(){
    let mod disp = Display::new();
    let mod first_page = Page::build_page(&disp, "Title")
                            .set_action(Box::new(move|display,response|{
                                match response {
                                    Response::Alt(num) => {Response::page(num)},
                                    _=>response
                                }
                            }));

    let home = disp.add_page(first_page);

    loop {
        match disp.show(){
                                         //adding more pages dynamically
            Response::Alt(x) => {let t = Page::build_page(&disp, &format!("this is page {}",x));disp.add_page(t);},
            Response::Exit => break,
            Response::Back => disp.back(),
            Response::Page(x)=>disp.set_page(x),
            Response::Home => disp.set_page(home),
            Response::Commands(_) => todo!(),
            _=>(),
    }
}


 ```

