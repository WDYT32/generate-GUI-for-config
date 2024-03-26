use html_gen::gen;
use parse_format::Parts;
use std::fs;
mod html_gen;
mod parse_format;
struct Config {
    input: String,
    out: String,
    src: String,
}
impl Config {
    fn new() -> Self {
        Config {
            input: String::new(),
            out: String::new(),
            src: String::new(),
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut mode = 0;
    let mut conf = Config::new();
    for arg in args {
        match arg.as_str() {
            "-o" => mode = 1,
            "-r" => mode = 2,
            s => match mode {
                0 => conf.input = s.to_string(),
                1 => conf.out = s.to_string(),
                2 => conf.src = s.to_string(),
                _ => (),
            },
        }
    }
    let format = fs::read_to_string(conf.input).expect("Read error");
    let res = gen(Parts::parse(format.clone()), conf.src, format);
    fs::write(conf.out, res).unwrap()
}
