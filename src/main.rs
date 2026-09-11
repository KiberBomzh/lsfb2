mod args;
mod parser;
mod metadata;

use metadata::Metadata;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = args::Args::parse()?;

    let meta = if args.zip {
        Metadata::from_zip(&args.book)?
    } else {
        Metadata::from_file(&args.book)?
    };

    meta.print();


    Ok(())
}

