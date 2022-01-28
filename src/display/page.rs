

use std::ops::Add;

use super::{Display,Response};


pub type Action = Box<dyn Fn(&Display,Response)->Response>;
pub trait DefaultAction {
    /// Default action that just passes on the response
    /// ## Example
    /// ``` rust
    /// # use simpleton::display::page::*;
    /// // to get a default Action
    /// let my_action = Action::default();
    /// 
    /// ```
    fn default()->Action{Box::new(move|_display,response|{response})}
}

impl DefaultAction for Action{}

pub struct Page{
    style: Style,
    pub parts: Parts,
    pub action: Action
}

impl std::ops::Deref for Page {
    type Target = Parts;

    fn deref(&self) -> &Self::Target {
        &self.parts
    }
}
impl std::ops::DerefMut for Page {
    

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parts
    }
}


impl Page{
    pub fn new(action:Action)->Self{
        Page{style:Style::default(),parts:Parts::new(90,15,""), action}
    }
    pub fn new_with_title(title:&str,action:Action)->Self{
        Page{style:Style::default(), parts:Parts::new_with_title(title,90,15,""), action}
    }
    pub fn with_title(mut self, title: &str)->Page{
        self.set_title(title);
        self
    }
    pub fn set_qery(&mut self, query: &str){
        self.parts.set_query(query);
        
    }
    pub fn rows(&self)->Vec<String>{
        self.get(self.style(), self.width)
    }

    /// Set the page's style.
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// Get a reference to the page's style.
    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn get_title(&self) -> String {
        self.parts.title.content.to_owned()
    }

    /// Get the page's width.
    pub fn width(&self) -> usize {
        self.parts.width
    }

    /// Set the page's width.
    pub fn set_width(&mut self, width: usize) {
        self.parts.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.parts.height = height;
    }
}

impl Page{
    pub fn new_page(title: &str, options: &[&str], info: &[&str],query: &str, width:usize, height:usize, action: Action)->Self{
        let mut parts = Parts::new(width,height,"");
        parts.set_title(title);
        parts.set_info(info);
        parts.set_options(options);
        parts.set_query(query);
        Page { style: Style::default(), parts: parts, action: action }
    }

    fn build_parts(title:&str, width:usize, height:usize)->Self{
        let mut parts = Parts::new(width,height,"");
        parts.set_title(title);
        parts.set_info(&[]);
        parts.set_options(&[]);
        parts.set_query("");

        Page{ style:Style::Default,  parts, action: Action::default() }
    }
    /// ### build page
    /// this is the start of the builder pattern
    /// ## Example
    /// ``` rust ignore
    /// pub use simpleton::display::page::Page;
    /// 
    /// let (width,height) = (90,20);
    /// let my_page = Page::build_page("this is a title",width, height);
    /// 
    /// ```
        pub fn build_page(display: &Display,title:&str)->Self{
        Page::build_parts(title,display.width, display.height)
    }
    pub fn build_page_(title:&str, width: usize, height: usize)->Self{
        Page::build_parts(title,width, height)
    }

    pub fn set_action(mut self, action:Action)->Self{
        self.action = action;
        self
    }
    pub fn set_page_width(mut self, width:usize)->Self{
        self.parts.width = width;
        self
    }
    pub fn set_page_info_from_slice(mut self, info: &[&str])->Self{
        self.parts.info = info.into();
        self
    }
    /// splits the str at '\n' new line
    pub fn set_page_info_from_str(mut self, info: &str)->Self{
        let temp:Vec<&str> = info.split("\n").collect();
        self.parts.info = temp.as_slice().into();
        self
    }
    /// the options is really an numbered list
    pub fn set_page_options_from_slice(mut self, options: &[&str])->Self{
        self.parts.options = options.into();
        self
    }
}


#[derive(std::cmp::PartialEq)]
pub enum Style{
    None,
    Default,
    Line,
    Star,
    Custom(char,char)
}

impl Default for Style {
    fn default() -> Self {
        Self::None
    }
}
impl From<(char,char)> for Style {
    fn from(style: (char,char)) -> Self {
        Self::Custom(style.0,style.1)
    }
}


impl Style {
    /// return the individual spacers (horizontal, vertical)
    pub fn get_parts(&self)->(char,char){
        let (horizontal, vertical) = match self{
            Self::None => (' ',' '),
            Self::Default => ('-','|'),
            Self::Custom(horizontal, vertical) => (horizontal.to_owned(),vertical.to_owned()),
            Style::Line => ('-','|'),
            Style::Star => ('*','*'),            
        };
        (horizontal, vertical)
    }
}
/* fn spacer(len:usize)->String{
   [' ';256].into_iter().take(len).collect::<String>()
} */
/// get a string of blanks of specified length
fn spacer(len:usize)->String{
    
    (0..len).map(|_|' ').collect()
}
fn horizontal_line(style: &Style,width: usize)->String{
    let (horizontal, _vertical) = style.get_parts();
    let mut buf = String::new();
        for _ in 0..width+1{buf.push(horizontal);}
        buf.push_str("\n");
        buf
}


pub struct Parts {
    title: Title,
    info: Info,
    options: Options,
    query: Query,
    width: usize,
    height: usize,
    help: Info
}

#[allow(dead_code)]
impl Parts{
    

    /// Get a formated vector of options.
    fn options(&self,placement: &Placement,style: &Style) -> Vec<String> {
        self.options.format(placement,style,self.width)
    }

    /// Set the parts's options.
    pub fn set_options(&mut self, options: &[&str]) {
        self.options.add( options);//options.iter().map(|f|f.to_owned().to_string()).collect::<Vec<String>>()
    }
    /// Get a formatted string of the parts's query.
    fn query(&self,placement: &Placement,style: &Style) -> String {
        self.query.format(placement,style,self.width)
    }
    /// Get a reference to the parts's query.
    fn get_query(&self) -> &str {
        self.query.get()
    }

    /// Set the parts's query.
    pub fn set_query(&mut self, query: &str) {
        self.query.0 = query.into();
    }

    /// Get a formated section of the parts's info.
    fn info(&self,placement: &Placement,style: &Style) -> Vec<String> {
        self.info.format(placement,style,self.width)
    }

    /// Set the parts's info.
    pub fn set_info(&mut self, info_lines: &[&str]) {
        //self.info = info_lines.into();
        self.info.add( info_lines.iter().map(|f|f.to_owned().to_string()).collect::<Vec<String>>());
    }
    /// gets the formatted page
    fn get(&self, style: &Style, width: usize)->Vec<String>{
        let mut page = Vec::new();
        
        page.extend(self.title(&Placement::Center,style).into_iter());
        page.extend(self.info(&Placement::Center,style).into_iter());
        page.extend(self.options(&Placement::Left,style).into_iter());
        
        for _ in 1..self.calculate_spacers() {
            page.push(" ".format(&Placement::Center, style, width));
        }
        page.extend(self.help(&Placement::Left,style).into_iter());
        page.push(self.query(&Placement::Left,style));
        
        page
    }

    fn calculate_spacers(&self)->usize{
        let i= self.info.content.len();
        let o= self.options.content.len();
        match i+o<self.height-4 {
            true => (self.height-4)-(i+o),
            false => 0,
        }
    }

    /// Get a formatted output of the parts's title.
    fn title(&self,placement: &Placement, style: &Style) -> Vec<String> {
        /* let mut title = Vec::new();
        title.push(horizontal_line(style,self.width));
        title.push(self.title.format(placement, style, self.width));
        title.push(horizontal_line(style,self.width));

        title */
        vec![
            horizontal_line(style,self.width),
            self.title.format(placement, style, self.width),
            horizontal_line(style,self.width)
            ]
    }

    /// Set the parts's title.
    fn set_title(&mut self, title: &str) {
        self.title.content = title.to_string();
    }

    pub fn new(width: usize, height:usize, help: &str) -> Parts {
        Parts { 
            title: Title{content:"Emtpy page".to_owned()}, 
            info: Info { content: vec![] }, 
            options: Options::new(),//Options { content: vec![Line::new("content".to_owned())] }, 
            query: Query("".to_owned()),
            width: width,
            height:height,
            help: Info{content:vec![Line::new(help)]} , }
    }
    pub fn new_with_title(title: &str, width: usize, height:usize, help: &str) -> Parts {
        Parts { 
            title: Title{content:title.to_owned()}, 
            info: Info { content: vec![] }, 
            options: Options::new(), 
            query: Query("".to_owned()),
            width,
            height,
            help: help.into(),//: Info::new(help) , 
        }
    }

    /// Get a reference to the parts's width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Set the parts's width.
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    /// Get a reference to the parts's height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Set the parts's help.
    pub fn set_help(&mut self, help: &str) {
        self.help = Info::new(help);
    }

    /// Get a formatted output of the parts's help.
    pub fn help(&self,placement: &Placement, style: &Style) -> Vec<String> {
        self.help.format(placement, style, self.width)
    }
}

pub enum Placement{
    Left,
    Center,
    Right
}
/* fn format_line(place: Placement,text: &str, vertical: &char, row_len: usize)->String{
    let text =if text.char_indices().count()>60{text.char_indices().take(59).map(|(_,x)|x).collect::<String>()}else{text.to_string()};//text.get(..59).unwrap_or("*guru meditation*")
    let offset = match place{
        Placement::Center=>(row_len/2).saturating_sub(text.char_indices().count()/2),
        Placement::Left=>row_len/6,
        Placement::Right=>row_len.saturating_sub(row_len/6)
    };
    let mut line =format!("{}{}{}",vertical,spacer(offset),text);
        line.push_str(&format!("{}{}\n",spacer(row_len-line.char_indices().count()),vertical)); 
        line
} */


trait LineFormat{
    type Content;
    fn format(&self,placement: &Placement,style: &Style,width:usize)->Self::Content;
}

struct Line{
    content: String
}
#[allow(dead_code)]
impl Line {
    fn new(content:&str)->Self{Line{content:content.to_string()}}
    fn set(&mut self, content:&str){
        self.content=content.into();
    }
}
impl From<&str> for Line {
    fn from(x: &str) -> Self {
        Line::new(x)
    }
}
impl LineFormat for Line{
    type Content = String;

    fn format(&self,place: &Placement, style: &Style, width: usize)->Self::Content {
        let offset = match place{
            Placement::Center=>(width/2).saturating_sub(self.content.char_indices().count()/2),
            Placement::Left=>width/6,
            Placement::Right=>width.saturating_sub(width/6)
        };
        let (_,vertical) = style.get_parts();
        let mut line =format!("{}{}{}",vertical,spacer(offset),self.content);
        line.push_str(&format!("{}{}\n",spacer(width-line.char_indices().count()),vertical)); 
        line
        //self.content
    }
}
impl LineFormat for &str{
    

    fn format(&self,placement: &Placement,style: &Style,width:usize)->String {
        Line{content:self.to_string()}.format(placement, style, width)
    }
type Content = String;
}

struct Title{content:String}
impl  LineFormat for Title {
    type Content = String;

    fn format(&self,placement: &Placement,style: &Style,width:usize)->Self::Content {
        let offset = match placement{
            Placement::Center=>(width/2).saturating_sub(self.content.char_indices().count()/2),
            Placement::Left=>width/6,
            Placement::Right=>width.saturating_sub(width/6)
        };
        let (_,vertical) = style.get_parts();
        let mut line =format!("{}{}{}",vertical,spacer(offset),self.content);
        line.push_str(&format!("{}{}\n",spacer(width-line.char_indices().count()),vertical)); 
        line
    }    
    


}
struct Info{
    content:Vec<Line>
}
/// Options are really a numbered list numbered from 0 and up
struct Options{
    content:Vec<Line>
}
struct Query(String);
impl  LineFormat for Query {
    type Content = String;

    fn format(&self,placement: &Placement,style: &Style,width:usize)->Self::Content {
        let offset = match placement{
            Placement::Center=>(width/2).saturating_sub(self.0.char_indices().count()/2),
            Placement::Left=>width/6,
            Placement::Right=>width.saturating_sub(width/6)
        };
        let (_,vertical) = style.get_parts();
        format!("{}{}{}: ",vertical,spacer(offset/3),self.0)        
    }    
    


}

impl From<&str> for Info {
    fn from(x: &str) -> Self {
        Info::new(x)
    }
}
impl From<&[&str]> for Info {
    fn from(x: &[&str]) -> Self {
        Info{ content: x.iter().map(|f| -> Line {f.to_owned().into()}).collect() }
    }
}
impl Info {
    fn new(content:&str)->Self{
        Info{content:vec![content.into()]}
    }
    /// Get a reference to the info's content.
    fn format(&self,placement: &Placement, style: &Style, width: usize) -> Vec<String> {
        self.content.iter().map(|s|s.format(placement, style, width)).collect::<Vec<_>>()
    }
    fn add(&mut self, content: Vec<String>){
        self.content.extend(content.into_iter().map(|f|{Line::new(&f)}));
    }
}

impl From<&[&str]> for Options {
    fn from(x: &[&str]) -> Self {
        Options{ content: x.iter().enumerate().map(|(i,f)| -> Line {format!("{} {}",i,f).as_str().into()}).collect() }
    }
}


impl Options {
    fn new()->Self{
        Options { content: Vec::new() }
    }
    ///replace a single option in the list
    fn replace_line(&mut self, idx:usize, option: &str){
        self.content[idx] = Line::new( &format!("{} {}",idx,option));
    }
    fn format(&self,placement: &Placement, style: &Style, width: usize) -> Vec<String> {
        self.content.iter().map(|s| {s.format(placement, style, width)}).collect::<Vec<_>>()
    }
    fn add(&mut self, content: &[&str]){
        self.content.extend(content.iter().enumerate().map(|(i,f)| Line::new( &format!("{} {}",i,f))));
    }
}
#[allow(dead_code)]
impl Query {
    fn get(&self) -> &str {
        self.0.as_ref()
    }
}


#[test]
fn page_test() {
    
    use crate::display::{Display};
    let mut disp = Display::default();
    let mut temp = Page::build_page(&disp,"title");
    let mut page2 = Page::new(Action::default());
    //page2.unformatted_content("this is some \n un formatted content\n\n\n what do you think?");
    //temp.format_page(Style::Custom('*','*'), &[], &["some info"], "query", Action::default());
    page2 = page2.set_action(Box::new(move|_display,response|{
        match response {
            Response::Alt(x)=> {println!("hello test");Response::Page(x)},
            _=>response
        }
    }));
    disp.add_page(page2);
    let tempidx =disp.add_page(temp);
    
    disp.get_page(tempidx).set_qery("this is where it's at");
    //disp.show();

}
#[test]
fn parts_test(){
    use crate::display::{Display};
    let mut disp = Display::default();
    //let mut temp = Page::new(Action::default());
    //temp.parts.set_title("my little title".to_string());
    let mut temp = Page::build_page(&disp, "hello")
                                    .set_page_info_from_str("some random info");
    temp.set_options(&["some random options"]);
    
    let home = disp.add_page(temp);
    
    loop{
        match disp.show(){
            
            Response::Alt(x) => {let t = Page::build_page(&disp,&format!("this is page {}",x));disp.add_page(t);},
            Response::Exit => break,
            Response::Back => disp.back(),
            Response::Page(x)=>disp.set_page(x),
            Response::Home => disp.set_page(home),
            Response::Commands(_) => todo!(),
            _=>()
        }; 
    }
    
    

}
