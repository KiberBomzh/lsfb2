use std::path::Path;
use std::io::{BufReader, Read};
use std::fs::File;

use quick_xml::reader::Reader as XmlReader;
use quick_xml::encoding::DecodingReader;


pub struct Parser { }

impl Parser {
    pub fn with_file<P, F, R>(path: P, func: F) -> Result<R, quick_xml::Error> 
        where
            P: AsRef<Path>,
            F: FnOnce(XmlReader<DecodingReader<BufReader<File>>>) -> R,
    {
        let file = File::open(path)?;
        let decoder = DecodingReader::new(BufReader::new(file));
        let reader = XmlReader::from_reader(decoder);

        Ok(func(reader))
    }

    pub fn with_zip<P, F, R>(path: P, func: F) -> Result<R, std::io::Error> 
        where
            P: AsRef<Path>,
            F: FnOnce(XmlReader<DecodingReader<BufReader<ZipReader<'_>>>>) -> R,
    {
        use std::io::{Error, ErrorKind};


        let file = File::open(path)?;
        let mut buffer = vec![0u8; rawzip::RECOMMENDED_BUFFER_SIZE];
        let archive = rawzip::ZipArchive::from_file(file, &mut buffer)
            .map_err(|err| Error::new(ErrorKind::InvalidData, format!("Cannot read from zip: {err}")))?;

        let mut entries = archive.entries(&mut buffer);
        let entry_header = entries.next_entry()
            .map_err(|err| Error::new(ErrorKind::InvalidData, format!("Cannot find a book in zip: {err}")))?
            .ok_or(Error::new(ErrorKind::InvalidData, "Cannot find a book in zip!"))?;

        let entry = archive.get_entry(entry_header.wayfinder())
            .map_err(|err| Error::new(ErrorKind::NotFound, format!("Cannot find book's data in zip: {err}")))?;
        let reader = entry.reader();
        let zip_reader = match entry_header.compression_method() {
            rawzip::CompressionMethod::STORE => {
                let verifier = entry.verifying_reader(reader);
                ZipReader::Store(verifier)
            },
            rawzip::CompressionMethod::DEFLATE => {
                let inflater = flate2::read::DeflateDecoder::new(reader);
                let verifier = entry.verifying_reader(inflater);
                ZipReader::Deflate(verifier)
            },
            _ => return Err(Error::new(ErrorKind::InvalidData, format!("Unsupported compression method!"))),
        };

        let decoder = DecodingReader::new(BufReader::new(zip_reader));
        let reader = XmlReader::from_reader(decoder);

        Ok(func(reader))
    }
}

pub enum ZipReader<'a> {
    Store(rawzip::ZipVerifier<rawzip::ZipReader<&'a rawzip::FileReader>>),
    Deflate(rawzip::ZipVerifier<flate2::read::DeflateDecoder<rawzip::ZipReader<&'a rawzip::FileReader>>>),
}
impl Read for ZipReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ZipReader::Store(r) => r.read(buf),
            ZipReader::Deflate(r) => r.read(buf),
        }
    }
}
