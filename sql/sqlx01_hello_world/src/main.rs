use sqlx::{
    mysql::{MySqlPoolOptions, MySqlRow},
    Row 
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost:3306/test")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);
    let users = sqlx::query("select * from user")
        .map(|row: MySqlRow| {
            let username = row.try_get("username").ok();
            let sex = row.try_get("sex").ok();
            let id: i64 = row.try_get("id").unwrap();
            User { id, username, sex }
        })
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("{:?}", user);
    }

    // use query macro to achieve compile time syntactic and semantic verification
    // the type of users is a anonymous record type
    let users = sqlx::query!("select * from user").fetch_all(&pool).await?;
    for user in users {
        println!("user: {:?}", user);
    }

    // use query_as to name the output type
    let users = sqlx::query_as!(User, "select * from user").fetch_all(&pool).await?;
    for user in users {
        println!("user: {:?}", user);
    }

    Ok(())
}

#[derive(Debug)]
struct User {
    id: i64,
    username: Option<String>, // a field which possible be a null
    sex: Option<String>,
}
