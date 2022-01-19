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
                "b" | "back" | "Back" =>{Response::Back},
                "h" | "home"| "Home"  =>{Response::Home},
                "m" | "menu"| "Menu"  =>{Response::Menu},
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
            height:60
        }
    }
    pub fn set_width(&mut self, width:usize){
        self.width = width;
    }
    pub fn set_height(&mut self, height:usize){
        self.height = height;
    }

    pub fn get_page_index(&self)->usize{
        self.page_index
    }

    pub fn build_menu_page(&mut self)->usize{
        let mut headers = self.page_buffer.iter().map(|page|(page.title.to_owned())).collect::<Vec<String>>().to_owned();
        let copy = headers.iter_mut().map(|f|f.as_str()).collect::<Vec<_>>();
        
        self.new_page("Page Menu", copy.as_slice(), &["info"], "Go to page",Box::new(move |disp,res|{res}) )
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
        let page = self.page_buffer[self.page_index].rows.iter().fold(String::from(""),|x,y|{x + y});
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
        
        let page = self.page_buffer[index].rows.iter().fold(String::from(""),|x,y|{x + y});
        let _ = out.write_all(page.as_bytes());
        
        let _ = std::io::stdout().flush();
        
        
        match (self.page_buffer[index].action)(&self, self.parse_input()){
            Response::Alt(x) => {
                self.page_buffer.remove(index);
                Response::Page(x)},
                //self.page_index = x;Response::Alt(x)},
            _=>{
                self.page_buffer.remove(index);
                Response::Back}
        }
        
    }
    pub fn menu(&mut self,title: &str, options: &[&str], info: &[&str],query: &str,action:Action)->usize{
        self.new_page(title, options,info, query,action);
        self.last_index = self.page_index;
        self.back_index.push(self.last_index);
        self.page_index = self.page_buffer.len()-1;
        self.page_index
    }
    pub fn new_page(&mut self,title: &str, options: &[&str], info: &[&str],query: &str, action: Action)->usize{
        let mut page = Vec::new();
        let row_len =self.width;
        let character = '-';

        let mut lbuff = String::new();
        for _ in 0..row_len+1{lbuff.push(character);}
        lbuff.push_str("\n");
        
        page.push(lbuff.clone());
        page.push(format_line(Placement::Center, title, row_len));
        page.push(lbuff.clone());
        if !info.is_empty(){
            page.push(format_line(Placement::Center, "  ", row_len));
            for opt in info{
                page.push(format_line(Placement::Left, &format!("{}",opt), row_len));
            }
            page.push(format_line(Placement::Center, "  ", row_len));
            //page.push(lbuff.clone());
        }
        let mut index = 0;
        for opt in options{
            page.push(format_line(Placement::Left, &format!("{}  {}",index,opt), row_len));
            index += 1;
        }
        page.push(format_line(Placement::Center, "  ", row_len));
        page.push(format_line(Placement::Center, "write: q to quit, b for back, h for home", row_len));
        
        page.push(lbuff.clone());
        page.push(format_line(Placement::Center, "  ", row_len));
        page.push(format!("    {}: ", query));
        //page.push(format_line(Placement::Left, query, row_len));
        //self.last_index = self.page_index;
        self.page_buffer.push(Page{title: title.to_string(),rows:page, width:self.width,action});
        self.page_buffer.len()-1
    }
    pub fn new_info_page(&mut self,title: &str, info: &[&str], query: &str, action: Action)->usize{
        let mut page = Vec::new();
        let row_len =90;
        let character = '-';

        let mut lbuff = String::new();
        for _ in 0..row_len{lbuff.push(character);}
        lbuff.push_str("\n");
        
        page.push(lbuff.clone());
        page.push(format_line(Placement::Center, title, row_len));
        page.push(lbuff.clone());
        for opt in info{
            page.push(format_line(Placement::Left, &format!("{}",opt), row_len));
          
        }
        page.push(format_line(Placement::Center, "  ", row_len));
        page.push(format_line(Placement::Center, "q för att avsluta, b för bakåt, h för hem", row_len));
        
        
        page.push(lbuff.clone());
        page.push(format_line(Placement::Center, "  ", row_len));
        page.push(format!("    {}: ", query));
        //page.push(format_line(Placement::Left, query, row_len));
        
        self.page_buffer.push(Page{title: title.to_string(),rows:page, width:self.width, action});
        //self.last_index = self.page_index;
        self.page_index = self.page_buffer.len()-1;
        self.page_buffer.len()-1
    }
   
} 



impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}


fn spacer(len:usize)->String{
    "                                                                                                        "
    .chars().take(len).collect::<String>()
}

enum Placement{
    Left,
    Center,
    Right
}
fn format_line(place: Placement,text: &str, row_len: usize)->String{
    let text =if text.char_indices().count()>60{text.char_indices().take(59).map(|(_,x)|x).collect::<String>()}else{text.to_string()};//text.get(..59).unwrap_or("*guru meditation*")
    let offset = match place{
        Placement::Center=>(row_len/2).saturating_sub(text.char_indices().count()/2),
        Placement::Left=>row_len/6,
        Placement::Right=>row_len.saturating_sub(row_len/6)
    };
    let mut line =format!("|{}{}",spacer(offset),text);
        line.push_str(&format!("{}|\n",spacer(row_len-line.char_indices().count()))); 
        line
}
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

        
    let mut my_display = Display::new();

    let my_page = my_display.new_page("testing title", &["option 1", "option 2"], &["this is some info"], "query", Action::default());
    let my_page2 = my_display.new_page("testing title2", &["option 1", "option 2", "option 3"], &["this is some info on the subject","in several lines"], "answer", Box::new(|_disp,res|{res}));
    
    /* match my_display.show() {
        Response::Page(x) => println!("Response Page is{} ",x),
        Response::Alt(x) => println!("Response Alt is{} ",x),
        Response::Exit => todo!(),
        Response::Back => todo!(),
        Response::New(_, _, _, _, _) => todo!(),
        Response::NewInfo(_, _, _, _) => todo!(),
        Response::Home => todo!(),
        Response::Commands(_) => todo!(),
        Response::Menu => {
            let res = my_display.show_menu_page();

            println!("Response menu is{} ",my_display.page_index)
        },
    } */
}