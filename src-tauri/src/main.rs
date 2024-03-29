// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod kraken;
mod user;
mod sec;

// TODO:
//  1. settings page
//      + Add keys (, ash)
//      + Remove keys (, ash)
//  2. portfolio page
//      + Ledger
//          - Pending transactions

// #[tauri::command(async)]
// async fn tickler() -> String {
//     thread::sleep(Duration::from_secs(10));
//     format!("Hey boooo!")
// }

#[tauri::command(async)]
async fn login(username: String, password: String) -> Result<user::User, String> {
    lib::log(lib::LogLevel::Info, "user", "login user").unwrap();
    match user::login_user(username, password) {
        Ok(u) => Ok(u),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "user", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn register(username: String, email: String, password: String) -> Result<String, String> {
    lib::log(lib::LogLevel::Info, "user", "register user").unwrap();
    match user::register_user(username, email, password) {
        Ok(_) => Ok("User registered.".to_string()),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "user", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn data(market: &str, interval: &str) -> Result<Vec<kraken::KrakenOHLC>, String> {
    let msg = format!("get data for {} at {} interval", market, interval);
    lib::log(lib::LogLevel::Info, "kraken", &msg).unwrap();
    match kraken::get_data(market, interval).await {
        Ok(v) => Ok(v.into()),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "kraken", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn status() -> bool {
    lib::log(lib::LogLevel::Info, "kraken", "query kraken online status").unwrap();
    match kraken::is_online().await {
        Ok(b) => b,
        Err(_) => false,
    }
}

#[tauri::command]
fn get_user() -> Result<user::User, String> {
    lib::log(lib::LogLevel::Info, "user", "get logged in user").unwrap();
    match user::get_logged_in_user() {
        Ok(u) => Ok(u),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn logout() -> bool {
    lib::log(lib::LogLevel::Info, "user", "logout user").unwrap();
    match lib::logout() {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command(async)]
async fn add_kraken(kraken_api: String, kraken_sk: String, password: String) -> Result<String, String> {
    lib::log(lib::LogLevel::Info, "user", "add user kraken api keys").unwrap();
    match user::add_kraken_keys(kraken_api, kraken_sk, password).await {
        Ok(_) => Ok("Keys updated".to_string()),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "user", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command]
fn change_password(old_password: String, new_password: String) -> Result<String, String> {
    lib::log(lib::LogLevel::Info, "user", "change user password").unwrap();
    match user::change_password(old_password, new_password) {
        Ok(_) => Ok("password changed".to_string()),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "user", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command]
fn change_email(password: String, email: String) -> Result<String, String> {
    lib::log(lib::LogLevel::Info, "user", "change user email").unwrap();
    match user::change_email(password, email) {
        Ok(_) => Ok("email changed".to_string()),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "user", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn unlock(password: String) -> Result<user::SensitiveParams, String> {
    lib::log(lib::LogLevel::Info, "user", "unlock user params").unwrap();
    match user::get_params(&password) {
        Ok(p) => Ok(p),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "user", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn remove_kraken_keys(password: String) -> Result<String, String> {
    lib::log(lib::LogLevel::Info, "user", "remove kraken api keys").unwrap();
    match user::rm_kraken_keys(password) {
        Ok(_) => Ok("Keys deleted".to_string()),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "user", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn get_balances(kraken_api: String, kraken_sk: String) -> Result<Vec<kraken::KrakenBalance>, String> {
    lib::log(lib::LogLevel::Info, "kraken", "get kraken account balances").unwrap();
    match kraken::balances(kraken_api, kraken_sk).await {
        Ok(v) => Ok(v),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "kraken", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn get_trading_pairs() -> Result<Vec<kraken::TradingPair>, String> {
    lib::log(lib::LogLevel::Info, "kraken", "get kraken trading pairs").unwrap();
    match kraken::get_trading_pairs().await {
        Ok(v) => Ok(v),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "kraken", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn get_ledger(kraken_api: String, kraken_sk: String) -> Result<Vec<kraken::LedgerEntry>, String> {
    lib::log(lib::LogLevel::Info, "kraken", "get kraken account ledger").unwrap();
    match kraken::get_ledger(kraken_api, kraken_sk).await {
        Ok(v) => Ok(v),
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "kraken", &err_str).unwrap();
            Err(err_str)
        },
    }
}

#[tauri::command(async)]
async fn submit_order(kraken_api: String, kraken_sk: String, quote: String, base: String, side: String, amt: f64) -> Result<(), String> {
    let info_msg = format!("submit {} order for {}/{}", &side, &base, &quote);
    let debug_msg = format!("submit {} {} order for {}/{}", &side, &amt, &base, &quote);
    lib::log(lib::LogLevel::Info, "kraken", &info_msg).unwrap();
    lib::log(lib::LogLevel::Debug, "kraken", &debug_msg).unwrap();
    match kraken::add_order(kraken_api, kraken_sk, quote, base, side, amt).await {
        Ok(s) => {
            lib::log(lib::LogLevel::Debug, "kraken", &s).unwrap();
            Ok(())
        },
        Err(e) => {
            let err_str = e.to_string();
            lib::log(lib::LogLevel::Error, "kraken", &err_str).unwrap();
            Err(err_str)
        },
    }
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![add_kraken,
                                                 change_email,
                                                 change_password,
                                                 data,
                                                 get_balances,
                                                 get_ledger,
                                                 get_trading_pairs,
                                                 get_user,
                                                 login,
                                                 logout,
                                                 register,
                                                 remove_kraken_keys,
                                                 status,
                                                 submit_order,
                                                 unlock])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
