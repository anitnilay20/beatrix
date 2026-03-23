use crate::relational::data_type::bind::BindValue;

type OptionalBoxedValue<DB> = Option<Box<dyn BindValue<DB>>>;

pub struct QueryBuilder<DB: sqlx::Database> {
    query: String,
    filter: Vec<(String, OptionalBoxedValue<DB>)>,
}

impl<DB: sqlx::Database> QueryBuilder<DB> {
    pub fn new(query: String) -> Self {
        Self {
            query,
            filter: Vec::new(),
        }
    }

    pub fn push<T>(&mut self, sql: &str, value: Option<T>)
    where
        T: BindValue<DB> + 'static
    {
        let boxed_val: Option<Box<dyn BindValue<DB>>> = value.map(|v| Box::new(v) as Box<dyn BindValue<DB>>);
        self.filter.push((sql.to_string(), boxed_val));
    }

    pub fn push_sql(&mut self, sql: &str) {
        self.filter.push((sql.to_string(), None));
    }

    pub fn query(&self) -> String {
        self.query.clone()
    }

    pub fn build(&self) -> &[(String, OptionalBoxedValue<DB>)] {
        &self.filter
    }
}

// enum QueryType {
//     CreateTable,
//     AlterTable,
//     DropTable,
//     Truncate,
//     Insert,
//     Update,
//     Delete,
//     // Select(Select),
//     // SubQuery(Select),
//     Transaction,
// }

// pub struct Query {
//     query_type: QueryType,
// }

// // #[async_trait]
// impl Query {
//     // fn query(&mut self) -> String {
//     //     match &self.query_type {
//     //         QueryType::Select(select) => select.clone().to_sql(),
//     //         _ => "".to_string(),
//     //     }
//     // }

//     // async fn fetch_all<T, DB>(mut self, db: DB) -> Result<Vec<T>, tokio_postgres::Error>
//     // where
//     //     T: Serialize + DeserializeOwned,
//     //     DB: Database,
//     // {
//     //     let db = db.client();
//     //     let rows = match db {
//     //         DBType::Postgres(client) => {
//     //             client
//     //                 .query(&client.prepare(&self.query()).await?, &[])
//     //                 .await?
//     //         }
//     //     };

//     //     let result: Vec<T> = serde_postgres::from_rows(&rows).unwrap();

//     //     Ok(result)
//     // }

//     // async fn fetch_one<T, DB, SDB, E>(&mut self, db: &'static DB) -> Result<T>
//     // where
//     //     SDB: sqlx::Database + sqlx::Database<Row = T>,
//     //     E: SqlxExec<'static, Database = SDB>,
//     //     DB: 'static,
//     //     DB: Database<'static, SDB, E> + std::marker::Sync,
//     //     <SDB as sqlx::database::HasArguments<'static>>::Arguments:
//     //         sqlx::IntoArguments<'static, SDB>,
//     //     // R: sqlx::SqlxExec<'a> + sqlx::Database + sqlx::Database<Row = T>,
//     //     // <R as sqlx::database::HasArguments<'a>>::Arguments: sqlx::IntoArguments<'a, R>
//     // {
//     //     let db = db.sqlx_db();
//     //     self.query();
//     //     let result: T = sqlx::query(self.query).fetch_one(db).await?;
//     //     Ok(result)
//     // }
// }
