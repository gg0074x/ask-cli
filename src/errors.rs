use std::error::{self};
use std::fmt;

#[derive(Clone)]
pub struct EmptyKey;

impl fmt::Display for EmptyKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GEMINI_TOKEN was not found in any way")
    }
}

impl std::fmt::Debug for EmptyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GEMINI_TOKEN was not found in any way")
    }
}

impl error::Error for EmptyKey {}

#[derive(Clone)]
pub struct DeserializeError;

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deserializing the JSON response failed")
    }
}

impl std::fmt::Debug for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Deserializing the JSON response failed")
    }
}

impl error::Error for DeserializeError {}

#[derive(Clone)]
pub struct CensorError;

impl fmt::Display for CensorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The prompt was inappropiate and couldn't be answered")
    }
}

impl std::fmt::Debug for CensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "The prompt was inappropiate and couldn't be answered")
    }
}

impl error::Error for CensorError {}
