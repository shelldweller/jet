use std::io::{BufRead};
use serde_json::{Result, Value};

struct JsonReader {
    reader: Box<dyn BufRead>,
}

impl JsonReader {
    fn new(reader: Box<dyn BufRead>) -> Self {
        Self { reader: reader }
    }
}

impl Iterator for JsonReader {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        self.reader.read_to_string(&mut buffer);
        if buffer.is_empty() {
            return None;
        }
        return Some(serde_json::from_str(&buffer));
    }
}


// struct JsonLineReader {
//     reader: Box<dyn BufRead>,
// }

// impl Iterator for JsonLineReader {
//     type Item = Value;

//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }


#[cfg(test)]
mod jsonreader_tests {
    use serde_json::{json};
    use super::JsonReader;

    #[test]
    fn jsonreader_iteration_produces_a_single_element() {
        let mut reader = JsonReader::new(Box::new(r#"{"some": "value"}"#.as_bytes()));

        let expected = json!{{"some": "value"}};
        let actual = reader.next().unwrap().unwrap();
        assert_eq!(actual, expected);

        assert!(reader.next().is_none());
    }
}
