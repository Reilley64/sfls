use domain::entities::media_type::MediaType;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateLibraryRequest {
    pub name: String,
    pub path: String,
    pub media_type: MediaType
}
