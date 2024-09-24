pub mod postgres;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use tokio_postgres::NoTls;
    use uuid::Uuid;

    use crate::postgres::{
        companies::Companies,
        users::{Users, UsersBuilder},
    };

    async fn setup() -> anyhow::Result<tokio_postgres::Client> {
        let (client, connection) =
            tokio_postgres::connect("host=postgresql user=user dbname=web password=pass", NoTls)
                .await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Companies::delete_all(&client).await?;
        Users::delete_all(&client).await?;
        Ok(client)
    }

    // cargo test test_companies1 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_companies1() -> anyhow::Result<()> {
        let conn = setup().await?;
        Companies::insert(&conn, &Uuid::now_v7(), "UV").await?;
        let list = Companies::select_all(&conn).await?;
        assert_eq!(list.len(), 1);
        Ok(())
    }

    // cargo test test_companies2 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_companies2() -> anyhow::Result<()> {
        let conn = setup().await?;
        Companies::insert(&conn, &Uuid::now_v7(), "UV1").await?;
        Companies::insert(&conn, &Uuid::now_v7(), "UV2").await?;
        let list = Companies::select_all(&conn).await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }

    // cargo test test_users1 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users1() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::insert(&conn, &mut UsersBuilder::default().user_email("taro")).await?;
        let list = Users::select_all(&conn).await?;
        assert_eq!(list.len(), 1);
        Ok(())
    }

    // cargo test test_users2 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users2() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::insert(&conn, &mut UsersBuilder::default().user_email("taro")).await?;
        Users::insert(&conn, &mut UsersBuilder::default().user_email("jiro")).await?;
        let list = Users::select_all(&conn).await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }
}
