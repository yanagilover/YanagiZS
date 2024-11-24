use std::io::Cursor;

use common::config::DatabaseSettings;
use protocol::player_info::PlayerInfo;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

use crate::player_util::{self, UidCounter};

const DB_NAMESPACE: &str = "yanagi";
const GAME_DB_NAME: &str = "nap";

const PLAYER_DATA_TABLE: &str = "player_data";

#[derive(Clone)]
pub struct DbContext(Surreal<Client>);
type Result<T> = std::result::Result<T, surrealdb::Error>;

#[derive(Deserialize, Serialize)]
struct PlayerData {
    pub player_uid: u64,
    pub game_uid_counter: u32,
    pub player_info_blob: String,
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
        Ok(Self(database))
    }

    pub async fn get_or_create_player_data(
        &self,
        player_uid: u64,
    ) -> Result<(UidCounter, PlayerInfo)> {
        let player_uid_str = player_uid.to_string();
        let data: Option<PlayerData> = self.0.select((PLAYER_DATA_TABLE, &player_uid_str)).await?;
        if let Some(data) = data {
            let uid_counter =
                UidCounter::new((player_uid & 0xFFFFFFFF) as u32, data.game_uid_counter);
            let player_info = deserialize_player_info(&data.player_info_blob);

            return Ok((uid_counter, player_info));
        }

        let (uid_counter, player_info) =
            player_util::create_starting_player_info(player_uid, "ReversedRooms");
        let player_info_blob = serialize_player_info(&player_info);

        let _: PlayerData = self
            .0
            .create((PLAYER_DATA_TABLE, player_uid_str))
            .content(PlayerData {
                player_uid,
                game_uid_counter: uid_counter.last_uid(),
                player_info_blob,
            })
            .await?
            .unwrap();

        Ok((uid_counter, player_info))
    }

    pub async fn save_player_data(&self, last_uid: u32, player_info: &PlayerInfo) -> Result<()> {
        let player_uid = player_info.uid.unwrap();

        let _: PlayerData = self
            .0
            .update((PLAYER_DATA_TABLE, player_uid.to_string()))
            .content(PlayerData {
                player_uid,
                game_uid_counter: last_uid,
                player_info_blob: serialize_player_info(player_info),
            })
            .await?
            .unwrap();

        Ok(())
    }
}

pub fn serialize_player_info(player_info: &PlayerInfo) -> String {
    use qwer::OctData;

    let mut buf = Vec::new();
    player_info
        .marshal_to(&mut Cursor::new(&mut buf), 2)
        .unwrap();

    rbase64::encode(&buf)
}

pub fn deserialize_player_info(blob_b64: &str) -> PlayerInfo {
    use qwer::OctData;

    let buf = rbase64::decode(blob_b64).unwrap();
    PlayerInfo::unmarshal_from(&mut Cursor::new(&buf[..]), 2).unwrap()
}
