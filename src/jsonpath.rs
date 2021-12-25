use serde_json::{Value};

pub struct JsonPath<'a> {
    parsed_expression: Vec<&'a str>,
}

impl<'a> JsonPath<'a> {
    pub fn new(expression: &'a str) -> Result<Self, String> {
        let mut chunks: Vec<&str> = Vec::new();
        let mut last_index = 0;
        for (i, &byte) in expression.as_bytes().iter().enumerate() {
            if byte == b'.' {
                chunks.push(&expression[last_index..i]);
                last_index = i + 1;
            }
        }
        chunks.push(&expression[last_index..]);
        Ok(Self {
            parsed_expression: chunks,
        })
    }

    pub fn resolve(&self, value: &'a Value) -> Option<&'a Value> {
        let mut search_value = value;
        for chunk in &self.parsed_expression {
            let result = search_value.get(chunk);
            match result {
                Some(val) => search_value = val,
                None => { return None; }
            }
        }
        Some(search_value)
    }
}


#[cfg(test)]
mod jsonpath_tests {
    use serde_json::{json};
    use super::JsonPath;

    #[test]
    fn simple_object_path() {
        let value = json!{ {"name": { "common": "Canada" } } };

        let jp = JsonPath::new("name").unwrap();
        let expected = &json!{ {"common": "Canada"} };
        let actual = jp.resolve(&value).unwrap();
        assert_eq!(actual, expected);

        let jp = JsonPath::new("name.common").unwrap();
        let expected = &json!{ "Canada" };
        let actual = jp.resolve(&value).unwrap();
        assert_eq!(actual, expected);

        let jp = JsonPath::new("none.such").unwrap();
        assert_eq!(jp.resolve(&value), None);
    }

}
