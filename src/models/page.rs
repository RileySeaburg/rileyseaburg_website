use crate::controllers::datetime::{self, datetime as local_datetime};
use crate::controllers::{execute_query, get_db_pool};
use chrono::NaiveDateTime;
use rustyroad::database::Database;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, EnumIter, PartialEq, Clone, Display)]
pub enum PageField {
    Id,
    Title,
    Slug,
    Content,
    Author,
    MetaDescription,
    SeoImage,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub meta_description: Option<String>,
    pub seo_image: Option<String>,
    #[serde(with = "local_datetime")] // use the custom serializer/deserializer
    pub created_at: Option<NaiveDateTime>,
    #[serde(with = "local_datetime")] // use the custom serializer/deserializer
    pub updated_at: Option<NaiveDateTime>,
}

impl Page {
    pub fn get_field(&self, field: &PageField) -> String {
        match field {
            PageField::Id => self.id.clone().unwrap_or_default().to_string(),
            PageField::Title => self.title.clone().expect("Page title is None").to_string(),
            PageField::Slug => self.slug.clone().expect("Page slug is None").to_string(),
            PageField::Content => self
                .content
                .clone()
                .expect("Page content is None")
                .to_string(),
            PageField::Author => self
                .author
                .clone()
                .expect("Page author is None")
                .to_string(),
            PageField::MetaDescription => self
                .meta_description
                .clone()
                .expect("Page meta description is None")
                .to_string(),
            PageField::SeoImage => self
                .seo_image
                .clone()
                .expect("Page SEO image is None")
                .to_string(),
            PageField::CreatedAt => self
                .created_at
                .clone()
                .expect("Page created at is None")
                .to_string(),
            PageField::UpdatedAt => self
                .updated_at
                .clone()
                .expect("Page updated at is None")
                .to_string(),
        }
    }

    pub fn create_page(page: Page) -> Result<Page, sqlx::Error> {
        Ok(page)
    }
}
