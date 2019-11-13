use getopts::Options;
use std::env;
use getopts::Fail;


pub struct SearchObject {
    pub in_dir: String,
    pub pattern: Vec<String>,
    pub out_dir: String,
}

fn print_usage(opts: Options) {
    let name = env!("CARGO_PKG_NAME");
    let brief = format!("Usage: {} FILE [options]", name);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    println!("{} {}", name, version);
}


pub fn parse_cli_args() -> Option<SearchObject> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt(
        "i",
        "in-directory",
        "directory containing PDFs to search",
        "<DIRECTORY>",
    );
    opts.optopt(
        "o",
        "out-directory",
        "directory containing PDFs to search",
        "<DIRECTORY>",
    );
    opts.optopt(
        "p",
        "patterns",
        "space separated search terms",
        "<PATTERNS>",
    );
    opts.optflag(
        "v",
        "version",
        "Print version info and exit",
    );
    opts.optflag(
        "h",
        "help",
        "Prints help information",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => match f {
           Fail::UnrecognizedOption(s) => {
               println!("Unrecognized option {}", s);
               print_usage(opts);
                return None;
           }
            _ =>  {
                println!("{}", f.to_string());
                return None;
            }
        },
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return None;
    }

    if matches.opt_present("v") {
        print_version();
        return None;
    }

    Some(SearchObject {
        pattern: matches.opt_strs("p"),
        in_dir: matches.opt_str("i").unwrap(),
        out_dir: matches.opt_str("o").unwrap(),
    })

}

#[cfg(test)]
mod test {
    #[test]
    fn test_something() {
        assert_eq!(3, 3);
    }
}
