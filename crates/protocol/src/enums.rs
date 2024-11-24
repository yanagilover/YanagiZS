use qwer::OctData;

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum ActionState {
    Init = 0,
    Running = 1,
    Finished = 2,
    Error = 3,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum EventState {
    Initing = 0,
    Running = 1,
    Pause = 2,
    WaitingMsg = 3,
    WaitingClient = 4,
    Finished = 5,
    Error = 6,
}

#[derive(OctData, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i16)]
pub enum HollowQuestType {
    Common = 0,
    MainQuest = 1,
    SideQuest = 2,
    Urgent = 3,
    UrgentSupplement = 4,
    Challenge = 5,
    ChallengeChaos = 6,
    AvatarSide = 7,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i16)]
pub enum HollowShopType {
    None = 0,
    Item = 1,
    Card = 2,
    Curse = 3,
    HollowItem = 4,
    Discount = 5,
    Gachashop = 6,
    UpgradeCard = 7,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum HollowShopCurrency {
    Coin = 1,
    Curse = 2,
    Random = 3,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum HollowBattleEventType {
    Default = 0,
    Normal = 1,
    Elite = 2,
    Boss = 3,
    LevelEnd = 4,
    LevelFin = 5,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum LocalPlayType {
    ArchiveLongFight = 212,
    TrainingRootTactics = 292,
    OperationBetaDemo = 216,
    LevelZero = 205,
    BossLittleBattleLongfight = 215,
    BuddyTowerdefenseBattle = 227,
    TrainingRoom = 290,
    ChessBoardLongfihgtBattle = 204,
    BossBattle = 210,
    DualElite = 208,
    HadalZoneBosschallenge = 224,
    BigBossBattle = 211,
    BigBossBattleLongfight = 217,
    SideScrollingThegunBattle = 221,
    MpBigBossBattle = 214,
    RallyLongFight = 207,
    PureHollowBattle = 280,
    S2RogueBattle = 226,
    AvatarDemoTrial = 213,
    GuideSpecial = 203,
    BossRushBattle = 218,
    HadalZone = 209,
    OperationTeamCoop = 219,
    PureHollowBattleLonghfight = 281,
    MapChallengeBattle = 291,
    BossNestHardBattle = 220,
    PureHollowBattleHardmode = 282,
    BabelTower = 223,
    MiniScapeBattle = 228,
    DailyChallenge = 206,
    HadalZoneAlivecount = 222,
    Unknown = 0,
    ArchiveBattle = 201,
    ChessBoardBattle = 202,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum WeatherType {
    None = -1,
    SunShine = 0,
    Cloudy = 2,
    Rain = 3,
    Thunder = 4,
    ThickFog = 5,
    ThickCloudy = 6,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum TimePeriodType {
    Morning = 0,
    Evening = 1,
    Night = 2,
}

impl WeatherType {
    pub fn to_protocol_string(&self) -> String {
        format!("{self:?}")
    }
}

impl TimePeriodType {
    pub fn to_protocol_string(&self) -> String {
        format!("{self:?}")
    }
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum MailState {
    New = 0,
    Old = 1,
    Read = 2,
    Awarded = 3,
    Removed = 4,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum ReportType {
    Fairy = 0,
    Dialog = 1,
    Task = 2,
    DialogInFairy = 3,
}

#[derive(OctData, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum UIType {
    Default = 0,
    None = 1,
    HollowQuest = 2,
    Archive = 3,
}

#[derive(OctData, Clone, Debug)]
#[repr(i16)]
pub enum QuestState {
    Unlocked = 0,
    Ready = 10,
    InProgress = 1,
    ToFinish = 2,
    Finished = 3,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum QuestStatisticsType {
    ArrivedLevel = 1,
    EventCount = 2,
    CostTime = 3,
    KilledEnemyCount = 4,
    ArcanaCount = 5,
    TarotCardCount = 6,
    StaminaOverLevelTimes = 7,
    RebornTimes = 8,
    FinishedEventTypeCount = 9,
    FinishedEventIDCount = 10,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DungeonContentDropPoolType {
    Card = 0,
    BaneCard = 1,
    Arcana = 2,
    Blessing = 3,
    Curse = 4,
    Reward = 5,
    HollowItem = 6,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FairyState {
    Unlock = 0,
    Close = 1,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i16)]
pub enum QuestType {
    ArchiveFile = 1,
    DungeonInner = 2,
    Hollow = 3,
    Manual = 4,
    MainCity = 5,
    HollowChallenge = 6,
    ArchiveBattle = 7,
    Knowledge = 8,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum HallRefreshStatus {
    Auto = 0,
    True = 1,
    False = 2,
}

#[derive(OctData, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PropertyType {
    SpMaxDelta = 11503,
    AtkDelta = 12103,
    ElementAbnormalPowerBase = 31401,
    BreakStunDeltaRl = 12205,
    BreakStun = 122,
    HpMaxBase = 11101,
    AddedDamageRatioFire3 = 31603,
    BreakStunDelta = 12203,
    HpHealRatio1 = 30601,
    CritBase = 20101,
    CritRes = 202,
    CurBuddyBattery = 320,
    DefDelta = 13103,
    AddedElementAccumulationRatio3 = 31303,
    AddedDamageRatioPhysics = 315,
    AddedDamageRatioPhysicsBattle = 1315,
    ShieldMax = 113,
    AddedDamageRatioPhysicsRl = 31505,
    EnduranceBattle = 1301,
    PenValue = 232,
    DamageTakeRatio3 = 30803,
    BreakStunRatioRl = 12204,
    ShieldMaxRatioRl = 11304,
    PenDeltaRl = 23205,
    MaxIndividualFever = 118,
    PenRatioBattle = 1231,
    PenRatioRl = 23105,
    AllDamageResistBattle = 1309,
    HpMax = 111,
    StunMax = 114,
    AtkBase = 12101,
    CritDmg = 211,
    AddedElementAccumulationRatio = 313,
    PenValueBase = 23201,
    StunMaxBattle = 1114,
    DefDeltaRl = 13105,
    AddedDamageRatioIce1 = 31701,
    HpMaxRatio = 11102,
    AddedDamageRatio = 307,
    FeverGetRatioRl = 31105,
    ElementAbnormalPowerBattle = 1314,
    DefBase = 13101,
    BreakStunRatio = 12202,
    SpRecover = 305,
    Sp = 5,
    AllDamageResist1 = 30901,
    ArmorMaxRatioRl = 11204,
    ElementMysteryBase = 31201,
    AddedDamageRatio1 = 30701,
    AddedDamageRatioEther1 = 31901,
    SpGetRatio1 = 31001,
    ElementMysteryDelta = 31203,
    CritResDelta = 20203,
    Crit = 201,
    HpHealRatio3 = 30603,
    AtkBattle = 1121,
    MapHpreserveCurhp = 10330,
    ArmorMaxDelta = 11203,
    UspMaxBase = 11601,
    ElementAbnormalPowerDelta = 31403,
    DamageTakeRatio1 = 30801,
    AddedDamageRatio3 = 30703,
    DamageTakeRatioBattle = 1308,
    CritDmgRes = 212,
    ArmorMaxBase = 11201,
    CritDmgResRl = 21205,
    PenDeltaBattle = 1232,
    AddedDamageRatioIceRl = 31705,
    AddedDamageRatioElec = 318,
    EnduranceBase = 30101,
    SpGetRatio = 310,
    AddedDamageRatioPhysics1 = 31501,
    Atk = 121,
    DefRatioRl = 13104,
    Armor = 2,
    AtkDeltaRl = 12105,
    ArmorMaxRatio = 11202,
    SpBattle = 1115,
    CritDmgRl = 21105,
    PenDelta = 23103,
    UspMax = 116,
    MapHpreserveMaxhp = 10320,
    HpMaxBattle = 1111,
    SpRecoverRatioRl = 30504,
    FeverGetRatio3 = 31103,
    HpHealRatioRl = 30605,
    EnumCount = 100,
    AddedDamageRatioFireRl = 31605,
    AddedDamageRatioElec3 = 31803,
    SpMaxDeltaRl = 11505,
    CritDmgResBase = 21201,
    SpGetRatioRl = 31005,
    AddedDamageRatioElecRl = 31805,
    ShieldMaxBattle = 1113,
    AllDamageResist = 309,
    EnduranceRatio = 30102,
    EnduranceDeltaRl = 30105,
    ElementAbnormalPower = 314,
    EnduranceDelta = 30103,
    HpHealRatio = 306,
    ArmorMax = 112,
    CritDmgResDelta = 21203,
    StunMaxRatio = 11402,
    AddedDamageRatioElecBattle = 1318,
    ShieldMaxRatio = 11302,
    AddedElementAccumulationRatioBattle = 1313,
    SpRecoverRatio = 30502,
    DamageTakeRatio = 308,
    ShieldMaxDelta = 11303,
    EnduranceRatioRl = 30104,
    AddedDamageRatioIce3 = 31703,
    CritResBattle = 1202,
    AddedElementAccumulationRatioRl = 31305,
    Stun = 4,
    AddedDamageRatioEther3 = 31903,
    AtkRatio = 12102,
    FeverGetRatioBattle = 1311,
    SpGetRatioBattle = 1310,
    ElementMysteryBattle = 1312,
    DamageTakeRatioRl = 30805,
    StunMaxBase = 11401,
    BreakStunBase = 12201,
    FeverGetRatio = 311,
    SpMax = 115,
    HpMaxDeltaRl = 11105,
    ActorMaxCurHP = 10350,
    ShieldMaxBase = 11301,
    MaxBuddyBattery = 321,
    Hp = 1,
    AtkRatioRl = 12104,
    AddedDamageRatioFire1 = 31601,
    ArmorMaxDeltaRl = 11205,
    ArmorMaxBattle = 1112,
    AddedDamageRatioPhysics3 = 31503,
    Shield = 3,
    CritBattle = 1201,
    Usp = 6,
    HpHealRatioBattle = 1306,
    CritDelta = 20103,
    AddedDamageRatioFire = 316,
    SpRecoverDeltaRl = 30505,
    AddedDamageRatioIce = 317,
    HpMaxRatioRl = 11104,
    DefBattle = 1131,
    AddedDamageRatioRl = 30705,
    DefRatio = 13102,
    AddedDamageRatioEtherRl = 31905,
    UspBattle = 1116,
    Endurance = 301,
    AddedDamageRatioEtherBattle = 1319,
    AllDamageResistRl = 30905,
    SpRecoverBase = 30501,
    AtkTrans = 12109,
    AddedDamageRatioFireBattle = 1316,
    AllDamageResist3 = 30903,
    CritResBase = 20201,
    StunMaxRatioRl = 11404,
    UspMaxDelta = 11603,
    Pen = 231,
    StunMaxDeltaRl = 11405,
    UspMaxDeltaRl = 11605,
    SpMaxBase = 11501,
    ShieldMaxDeltaRl = 11305,
    AddedDamageRatioBattle = 1307,
    Def = 131,
    AddedDamageRatioEther = 319,
    MapHpreserveAbsolute = 10340,
    PenValueDelta = 23203,
    SpRecoverDelta = 30503,
    IndividualFever = 8,
    AtkTransBase = 12108,
    FeverGetRatio1 = 31101,
    AddedDamageRatioElec1 = 31801,
    CritRl = 20105,
    SpGetRatio3 = 31003,
    CritResRl = 20205,
    BreakStunBattle = 1122,
    ElementAbnormalPowerRatio = 31402,
    SpRecoverBattle = 1305,
    CritDmgResBattle = 1212,
    Dead = 99,
    CritDmgBattle = 1211,
    AddedElementAccumulationRatio1 = 31301,
    StunMaxDelta = 11403,
    HpMaxDelta = 11103,
    AddedDamageRatioIceBattle = 1317,
    CritDmgBase = 21101,
    PenBase = 23101,
    CritDmgDelta = 21103,
    ElementMystery = 312,
}
