use std::str::FromStr;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum SerdeFormat {
    Json,
    JsonLine,
}

impl FromStr for SerdeFormat {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized_value: &str = &value.to_lowercase();
        match normalized_value {
            "j" => Ok(SerdeFormat::Json),
            "json" => Ok(SerdeFormat::Json),

            "jl" => Ok(SerdeFormat::JsonLine),
            "jsonline" => Ok(SerdeFormat::JsonLine),
            "json-line" => Ok(SerdeFormat::JsonLine),
            _ => Err(format!("Unsupported format: {}", value)),
        }
    }
}


#[cfg(test)]
mod sedre_format_tests {
    use super::SerdeFormat;
    use std::str::FromStr;

    #[test]
    fn test_valid_formats() {
        let pairs = [
            ("j", SerdeFormat::Json),
            ("json", SerdeFormat::Json),
            ("JSON", SerdeFormat::Json),
            ("jSon", SerdeFormat::Json),

            ("jl", SerdeFormat::JsonLine),
            ("jsonline", SerdeFormat::JsonLine),
            ("jsonLine", SerdeFormat::JsonLine),
            ("JSON-line", SerdeFormat::JsonLine),
        ];
        for pair in pairs {
            assert_eq!(SerdeFormat::from_str(pair.0).unwrap(), pair.1);
        }
    }

    #[test]
    fn test_invalid_format() {
        match SerdeFormat::from_str("garbage") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }
}
