use std::io::{Write};
use serde_json::{Value};


pub trait Writer {
    fn write(&mut self, document: Value);
    fn done(&mut self);
}


pub struct JsonWriter {
    writer: Box<dyn Write>,
    started: bool
}

impl JsonWriter {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self {
            writer: writer,
            started: false
        }
    }
}

impl Writer for JsonWriter {
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


pub struct JsonLineWriter{
    writer: Box<dyn Write>,
}

impl JsonLineWriter {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self {
            writer: writer,
        }
    }
}

impl Writer for JsonLineWriter {
    fn write(&mut self, document: Value) {
        self.writer.write(&document.to_string().into_bytes());
        self.writer.write(b"\n");
    }
    fn done(&mut self) {}
}


#[cfg(test)]
mod json_writer_tests {
    use std::io::{Cursor, Seek, SeekFrom};
    use serde_json::{self, json};
    use super::{JsonWriter, Writer};

    #[test]
    fn write_documents() {
        let mut buffer = Cursor::new(Vec::new());
        let doc1 = json!({"some": "doc"});
        let doc2 = json!({"other": "doc"});
        let mut writer = JsonWriter::new(Box::new(&mut buffer));
        writer.write(doc1);
        writer.write(doc2);
        writer.done();

        buffer.seek(SeekFrom::Start(0));
        // let mut json_string = String::new();
        // buffer.read_to_string(&mut json_string);
        let actual: serde_json::Value = serde_json::from_reader(buffer).unwrap();
        let expected = json!([{"some": "doc"}, {"other": "doc"} ]);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod jsonline_writer_tests {

}
