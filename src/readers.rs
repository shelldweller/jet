use std::io::{BufRead};
use serde_json::{Value};

struct JsonReader {
    reader: Box<dyn BufRead>,
}

impl JsonReader {
    fn new(reader: Box<dyn BufRead>) -> Self {
        Self { reader: reader }
    }
}

impl Iterator for JsonReader {
    type Item = Result<Value, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();

        match self.reader.read_to_string(&mut buffer) {
            Err(e) => {
                return Some(Err(format!("I/O error: {}", e)));
            },
            Ok(_) => ()
        }

        if buffer.is_empty() {
            return None;
        }

        match serde_json::from_str(&buffer) {
            Ok(value) => {
                return Some(Ok(value));
            },
            Err(e) => {
                return Some(Err(format!("Parsing error: {}", e)));
            },
        }
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
    fn iteration_produces_a_single_element() {
        let mut reader = JsonReader::new(Box::new(r#"{"some": "value"}"#.as_bytes()));

        let expected = json!{{"some": "value"}};
        let actual = reader.next().unwrap().unwrap();
        assert_eq!(actual, expected);

        assert!(reader.next().is_none());
    }

    #[test]
    fn invalid_json_is_handled() {
        let mut reader = JsonReader::new(Box::new(r#"{"some": "#.as_bytes()));
        match reader.next().unwrap() {
            Ok(_) => assert!(false, "Invalid JSON should result in error"),
            Err(error) => {
                println!("JSON error: {:?}", error);
                println!("JSON error: {:?}", error);
            }
        }

        assert!(reader.next().is_none());
    }

    #[test]
    fn io_error_is_handled() {
        let bad_utf8: &[u8] = &[104, 101, 108, 108, 111, 255];
        let mut reader = JsonReader::new(Box::new(bad_utf8));
        match reader.next().unwrap() {
            Ok(_) => assert!(false, "Invalid UTF-8 should result in error"),
            Err(error) => {
                println!("JSON error: {:?}", error);
                println!("JSON error: {:?}", error);
            }
        }

        assert!(reader.next().is_none());
    }

}
