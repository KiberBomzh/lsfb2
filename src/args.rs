use std::path::PathBuf;


const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");


pub struct Args {
    pub book: PathBuf,

    #[cfg(feature = "zip")]
    pub zip: bool,
}

impl Args {
    pub fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        
        let mut book = None;

        #[cfg(feature = "zip")]
        let mut zip = false;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Value(v) if book.is_none() => {
                    let s = v.string()?;
                    let p = PathBuf::from(s);
                    book = Some(p);
                },

                #[cfg(feature = "zip")]
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

            #[cfg(feature = "zip")]
            zip,
        })
    }
}

fn get_help() -> String {
    let mut s = String::from(DESCRIPTION);

    s.push_str(&format!("\n\nUsage: {NAME} [OPTION]... [BOOK]\n\n"));
    s.push_str("[BOOK] - path to a fb2 book\n\n");

    s.push_str("Options:\n");
    #[cfg(feature = "zip")]
    s.push_str("    -z, --zip       Unzip [BOOK] before parsing\n");

    s.push_str("\n    -h, --help      Print this message and exit\n");
    s.push_str("    -V, --version   Print version and exit\n");


    s
}
