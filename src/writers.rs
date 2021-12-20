use std::io::{Write};
use serde_json::{Value};


pub trait Writer {
    fn write(&mut self, document: Value);
    fn done(&mut self);
}


pub struct JsonWriter<'a> {
    writer: Box<dyn Write + 'a>,
    started: bool
}

impl<'a> JsonWriter<'a> {
    pub fn new(writer: Box<dyn Write + 'a>) -> Self {
        Self {
            writer: writer,
            started: false
        }
    }
}

impl<'a> Writer for JsonWriter<'a> {
    fn write(&mut self, document: Value) {
        if self.started {
            self.writer.write(b",");
        } else {
            self.writer.write(b"[");
            self.started = true;
        }
        self.writer.write(&document.to_string().into_bytes());
    }

    fn done(&mut self) {
        if self.started {
            self.writer.write(b"]");
        }
    }
}


pub struct JsonLineWriter<'a>{
    writer: Box<dyn Write + 'a>,
}

impl<'a> JsonLineWriter<'a> {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self {
            writer: writer,
        }
    }
}

impl<'a> Writer for JsonLineWriter<'a> {
    fn write(&mut self, document: Value) {
        self.writer.write(&document.to_string().into_bytes());
        self.writer.write(b"\n");
    }
    fn done(&mut self) {}
}


#[cfg(test)]
mod json_writer_tests {
    use std::io::{Cursor, Read, Seek, SeekFrom};
    use serde_json::{self, json};
    use super::{JsonWriter, Writer};

    #[test]
    fn write_documents() {
        let mut buffer = Cursor::new(Vec::new());
        let mut json_string = String::new();
        {
            let mut writer = JsonWriter::new(Box::new(&mut buffer));
            let doc1 = json!({"some": "doc"});
            let doc2 = json!({"other": "doc"});
            writer.write(doc1);
            writer.write(doc2);
            writer.done();
        }
        buffer.seek(SeekFrom::Start(0)).unwrap();
        buffer.read_to_string(&mut json_string).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        let expected = json!([{"some": "doc"}, {"other": "doc"} ]);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod jsonline_writer_tests {

}
