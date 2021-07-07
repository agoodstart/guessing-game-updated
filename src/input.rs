use console::{Style, Term, Color};

pub struct Input {
    pub number: i32,
}

impl Input {
    pub fn new(terminal: &Term) -> Result<Input, &'static str> {
        let inputline = match terminal.read_line() {
            Ok(line) => line,
            Err(_) => return Err("Something went wrong"),
        };

        if inputline.len() <= 0 {
            return Err("No arguments");
        }

        let number = match inputline.parse::<i32>() {
            Ok(num) => num,
            Err(_) => return Err("Can't convert number")
        };

        Ok(Input{ number})
    }
}