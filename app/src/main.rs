pub mod postgres;
pub mod redis;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use tokio_postgres::NoTls;
    use uuid::Uuid;

    use crate::{postgres::users::{UserKbn, Users, UsersBuilder}, redis::contents::Content};

    async fn setup() -> anyhow::Result<tokio_postgres::Client> {
        let (client, connection) =
            tokio_postgres::connect("host=postgresql user=user dbname=web password=pass", NoTls)
                .await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Users::delete_all(&client).await?;
        Ok(client)
    }

    async fn redis_setup() -> anyhow::Result<redis::Connection> {
        let client = redis::Client::open("redis://valkey/")?;
        client.get_connection().map_err(Into::into)
    }

    // cargo test test_users_columns1 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users_columns1() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::insert_columns(
            &conn,
            &Uuid::now_v7(),
            "taro",
            "taro@example.com",
            &UserKbn::Normal,
        )
        .await?;
        let list = Users::select_all(&conn).await?;
        assert_eq!(list.len(), 1);
        Ok(())
    }

    // cargo test test_users_columns2 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users_columns2() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::insert_columns(
            &conn,
            &Uuid::now_v7(),
            "taro",
            "taro@example.com",
            &UserKbn::Normal,
        )
        .await?;
        Users::insert_columns(
            &conn,
            &Uuid::now_v7(),
            "jiro",
            "jiro@example.com",
            &UserKbn::Admin,
        )
        .await?;
        let list = Users::select_all(&conn).await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }

    // cargo test test_users_default1 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users_default1() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::insert(
            &conn,
            &Users {
                user_name: "taro".to_owned(),
                user_email: "taro@exeample.com".to_owned(),
                ..Default::default()
            },
        )
        .await?;
        let list = Users::select_all(&conn).await?;
        assert_eq!(list.len(), 1);
        Ok(())
    }

    // cargo test test_users_default2 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users_default2() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::insert(
            &conn,
            &Users {
                user_name: "taro".to_owned(),
                user_email: "taro@exeample.com".to_owned(),
                user_kbn: UserKbn::Normal,
                ..Default::default()
            },
        )
        .await?;
        Users::insert(
            &conn,
            &Users {
                user_name: "jiro".to_owned(),
                user_email: "jiro@exeample.com".to_owned(),
                user_kbn: UserKbn::Admin,
                ..Default::default()
            },
        )
        .await?;
        let list = Users::select_all(&conn).await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }

    // cargo test test_users_builder1 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users_builder1() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::make_normal(&conn, &mut UsersBuilder::default().user_name("taro")).await?;
        let list = Users::select_all(&conn).await?;
        assert_eq!(list.len(), 1);
        Ok(())
    }

    // cargo test test_users_builder2 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_users_builder2() -> anyhow::Result<()> {
        let conn = setup().await?;
        Users::make_normal(&conn, &mut UsersBuilder::default().user_name("taro")).await?;
        Users::make_admin(&conn, &mut UsersBuilder::default().user_name("jiro")).await?;
        let list = Users::select_all(&conn).await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }

    // cargo test test_contents -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_contents() -> anyhow::Result<()> {
        let mut conn = redis_setup().await?;
        Content::set(
            &Content {
                key: "key1".to_owned(),
                title: "title1".to_owned(),
                body: "body1".to_owned(),
            },
            &mut conn,
        )?;
        let content = Content::get(&mut conn, "key1")?;
        assert_eq!(
            content,
            Content {
                key: "key1".to_owned(),
                title: "title1".to_owned(),
                body: "body1".to_owned(),
            }
        );
        Ok(())
    }
}
