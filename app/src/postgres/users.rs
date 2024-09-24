use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
use uuid::Uuid;

const INSERT_SQL: &str =
    "INSERT INTO public.users (uuid, user_name, user_email) VALUES ($1, $2, $3)";
const SELECT_SQL: &str =
    "SELECT to_json(t1.*) FROM public.users AS t1 WHERE $1::UUID IS NULL OR t1.uuid = $1::UUID";
const DELETE_SQL: &str = "DELETE FROM public.users";

#[derive(Serialize, Deserialize, Debug, Clone, Builder, Default, PartialEq, Eq)]
#[builder(setter(into))]
#[builder(default)]
#[builder(field(public))]
pub struct Users {
    pub uuid: Uuid,
    pub user_name: String,
    pub user_email: String,
}

impl Users {
    pub async fn insert(client: &Client, builder: &mut UsersBuilder) -> anyhow::Result<()> {
        if builder.uuid.is_none() {
            builder.uuid = Some(Uuid::now_v7());
        }
        if builder.user_name.is_none() {
            builder.user_name = Some("テスト太郎".to_owned());
        }
        if builder.user_email.is_none() {
            builder.user_email = Some("test@example.com".to_owned());
        }
        client
            .execute(
                INSERT_SQL,
                &[&builder.uuid, &builder.user_name, &builder.user_email],
            )
            .await?;
        Ok(())
    }

    pub async fn delete_all(client: &Client) -> anyhow::Result<()> {
        client.execute(DELETE_SQL, &[]).await?;
        Ok(())
    }

    pub async fn select_all(client: &Client) -> anyhow::Result<Vec<Users>> {
        let uuid: Option<Uuid> = None;
        let res = client.query(SELECT_SQL, &[&uuid]).await?;
        res.iter()
            .map(|row| serde_json::from_value(row.get(0)).map_err(Into::into))
            .collect::<anyhow::Result<Vec<Users>>>()
    }
}
