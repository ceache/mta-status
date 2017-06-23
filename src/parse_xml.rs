#![deny(warnings)]
extern crate xml;
extern crate serde;

use self::xml::reader::{EventReader, XmlEvent};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug)]
pub struct Query {
    timestamp: String,
    lines: Vec<Line>,
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug)]
struct Line {
    name: String,
    status: String,
    date: String,
    time: String,
}

#[derive(PartialEq)]
enum XmlTag {
    TimeStamp,
    LineName,
    LineStatus,
    LineDate,
    LineTime,
    Ignore,
}

pub fn parse_xml(xml: &mut str) -> Query {
    let reader = EventReader::new(xml.as_bytes());

    let mut query = Query { ..Default::default() };
    let mut lines: Vec<Line> = Vec::new();
    let mut xml_tag: XmlTag = XmlTag::Ignore;

    let mut temp_line = Line { ..Default::default() };

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match name.local_name.as_ref() {
                    "timestamp" => {
                        xml_tag = XmlTag::TimeStamp;
                    }
                    "text" => {
                        xml_tag = XmlTag::Ignore;
                    }
                    "name" => {
                        xml_tag = XmlTag::LineName;
                    }
                    "status" => {
                        xml_tag = XmlTag::LineStatus;
                    }
                    "Date" => {
                        xml_tag = XmlTag::LineDate;
                    }
                    "Time" => {
                        xml_tag = XmlTag::LineTime;
                    }
                    _ => xml_tag = XmlTag::Ignore,
                }
            }
            Ok(XmlEvent::Characters(e)) => {
                let txt: String = e;
                match xml_tag {
                    XmlTag::TimeStamp => query.timestamp = txt,
                    XmlTag::LineName => temp_line.name = txt,
                    XmlTag::LineStatus => temp_line.status = txt,
                    XmlTag::LineDate => temp_line.date = txt,
                    XmlTag::LineTime => temp_line.time = txt,
                    XmlTag::Ignore => (),
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_ref() {
                    "line" => {
                        lines.push(temp_line);
                        temp_line = Line { ..Default::default() };
                    }
                    "subway" => break,
                    _ => (),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => (),
        }
    }

    query.lines = lines;
    query
}
