use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use redis::Connection;

#[derive(Serialize, Deserialize, Debug, Clone, Builder, PartialEq, Eq, Default)]
#[builder(setter(into))]
#[builder(default)]
#[builder(field(public))]
pub struct Content {
    pub key: String,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContentRow {
    pub title: String,
    pub body: String,
}

impl From<Content> for ContentRow {
    fn from(content: Content) -> ContentRow {
        ContentRow {
            title: content.title,
            body: content.body,
        }
    }
}


impl Content {
    pub fn get(conn: &mut Connection, key: &str) -> anyhow::Result<Content> {
        let value: String = redis::cmd("GET").arg(key).query(conn)?;
        let row: ContentRow = serde_json::from_str(&value)?;
        Ok(Content{
            key: key.to_owned(),
            title: row.title,
            body: row.body,
        })
    }

    pub fn set(&self, conn: &mut Connection) -> anyhow::Result<()> {
        let row: ContentRow = self.clone().into();
        let value = serde_json::to_string(&row)?;
        redis::cmd("SET").arg(&self.key).arg(value).exec(conn)?;
        Ok(())
    }
}