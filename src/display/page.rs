use super::{Display,Response};



pub type Action = Box<dyn Fn(&Display,Response)->Response>;
pub trait DefaultAction {
    /// Default action that just passes on the response
    /// ## Example
    /// ```
    /// // to get a default Action
    /// let my_action = Action::default();
    /// 
    /// ```
    fn default()->Action{Box::new(move|_display,response|{response})}
}

impl DefaultAction for Action{}

pub struct Page{
    pub title: String,
    pub rows: Vec<String>,
    pub width: usize,
    pub action: Action
}


impl Page{
    pub fn new(action:Action)->Self{
        Page{title:String::new(),rows:Vec::new(), width:90, action}
    }
    pub fn new_with_title(title:&str,action:Action)->Self{
        Page{title:String::from(title),rows:Vec::new(), width:90, action}
    }
    pub fn push(&mut self, value: String){
        self.rows.push(value)
    }
    pub fn set_qery(&mut self, value: &str){
        let i =self.rows.len()-1;
        self.rows[i] = format_line(Placement::Left, value, &'-',self.width);
        //self.rows.push(value)
    }
}

impl Page{
    /// ### build page
    /// is the start of the builder pattern
    /// ## Example
    /// ```
    /// use simpleton::display::Page;
    /// 
    /// let my_page = Page::build_page("this is a title");
    /// 
    /// ```
    pub fn build_page(title:&str)->Self{
        Page{ title:title.to_string(), rows: Vec::new(), width: 90, action: Action::default() }
    }

    pub fn set_action(mut self, action:Action)->Self{
        self.action = action;
        self
    }
    pub fn set_width(mut self, width:usize)->Self{
        self.width = width;
        self
    }
    pub fn unformatted_content(&mut self, text:&str){
        //let temp:Vec<&str> = text.split("\n").collect();
        //self.rows.extend(temp.iter().map(|f|f.to_string()));
        self.rows.extend(text.split("\n").collect::<Vec<_>>().iter().map(|f|f.to_string()))
    }

    fn format_page(&mut self,style: Style, options: &[&str], info: &[&str],query: &str, action: Action){
        let mut page = Vec::new();
        let row_len =self.width;
        let character = '-';
        let (horizontal, vertical) = match style{
            Style::None => (' ',' '),
            Style::Default => ('-','|'),
            Style::Border(horizontal, vertical) => (horizontal,vertical),
        };

        let mut lbuff = String::new();
        for _ in 0..row_len+1{lbuff.push(horizontal);}
        lbuff.push_str("\n");
        
        page.push(lbuff.clone());
        page.push(format_line(Placement::Center, &self.title, &vertical, row_len));
        page.push(lbuff.clone());
        if !info.is_empty(){
            page.push(format_line(Placement::Center, "  ",  &vertical, row_len));
            for opt in info{
                page.push(format_line(Placement::Left, &format!("{}",opt),  &vertical, row_len));
            }
            page.push(format_line(Placement::Center, "  ",  &vertical, row_len));
            //page.push(lbuff.clone());
        }
        let mut index = 0;
        for opt in options{
            page.push(format_line(Placement::Left, &format!("{}  {}",index,opt),  &vertical, row_len));
            index += 1;
        }
        page.push(format_line(Placement::Center, "  ",  &vertical, row_len));
        page.push(format_line(Placement::Center, "write: q to quit, b for back, h for home",  &vertical, row_len));
        
        page.push(lbuff.clone());
        page.push(format_line(Placement::Center, "  ",  &vertical, row_len));
        page.push(format!("    {}: ", query));
        self.rows = page;
        //page.push(format_line(Placement::Left, query, row_len));
        //self.last_index = self.page_index;
        //self.page_buffer.push(Page{title: title.to_string(),rows:page, action});
        //self.page_buffer.len()-1
    }
}

enum Style{
    None,
    Default,
    Border(char,char)
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
fn format_line(place: Placement,text: &str, vertical: &char, row_len: usize)->String{
    let text =if text.char_indices().count()>60{text.char_indices().take(59).map(|(_,x)|x).collect::<String>()}else{text.to_string()};//text.get(..59).unwrap_or("*guru meditation*")
    let offset = match place{
        Placement::Center=>(row_len/2).saturating_sub(text.char_indices().count()/2),
        Placement::Left=>row_len/6,
        Placement::Right=>row_len.saturating_sub(row_len/6)
    };
    let mut line =format!("{}{}{}",vertical,spacer(offset),text);
        line.push_str(&format!("{}{}\n",spacer(row_len-line.char_indices().count()),vertical)); 
        line
}

#[test]
fn page_test() {
    
    use crate::display::{Display,Result};
    let mut disp = Display::default();
    let mut temp = Page::build_page("title");
    let mut page2 = Page::new(Action::default());
    page2.unformatted_content("this is some \n un formatted content\n\n\n what do you think?");
    temp.format_page(Style::Border('*','*'), &[], &["some info"], "query", Action::default());
    page2 = page2.set_action(Box::new(move|_display,response|{
        match response {
            Response::Alt(x)=> {println!("hello test");Response::Page(x)},
            _=>response
        }
    }));
    disp.add_page(page2);
    let tempidx =disp.add_page(temp);
    
    disp.get_page(tempidx).set_qery("this is where it's at");
    disp.show();

}