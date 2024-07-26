use config::{FileStoredFormat, Format, Map, Value, ValueKind};
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct ConfigFile;

impl Format for ConfigFile {
    fn parse(
        &self,
        uri: Option<&String>,
        text: &str,
    ) -> Result<Map<String, config::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut result = Map::new();

        let key_type = vec!["GEMINI_TOKEN"].into_iter().find(|s| text.contains(s));
        let key = match key_type {
            Some("GEMINI_TOKEN") => "token",
            _ => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidData,
                    "Config file doesnt have a GEMINI_TOKEN field",
                )))
            }
        };

        let token = text.split('=').collect::<Vec<&str>>();

        result.insert(
            key.to_owned(),
            Value::new(uri, ValueKind::String((*token.last().unwrap()).to_string())),
        );

        Ok(result)
    }
}

impl FileStoredFormat for ConfigFile {
    fn file_extensions(&self) -> &'static [&'static str] {
        &["toml"]
    }
}
