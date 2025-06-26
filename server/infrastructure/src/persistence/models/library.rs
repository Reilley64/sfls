use diesel::{Insertable, Queryable, Selectable};
use domain::entities::media_type::MediaType;

#[derive(Debug, Default, Queryable, Selectable)]
#[diesel(table_name = crate::schema::libraries)]
pub struct Library {
    pub id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: String,
    pub updated_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub name: String,
    pub path: String,
    pub media_type: String,
}

impl Into<domain::entities::library::Library> for Library {
    fn into(self) -> domain::entities::library::Library {
        domain::entities::library::Library {
            id: self.id.clone(),
            created_at: self.created_at.clone(),
            created_by: self.created_by.clone(),
            updated_at: self.updated_at.clone(),
            updated_by: self.updated_by.clone(),
            name: self.name.clone(),
            path: self.path.clone(),
            media_type: MediaType::from(self.media_type.clone()),
        }
    }
}

#[derive(Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::libraries)]
pub struct InsertableLibrary {
    pub created_by: String,
    pub updated_by: String,
    pub name: String,
    pub path: String,
    pub media_type: String,
}