use std::{io::*};
pub mod page;
//mod macros;
pub use page::{Page, Action, DefaultAction};


/// Display struct
pub struct Display{
    page_index: usize,
    last_index: usize,
    back_index: Vec<usize>,
    pub page_buffer: Vec<Page>,
    width: usize,
    height: usize
}

pub trait Input {
    
     fn parse_input(&self)->Response {
        let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim(){
                "q" | "Q" | "quit" | "e" | "exit" | "x"=>{Response::Exit},
                "b" | "B" | "back" | "Back" =>{Response::Back},
                "h" | "H" | "home"| "Home"  =>{Response::Home},
                "m" | "M" | "menu"| "Menu"  =>{Response::Menu},
                x =>{
                    let args:Vec<&str> = x.split(' ').collect();
                    if args.len() >1{
                        Response::Commands(args.iter().map(|x| x.to_owned().to_string()).collect())
                    }else
                    {
                        if let Ok(num) = x.parse::<usize>(){
                            Response::Alt(num)}else{Response::Home
                            }
                        }
                    },
                _=>{Response::Home}
            }
        
    }
}



impl Input for Display{
    
}

impl Display{
    pub fn new()->Self{
        Display{
            page_index:0,
            last_index:0,
            back_index:Vec::new(),
            page_buffer:Vec::new(),
            width:90,
            height:20
        }
    }
    pub fn set_width(&mut self, width:usize){
        self.width = width;
        for elem in self.page_buffer.iter_mut() {
            elem.set_width(width);
        }
    }
    pub fn set_height(&mut self, height:usize){
        self.height = height;
        for elem in self.page_buffer.iter_mut() {
            elem.set_height(height);
        }
    }

    pub fn get_page_index(&self)->usize{
        self.page_index
    }

    pub fn build_menu_page(&mut self)->usize{
        let mut headers = self.page_buffer.iter().map(|page|(page.get_title().to_owned())).collect::<Vec<String>>().to_owned();
        let copy = headers.iter_mut().map(|f|f.as_str()).collect::<Vec<_>>();
        
        self.new_page("Page Menu", copy.as_slice(), &["info"], "Go to page", Box::new(move |_,res|{res}) )
    }

    pub fn set_page(&mut self, index: usize){
        if index < self.page_buffer.len(){
            self.last_index = self.page_index;
            self.back_index.push(self.last_index);
            self.page_index = index;
        }
    }
    pub fn get_page(&mut self, idx:usize)->&mut Page{
            &mut self.page_buffer[idx]
    }
    pub fn add_page(&mut self, page:Page)->usize{
        self.page_buffer.push(page);
        self.page_buffer.len()-1
    }

    pub fn back(&mut self){
        let i = self.page_index;
        if let Some(index) = self.back_index.pop(){
            self.last_index=index;
        }
        self.page_index = self.last_index;
        self.last_index = i;

    }
    

    pub fn show(&mut self)->Response{
        print!("\x1B[2J");
        std::process::Command::new("cmd")
        .args(&["/Q","/C", "cls"]).status().expect("failed to clear screen");
 
        let mut out = std::io::stdout();
        //out.write_all(b"\x1B[2J");
        let page = self.page_buffer[self.page_index].rows().iter().fold(String::from(""),|x,y|{x + y});
        let _ = out.write_all(page.as_bytes());        
        let _ = std::io::stdout().flush();
        
        match (self.page_buffer[self.page_index].action)(&self, self.parse_input()){
            Response::Menu=>self.show_menu_page(),
            x=>x
        }
        
    }
    pub fn show_menu_page(&mut self)->Response{
        let index = self.build_menu_page();
        print!("\x1B[2J");
        std::process::Command::new("cmd")
        .args(&["/Q","/C", "cls"]).status().expect("failed to clear screen");
        
        
        let mut out = std::io::stdout();
        
        let page = self.page_buffer[index].rows().iter().fold(String::from(""),|x,y|{x + y});
        let _ = out.write_all(page.as_bytes());
        
        let _ = std::io::stdout().flush();
        
        
        match (self.page_buffer[index].action)(&self, self.parse_input()){
            Response::Alt(x) => {
                self.page_buffer.remove(index);
                self.set_page(x);
                Response::Page(x)},
                //self.page_index = x;Response::Alt(x)},
            _=>{
                self.page_buffer.remove(index);
                Response::Back}
        }
        
    }
    pub fn menu(&mut self,title: &str, options: &[&str], info: &[&str],query: &str,action:Action)->usize{
        self.new_page(title, options,info, query, action);
        self.last_index = self.page_index;
        self.back_index.push(self.last_index);
        self.page_index = self.page_buffer.len()-1;
        self.page_index
    }
    pub fn new_page(&mut self,title: &str, options: &[&str], info: &[&str],query: &str, action: Action)->usize{
        self.page_buffer.push(Page::new_page(title, options, info, query,self.width, self.height, action));
        self.page_buffer.len()-1
    }
    pub fn new_info_page(&mut self,title: &str, info: &[&str], query: &str, action: Action)->usize{
        self.page_buffer.push(Page::new_page(title, &[], info, query, self.width, self.height, action));
        self.page_index = self.page_buffer.len()-1;
        self.page_buffer.len()-1
    }
   
} 



impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}


/// ## Response
/// The response enum is returned from each page after user input.
/// currently the internal Response handling handles:
/// ```rust ignore
///     "q" | "Q" | "quit" | "e" | "exit" | "x"=>{Response::Exit}, 
///     "b" | "B" | "back" | "Back" =>{Response::Back}, 
///     "h" | "H" | "home"| "Home"  =>{Response::Home}, 
///     "m" | "M" | "menu"| "Menu"  =>{Response::Menu}, 
/// ```
///     
/// if the user response is a numeric it will send ` Response::Alt(num) `
/// 
///     the menu response is also handled internally and will create and
///     show a page index page and also handle any numeric response from that page
/// ### Example
/// ```rust ignore
/// //handle in a match statement
/// let response = Response::Alt(1);
/// match response{
///     Response:Alt(i) => display.set_page(i),
///     _=>()
/// }
/// ```
pub enum Response{
    Page(usize),
    Alt(usize),
    Exit,
    Back,
    New(String, Vec<String>, Vec<String>, String, Action),
    NewInfo(String, Vec<String>, String, Action),
    Home,
    Commands(Vec<String>),
    Menu
} 

#[test]
fn name() {
   
    use crate::display::*;

        
    let mut disp = Display::new();

    let my_page = disp.new_page("testing title", &["option 1", "option 2"], &["this is some info"], "query", Action::default());
    let my_page2 = disp.new_page("testing title2", &["option 1", "option 2", "option 3"], &["this is some info on the subject","in several lines"], "answer", Box::new(|_disp,res|{res}));
    
    let mut my_home_page = Page::build_page("home",disp.width,disp.height).set_page_info_from_slice(&["this is some info","on the home page"]);
    my_home_page.set_help("this is wome friendly help text, press q to quit, m for menu");
    my_home_page.set_query("what do you want to do?:");
    my_home_page.set_height(25);
    let home =disp.add_page(my_home_page);
    /* loop{
        
        match disp.show(){
            
            Response::Alt(x) => {let t = Page::build_page(&format!("this is page {}",x),disp.width,disp.height);disp.add_page(t);},
            Response::Exit => break,
            Response::Back => disp.back(),
            Response::Page(x)=>disp.set_page(x),
            Response::Home => disp.set_page(home),
            Response::Commands(_) => todo!(),
            _=>(),
            }
        };  */
}