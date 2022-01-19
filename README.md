# Simpleton - The simpletons tui
 
 ![](https://github.com/d34dmeat/simpleton/workflows/simpleton%20Build/badge.svg) ![](https://github.com/d34dmeat/simpleton/workflows/simpleton%20Examples/badge.svg) ![Simpleton DocTest](https://github.com/d34dmeat/simpleton/workflows/docTest/badge.svg)

 Simpleton is a simple, no bells and whistles tui
 and to be perfectly clear... it's not in a usable state, so don't use it!

## Example
 ```rust ignore
extern crate simpleton;
use simpleton::*;

fn main(){
    let mod my_display = Display::new();
    let mod first_page = Page::build_page("Title")
                            .set_action(Box::new(move|display,response|{
                                match response {
                                    Response::Alt(num) => {Response::page(num)},
                                    _=>response
                                }
                            }));

    let first_page_index = my_display.add_page(first_page);

    loop {
        match my_display.show() {
            Response::Exit => break,
            _=>()
        }
    }
}


 ```

