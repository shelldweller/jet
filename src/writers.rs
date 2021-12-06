use std::io::{self, Write};
use serde_json::{Value};


pub trait Writer {
    fn write(&mut self, document: Value);
    fn done(&self);
}


pub struct JsonWriter {
    started: bool
}

impl JsonWriter {
    pub fn new() -> Self {
        Self { started: false }
    }
}

impl Writer for JsonWriter {
    fn write(&mut self, document: Value) {
        if self.started {
            io::stdout().write(b",");
        } else {
            io::stdout().write(b"[");
            self.started = true;
        }
        io::stdout().write(&document.to_string().into_bytes());
    }

    fn done(&self) {
        if self.started {
            io::stdout().write(b"]");
        }
    }
}


pub struct JsonLineWriter{}

impl JsonLineWriter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Writer for JsonLineWriter {
    fn write(&mut self, document: Value) {
        io::stdout().write(&document.to_string().into_bytes());
        io::stdout().write(b"\n");
    }
    fn done(&self) {}
}


#[cfg(test)]
mod json_writer_tests {

}

#[cfg(test)]
mod jsonline_writer_tests {

}
