use beatrix_core::{
    bson::{doc, oid::ObjectId},
    mongo::MongoModel,
};
use beatrix_macro::MongoModel;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

mod factory;
use factory::mongo::{db, setup, Customer};

#[test]
pub fn test_default_entity_name() {
    #[derive(MongoModel, Serialize, Deserialize)]
    struct User {
        id: Option<ObjectId>,
    }

    assert_eq!(User::entity_name(), "user");

    #[derive(MongoModel, Serialize, Deserialize)]
    #[entity(name = "user_table")]
    struct UserCustom {
        id: Option<ObjectId>,
    }

    assert_eq!(UserCustom::entity_name(), "user_table");
}

#[test]
pub fn test_collection_name() {
    assert_eq!(Customer::COLLECTION_NAME, "customer")
}

#[test]
pub fn test_conversion_to_and_from_bson() {
    let customer = Customer {
        id: Some(ObjectId::new()),
        name: "Customer".into(),
        age: 10,
    };

    let doc = customer.to_document().unwrap();
    assert_eq!(customer, Customer::from_bson(doc).unwrap())
}

#[tokio::test]
pub async fn test_collection() {
    setup().await;

    let col = Customer::collection(db().await);
    assert_eq!(col.name(), "customer");
    assert_eq!(col.read_concern(), None);
    assert_eq!(col.write_concern(), None);
}

#[tokio::test]
pub async fn test_save() {
    setup().await;

    let mut customer = Customer {
        id: None,
        name: "Customer".into(),
        age: 10,
    };

    let pre_add_count = Customer::count(db().await, None, None).await.unwrap();
    customer.save(db().await).await.unwrap();
    let post_add_count = Customer::count(db().await, None, None).await.unwrap();
    assert_eq!(pre_add_count, 0);
    assert_eq!(post_add_count, 1);
    assert!(customer.id != None);
    assert_eq!(customer.name, "Customer".to_string());
}

#[tokio::test]
pub async fn test_find() {
    setup().await;

    let mut customer = Customer {
        id: None,
        name: "Customer".into(),
        age: 10,
    };

    customer.save(db().await).await.unwrap();

    assert_eq!(
        customer.clone(),
        Customer::find(db().await, doc! { "_id": customer.id.unwrap() }, None)
            .await
            .unwrap()
            .unwrap()
    );
}

#[tokio::test]
pub async fn test_filter_with_no_params() {
    setup().await;

    let mut customer = Customer {
        id: None,
        name: "Customer".into(),
        age: 10,
    };

    customer.save(db().await).await.unwrap();

    let mut customers_from_db: Vec<_> = Customer::filter(db().await, None, None)
        .await
        .unwrap()
        .collect()
        .await;

    assert_eq!(customers_from_db.len(), 1);

    let customer_from_db = customers_from_db.pop().unwrap();

    assert_eq!(customer_from_db.unwrap(), customer);
}

pub async fn test_filter_with_given_filters() {
    setup().await;

    let mut customer = Customer {
        id: None,
        name: "Customer".into(),
        age: 10,
    };

    customer.save(db().await).await.unwrap();

    let mut customer1 = Customer {
        id: None,
        name: "Customer".into(),
        age: 12,
    };

    customer1.save(db().await).await.unwrap();

    let mut customers_from_db: Vec<_> = Customer::filter(db().await, doc! {"age": 12}, None)
        .await
        .unwrap()
        .collect()
        .await;

    assert_eq!(customers_from_db.len(), 1);

    let customer_from_db = customers_from_db.pop().unwrap();

    assert_eq!(customer_from_db.unwrap(), customer);
}
