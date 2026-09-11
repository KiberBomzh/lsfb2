use std::path::PathBuf;


const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");


pub struct Args {
    pub book: PathBuf,
    pub zip: bool,
}

impl Args {
    pub fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        
        let mut book = None;
        let mut zip = false;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Value(v) if book.is_none() => {
                    let s = v.string()?;
                    let p = PathBuf::from(s);
                    book = Some(p);
                },
                Short('z') | Long("zip") if !zip => 
                    zip = true,

                Short('h') | Long("help") => {
                    print!("{}", get_help());
                    std::process::exit(0);
                },
                Short('V') | Long("version") => {
                    println!("{NAME} {VERSION}");
                    std::process::exit(0);
                },

                _ => return Err(arg.unexpected()),
            }
        }


        Ok( Self{
            book: book.ok_or("missing argument [BOOK]")?,
            zip,
        })
    }
}

fn get_help() -> String {
format!(r#"{DESCRIPTION}

Usage: {NAME} [OPTION]... [BOOK]

[BOOK] - path to a fb2 book

Options:
    -z, --zip       Unzip [BOOK] before parsing

    -h, --help      Print this message and exit
    -V, --version   Print version and exit
"#)
}
