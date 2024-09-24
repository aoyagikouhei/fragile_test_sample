pub mod postgres;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use tokio_postgres::NoTls;

    use crate::postgres::users::{Users, UsersBuilder};

    async fn setup() -> anyhow::Result<tokio_postgres::Client> {
        let (client, connection) =
            tokio_postgres::connect("host=postgresql user=user dbname=web password=pass", NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Users::delete_all(&client).await?;
        Ok(client)
    }

    // cargo test test_users_fragile -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users_fragile() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::insert(&conn, &uuid::Uuid::now_v7(), "テスト太郎", "test@example.com").await?;
        let users = Users::select_all(&conn).await?;
        assert_eq!(users.len(), 1);
        Ok(())
    }

    // cargo test test_users_kotikoti -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users_kotikoti() -> anyhow::Result<()> {
        let conn = setup().await?;
        let mut builder = UsersBuilder::default();
        Users::insert_from_builder(&mut builder, &conn).await?;
        let users = Users::select_all(&conn).await?;
        assert_eq!(users.len(), 1);
        Ok(())
    }
}