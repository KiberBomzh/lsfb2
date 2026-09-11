mod args;
mod parser;
mod metadata;

use colored::Colorize;

use metadata::Metadata;


fn main() {
    let args = match args::Args::parse() {
        Ok(a) => a,
        Err(err) => {
            eprintln!("{}", format!("Error while parsing cli args: {err}").red().bold());
            std::process::exit(1);
        },
    };

    #[cfg(feature = "zip")]
    let result = if args.zip {
        Metadata::from_zip(&args.book)
    } else {
        Metadata::from_file(&args.book)
    };

    #[cfg(not(feature = "zip"))]
    let result = Metadata::from_file(&args.book);

    match result {
        Ok(meta) => meta.print(),
        Err(err) => {
            eprintln!("{}", format!("Error while parsing book: {err}").red().bold());
            std::process::exit(1);
        },
    }
}

