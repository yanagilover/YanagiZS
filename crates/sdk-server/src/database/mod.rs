use common::{config::DatabaseSettings, db_const};
use schema::{Account, Password, Username};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

pub mod schema;

const SDK_NAMESPACE: &str = "yanagi_sdk";
const SDK_DB_NAME: &str = "user";

const ACCOUNT_UID_COUNTER: &str = "account_uid";
const ACCOUNT_TABLE: &str = "account";

#[derive(Clone)]
pub struct DbContext(Surreal<Client>);
type Result<T> = std::result::Result<T, surrealdb::Error>;

impl DbContext {
    pub async fn connect(settings: &DatabaseSettings) -> Result<Self> {
        use surrealdb::opt::auth::Root;

        let database = Surreal::new::<Ws>(&settings.url).await?;
        database
            .signin(Root {
                username: &settings.username,
                password: &settings.password,
            })
            .await?;

        database.use_ns(SDK_NAMESPACE).use_db(SDK_DB_NAME).await?;

        // For uid auto-increment
        database
            .query(db_const::DEFINE_INCREMENT_FUNC_QUERY)
            .await?;

        Ok(Self(database))
    }

    pub async fn create_account(
        &self,
        username: Username,
        password: Password,
    ) -> Result<Option<Account>> {
        if self
            .get_account_by_name(username.as_str().to_string())
            .await?
            .is_some()
        {
            return Ok(None);
        }

        let account = Account::new(username, password);
        let id = self.get_next_uid(ACCOUNT_UID_COUNTER).await?;

        let account: Account = self
            .0
            .create((ACCOUNT_TABLE, id.to_string()))
            .content(account)
            .await?
            .unwrap();

        Ok(Some(account))
    }

    pub async fn get_account_by_name(&self, username: String) -> Result<Option<Account>> {
        const SELECT_WHERE_QUERY: &str = r#"SELECT * FROM account WHERE username = $value"#;
        const VALUE_BIND: &str = "value";

        self.0
            .query(SELECT_WHERE_QUERY)
            .bind((VALUE_BIND, username))
            .await?
            .take(0)
    }

    pub async fn get_account_by_uid(&self, uid: &str) -> Result<Option<Account>> {
        self.0.select((ACCOUNT_TABLE, uid)).await
    }

    async fn get_next_uid(&self, counter_name: &'static str) -> Result<u32> {
        Ok(self
            .0
            .query(db_const::UID_INCREMENT_QUERY)
            .bind((db_const::UID_COUNTER_NAME_BIND, counter_name))
            .await?
            .check()?
            .take::<Option<u32>>(0)?
            .unwrap())
    }
}
