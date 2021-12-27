use serde_json::{Value};

#[derive(Debug)]
enum JsonKey<'a> {
    StrKey(&'a str),
    IntKey(i32),
}

pub struct JsonPath<'a> {
    parsed_expression: Vec<JsonKey<'a>>,
}

impl<'a> JsonPath<'a> {
    pub fn new(expression: &'a str) -> Result<Self, String> {
        let mut chunks: Vec<JsonKey> = Vec::new();
        let mut last_index = 0;
        let mut in_bracket = false;

        for (i, &byte) in expression.as_bytes().iter().enumerate() {
            let mut flush = false;
            match byte {
                b'.' => {
                    if last_index == i {
                        // [1].foo
                        last_index += 1
                    } else {
                        flush = true;
                    }
                },
                b'[' => {
                    if in_bracket {
                        return Err(format!("Second `[` opened in position {}", i));
                    }
                    flush = true;
                    in_bracket = true;
                },
                b']' => {
                    if ! in_bracket {
                        return Err(format!("Unmatched `]` opened in position {}", i));
                    }
                    match expression[last_index..i].parse::<i32>() {
                        Ok(x) => chunks.push(JsonKey::IntKey(x)),
                        Err(_) => { return Err(format!("Invalid subscript '{}'", &expression[last_index..i])); }
                    }
                    flush = true;
                    in_bracket = false;
                    last_index = i + 1;
                },
                _ => {}
            }
            if flush && last_index < i {
                chunks.push(JsonKey::StrKey(&expression[last_index..i]));
                last_index = i + 1;
            }
        }
        if in_bracket {
            return Err(format!("Unmatched bracket at the end of `{}`", expression));
        }
        chunks.push(JsonKey::StrKey(&expression[last_index..]));
        Ok(Self {
            parsed_expression: chunks,
        })
    }

    pub fn resolve(&self, value: &'a Value) -> Option<&'a Value> {
        let mut search_value = value;
        for chunk in &self.parsed_expression {
            let result = match chunk{
                JsonKey::StrKey(key) => search_value.get(key),
                JsonKey::IntKey(key) => {
                    // TODO: Why is `key` of `&u32` type here and not of `u32`?
                    let index = *key;
                    if index < 0 {
                        if search_value.is_array() {
                            let len = search_value.as_array().unwrap().len() as i32;
                            let new_index = len + index;
                            if new_index < 0 {
                                None
                            } else {
                                search_value.get(new_index as usize)
                            }
                        } else {
                            None
                        }
                    } else {
                        search_value.get(index as usize)
                    }
                },
            };
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

    #[test]
    fn array_positive_subscript() {
        let value = json!(
            {
                "items": [
                    {"volumeInfo": {"title": "Cien años de soledad"}},
                    {"volumeInfo": {"title": "Los jefes"}},
                ]
            }
        );

        let jp = JsonPath::new("items[0].volumeInfo.title").unwrap();
        let expected = &json!{ "Cien años de soledad" };
        let actual = jp.resolve(&value).unwrap();
        assert_eq!(actual, expected);

        let jp = JsonPath::new("items[1].volumeInfo.title").unwrap();
        let expected = &json!{ "Los jefes" };
        let actual = jp.resolve(&value).unwrap();
        assert_eq!(actual, expected);

        let jp = JsonPath::new("items[10].volumeInfo.title").unwrap();
        assert_eq!(jp.resolve(&value), None);
    }

    #[test]
    fn array_negative_subscript() {
        let value = json!(
            {
                "items": [
                    {"volumeInfo": {"title": "Cien años de soledad"}},
                    {"volumeInfo": {"title": "Los jefes"}},
                ]
            }
        );

        let jp = JsonPath::new("items[-1].volumeInfo.title").unwrap();
        let expected = &json!{ "Los jefes" };
        let actual = jp.resolve(&value).unwrap();
        assert_eq!(actual, expected);

        let jp = JsonPath::new("items[-2].volumeInfo.title").unwrap();
        let expected = &json!{ "Cien años de soledad" };
        let actual = jp.resolve(&value).unwrap();
        assert_eq!(actual, expected);

        let jp = JsonPath::new("items[-3].volumeInfo.title").unwrap();
        assert_eq!(jp.resolve(&value), None);
    }

    #[test]
    fn invalid_jsonpath() {
        let invalid_expressions = vec![
            "array[foo]",
            "array[[1]",
            "array[1]]",
            "array[[1]]",
            "array[",
            "array[0",
            "array]",
            "array[].foo",
        ];
        for garbage in invalid_expressions {
            match JsonPath::new(garbage) {
                Ok(_) => assert!(false, "Successfully parsed invalid expression `{}`", garbage),
                Err(error) => eprintln!("Error parsing `{}`: {}", garbage, error),
            }
        }
    }

    // TODO: array[*]
    // TODO: object["foo"]
    // TODO: object.* and object.*.foo
}
