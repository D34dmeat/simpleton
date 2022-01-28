

#[derive(Debug)]
#[allow(dead_code)]
pub enum Colors {
    Default,
    Positive,
    Negative,
    Underline,
    NoUnderline,
    Bold,
    BlackBG,
    RedBG,
    GreenBG,
    YellowBG,
    BlueBG,
    MagentaBG,
    CyanBG,
    LightGrayBG,
    DarkGreyBG,
    LightRedBG,
    LightGreenBG,
    LightYellowBG,
    LightBlueBG,
    LightMagentaBG,
    LightCyanBG,
    WhiteBG,
    BlackFG,
    RedFG,
    GreenFG,
    YellowFG,
    BlueFG,
    MagentaFG,
    CyanFG,
    LightGrayFG,
    DarkGreyFG,
    LightRedFG,
    LightGreenFG,
    LightYellowFG,
    LightBlueFG,
    LightMagentaFG,
    LightCyanFG,
    WhiteFG,
}

impl From<Colors> for &str {
    
    fn from(color:Colors) -> Self { 
        match color {
            Colors::WhiteBG => "\x1b[107m",
            Colors::BlackBG => "\x1b[40m",
            Colors::RedBG => "\x1b[41m",
            Colors::GreenBG => "\x1b[42m",
            Colors::YellowBG => "\x1b[43m",
            Colors::BlueBG => "\x1b[44m",
            Colors::MagentaBG => "\x1b[45m",
            Colors::CyanBG => "\x1b[46m",
            Colors::LightGrayBG => "\x1b[47m",
            Colors::DarkGreyBG => "\x1b[48m",
            Colors::Default => "\x1b[0m",
            Colors::Positive => "\x1b[27m",
            Colors::Negative => "\x1b[7m",
            Colors::Underline => "\x1b[4m",
            Colors::NoUnderline => "\x1b[24m",
            Colors::Bold => "\x1b[1m",
            Colors::LightRedBG => "\x1b[101m",
            Colors::LightGreenBG => "\x1b[102m",
            Colors::LightYellowBG => "\x1b[103m",
            Colors::LightBlueBG => "\x1b[104m",
            Colors::LightMagentaBG => "\x1b[105m",
            Colors::LightCyanBG => "\x1b[106m",
            Colors::BlackFG => "\x1b[30m",
            Colors::RedFG => "\x1b[31m",
            Colors::GreenFG => "\x1b[32m",
            Colors::YellowFG => "\x1b[33m",
            Colors::BlueFG => "\x1b[34m",
            Colors::MagentaFG => "\x1b[35m",
            Colors::CyanFG => "\x1b[36m",
            Colors::LightGrayFG => "\x1b[37m",
            Colors::DarkGreyFG => "\x1b[90m",
            Colors::LightRedFG => "\x1b[91m",
            Colors::LightGreenFG => "\x1b[92m",
            Colors::LightYellowFG => "\x1b[93m",
            Colors::LightBlueFG => "\x1b[94m",
            Colors::LightMagentaFG => "\x1b[95m",
            Colors::LightCyanFG => "\x1b[96m",
            Colors::WhiteFG => "\x1b[97m",
        } 
    }
}


impl Colors {
    
pub fn to_str(&self) -> &'static str { 
    match *self {
        Colors::WhiteBG => "\x1b[107m",
        Colors::BlackBG => "\x1b[40m",
        Colors::RedBG => "\x1b[41m",
        Colors::GreenBG => "\x1b[42m",
        Colors::YellowBG => "\x1b[43m",
        Colors::BlueBG => "\x1b[44m",
        Colors::MagentaBG => "\x1b[45m",
        Colors::CyanBG => "\x1b[46m",
        Colors::LightGrayBG => "\x1b[47m",
        Colors::DarkGreyBG => "\x1b[100m",
        Colors::Default => "\x1b[0m",
        Colors::Positive => "\x1b[27m",
        Colors::Negative => "\x1b[7m",
        Colors::Underline => "\x1b[4m",
        Colors::NoUnderline => "\x1b[24m",
        Colors::Bold => "\x1b[1m",
        Colors::LightRedBG => "\x1b[101m",
        Colors::LightGreenBG => "\x1b[102m",
        Colors::LightYellowBG => "\x1b[103m",
        Colors::LightBlueBG => "\x1b[104m",
        Colors::LightMagentaBG => "\x1b[105m",
        Colors::LightCyanBG => "\x1b[106m",
        Colors::BlackFG => "\x1b[30m",
        Colors::RedFG => "\x1b[31m",
        Colors::GreenFG => "\x1b[32m",
        Colors::YellowFG => "\x1b[33m",
        Colors::BlueFG => "\x1b[34m",
        Colors::MagentaFG => "\x1b[35m",
        Colors::CyanFG => "\x1b[36m",
        Colors::LightGrayFG => "\x1b[37m",
        Colors::DarkGreyFG => "\x1b[90m",
        Colors::LightRedFG => "\x1b[91m",
        Colors::LightGreenFG => "\x1b[92m",
        Colors::LightYellowFG => "\x1b[93m",
        Colors::LightBlueFG => "\x1b[94m",
        Colors::LightMagentaFG => "\x1b[95m",
        Colors::LightCyanFG => "\x1b[96m",
        Colors::WhiteFG => "\x1b[97m",
    } 
}
pub fn to_string<Color>(&self) -> String { 
    match *self {
        Colors::WhiteBG => "\x1b[107m",
        Colors::BlackBG => "\x1b[40m",
        Colors::RedBG => "\x1b[41m",
        Colors::GreenBG => "\x1b[42m",
        Colors::YellowBG => "\x1b[43m",
        Colors::BlueBG => "\x1b[44m",
        Colors::MagentaBG => "\x1b[45m",
        Colors::CyanBG => "\x1b[46m",
        Colors::LightGrayBG => "\x1b[47m",
        Colors::DarkGreyBG => "\x1b[48m",
        Colors::Default => "\x1b[0m",
        Colors::Positive => "\x1b[27m",
        Colors::Negative => "\x1b[7m",
        Colors::Underline => "\x1b[4m",
        Colors::NoUnderline => "\x1b[24m",
        Colors::Bold => "\x1b[1m",
        Colors::LightRedBG => "\x1b[101m",
        Colors::LightGreenBG => "\x1b[102m",
        Colors::LightYellowBG => "\x1b[103m",
        Colors::LightBlueBG => "\x1b[104m",
        Colors::LightMagentaBG => "\x1b[105m",
        Colors::LightCyanBG => "\x1b[106m",
        Colors::BlackFG => "\x1b[30m",
        Colors::RedFG => "\x1b[31m",
        Colors::GreenFG => "\x1b[32m",
        Colors::YellowFG => "\x1b[33m",
        Colors::BlueFG => "\x1b[34m",
        Colors::MagentaFG => "\x1b[35m",
        Colors::CyanFG => "\x1b[36m",
        Colors::LightGrayFG => "\x1b[37m",
        Colors::DarkGreyFG => "\x1b[90m",
        Colors::LightRedFG => "\x1b[91m",
        Colors::LightGreenFG => "\x1b[92m",
        Colors::LightYellowFG => "\x1b[93m",
        Colors::LightBlueFG => "\x1b[94m",
        Colors::LightMagentaFG => "\x1b[95m",
        Colors::LightCyanFG => "\x1b[96m",
        Colors::WhiteFG => "\x1b[97m",
    } .into()
}
}

#[derive(Debug,PartialEq)]
#[allow(dead_code)]
pub enum SpecialCharacters {
    Up,
    Down,
    Left,
    Right,
    Unknown
}

impl From<SpecialCharacters> for &str {
    fn from(f: SpecialCharacters) -> Self {
        match f{
            SpecialCharacters::Up => "\x1b[A",
            SpecialCharacters::Down => "\x1b[B",
            SpecialCharacters::Left => "\x1b[D",
            SpecialCharacters::Right => "\x1b[C",
            SpecialCharacters::Unknown => " ",
        }
    }
}



impl From<&'static str> for SpecialCharacters {
    fn from(c: &'static str) -> Self {
        match c {
           x if x == "\x1b[A" =>{SpecialCharacters::Up},
           x if x == "\x1b[B" =>{SpecialCharacters::Down},
           x if x == "\x1b[D" =>{SpecialCharacters::Left},
           x if x == "\x1b[C" =>{SpecialCharacters::Right},
           _ =>SpecialCharacters::Unknown

        }
    }
}


#[cfg(test)]
mod char_test{
    use crate::display::SpecialCharacters;
    #[test]
    fn character_test() {

        let ab = "\x1b[A".as_bytes().as_ptr() as i32;
        
        let c:&'static str = SpecialCharacters::Up.into();
        assert_eq!("\x1b[A", c);
        let x :&str= SpecialCharacters::Up.into();
        print!("{:?}",(x) );
    }
}
impl SpecialCharacters {


pub(crate) fn into<SpecialCharacters>(&self) -> char {
    match self {
        Self::Up => "\x1b[A".chars().take(1).next().unwrap(),
            Self::Down => "\x1b[B".chars().take(1).next().unwrap(),
            Self::Left => "\x1b[D".chars().take(1).next().unwrap(),
            Self::Right => "\x1b[C".chars().take(1).next().unwrap(),
            Self::Unknown => " ".chars().take(1).next().unwrap(),
    }
}
}

impl From<SpecialCharacters> for char{
    
fn from(f: SpecialCharacters) -> Self { 
    match f {
        SpecialCharacters::Up => "\x1b[A".chars().take(1).next().unwrap(),
        SpecialCharacters::Down => "\x1b[B".chars().take(1).next().unwrap(),
        SpecialCharacters::Left => "\x1b[D".chars().take(1).next().unwrap(),
        SpecialCharacters::Right => "\x1b[C".chars().take(1).next().unwrap(),
        SpecialCharacters::Unknown => " ".chars().take(1).next().unwrap(),
    }
}

}