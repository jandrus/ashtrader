use std::collections::HashMap;

use anyhow::{anyhow, bail, ensure, Result};
use base64::prelude::*;
use hmac::{Hmac, Mac};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256, Sha512};

const BASE_URL: &str = "https://api.kraken.com";

type HmacSha512 = Hmac<Sha512>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct KrakenTickerResponse {
    error: Vec<Option<String>>,
    result: Option<HashMap<String, KrakenTicker>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenTicker {
    pub a: Vec<String>,
    pub b: Vec<String>,
    pub c: Vec<String>,
    pub v: Vec<String>,
    pub p: Vec<String>,
    pub t: Vec<i64>,
    pub l: Vec<String>,
    pub h: Vec<String>,
    pub o: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct KrakenStatusResponse {
    error: Vec<Option<String>>,
    result: Option<KrakenStatus>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct KrakenStatus {
    status: String,
    timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct KrakenPairsResponse {
    error: Vec<Option<String>>,
    result: Option<HashMap<String, KrakenPairs>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenPairs {
    pub altname: String,
    pub wsname: String,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub lot: String,
    pub cost_decimals: i64,
    pub pair_decimals: i64,
    pub lot_decimals: i64,
    pub lot_multiplier: i64,
    pub leverage_buy: Vec<f64>,
    pub leverage_sell: Vec<f64>,
    pub fees: Vec<Vec<f64>>,
    pub fees_maker: Vec<Vec<f64>>,
    pub fee_volume_currency: String,
    pub margin_call: f64,
    pub margin_stop: f64,
    pub ordermin: String,
    pub costmin: String,
    pub tick_size: String,
    pub status: String,
    pub long_position_limit: Option<f64>,
    pub short_position_limit: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenTradingPair {
    pub exchange_name: String,
    pub base: String,
    pub quote: String,
    pub price: f64,
    pub base_decimals: i64,
    pub quote_decimals: i64,
    pub ordermin: f64,
    pub costmin: f64,
    pub tick_size: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenOHLCResponse {
    error: Vec<Option<String>>,
    result: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenOHLC {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenBalancesResponse {
    error: Vec<Option<String>>,
    result: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenBalance {
    name: String,
    amount: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenLedgerResponse {
    error: Vec<Option<String>>,
    result: Option<KrakenLedgerResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenLedgerResult {
    pub ledger: HashMap<String, KrakenLedgerEntry>,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KrakenLedgerEntry {
    pub refid: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub subtype: String,
    pub aclass: String,
    pub asset: String,
    pub amount: String,
    pub fee: String,
    pub balance: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub txid: String,
    pub refid: String,
    pub time: f64,
    pub type_field: String,
    pub asset: String,
    pub amount: f64,
    pub fee: f64,
    pub balance: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingPair {
    pub exchange_name: String,
    pub base: String,
    pub quote: String,
    pub price: f64,
    pub high: f64,
    pub delta: f64,
    pub base_decimals: i64,
    pub quote_decimals: i64,
    pub ordermin: f64,
    pub costmin: f64,
    pub tick_size: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct KrakenOrderResponse {
    error: Vec<Option<String>>,
    result: Option<KrakenOrderResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct KrakenOrderResult {
    descr: KrakenOrder,
    txid: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct KrakenOrder {
    order: Option<String>,
    close: Option<String>
}

pub async fn add_order(api_key: String, sk: String, quote: String, base: String, side: String, amt: f64) -> Result<String> {
    let pairs = get_trading_pairs().await?;
    let trading_pair = match pairs.into_iter().find(|p| p.quote == quote && p.base == base) {
        Some(p) => p,
        None => bail!("invalid market"),
    };
    let balances = balances(api_key.clone(), sk.clone()).await?;
    let balance;
    match side.as_str() {
        "buy" => {
            ensure!(trading_pair.costmin < amt, "amount below minimum buy");
            balance = match balances.into_iter().find(|b| b.name == quote) {
                Some(q) => q,
                None => bail!("balance not found"),
            };
        },
        "sell" => {
            ensure!(trading_pair.ordermin < amt, "amount below minimum sell");
            balance = match balances.into_iter().find(|b| b.name == base) {
                Some(q) => q,
                None => bail!("balance not found"),
            };
        },
        _ => bail!("invalid side"),
    };
    ensure!(balance.amount >= amt, "amount exceeds balance");
    let uri_path = "/0/private/AddOrder";
    let mut payload: HashMap<String, String> = HashMap::new();
    let nonce = chrono::Utc::now().timestamp_millis() + 1;
    payload.insert("nonce".to_string(), nonce.to_string());
    payload.insert("ordertype".to_string(), "market".to_string());
    payload.insert("type".to_string(), side.clone());
    payload.insert("volume".to_string(), amt.to_string());
    payload.insert("pair".to_string(), trading_pair.exchange_name);
    // payload.insert("validate".to_string(), "true".to_string());
    if side == "buy" {
        payload.insert("oflags".to_string(), "viqc".to_string());
    }
    let sig = get_kraken_sig(uri_path, sk, payload.clone())?;
    let full_url = format!("{}{}", BASE_URL, uri_path);
    let body = get_postdata(payload);
    let client = reqwest::Client::builder().build()?;
    let resp: KrakenOrderResponse = client
        .post(full_url)
        .body(body)
        .header("API-Key", api_key)
        .header("API-Sign", sig)
        .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
        .send()
        .await?.json().await?;
    match resp.result {
        Some(res) => {
            let msg = serde_json::to_string(&res)?;
            Ok(msg)
        },
        None => {
            let err_msg = match resp.error.first().unwrap() {
                Some(s) => s.to_string(),
                None => "Unknown Error".to_string(),
            };
            Err(anyhow!("{}", err_msg))
        }
    }
}

pub async fn get_trading_pairs() -> Result<Vec<TradingPair>> {
    let pairs = get_pairs().await?;
    let tickers = get_tickers().await?;
    let mut trading_pairs: Vec<TradingPair> = vec![];
    for (key, val) in pairs.into_iter() {
        if lib::ILLEGAL_ASSETS.contains(&val.base.as_str()) || lib::ILLEGAL_ASSETS.contains(&val.quote.as_str()) || val.base.contains(".") || val.quote.contains(".") {
            continue;
        }
        let mut price = 0.0;
        let mut high = 0.0;
        let mut delta = 0.0;
        if tickers.contains_key(&key) {
            if let Some(s) = tickers[&key].c.first() {
                price = s.parse::<f64>()?;
            }
            if let Some(s) = tickers[&key].h.last() {
                high = s.parse::<f64>()?;
            }
            let open = tickers[&key].o.parse::<f64>()?;
            delta = (price - open) / open;
        }
        let base;
        if lib::TRANSLATIONS.contains_key(&val.base) {
            base = lib::TRANSLATIONS[&val.base].to_string();
        } else {
            base = val.base;
        }
        let quote;
        if lib::TRANSLATIONS.contains_key(&val.quote) {
           quote = lib::TRANSLATIONS[&val.quote].to_string();
        } else {
            quote = val.quote;
        }
        trading_pairs.push(TradingPair {
            exchange_name: key,
            base,
            quote,
            price,
            high,
            delta,
            base_decimals: val.cost_decimals,
            quote_decimals: val.pair_decimals,
            ordermin: val.ordermin.parse::<f64>()?,
            costmin: val.costmin.parse::<f64>()?,
            tick_size: val.tick_size.parse::<f64>()?,

        })
    }
    trading_pairs.sort_by(|a, b| a.base.partial_cmp(&b.base).unwrap());
    Ok(trading_pairs)
}

pub async fn get_ledger(api_key: String, sk: String) -> Result<Vec<LedgerEntry>> {
    let uri_path = "/0/private/Ledgers";
    let mut ofs: usize = 0;
    let mut ledger: Vec<LedgerEntry> = vec![];
    let mut nonce = chrono::Utc::now().timestamp_millis();
    loop {
        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("nonce".to_string(), nonce.to_string());
        payload.insert("ofs".to_string(), ofs.to_string());
        let sig = get_kraken_sig(uri_path, sk.clone(), payload.clone())?;
        let full_url = format!("{}{}", BASE_URL, uri_path);
        let body = get_postdata(payload);
        let client = reqwest::Client::builder().build()?;
        let resp: KrakenLedgerResponse = client
            .post(full_url)
            .body(body)
            .header("API-Key", api_key.clone())
            .header("API-Sign", sig)
            .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
            .send()
            .await?.json().await?;
        match resp.result {
            Some(res) => {
                let count = res.count;
                for (k, v) in res.ledger.into_iter() {
                    let asset;
                    if lib::TRANSLATIONS.contains_key(&v.asset) {
                        asset = lib::TRANSLATIONS[&v.asset].to_string();
                    } else {
                        asset = v.asset;
                    }
                    let amount: f64 = v.amount.parse()?;
                    let mut type_field = v.type_field;
                    if type_field == "trade" {
                        if amount > 0.0 {
                            type_field = "buy".to_string();
                        } else {
                            type_field = "sell".to_string();
                        }
                    }
                    ledger.push(LedgerEntry {
                        txid: k,
                        refid: v.refid,
                        time: v.time,
                        type_field,
                        asset,
                        amount,
                        fee: v.fee.parse()?,
                        balance: v.balance.parse()?
                    });
                }
                if ledger.len() >= count as usize {
                    return Ok(ledger);
                }
                ofs += 50;
                nonce += 1;
            },
            None => {
                let err_msg = match resp.error.first().unwrap() {
                    Some(s) => s.to_string(),
                    None => "Unknown Error".to_string(),
                };
                bail!("KrakenError [LEDGER]: {}", err_msg);
            }
        }
    };
}

pub async fn balances(api_key: String, sk: String) -> Result<Vec<KrakenBalance>> {
    let uri_path = "/0/private/Balance";
    let mut payload: HashMap<String, String> = HashMap::new();
    let nonce = chrono::Utc::now().timestamp_millis().to_string();
    payload.insert("nonce".to_string(), nonce.clone());
    let sig = get_kraken_sig(uri_path, sk, payload.clone())?;
    let full_url = format!("{}{}", BASE_URL, uri_path);
    let body = get_postdata(payload);
    let client = reqwest::Client::builder().build()?;
    let resp: KrakenBalancesResponse = client
        .post(full_url)
        .body(body)
        .header("API-Key", api_key)
        .header("API-Sign", sig)
        .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
        .send()
        .await?.json().await?;
    match resp.result {
        Some(res) => {
            let mut bals: Vec<KrakenBalance> = vec![];
            for bal in res.into_iter() {
                let amount: f64 = bal.1.parse()?;
                let name;
                if lib::TRANSLATIONS.contains_key(&bal.0) {
                    name = lib::TRANSLATIONS[&bal.0].to_string();
                } else {
                    name = bal.0;
                }
                bals.push(KrakenBalance {
                    name,
                    amount,
                });
            }
            Ok(bals)
        },
        None => {
            let err_msg = match resp.error.first().unwrap() {
                Some(s) => s.to_string(),
                None => "Unknown Error".to_string(),
            };
            Err(anyhow!("KrakenError [BALANCES]: {}", err_msg))
        }
    }
}

pub async fn is_online() -> Result<bool> {
    let full_url = format!("{}/0/public/SystemStatus", BASE_URL);
    let resp: KrakenStatusResponse = reqwest::get(full_url).await?.json().await?;
    if !resp.error.is_empty() {
        return Ok(false)
    }
    let status_msg = match resp.result {
        Some(ks) => ks,
        None => {
            return Ok(false)
        }
    };
    if status_msg.status == "online" {
        return Ok(true)
    }
    Ok(false)
}

pub async fn get_data(market: &str, interval: &str) -> Result<Vec<KrakenOHLC>> {
    let full_url = format!("{}/0/public/OHLC?pair={}&interval={}", BASE_URL, market, interval);
    let resp: KrakenOHLCResponse = reqwest::get(full_url).await?.json().await?;
    if !resp.error.is_empty() {
        let err_msg = match resp.error.first().unwrap() {
            Some(s) => s.to_owned(),
            None => "Asset pairs request failed".to_string(),
        };
        bail!(err_msg);
    }
    let mut data: Vec<KrakenOHLC> = vec![];
    if let Some(r) = resp.result {
        let obj = r.as_object().unwrap().values().next().unwrap().as_array().unwrap();
        for x in obj.into_iter() {
            let arr = x.as_array().unwrap();
            data.push(KrakenOHLC {
                timestamp: arr[0].as_u64().unwrap(),
                open: arr[1].as_str().unwrap().parse::<f64>()?,
                high: arr[2].as_str().unwrap().parse::<f64>()?,
                low: arr[3].as_str().unwrap().parse::<f64>()?,
                close: arr[4].as_str().unwrap().parse::<f64>()?,
                volume: arr[6].as_str().unwrap().parse::<f64>()?,
            });
        }
    } else {
        bail!("No data");
    }
    Ok(data)
}

async fn get_pairs() -> Result<HashMap<String, KrakenPairs>> {
    let full_url = format!("{}/0/public/AssetPairs", BASE_URL);
    let resp: KrakenPairsResponse = reqwest::get(full_url).await?.json().await?;
    match resp.result {
        Some(hm) => Ok(hm),
        None => {
            let err_msg = match resp.error.first().unwrap() {
                Some(s) => s.to_string(),
                None => "Unknown Error".to_string(),
            };
            Err(anyhow!("KrakenError [PAIRS]: {}", err_msg))
        }
    }
}

async fn get_tickers() -> Result<HashMap<String, KrakenTicker>> {
    let full_url = format!("{}/0/public/Ticker", BASE_URL);
    let resp: KrakenTickerResponse = reqwest::get(full_url).await?.json().await?;
    match resp.result {
        Some(hm) => Ok(hm),
        None => {
            let err_msg = match resp.error.first().unwrap() {
                Some(s) => s.to_string(),
                None => "Unknown Error".to_string(),
            };
            Err(anyhow!("KrakenError [TICKERS]: {}", err_msg))
        }
    }
}

fn get_kraken_sig(uri: &str, sk: String, payload: HashMap<String, String>) -> Result<String> {
    let postdata = format!("{}{}", payload.get("nonce").unwrap().clone(), get_postdata(payload));
    let encoded = postdata.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(encoded);
    let res = hasher.finalize();
    let sk_bytes = BASE64_STANDARD.decode(sk)?;
    let mut mac = HmacSha512::new_from_slice(&sk_bytes)?;
    mac.update(&uri.as_bytes());
    mac.update(&res);
    let sig_digest = mac.finalize();
    let sig_digest_bytes = sig_digest.into_bytes();
    Ok(BASE64_STANDARD.encode(sig_digest_bytes))
}

fn get_postdata(hm: HashMap<String, String>) -> String {
    let mut output = String::new();
    for (k, v) in hm.iter() {
        let segment = format!("{}={}&", k, v);
        output.push_str(&segment);
    }
    if !hm.is_empty() {
        output.pop();
    }
    output
}



// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// struct KrakenAssetsResponse {
//     error: Vec<Option<String>>,
//     result: Option<HashMap<String, KrakenAsset>>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct KrakenAsset {
//     pub aclass: String,
//     pub altname: String,
//     pub decimals: i64,
//     pub display_decimals: i64,
//     pub collateral_value: Option<f64>,
//     pub status: String,
// }

// pub async fn get_assets() -> Result<HashMap<String, KrakenAsset>> {
//     let full_url = format!("{}/0/public/Assets", BASE_URL);
//     let resp: KrakenAssetsResponse = reqwest::get(full_url).await?.json().await?;
//     if !resp.error.is_empty() {
//         let err_msg = match resp.error.first().unwrap() {
//             Some(s) => s.to_owned(),
//             None => "Assets request failed".to_string(),
//         };
//         bail!(err_msg);
//     }
//     match resp.result {
//         Some(hm) => Ok(hm),
//         None => {
//             bail!("Assets: No assets in response");
//         }
//     }
// }
