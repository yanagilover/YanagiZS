pub mod config;
pub mod db_const;
pub mod logging;
pub mod time_util;

pub enum ServiceStatus {
    StopServer(u64),
    Running,
}

pub fn print_splash() {
    println!(" __     __                     _ ______ _____ \n \\ \\   / /                    (_)___  // ____|\n  \\ \\_/ /_ _ _ __   __ _  __ _ _   / /| (___  \n   \\   / _` | '_ \\ / _` |/ _` | | / /  \\___ \\ \n    | | (_| | | | | (_| | (_| | |/ /__ ____) |\n    |_|\\__,_|_| |_|\\__,_|\\__, |_/_____|_____/ \n                          __/ |               \n                         |___/                ");
}
