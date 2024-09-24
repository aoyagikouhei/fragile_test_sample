use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
use uuid::Uuid;

const INSERT_SQL: &str = "INSERT INTO public.companies (uuid, company_name) VALUES ($1, $2)";
const SELECT_SQL: &str =
    "SELECT to_json(t1.*) FROM public.companies AS t1 WHERE $1::UUID IS NULL OR t1.uuid = $1::UUID";
const DELETE_SQL: &str = "DELETE FROM public.companies";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Companies {
    pub uuid: Uuid,
    pub company_name: String,
}

impl Companies {
    pub async fn insert(client: &Client, uuid: &Uuid, company_name: &str) -> anyhow::Result<()> {
        client.execute(INSERT_SQL, &[&uuid, &company_name]).await?;
        Ok(())
    }

    pub async fn delete_all(client: &Client) -> anyhow::Result<()> {
        client.execute(DELETE_SQL, &[]).await?;
        Ok(())
    }

    pub async fn select_all(client: &Client) -> anyhow::Result<Vec<Companies>> {
        let uuid: Option<Uuid> = None;
        let res = client.query(SELECT_SQL, &[&uuid]).await?;
        res.iter()
            .map(|row| serde_json::from_value(row.get(0)).map_err(Into::into))
            .collect::<anyhow::Result<Vec<Companies>>>()
    }
}
