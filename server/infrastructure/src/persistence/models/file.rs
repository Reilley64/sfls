use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileType {
    Video,
    Poster,
    Logo,
    Thumbnail,
    Background,
}

impl FromStr for FileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "video" => Ok(Self::Video),
            "poster" => Ok(Self::Poster),
            "logo" => Ok(Self::Logo),
            "thumbnail" => Ok(Self::Thumbnail),
            "background" => Ok(Self::Background),
            _ => Err(format!("Invalid file type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub type_: FileType,
    pub path: String,
    pub blur_hash: Option<String>,
}
