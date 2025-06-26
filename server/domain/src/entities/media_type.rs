use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum MediaType {
    #[default]
    Movie,
    Show,
}

impl Into<String> for MediaType {
    fn into(self) -> String {
        match self {
            MediaType::Movie => "movie".to_string(),
            MediaType::Show => "show".to_string(),
        }
    }   
}

impl From<String> for MediaType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "movie" => MediaType::Movie,
            "show" => MediaType::Show,
            _ => unreachable!(),
        }
    }  
}
