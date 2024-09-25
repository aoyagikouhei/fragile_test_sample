use derive_builder::Builder;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
use uuid::Uuid;

const INSERT_SQL: &str =
    "INSERT INTO public.users (uuid, user_name, user_email, user_kbn) VALUES ($1, $2, $3, $4)";
const SELECT_SQL: &str =
    "SELECT to_json(t1.*) FROM public.users AS t1 WHERE $1::UUID IS NULL OR t1.uuid = $1::UUID";
const DELETE_SQL: &str = "DELETE FROM public.users";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "type_user_kbn")]
pub enum UserKbn {
    Admin,
    Normal,
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder, PartialEq, Eq)]
#[builder(setter(into))]
#[builder(default)]
#[builder(field(public))]
pub struct Users {
    pub uuid: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_kbn: UserKbn,
}

impl Default for Users {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            user_name: "テスト太郎".to_owned(),
            user_email: "test@example.com".to_owned(),
            user_kbn: UserKbn::Normal,
        }
    }
}

impl Users {
    pub async fn insert_columns(
        client: &Client,
        uuid: &Uuid,
        user_name: &str,
        user_email: &str,
        user_kbn: &UserKbn,
    ) -> anyhow::Result<()> {
        client
            .execute(INSERT_SQL, &[&uuid, &user_name, &user_email, &user_kbn])
            .await?;
        Ok(())
    }

    pub async fn insert(client: &Client, params: &Users) -> anyhow::Result<()> {
        client
            .execute(
                INSERT_SQL,
                &[
                    &params.uuid,
                    &params.user_name,
                    &params.user_email,
                    &params.user_kbn,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn make(client: &Client, builder: &mut UsersBuilder) -> anyhow::Result<()> {
        if builder.uuid.is_none() {
            builder.uuid = Some(Uuid::now_v7());
        }
        if builder.user_name.is_none() {
            builder.user_name = Some("太郎".to_owned());
        }
        if builder.user_email.is_none() {
            builder.user_email = Some(format!(
                "{}@example.com",
                builder.user_name.as_ref().unwrap()
            ));
        }
        if builder.user_kbn.is_none() {
            builder.user_kbn = Some(UserKbn::Normal);
        }
        let params = builder.build()?;
        Self::insert(client, &params).await
    }

    pub async fn make_normal(client: &Client, builder: &mut UsersBuilder) -> anyhow::Result<()> {
        builder.user_kbn = Some(UserKbn::Normal);
        Self::make(client, builder).await
    }

    pub async fn make_admin(client: &Client, builder: &mut UsersBuilder) -> anyhow::Result<()> {
        builder.user_kbn = Some(UserKbn::Admin);
        Self::make(client, builder).await
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
