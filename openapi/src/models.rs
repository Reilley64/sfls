#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MediaMediaIdGetPathParams {
                pub media_id: String,
    }




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LibrariesPost200Response {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "mediaType")]
    pub media_type: String,

}





impl LibrariesPost200Response {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, path: String, name: String, media_type: String, ) -> LibrariesPost200Response {
        LibrariesPost200Response {
            id,
            path,
            name,
            media_type,
        }
    }
}

/// Converts the LibrariesPost200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for LibrariesPost200Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("path".to_string()),
            Some(self.path.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("mediaType".to_string()),
            Some(self.media_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LibrariesPost200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LibrariesPost200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub path: Vec<String>,
            pub name: Vec<String>,
            pub media_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LibrariesPost200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "path" => intermediate_rep.path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "mediaType" => intermediate_rep.media_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LibrariesPost200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LibrariesPost200Response {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in LibrariesPost200Response".to_string())?,
            path: intermediate_rep.path.into_iter().next().ok_or_else(|| "path missing in LibrariesPost200Response".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in LibrariesPost200Response".to_string())?,
            media_type: intermediate_rep.media_type.into_iter().next().ok_or_else(|| "mediaType missing in LibrariesPost200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LibrariesPost200Response> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<LibrariesPost200Response>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LibrariesPost200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LibrariesPost200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<LibrariesPost200Response> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LibrariesPost200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LibrariesPost200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LibrariesPostRequest {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "path")]
    pub path: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "mediaType")]
    pub media_type: String,

}





impl LibrariesPostRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(name: String, path: String, media_type: String, ) -> LibrariesPostRequest {
        LibrariesPostRequest {
            name,
            path,
            media_type,
        }
    }
}

/// Converts the LibrariesPostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for LibrariesPostRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("path".to_string()),
            Some(self.path.to_string()),


            Some("mediaType".to_string()),
            Some(self.media_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LibrariesPostRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LibrariesPostRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub path: Vec<String>,
            pub media_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LibrariesPostRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "path" => intermediate_rep.path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "mediaType" => intermediate_rep.media_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LibrariesPostRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LibrariesPostRequest {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in LibrariesPostRequest".to_string())?,
            path: intermediate_rep.path.into_iter().next().ok_or_else(|| "path missing in LibrariesPostRequest".to_string())?,
            media_type: intermediate_rep.media_type.into_iter().next().ok_or_else(|| "mediaType missing in LibrariesPostRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LibrariesPostRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<LibrariesPostRequest>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LibrariesPostRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LibrariesPostRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<LibrariesPostRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LibrariesPostRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LibrariesPostRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Media {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "season")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub season: Option<i32>,

    #[serde(rename = "episode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub episode: Option<i32>,

    #[serde(rename = "attributes")]
    pub attributes: std::collections::HashMap<String, crate::types::Object>,

}





impl Media {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, created_at: chrono::DateTime::<chrono::Utc>, updated_at: chrono::DateTime::<chrono::Utc>, title: String, attributes: std::collections::HashMap<String, crate::types::Object>, ) -> Media {
        Media {
            id,
            created_at,
            updated_at,
            title,
            season: None,
            episode: None,
            attributes,
        }
    }
}

/// Converts the Media value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Media {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),

            // Skipping created_at in query parameter serialization

            // Skipping updated_at in query parameter serialization


            Some("title".to_string()),
            Some(self.title.to_string()),


            self.season.as_ref().map(|season| {
                [
                    "season".to_string(),
                    season.to_string(),
                ].join(",")
            }),


            self.episode.as_ref().map(|episode| {
                [
                    "episode".to_string(),
                    episode.to_string(),
                ].join(",")
            }),

            // Skipping attributes in query parameter serialization
            // Skipping attributes in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Media value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Media {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub title: Vec<String>,
            pub season: Vec<i32>,
            pub episode: Vec<i32>,
            pub attributes: Vec<std::collections::HashMap<String, crate::types::Object>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Media".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created_at" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updated_at" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "season" => intermediate_rep.season.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "episode" => intermediate_rep.episode.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "attributes" => return std::result::Result::Err("Parsing a container in this style is not supported in Media".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Media".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Media {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in Media".to_string())?,
            created_at: intermediate_rep.created_at.into_iter().next().ok_or_else(|| "created_at missing in Media".to_string())?,
            updated_at: intermediate_rep.updated_at.into_iter().next().ok_or_else(|| "updated_at missing in Media".to_string())?,
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in Media".to_string())?,
            season: intermediate_rep.season.into_iter().next(),
            episode: intermediate_rep.episode.into_iter().next(),
            attributes: intermediate_rep.attributes.into_iter().next().ok_or_else(|| "attributes missing in Media".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Media> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Media>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Media>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Media - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Media> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Media as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Media - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Problem {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<String>,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "status")]
    pub status: i32,

    #[serde(rename = "detail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,

    #[serde(rename = "instance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance: Option<String>,

}





impl Problem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, status: i32, ) -> Problem {
        Problem {
            r#type: None,
            title,
            status,
            detail: None,
            instance: None,
        }
    }
}

/// Converts the Problem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.r#type.as_ref().map(|r#type| {
                [
                    "type".to_string(),
                    r#type.to_string(),
                ].join(",")
            }),


            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("status".to_string()),
            Some(self.status.to_string()),


            self.detail.as_ref().map(|detail| {
                [
                    "detail".to_string(),
                    detail.to_string(),
                ].join(",")
            }),


            self.instance.as_ref().map(|instance| {
                [
                    "instance".to_string(),
                    instance.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Problem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub title: Vec<String>,
            pub status: Vec<i32>,
            pub detail: Vec<String>,
            pub instance: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Problem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "detail" => intermediate_rep.detail.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "instance" => intermediate_rep.instance.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Problem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Problem {
            r#type: intermediate_rep.r#type.into_iter().next(),
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in Problem".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in Problem".to_string())?,
            detail: intermediate_rep.detail.into_iter().next(),
            instance: intermediate_rep.instance.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Problem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Problem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Problem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Problem - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Problem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Problem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Problem - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



