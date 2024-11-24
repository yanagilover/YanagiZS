use common::{config::DatabaseSettings, db_const};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

const DB_NAMESPACE: &str = "yanagi";
const GAME_DB_NAME: &str = "nap";

const USER_UID_COUNTER: &str = "user_uid_cnt";
const USER_UID_TABLE: &str = "user_uid";

#[derive(Clone)]
pub struct DbContext(Surreal<Client>);
type Result<T> = std::result::Result<T, surrealdb::Error>;

#[derive(Serialize, Deserialize)]
pub struct UserUid {
    account_uid: String,
    account_token: String,
    player_uid: u32,
}

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

        database.use_ns(DB_NAMESPACE).use_db(GAME_DB_NAME).await?;

        // For uid auto-increment
        database
            .query(db_const::DEFINE_INCREMENT_FUNC_QUERY)
            .await?;

        Ok(Self(database))
    }

    pub async fn get_or_create_uid(
        &self,
        account_uid: &str,
        account_token: &str,
    ) -> Result<Option<u32>> {
        if let Some(user_uid) = self.get_user_uid_by_account(account_uid).await? {
            return Ok((user_uid.account_token == account_token).then_some(user_uid.player_uid));
        }

        let uid = self.get_next_uid(USER_UID_COUNTER).await?;
        let _: UserUid = self
            .0
            .create((USER_UID_TABLE, account_uid.to_string()))
            .content(UserUid {
                account_uid: account_uid.to_string(),
                account_token: account_token.to_string(),
                player_uid: uid,
            })
            .await?
            .unwrap();

        Ok(Some(uid))
    }

    async fn get_user_uid_by_account(&self, account_uid: &str) -> Result<Option<UserUid>> {
        self.0.select((USER_UID_TABLE, account_uid)).await
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
