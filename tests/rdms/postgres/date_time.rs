use std::collections::HashMap;
use std::ops::Not;
use std::str::FromStr;

use beatrix::relational::db::Database;
use beatrix::relational::table::Table;
use beatrix::relational::Postgres;
use beatrix::sqlx;
use beatrix::sqlx::types::chrono::NaiveDateTime;
use beatrix_macro::RelationalModel;

#[derive(Debug, Clone, RelationalModel, PartialEq, Eq)]
struct User {
    id: i32,
    name: String,
    email: String,
    gender: String,
    salary: i32,
    dob: NaiveDateTime,
    // jsonb: Option<sqlx::types::Json<HashMap<String, String>>>,
}

#[tokio::test]
pub async fn date_filters() {
    let db = Postgres::new("postgres://postgres:asdqwe@localhost/beatrix", None)
        .await
        .unwrap();

    let users = User::select()
        .filter(
            User::dob()
                .eq(NaiveDateTime::from_str("1997-10-29T01:00:53").unwrap())
                .boxed(),
        )
        .fetch_all::<User, _>(&db)
        .await
        .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(
        users[0],
        User {
            id: 17,
            name: "Giff Ommanney".to_string(),
            email: "gommanneyg@ocn.ne.jp".to_string(),
            gender: "Non-binary".to_string(),
            salary: 68798,
            dob: NaiveDateTime::from_str("1997-10-29T01:00:53").unwrap(),
            // jsonb: None,
        }
    );
}

#[tokio::test]
pub async fn date_between_filters() {
    let db = Postgres::new("postgres://postgres:asdqwe@localhost/beatrix", None)
        .await
        .unwrap();

    let users = User::select()
        .filter(
            User::dob()
                .between(
                    NaiveDateTime::from_str("1997-10-29T01:00:00").unwrap(),
                    NaiveDateTime::from_str("1997-10-29T02:00:00").unwrap(),
                )
                .into(),
        )
        .fetch_all::<User, _>(&db)
        .await
        .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(
        users[0],
        User {
            id: 17,
            name: "Giff Ommanney".to_string(),
            email: "gommanneyg@ocn.ne.jp".to_string(),
            gender: "Non-binary".to_string(),
            salary: 68798,
            dob: NaiveDateTime::from_str("1997-10-29T01:00:53").unwrap(),
            // jsonb: None,
        }
    );
}

#[tokio::test]
pub async fn date_not_between_filters() {
    let db = Postgres::new("postgres://postgres:asdqwe@localhost/beatrix", None)
        .await
        .unwrap();

    let users = User::select()
        .filter(
            User::dob()
                .between(
                    NaiveDateTime::from_str("1997-10-29T01:00:00").unwrap(),
                    NaiveDateTime::from_str("1997-10-29T02:00:00").unwrap(),
                )
                .not()
                .into(),
        )
        .fetch_all::<User, _>(&db)
        .await
        .unwrap();

    assert_eq!(users.len(), 49);
    assert_eq!(
        users[0],
        User {
            id: 1,
            name: "Waldo Brookzie".to_string(),
            email: "wbrookzie0@nationalgeographic.com".to_string(),
            gender: "Agender".to_string(),
            salary: 78818,
            dob: NaiveDateTime::from_str("1997-04-01T08:58:44").unwrap(),
            // jsonb: None,
        }
    );
}
