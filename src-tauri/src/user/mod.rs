use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::{kraken, sec};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub pwhash: String,
    pub uid: String,
    pub kraken_auth: bool,
    pub ash_auth: bool,
    pub sensitive: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SensitiveParams {
    pub email: String,
    pub kraken_key: Option<String>,
    pub kraken_sk: Option<String>,
    pub ash_key: Option<String>,
    pub ash_sk: Option<String>,
}

pub fn change_password(old_password: String, new_password: String) -> Result<()> {
    let mut u = get_logged_in_user()?;
    let sensitive_params = get_params(&old_password)?;
    let sensitive_str = serde_json::to_string(&sensitive_params)?;
    let sensitive = sec::encrypt(&new_password, &u.uid, &sensitive_str)?;
    // FIXME: Validate with ASH server -> makes async
    u.pwhash = sec::pwhash(&new_password)?;
    u.sensitive = sensitive;
    update_users(&u)?;
    lib::logout()
}

pub fn rm_kraken_keys(password: String) -> Result<()> {
    let mut u = get_logged_in_user()?;
    let mut sensitive_params = get_params(&password)?;
    sensitive_params.kraken_key = None;
    sensitive_params.kraken_sk = None;
    let sensitive_str = serde_json::to_string(&sensitive_params)?;
    let sensitive = sec::encrypt(&password, &u.uid, &sensitive_str)?;
    u.kraken_auth = false;
    u.sensitive = sensitive;
    update_login(&u)?;
    update_users(&u)
}

pub fn change_email(password: String, email: String) -> Result<()> {
    let mut u = get_logged_in_user()?;
    let mut sensitive_params = get_params(&password)?;
    // FIXME: Validate with ASH server -> makes async
    if email == sensitive_params.email {
        bail!("Email already exists");
    }
    sensitive_params.email = email;
    let sensitive_str = serde_json::to_string(&sensitive_params)?;
    let sensitive = sec::encrypt(&password, &u.uid, &sensitive_str)?;
    u.sensitive = sensitive;
    update_login(&u)?;
    update_users(&u)
}

pub fn get_params(password: &str) -> Result<SensitiveParams> {
    let u = get_logged_in_user()?;
    if !sec::verify_pass(&password, &u.pwhash)? {
        bail!("Invalid password");
    }
    let sensitive_params: SensitiveParams =
        serde_json::from_str(&sec::decrypt(&password, &u.uid, &u.sensitive)?)?;
    Ok(sensitive_params)
}

pub async fn add_kraken_keys(kraken_api: String, kraken_sk: String, password: String) -> Result<()> {
    let mut u = get_logged_in_user()?;
    if !sec::verify_pass(&password, &u.pwhash)? {
        bail!("Invalid password");
    }
    if u.kraken_auth {
        bail!("User already has Kraken API keys. Remove keys to continue.");
    }
    kraken::balances(kraken_api.clone(), kraken_sk.clone()).await?;
    let mut sensitive_params: SensitiveParams =
        serde_json::from_str(&sec::decrypt(&password, &u.uid, &u.sensitive)?)?;
    sensitive_params.kraken_key = Some(kraken_api);
    sensitive_params.kraken_sk = Some(kraken_sk);
    let sensitive_str = serde_json::to_string(&sensitive_params)?;
    let sensitive = sec::encrypt(&password, &u.uid, &sensitive_str)?;
    u.sensitive = sensitive;
    u.kraken_auth = true;
    update_users(&u)?;
    update_login(&u)?;
    Ok(())
}

pub fn get_logged_in_user() -> Result<User> {
    let login_file = lib::get_file(lib::ProjFiles::Login)?;
    if Path::new(&login_file).exists() {
        let content = lib::read_file(&login_file)?;
        let user: User = serde_json::from_str(&content)?;
        return Ok(user);
    }
    Err(anyhow!("User not logged in"))
}

pub fn login_user(username: String, password: String) -> Result<User> {
    lib::setup_file_struct()?;
    let users = get_users()?;
    if !user_exists(&username, &users) {
        bail!("Username not found");
    }
    if let Some(user) = get_user_by_name(&username, users) {
        if sec::verify_pass(&password, &user.pwhash)? {
            update_login(&user)?;
            return Ok(user);
        }
        bail!("Invalid password");
    }
    Err(anyhow!("Could not get user by name"))
}

pub fn register_user(username: String, email: String, password: String) -> Result<()> {
    lib::setup_file_struct()?;
    let pwhash = sec::pwhash(&password)?;
    let user_file = lib::get_file(lib::ProjFiles::Users)?;
    let mut users = get_users()?;
    if user_exists(&username, &users) {
        bail!("Username already exists");
    }
    let uid = sec::get_salt();
    let sensitive_params = serde_json::to_string(&SensitiveParams {
        email,
        ..Default::default()
    })?;
    let sensitive = sec::encrypt(&password, &uid, &sensitive_params)?;
    let user = User {
        username,
        pwhash,
        uid,
        kraken_auth: false,
        ash_auth: false,
        sensitive,
    };
    users.push(user);
    let mut f = File::create(user_file)?;
    f.write_all(serde_json::to_string(&users)?.as_bytes())?;
    Ok(())
}

fn get_user_by_name(username: &str, users: Vec<User>) -> Option<User> {
    users.into_iter().find(|u| u.username == username)
}

fn user_exists(username: &str, users: &Vec<User>) -> bool {
    for user in users.iter() {
        if user.username == username {
            return true;
        }
    }
    false
}

fn get_users() -> Result<Vec<User>> {
    let user_file = lib::get_file(lib::ProjFiles::Users)?;
    if Path::new(&user_file).exists() {
        let content = lib::read_file(&user_file)?;
        let users: Vec<User> = serde_json::from_str(&content)?;
        return Ok(users);
    }
    let v: Vec<User> = vec![];
    Ok(v)
}

fn update_login(user: &User) -> Result<()> {
    let login_file = lib::get_file(lib::ProjFiles::Login)?;
    let mut f = File::create(login_file)?;
    f.write_all(serde_json::to_string(&user)?.as_bytes())?;
    Ok(())
}

fn update_users(user: &User) -> Result<()> {
    let user_file = lib::get_file(lib::ProjFiles::Users)?;
    let users = get_users()?;
    let mut new_users: Vec<User> = vec![];
    let username = user.username.clone();
    for tmp_user in users.into_iter() {
        if tmp_user.username == username {
            new_users.push(user.clone());
        }
        else {
            new_users.push(tmp_user);
        }
    }
    let mut f = File::create(user_file)?;
    f.write_all(serde_json::to_string(&new_users)?.as_bytes())?;
    Ok(())
}
