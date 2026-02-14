#![allow(dead_code)]

use beatrix::relational::db::Database;
use beatrix::relational::table::Table;
use beatrix::relational::Postgres;
use beatrix_macro::RelationalModel;

#[test]
pub fn test_default_name() {
    #[derive(Debug, Clone, RelationalModel)]
    struct SomeComplexStructName {
        id: i32,
        name: String,
    }
    assert_eq!(
        SomeComplexStructName::table_details().table_name(),
        "some_complex_struct_name"
    );
}

#[test]
pub fn test_custom_enitity_name() {
    #[derive(Debug, Clone, RelationalModel)]
    #[table_name = "user"]
    struct UserInsert {
        id: i32,
        name: String,
    }

    assert_eq!(UserInsert::table_details().table_name(), "user");
}

#[test]
pub fn test_field_name() {
    #[derive(Debug, Clone, RelationalModel)]
    struct TableXyz {
        id: i32,
        name: String,
    }

    let columns = TableXyz::columns();
    let fields = columns
        .iter()
        .map(|v| v.column_name())
        .collect::<Vec<&str>>();

    assert_eq!(fields, vec!["id", "name"]);
}

#[test]
pub fn test_custom_field_name() {
    #[derive(Debug, Clone, RelationalModel)]
    struct TableXyz {
        id: i32,
        #[name = "custom_name"]
        complex_field_name: String,
    }

    let columns = TableXyz::columns();
    let fields = columns
        .iter()
        .map(|v| v.column_name())
        .collect::<Vec<&str>>();

    assert_eq!(fields, vec!["id", "custom_name"]);
}

#[derive(Debug, Clone, RelationalModel, PartialEq, Eq)]
struct User {
    id: i32,
    name: String,
    email: String,
    gender: String,
    salary: i32,
}

#[tokio::test]
pub async fn test_fetch_all() {
    let db = Postgres::new("postgres://postgres:asdqwe@localhost/beatrix", None)
        .await
        .unwrap();

    let user: Vec<User> = User::select().fetch_all::<User, _>(&db).await.unwrap();

    assert_eq!(
        user[0],
        User {
            id: 1,
            name: "Waldo Brookzie".to_string(),
            email: "wbrookzie0@nationalgeographic.com".to_string(),
            gender: "Agender".to_string(),
            salary: 78818,
        }
    )
}

#[tokio::test]
pub async fn test_fetch_one() {
    let db = Postgres::new("postgres://postgres:asdqwe@localhost/beatrix", None)
        .await
        .unwrap();

    let user: User = User::select().fetch_one::<User, _>(&db).await.unwrap();

    assert_eq!(
        user,
        User {
            id: 1,
            name: "Waldo Brookzie".to_string(),
            email: "wbrookzie0@nationalgeographic.com".to_string(),
            gender: "Agender".to_string(),
            salary: 78818,
        }
    )
}

#[tokio::test]
pub async fn test_eq_filters() {
    let db = Postgres::new("postgres://postgres:asdqwe@localhost/beatrix", None)
        .await
        .unwrap();

    // TODO: Fix filter implementation to work with Column<()>
    let user: User = User::select()
        .filter(User::id().eq(2).boxed())
        .fetch_one::<User, _>(&db)
        .await
        .unwrap();

    assert_eq!(user.id, 2,);

    let user: User = User::select()
        .filter(User::email().eq("kfundell2@tiny.cc".into()).boxed())
        .fetch_one::<User, _>(&db)
        .await
        .unwrap();

    assert_eq!(user.email, "kfundell2@tiny.cc");
}

#[tokio::test]
pub async fn test_and_or_filters() {
    let db = Postgres::new("postgres://postgres:asdqwe@localhost/beatrix", None)
        .await
        .unwrap();

    let user: Vec<User> = User::select()
        .filter(
            User::salary()
                .gt(20000)
                .and(Box::new(User::gender().eq("Male".into()))),
        )
        .fetch_all::<User, _>(&db)
        .await
        .unwrap();

    assert_eq!(user.len(), 4);
    assert_eq!(user[0].id, 3);
    assert_eq!(user[0].email, "kfundell2@tiny.cc");
}

#[tokio::test]
pub async fn test_like_filters() {
    let db = Postgres::new("postgres://postgres:asdqwe@localhost/beatrix", None)
        .await
        .unwrap();

    let user: Vec<User> = User::select()
        .filter(User::email().like("%@live.com".to_string()).boxed())
        .fetch_all(&db)
        .await
        .unwrap();

    assert_eq!(user.len(), 2);

    assert_eq!(
        user.iter()
            .map(|f| f.email.clone())
            .filter(|f| f.contains("@live.com"))
            .collect::<Vec<String>>()
            .len(),
        2
    );
}
