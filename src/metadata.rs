use std::path::Path;

use quick_xml::reader::Reader as XmlReader;

use super::parser::{Parser, ZipReader};


pub struct Metadata {
    title: String,
    authors: Vec<String>,
    language: String,
    sequence: Option<Sequence>,
    annotation: Option<Vec<String>>,
}
impl Metadata {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, quick_xml::Error> {
        Ok(Parser::with_file(path, Self::parse)??)
    }
    pub fn from_zip<P: AsRef<Path>>(path: P) -> Result<Self, quick_xml::Error> {
        Ok(Parser::with_zip(path, 
            |reader: XmlReader<quick_xml::encoding::DecodingReader<std::io::BufReader<ZipReader<'_>>>>|
            Self::parse(reader)
        )??)
    }

    fn parse<R: std::io::BufRead>(mut reader: XmlReader<R>) -> Result<Self, quick_xml::Error> {
        use quick_xml::events::Event;


        reader.config_mut().trim_text(true);

        let mut xml_version = quick_xml::XmlVersion::Implicit1_0;

        let mut title: Option<String> = None;
        let mut authors: Vec<String> = Vec::new();
        let mut language: Option<String> = None;
        let mut sequence: Option<Sequence> = None;
        let mut annotation: Option<Vec<String>> = None;

        let mut current_author = Author::new();
        let mut position: Option<Position> = None;
        let mut in_title_info = false;
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Err(err) => return Err(err),
                Ok(Event::Eof) => break,
                Ok(Event::Decl(e)) => if let Ok(v) = e.xml_version() {
                    xml_version = v;
                },

                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        "title-info" => in_title_info = true,
                        _ if !in_title_info => continue,

                        "book-title" => position = Some(Position::Title),

                        "author" => position = Some(Position::Author(None)),
                        "first-name" =>
                            position = Some(Position::Author(Some(AuthorPosition::FirstName))),
                        "middle-name" =>
                            position = Some(Position::Author(Some(AuthorPosition::MiddleName))),
                        "last-name" =>
                            position = Some(Position::Author(Some(AuthorPosition::LastName))),

                        "annotation" => position = Some(Position::Annotation),

                        "lang" => position = Some(Position::Language),

                        _ => {},
                    }
                },
                Ok(Event::End(e)) => {
                    match e.name().as_ref() {
                        "title-info" => break,

                        "author" => authors.push(current_author.construct()),

                        "book-title" | "lang" | "annotation" => position = None,
                        "first-name" | "middle-name" | "last-name" =>
                            position = Some(Position::Author(None)),

                        _ => {},
                    }
                },

                Ok(Event::Empty(e)) => if e.name().as_ref() == "sequence" {
                    let name = if let Ok(Some(a)) = e.try_get_attribute("name")
                    && let Ok(v) = a.normalized_value(xml_version) {
                        Some(v.to_string())
                    } else {
                        None
                    };

                    let number = if let Ok(Some(a)) = e.try_get_attribute("number")
                    && let Ok(v) = a.normalized_value(xml_version) {
                        Some(v.to_string())
                    } else {
                        None
                    };


                    sequence = Some(Sequence{ name, number });
                },

                Ok(Event::Text(e)) => {
                    if let Some(p) = &position {
                        match p {
                            Position::Title if title.is_none() =>
                                title = Some(e.into_inner().to_string()),

                            Position::Language if language.is_none() =>
                                language = Some(e.into_inner().to_string()),

                            Position::Author(ap) => if let Some(p) = ap {
                                let t = e.into_inner().to_string();

                                match p {
                                    AuthorPosition::FirstName => 
                                        current_author.first_name = Some(t),

                                    AuthorPosition::MiddleName =>
                                        current_author.middle_name = Some(t),

                                    AuthorPosition::LastName =>
                                        current_author.last_name = Some(t),
                                }
                            },

                            Position::Annotation => {
                                if let Some(a) = annotation.as_mut() {
                                    a.push(e.into_inner().to_string());
                                } else {
                                    annotation = Some(vec![e.into_inner().to_string()]);
                                }
                            },


                            _ => {},
                        }
                    }
                },

                _ => {},
            }
            buf.clear();
        }

        const NF: &str = "NOT FOUND!";
        Ok(Self{ 
            title: title.unwrap_or(NF.to_string()),
            authors: authors,
            language: language.unwrap_or(NF.to_string()),
            sequence, annotation,
        })
    }

    pub fn print(&self) {
        println!("title: {}", self.title);
        println!("author(s): {}", self.authors.join(", "));
        println!("language: {}", self.language);
        if let Some(s) = &self.sequence {
            println!("sequence: {} {}", s.name.as_ref().unwrap_or(&String::new()), s.number.as_ref().unwrap_or(&String::new()));
        }
        if let Some(a) = &self.annotation {
            println!("annotaion:");
            println!("{}", a.join("\n"));
        }
    }
}

enum Position {
    Title,
    Author(Option<AuthorPosition>),
    Language,
    Annotation,
}
enum AuthorPosition {
    FirstName,
    MiddleName,
    LastName
}

struct Author {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
}
impl Author {
    fn new() -> Self {
        Self{
            first_name: None,
            middle_name: None,
            last_name: None,
        }
    }

    fn construct(&self) -> String {
        let mut s = String::new();
        if let Some(first) = &self.first_name {
            s.push_str(first);
        }
        if let Some(middle) = &self.middle_name {
            if !s.is_empty() && !s.ends_with(" ") {
                s.push(' ');
            }
            s.push_str(middle);
        }
        if let Some(last) = &self.last_name {
            if !s.is_empty() && !s.ends_with(" ") {
                s.push(' ');
            }
            s.push_str(last);
        }


        s
    }
}

struct Sequence {
    name: Option<String>,
    number: Option<String>,
}
