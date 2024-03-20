#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
use chrono::{prelude::*, Duration, TimeDelta};
use easylog::{
    log_file::{LogFile, LogLevel},
    log_file_config::LogFileConfig,
};
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use rand::Rng;
use rocket::{config, data::N, futures::future::ok};
use rusqlite::{Connection, OpenFlags, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::prelude::*;
use std::process::exit;
use std::{
    fmt::{self, format, Debug, Formatter},
    fs::File,
};
use std::{fs, path::Path, time::SystemTime};
pub mod responce;
pub mod sender;
pub mod stripe;
pub mod users;
pub mod mutithread_check;
use toml;

use crate::{Plans, tools::users::ManagedPlans};

use self::users::{MangedPlan, Users};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ApiTokenType {
    Starter,
    Premium,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct CustomHttpResponce {
    pub code: u32,
    pub success: bool,
    pub reason: String,
    pub plan_id: Option<u32>
}

impl CustomHttpResponce {
    pub fn invalid_token() -> CustomHttpResponce {
        CustomHttpResponce {
            code: 498,
            success: false,
            reason: "Invalid Token!".into(),
            ..Default::default()
        }
    }
    pub fn invalid_plan() -> CustomHttpResponce {
        CustomHttpResponce {
            code: 498,
            success: false,
            reason: "Invalid Plan!".into(),
            ..Default::default()
        }
    }
    pub fn invalid_plan1() -> CustomHttpResponce {
        CustomHttpResponce {
            code: 498,
            success: false,
            reason: "Invalid Plan (1)!".into(),
            ..Default::default()
        }
    }
    pub fn invalid_plan2() -> CustomHttpResponce {
        CustomHttpResponce {
            code: 498,
            success: false,
            reason: "Invalid Plan (2)!".into(),
            ..Default::default()
        }
    }
    pub fn day_limit() -> CustomHttpResponce {
        CustomHttpResponce {
            code: 498,
            success: false,
            reason: "Day limit exeeded!".into(),
            ..Default::default()
        }
    }
    pub fn email_limit() -> CustomHttpResponce {
        CustomHttpResponce {
            code: 498,
            success: false,
            reason: "emails limit per request exeeded!".into(),
            ..Default::default()
        }
    }
    pub fn success(id:u32) -> CustomHttpResponce {
        CustomHttpResponce {
            code: 200,
            success: true,
            reason: "Accepted".into(),
            plan_id: Some(id),
        }
    }
}
impl ApiTokenType {
    pub fn new(arg: &str) -> ApiTokenType {
        // todo!()
        match arg.to_ascii_lowercase().as_str() {
            "starter" => ApiTokenType::Starter,
            "premium" => ApiTokenType::Premium,
            _ => ApiTokenType::Starter,
        }
    }
    pub fn expect(&self, arg: &str) -> ApiTokenType {
        println!("ApiTokenType Error: ({})", arg);
        todo!()
    }
    pub fn string(&self) -> String {
        format!("{:?}", &self)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalData {
    plan: Config,
    user_id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdditionalDataUser {
    plan: MangedPlan,
    user_id: u32,
}
#[derive(Clone)]
pub struct ApiGen {
    public_key: DecodingKey,
    key_pair: EncodingKey,
}
// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct Data {
    plan0: Config,
    plan1: Config,
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    company: String,
    exp: usize,
    data: AdditionalData,
}
#[derive(Debug, Serialize, Deserialize)]
struct ClaimsUser {
    iss: String,
    company: String,
    exp: usize,
    data: AdditionalDataUser,
}
// Config struct holds to data from the `[config]` section.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub name: String, // plan name string
    pub requests_per_day: u32,
    pub items_per_req: u32,
    pub phone: String,
    pub email: String,
    pub cus_id: String,
    pub last_usage_day: String,
    pub last_count: u32,
}

// pub const UNIX_EPOCH: SystemTime = SystemTime(std::time::UNIX_EPOCH);

impl ApiGen {
    pub fn generate(
        &self,
        months: i32,
        id: String,
        plan: Config,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        // plan.expect("no plan setlected");
        let db = Costumers::new();
        let exp = (self.get_current_timestamp()
            + Duration::days(30 * months as i64).num_seconds() as u64) as usize;
        let data = AdditionalData {
            plan,
            user_id: id.clone(),
        };
        let this_clame = Claims {
            iss: "mailvalidator.dup.company".into(),
            company: "bulkus_LTD".into(),
            exp,
            data,
        };

        let mut header = Header::new(Algorithm::RS512);
        header.kid = Some(id.clone());
        let token = encode(&header, &this_clame, &self.key_pair);
        // db.save_customer(this_clame.jwt_id.unwrap(), id, selectedplan);
        token
    }
    pub fn generate_by_user(
        &self,
        months: u32,
        id: u32,
        plan: MangedPlan,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        // plan.expect("no plan setlected");
        let db = Users::default();
        let exp = (self.get_current_timestamp()
            + TimeDelta::try_days(30 * months as i64).unwrap().num_seconds() as u64) as usize;
        let data = AdditionalDataUser {
            plan,
            user_id: id.clone(),
        };
        let this_clame = ClaimsUser {
            iss: "mailvalidator.dup.company".into(),
            company: "dup_inc".into(),
            exp,
            data,
        };

        let mut header = Header::new(Algorithm::RS512);
        header.kid = Some(id.to_string());
        let token = encode(&header, &this_clame, &self.key_pair);
        // db.save_customer(this_clame.jwt_id.unwrap(), id, selectedplan);
        token
    }
    pub fn get_current_timestamp(&self) -> u64 {
        let start = SystemTime::now();
        start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
    pub fn validate(&self, token: &str) -> CustomHttpResponce {
        let db = Costumers::new();
        let default = LogFileConfig::new();
        let mut logfile = LogFile::new(default).unwrap();
        let metadata = decode_header(&token).unwrap();
        let key_id = metadata.kid.unwrap();
        let algorithm = metadata.alg;
        // let claims = self.public_key.verify_token::<AdditionalData>(&token, None).unwrap();
        let token_row =
            decode::<Claims>(&token, &self.public_key, &Validation::new(Algorithm::RS512));
        let is_token = match &token_row {
            Ok(m) => true,
            Err(m) => false,
        };

        if !is_token {
            return CustomHttpResponce::invalid_token();
        }

        let token = token_row.unwrap();
        logfile.write(
            LogLevel::INFO,
            format!(
                "key_id:{:?} | algorithm:{:?} | plan:{:?} | userId:{}",
                key_id, algorithm, token.claims.data.plan, token.claims.data.user_id
            )
            .as_str(),
        );
        let now = Utc::now().timestamp();
        let _customer_data = db.find_customer(key_id.clone());
        let is_customer = match &_customer_data {
            Ok(m) => true,
            Err(m) => false,
        };
        if !is_customer {
            return CustomHttpResponce::invalid_token();
        }

        let customer = _customer_data.unwrap();
        let diff_time = token.claims.exp as i64 - now;
        let lastusage =
            DateTime::parse_from_rfc2822(customer.config.last_usage_day.as_str()).unwrap();
        let diff_time_count = lastusage.timestamp() - now;
        if diff_time < 0 {
            let _ = db.remove_customer(key_id.clone());
            return CustomHttpResponce::invalid_token();
        }
        if customer.config.last_count < token.claims.data.plan.requests_per_day {
            return CustomHttpResponce::invalid_token();
        }
        CustomHttpResponce::success(key_id.parse::<u32>().unwrap())
    }
    pub fn validate_by_user(&self, token: &str, items_count: u32) -> CustomHttpResponce {
        let filename = "Plan.toml";
        let contents = fs::read_to_string(filename).unwrap();
        let all_plans =  toml::from_str::<Plans>(&contents).unwrap();
        let db = Users::default();
        let _plans = ManagedPlans::default();
        let default = LogFileConfig::new();
        let logfile = LogFile::new(default).unwrap();
        if token.is_empty() {
            return CustomHttpResponce::invalid_token();
        }
        if !decode_header(&token).is_ok() {
            return CustomHttpResponce::invalid_token();
        }
        let metadata = decode_header(&token).unwrap();
        let user_id = metadata.kid.unwrap();
        let algorithm = metadata.alg;
        let token_row =
            decode::<ClaimsUser>(&token, &self.public_key, &Validation::new(Algorithm::RS512));
        let is_token = match &token_row {
            Ok(m) => true,
            Err(m) => false,
        };

        if !is_token {
            return CustomHttpResponce::invalid_token();
        }

        let token = token_row.unwrap();
        let now = Utc::now().timestamp();
        let _customer_data = db.get_user(&user_id.clone().as_str());
        let is_customer = match &_customer_data {
            Some(m) => true,
            None => false,
        };
        if !is_customer {
            return CustomHttpResponce::invalid_token();
        }
        let one_day = chrono::Duration::days(1).num_seconds() as u32;
        let _user = _customer_data.unwrap();
        let diff_time = token.claims.exp as i64 - now;
        // println!("plan: {:?}", token.claims.data.plan);
        if diff_time < 0 {
            return CustomHttpResponce::invalid_token();
        }
        let _plan = _plans.getplan_by_user(token.claims.data.plan.user_id,token.claims.data.plan.plan);
        let his_plan_exp = _plan.first();
        if his_plan_exp.is_none() {
            return CustomHttpResponce::invalid_plan();
        }
        let his_plan =  his_plan_exp.unwrap();
        let selected_plan = match his_plan.plan.as_str() {
            "Demo" => all_plans.demo,
            "Starter" => all_plans.starter,
            "Premium" => all_plans.premium,
            _ => all_plans.demo,
        };
        if selected_plan.requests_per_day <  his_plan.last_count.unwrap() {
            return CustomHttpResponce::day_limit();
        }
        if selected_plan.items_per_req <  items_count {
            return CustomHttpResponce::email_limit();
        }
        let last_edit_dif = token.claims.data.plan.last_edit.abs_diff(now as u32);
        if last_edit_dif > one_day {
            if !_plans.update_last_count(his_plan.id.unwrap(),now as u32, 0 as u32) {
                return CustomHttpResponce::invalid_plan1();
            }
        } else {
            let _count =  his_plan.last_count.unwrap()+1;
            if !_plans.update_last_count(his_plan.id.unwrap(),now as u32, _count) {
                return CustomHttpResponce::invalid_plan2();
            }
        }
        CustomHttpResponce::success(his_plan.id.unwrap())
    }
    pub fn new() -> ApiGen {
        // let mut privatefile = File::open("_private.pem").unwrap();
        // let mut privatefiledata = String::new();
        // privatefile.read_to_string(&mut privatefiledata).expect("Error while reading file private.pem");
        // let mut publicfile = File::open("_public.pem").unwrap();
        // let  publicfiledata = String::new();
        // publicfile.read_to_string(&mut privatefiledata).expect("Error while reading file public.pem");
        let key_pair = EncodingKey::from_rsa_pem(include_bytes!("../../private_key.pem")).unwrap();
        let public_key = DecodingKey::from_rsa_pem(include_bytes!("../../public_key.pem")).unwrap();
        ApiGen {
            key_pair,
            public_key,
        }
    }
}

// #[derive(Debug)]
pub struct Costumers {
    _costumers: Connection,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Costumer {
    keyid: String,
    userid: String,
    config: Config,
}
impl Costumers {
    pub fn new() -> Costumers {
        let database_path = Path::new("./database.sqlite");
        let conn = Connection::open_with_flags(
            database_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
        )
        .expect("database not loaded!");
        println!("database autocommit: {}", conn.is_autocommit());
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS costumer (
            id    INTEGER PRIMARY KEY,
            keyid  TEXT NOT NULL,
            userid  TEXT NOT NULL,
            last_usage_day TEXT, 
            last_count INTEGER DEFAULT 0 NOT NULL,
            items_per_req INTEGER,
            phone TEXT NOT NULL,
            email TEXT NOT NULL,
            cus_id TEXT NOT NULL,
            requests_per_day INTEGER, 
            name TEXT NOT NULL
            )",
            (),
        )
        .unwrap();
        Costumers { _costumers: conn }
    }
    pub fn check_db_path(&self) -> &Path {
        return self._costumers.path().expect("no database path!");
    }
    pub fn save_customer(&self, keyid: String, userid: String, config: Config) -> usize {
        let last_usage_day = Utc::now().timestamp() as u32;
        let userid = userid.as_str();
        let keyid = keyid.as_str();
        let last_count = config.last_count;
        let items_per_req = config.items_per_req;
        let phone = config.phone;
        let email = config.email;
        let cus_id = config.cus_id;
        let requests_per_day = config.requests_per_day;
        let last_usage_day = config.last_usage_day;
        let name = config.name.as_str();
        self._costumers.execute("
        INSERT INTO costumer (keyid, userid, last_count, items_per_req, phone, email, cus_id, requests_per_day, last_usage_day, name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        ", (
        &keyid,
        &userid,
        &last_count,
        &items_per_req,
        &phone,
        &email,
        &cus_id,
        &requests_per_day,
        &last_usage_day,
        &name)).unwrap()
    }
    pub fn update_customer(&self, keyid: String, userid: &String, last_count: u32) -> usize {
        self._costumers
            .execute(
                "
            UPDATE costumer
            SET keyid = ?1,userid = ?2,last_count = ?3
            WHERE keyid = ?1;
            ",
                (&keyid, userid, &last_count),
            )
            .unwrap()
    }
    pub fn remove_customer(&self, userid: String) -> usize {
        self._costumers
            .execute(
                "
            DELETE FROM costumer
            WHERE keyid = ?1;
            ",
                (&userid, &userid),
            )
            .unwrap()
    }
    pub fn find_customer(&self, userid: String) -> Result<Costumer, String> {
        let mut stmt = self._costumers.prepare("SELECT keyid, userid, last_count, items_per_req, phone, email, cus_id, requests_per_day, last_usage_day, name FROM costumer where userid = ?").unwrap();
        let mut rows = stmt.query(rusqlite::params![userid]).unwrap();
        match rows.next() {
            Ok(Some(row)) => Ok(Costumer {
                keyid: row.get(0).unwrap(),
                userid: row.get(1).unwrap(),
                config: Config {
                    last_count: row.get(2).unwrap(),
                    items_per_req: row.get(3).unwrap(),
                    phone: row.get(4).unwrap(),
                    email: row.get(5).unwrap(),
                    cus_id: row.get(6).unwrap(),
                    requests_per_day: row.get(7).unwrap(),
                    last_usage_day: row.get(8).unwrap(),
                    name: row.get(9).unwrap(),
                },
            }),
            _ => Err("no Customer if found".into()),
        }
    }
}

pub fn get_id() -> String {
    const CHARSET: &[u8] = b"0123456789";
    const ID_LEN: usize = 6;
    let mut rng = rand::thread_rng().clone();
    (0..ID_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect::<String>()
}
