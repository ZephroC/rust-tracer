use getopts::Options;
use std::convert::TryFrom;
use std::string::ToString;

fn print_usage(opts: Options) {
    let brief = format!("Usage: {} FILE [options]", "rust-tracer");
    print!("{}", opts.usage(&brief));
}

pub struct Resolution {
    pub width: u16,
    pub height: u16,
}

pub struct Config {
    filename: String,
    threads: u8,
    pub resolution: Resolution
}

pub fn parse_args(args: Vec<String>) -> Config {
    let mut opts = Options::new();
    opts.optopt("f", "file", "Input json file", "file name");
    opts.optopt("t", "threads", "threads to use", "number of threads to use");
    opts.optopt("h", "height", "window height", "window height");
    opts.optopt("", "help", "window height", "window height");
    opts.optopt("w", "width", "window width", "window width");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("help"){
        print_usage( opts);
        panic!();
    }

    let height: u16 = matches.opt_get_default("h", 720).unwrap();
    let width: u16 = matches.opt_get_default("w", 1280).unwrap();
    let threads:u8 = matches.opt_get_default("t", 1).unwrap();
    let file:String = matches.opt_get_default("f", "scene.json".to_string()).unwrap();
    Config {
        filename: file.clone(),
        threads,
        resolution: Resolution {
            width,
            height
        }
    }
}
