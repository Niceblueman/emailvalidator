use crate::api_key::{session, ApiKey};
use chrono::Utc;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header as JWTHeader,
    Validation,
};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use rusqlite::*;
use serde_rusqlite::*;

use super::{sender, ApiGen};
pub struct Users {
    db: Connection,
    public_key: DecodingKey,
    key_pair: EncodingKey,
}
#[derive(Default)]
pub struct RegisterResponce {
    pub status: bool,
    pub msg: String,
}
#[derive(Default)]
pub struct LoginResponce {
    pub status: bool,
    pub msg: String,
    pub user_id: Option<u32>,
}
#[derive(Default)]
pub struct RecoverResponce {
    pub status: bool,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}
#[derive(Default, Clone, Debug)]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub cus_id: Option<String>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    password: String,
    exp: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SessionClaims {
    exp: usize,
}

const SESSION_LIFE: usize = 30 * 24 * 60 * 60;
const SESSION_DAY: u32 = 24 * 60 * 60;

impl Users {
    pub fn default() -> Users {
        let key_pair = EncodingKey::from_rsa_pem(include_bytes!("../../private_key.pem")).unwrap();
        let public_key = DecodingKey::from_rsa_pem(include_bytes!("../../public_key.pem")).unwrap();
        let database_path = std::path::Path::new("./users.sqlite");
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
            CREATE TABLE IF NOT EXISTS users (
            id    INTEGER PRIMARY KEY,
            email  TEXT NOT NULL,
            username  TEXT NOT NULL,
            password TEXT NOT NULL,
            cus_id TEXT,
            createdAt TEXT NOT NULL
            )",
            (),
        )
        .unwrap();
        Users {
            db: conn,
            key_pair,
            public_key,
        }
    }
    pub fn generate_session(self, userid: u32) -> String {
        let _duration = Utc::now().timestamp() as usize + SESSION_LIFE;
        let my_claims = SessionClaims { exp: _duration };
        let mut header = JWTHeader::new(Algorithm::RS512);
        header.kid = Some(userid.to_string());
        encode(&header, &my_claims, &self.key_pair).expect("error encoding the password!")
    }
    pub fn validat_session(self, _session: String) -> bool {
        let _session_row = decode::<SessionClaims>(
            &_session,
            &self.public_key,
            &Validation::new(Algorithm::RS512),
        );
        // println!("_session_row: {:?}", _session_row);
        let is_token = match &_session_row {
            Ok(m) => true,
            Err(m) => false,
        };
        if !is_token {
            return false;
        }
        let now: usize = Utc::now().timestamp() as usize;
        if _session_row.unwrap().claims.exp - now > SESSION_LIFE {
            return false;
        }
        return true;
    }
    pub fn encode_password(password: String) -> String {
        let my_claims = Claims {
            password: password.to_owned(),
            exp: 18446744073709,
        };
        let mut header = JWTHeader::new(Algorithm::HS512);
        header.kid = Some("blabla".to_owned());
        encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret(password.as_str().as_ref()),
        )
        .expect("Error encoding the password!")
    }
    pub fn get_user(self, id: &str) -> Option<User> {
        let mut stmt = self
            .db
            .prepare("SELECT username, email, cus_id, id FROM users where id = ?")
            .unwrap();
        let mut rows = stmt.query(rusqlite::params![id]).unwrap();
        match rows.next() {
            Ok(Some(row)) => Some(User {
                username: row.get(0).unwrap(),
                email: row.get(1).unwrap(),
                cus_id: row.get(2).unwrap(),
                id: row.get(3).unwrap(),
                ..Default::default()
            }),
            Ok(None) => None,
            _ => None,
        }
    }
    pub fn validate_password(password: String, stored_password: &str) -> bool {
        let metadata = decode_header(stored_password).unwrap();
        let algorithm = metadata.alg;

        let pass_row = decode::<Claims>(
            stored_password,
            &DecodingKey::from_secret(password.as_str().as_ref()),
            &Validation::new(algorithm),
        );
        let is_token = match &pass_row {
            Ok(m) => true,
            Err(m) => false,
        };
        if !is_token {
            return false;
        }
        // print!("{:?}", pass_row.as_ref().unwrap());
        let pass = pass_row.unwrap();
        if pass.claims.password == password {
            return true;
        } else {
            return false;
        }
    }
    pub fn register(self, user: User) -> RegisterResponce {
        let mut stmt = self
            .db
            .prepare("SELECT email FROM users where email = ?")
            .unwrap();
        let mut stmt2 = self
            .db
            .prepare("SELECT username FROM users where username = ?")
            .unwrap();
        let mut rows = stmt.query(rusqlite::params![user.email]).unwrap();
        match rows.next() {
            Ok(Some(row)) => RegisterResponce {
                status: false,
                msg: "email_found".into(),
            },
            _ => {
                let mut rows = stmt2.query(rusqlite::params![user.email]).unwrap();
                match rows.next() {
                    Ok(Some(row)) => RegisterResponce {
                        status: false,
                        msg: "username_used".into(),
                    },
                    _ => {
                        let utc = chrono::Utc::now().to_rfc2822();
                        let cus_id = match user.cus_id {
                            Some(m) => m,
                            None => "".into(),
                        };
                        let _pass = Users::encode_password(user.password.clone());
                        let exec_res = self.db.execute("
                        INSERT INTO users (username, email, password, createdAt, cus_id) VALUES (?1, ?2, ?3, ?4, ?5)
                        ", (
                            &user.username.as_str(),
                            &user.email.as_str(),
                            &_pass.as_str(),
                            &utc.as_str(),
                            &cus_id.as_str(),
                        )).expect("sql requets error!");
                        RegisterResponce {
                            status: true,
                            msg: "created".into(),
                        }
                    }
                }
            }
        }
    }
    pub fn login(self, username: String, password: String) -> LoginResponce {
        // login code here
        let mut stmt = match username.find("@") {
            Some(m) => self
                .db
                .prepare("SELECT username, password, email, id FROM users where email = ?")
                .unwrap(),
            None => self
                .db
                .prepare("SELECT username, password, email, id FROM users where username = ?")
                .unwrap(),
        };
        let mut rows = stmt.query(rusqlite::params![username]).unwrap();
        match rows.next() {
            Ok(Some(row)) => {
                let _username: String = row.get(0).unwrap();
                let _password: String = row.get(1).unwrap();
                let _email: String = row.get(2).unwrap();
                let _id: u32 = row.get(3).unwrap();
                if Users::validate_password(password, _password.as_str()) {
                    LoginResponce {
                        status: true,
                        msg: "welcome_again".into(),
                        user_id: Some(_id),
                    }
                } else {
                    LoginResponce {
                        status: false,
                        msg: "wrong_password".into(),
                        user_id: None,
                    }
                }
            }
            _ => LoginResponce {
                status: false,
                msg: "not_found".into(),
                user_id: None,
            },
        }
    }
    pub fn recover(self, email: String) -> RecoverResponce {
        // login code here
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let _pass = Users::encode_password(rand_string.clone());
        let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
        match re.is_match(&email.as_str()) {
            true => {
                let mut stmnt = self
                    .db
                    .prepare("SELECT username, password, email FROM users where email = ?")
                    .unwrap();
                let mut rows = stmnt.query(rusqlite::params![email]).unwrap();
                let res_user = rows.next();
                match res_user {
                    Ok(Some(row)) => {
                        let _username: String = row.get(0).unwrap();
                        let _password: String = row.get(1).unwrap();
                        let _email: String = row.get(2).unwrap();
                        let exec_res = match self.db.execute(
                            "
                            UPDATE users SET password = ?1 WHERE email = ?2",
                            (&_pass, email.clone()),
                        ) {
                            Ok(s) => s,
                            Err(_) => 0,
                        };
                        let user = _email.split("@").next().unwrap();
                        let from = format!("kmoz000@yandex.com");
                        let to = format!("{} <{}>", user, _email);
                        let subject = format!("Pasword recovery: {}", _username);
                        let body = format!("<!doctype html>
                        <html lang=\"en-US\">
                        <head>
                            <meta content=\"text/html; charset=utf-8\" http-equiv=\"Content-Type\" />
                            <title>Reset Password Email Template</title>
                            <meta name=\"description\" content=\"Reset Password Email Template.\">
                        </head>
                        <body marginheight=\"0\" topmargin=\"0\" marginwidth=\"0\" style=\"margin: 0px; background-color: #f2f3f8;\" leftmargin=\"0\">
                            <!--100% body table-->
                            <table cellspacing=\"0\" border=\"0\" cellpadding=\"0\" width=\"100%\" bgcolor=\"#f2f3f8\"
                                style=\"@import url(https://fonts.googleapis.com/css?family=Rubik:300,400,500,700|Open+Sans:300,400,600,700); font-family: 'Open Sans', sans-serif;\">
                                <tr>
                                    <td>
                                        <table style=\"background-color: #f2f3f8; max-width:670px;  margin:0 auto;\" width=\"100%\" border=\"0\"
                                            align=\"center\" cellpadding=\"0\" cellspacing=\"0\">
                                            <tr>
                                                <td style=\"height:80px;\">&nbsp;</td>
                                            </tr>
                                            <tr>
                                                <td style=\"text-align:center;\">
                                                  <a href=\"https://mailvalidator.dup.company\" title=\"logo\" target=\"_blank\">
                                                    <img width=\"80\" src=\"https://res.cloudinary.com/dupagadir/image/upload/v1665420402/notification-icon_wxfgd2.jpg\" title=\"logo\" alt=\"logo\">
                                                  </a>
                                                </td>
                                            </tr>
                                            <tr>
                                                <td style=\"height:20px;\">&nbsp;</td>
                                            </tr>
                                            <tr>
                                                <td>
                                                    <table width=\"95%\" border=\"0\" align=\"center\" cellpadding=\"0\" cellspacing=\"0\"
                                                        style=\"max-width:670px;background:#fff; border-radius:3px; text-align:center;-webkit-box-shadow:0 6px 18px 0 rgba(0,0,0,.06);-moz-box-shadow:0 6px 18px 0 rgba(0,0,0,.06);box-shadow:0 6px 18px 0 rgba(0,0,0,.06);\">
                                                        <tr>
                                                            <td style=\"height:40px;\">&nbsp;</td>
                                                        </tr>
                                                        <tr>
                                                            <td style=\"padding:0 35px;\">
                                                                <h1 style=\"color:#1e1e2d; font-weight:500; margin:0;font-size:32px;font-family:'Rubik',sans-serif;\">You have
                                                                    requested  your password</h1>
                                                                <span
                                                                    style=\"display:inline-block; vertical-align:middle; margin:29px 0 26px; border-bottom:1px solid #cecece; width:100px;\"></span>
                                                                <p style=\"color:#455056; font-size:15px;line-height:24px; margin:0;\">
                                                                    if you didn't made this request just ignore it.
                                                                </p>
                                                                <p
                                                                    style=\"background:#20e277;text-decoration:none !important; font-weight:500; margin-top:35px; color:#fff;text-transform:uppercase; font-size:14px;padding:10px 24px;display:inline-block;border-radius:50px;\">{}</p>
                                                            </td>
                                                        </tr>
                                                        <tr>
                                                            <td style=\"height:40px;\">&nbsp;</td>
                                                        </tr>
                                                    </table>
                                                </td>
                                            <tr>
                                                <td style=\"height:20px;\">&nbsp;</td>
                                            </tr>
                                            <tr>
                                                <td style=\"text-align:center;\">
                                                    <p style=\"font-size:14px; color:rgba(69, 80, 86, 0.7411764705882353); line-height:18px; margin:0 0 0;\">&copy; <strong>mailvalidator.dup.company</strong></p>
                                                </td>
                                            </tr>
                                            <tr>
                                                <td style=\"height:80px;\">&nbsp;</td>
                                            </tr>
                                        </table>
                                    </td>
                                </tr>
                            </table>
                            <!--/100% body table-->
                        </body>
                        </html>",rand_string);
                        RecoverResponce {
                            status: true,
                            from,
                            to,
                            subject,
                            body,
                        }
                    }
                    _ => RecoverResponce {
                        status: false,
                        ..Default::default()
                    },
                }
            }
            false => RecoverResponce {
                status: false,
                ..Default::default()
            },
        }
    }
}

pub struct AnonymousUsers {
    db: Connection,
}
pub struct ManagedPlans {
    db: Connection,
}
#[derive(Default)]
pub struct AnonymousUser {
    pub uuid: String,
    pub last_usage: u32,
    pub lastcount: u32,
}
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MangedPlan {
    pub id: Option<u32>,
    pub plan: String,
    pub last_edit: u32,
    pub plan_count: Option<u32>,
    pub user_id: u32,
    pub cus_id: Option<String>,
    pub key: Option<String>,
    pub is_paid: Option<bool>,
    pub payment_id: Option<String>,
    pub last_count: Option<u32>,
}
#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewMangedPlan {
    pub plan: String,
    pub last_edit: u32,
    pub plan_count: u32,
    pub user_id: u32,
    pub is_paid: bool,
    pub key: String,
    pub last_count: u32,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AnonymousUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<AnonymousUser, ()> {
        let client = request.headers().get_one("cf-connecting-ip");
        // println!("client ip: {:?}", request.headers().get_one("cf-connecting-ip"));
        match client {
            Some(m) => {
                let db_anonymous = AnonymousUsers::default();
                let _user = db_anonymous.update_or_create(AnonymousUser {
                    uuid: m.to_string(),
                    last_usage: 0,
                    lastcount: 0,
                });
                Outcome::Success(_user)
            }
            None => Outcome::Failure((Status::Ok, ())),
        }
    }
}
impl ManagedPlans {
    pub fn default() -> ManagedPlans {
        let database_path = std::path::Path::new("./managed_accounts.sqlite");
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
            CREATE TABLE IF NOT EXISTS plans (
            id    INTEGER PRIMARY KEY,
            plan TEXT NOT NULL,
            last_edit INTEGER NOT NULL,
            plan_count INTEGER NOT NULL DEFAULT 1,
            user_id INTEGER NOT NULL,
            cus_id TEXT,
            key TEXT,
            is_paid BOOL NOT NULL DEFAULT 0,
            payment_id TEXT,
            last_count INTEGER NOT NULL DEFAULT 0
            )",
            (),
        )
        .unwrap();
        ManagedPlans { db: conn }
    }
    pub fn update_last_count(&self, id: u32, last_edit: u32, last_count: u32) -> bool {
        match self.db.execute(
            "
                            UPDATE plans SET last_count = ?1, last_edit = ?2  WHERE id = ?3",
            (&last_count, &last_edit, id),
        ) {
            Ok(s) => true,
            Err(_) => false,
        }
    }
    pub fn create_update(&self, plan: MangedPlan) -> Option<Vec<MangedPlan>> {
        // code for creating demo users and count possible usage mitrics
        let mut stmt = self
            .db
            .prepare("SELECT id, plan, last_edit, plan_count, user_id, key, is_paid, payment_id, cus_id FROM plans where id = ?")
            .unwrap();
        let mut demo_key = "".to_string();
        let mut demo_is_paid = false;
        let demo_last_count = 0;
        let keygen = ApiGen::new();
        if plan.plan == "Demo".to_string() {
            demo_key = keygen
                .generate_by_user(1, plan.user_id.clone(), plan.clone())
                .unwrap();
            demo_is_paid = true;
        }
        // println!("_count: {:?}",plan);
        match plan.id {
            Some(_id) => {
                let mut rows = stmt.query(rusqlite::params![_id]).unwrap();
                match rows.next() {
                    Ok(Some(row)) => {
                        let __now = chrono::Utc::now().timestamp() as u32;
                        let __id: u32 = row.get(0).unwrap();
                        let _count = match plan.plan_count {
                            Some(cnt) => cnt,
                            None => row.get(3).unwrap(),
                        };
                        let _key = match plan.key {
                            Some(cnt) => cnt,
                            None => match row.get(5) {
                                Ok(__col) => __col,
                                Err(_) => demo_key,
                            },
                        };
                        let _is_paid = match plan.is_paid {
                            Some(cnt) => cnt,
                            None => match row.get(6) {
                                Ok(__col) => __col,
                                Err(_) => demo_is_paid,
                            },
                        };
                        let _payment_id = match plan.payment_id {
                            Some(cnt) => cnt,
                            None => match row.get(7) {
                                Ok(__col) => __col,
                                Err(_) => "".into(),
                            },
                        };
                        let _cus_id = match plan.cus_id {
                            Some(cnt) => cnt,
                            None => match row.get(8) {
                                Ok(__col) => __col,
                                Err(_) => "".into(),
                            },
                        };
                        if !_is_paid {
                            let exec_res = match self
                            .db
                            .execute(
                                "
                            UPDATE plans SET last_edit = ?1, plan_count = ?2, key = ?3, is_paid = ?4, payment_id = ?5, cus_id = ?6  WHERE id = ?7",
                                (&__now, &_count, &_key, &_is_paid, &_payment_id, &_cus_id, &_id)
                            ) {
                                Ok(s)=>s,
                                Err(_)=>0
                            };
                        }
                        let res = from_rows::<MangedPlan>(rows)
                            .filter(|x| x.is_ok())
                            .map(|m| m.unwrap())
                            .collect::<Vec<MangedPlan>>();
                        Some(res)
                    }
                    _ => {
                        let utc = chrono::Utc::now().timestamp() as u32;
                        let mut statmnt = self
                            .db
                            .prepare("SELECT * FROM plans WHERE user_id = ?1")
                            .unwrap();
                        self.db.execute("INSERT INTO plans (plan, last_edit, plan_count, user_id, key, is_paid, payment_id, cus_id) VALUES (:plan, :last_edit, :plan_count, :user_id, :key, :is_paid, :payment_id, :cus_id)", 
                        to_params_named(&plan).unwrap().to_slice().as_slice())
                        .expect("sql INSERT error!");
                        let res = from_rows::<MangedPlan>(
                            statmnt.query(rusqlite::params![plan.user_id]).unwrap(),
                        )
                        .filter(|x| x.is_ok())
                        .map(|m| m.unwrap())
                        .collect::<Vec<MangedPlan>>();
                        Some(res)
                    }
                }
            }
            None => {
                let utc = chrono::Utc::now().timestamp() as u32;
                let new = NewMangedPlan {
                    plan: plan.plan.clone(),
                    last_edit: utc,
                    plan_count: 1,
                    user_id: plan.user_id,
                    is_paid: demo_is_paid,
                    key: demo_key,
                    last_count: 0,
                };
                let mut search_statmnt = self
                    .db
                    .prepare("SELECT id, plan, last_edit, plan_count, user_id, key, is_paid, payment_id, cus_id FROM plans WHERE user_id = ?1 AND plan = ?2")
                    .unwrap();
                let mut rows_list = search_statmnt
                    .query(rusqlite::params![plan.user_id, plan.plan])
                    .unwrap();
                match rows_list.next() {
                    Ok(Some(row)) => {
                        let utc = chrono::Utc::now().timestamp() as u32;
                        let _count: u32 = match plan.plan_count {
                            Some(cnt) => cnt,
                            None => row.get(3).unwrap(),
                        };
                        let exec_res = self
                            .db
                            .execute(
                                "
                            UPDATE plans SET last_edit = ?1, plan_count = ?2 WHERE user_id = ?3 AND plan = ?4",
                                (&utc, &_count, plan.user_id, plan.plan)
                            )
                        .expect("sql requets error!");
                        let mut statmnt = self
                            .db
                            .prepare("SELECT * FROM plans WHERE user_id = ?1")
                            .unwrap();
                        let res = from_rows::<MangedPlan>(
                            statmnt.query(rusqlite::params![plan.user_id]).unwrap(),
                        )
                        .filter(|x| x.is_ok())
                        .map(|m| m.unwrap())
                        .collect::<Vec<MangedPlan>>();
                        Some(res)
                    }
                    Ok(None) => {
                        let mut statmnt = self
                            .db
                            .prepare("SELECT * FROM plans WHERE user_id = ?1")
                            .unwrap();
                        self.db.execute("INSERT INTO plans (plan, last_edit, plan_count, user_id, key, is_paid, payment_id, cus_id, last_count) VALUES (:plan, :last_edit, :plan_count, :user_id, :key, :is_paid, :payment_id, :cus_id, :last_count)", 
                to_params_named(&new.clone()).unwrap().to_slice().as_slice())
                .expect("sql INSERT error!");
                        let res = from_rows::<MangedPlan>(
                            statmnt.query(rusqlite::params![plan.user_id]).unwrap(),
                        )
                        .filter(|x| x.is_ok())
                        .map(|m| m.unwrap())
                        .collect::<Vec<MangedPlan>>();
                        Some(res)
                    }
                    Err(_) => {
                        let mut statmnt = self
                            .db
                            .prepare("SELECT * FROM plans WHERE user_id = ?1")
                            .unwrap();
                        let res = from_rows::<MangedPlan>(
                            statmnt.query(rusqlite::params![plan.user_id]).unwrap(),
                        )
                        .filter(|x| x.is_ok())
                        .map(|m| m.unwrap())
                        .collect::<Vec<MangedPlan>>();
                        Some(res)
                    }
                }
            }
        }
    }
    pub fn delete_plan(&self, id: u32) -> bool {
        // let mut statement = self
        //     .db
        //     .prepare("SELECT * FROM plans WHERE id = ?1")
        //     .unwrap();
        let mut delete_statement = self.db.prepare("DELETE FROM plans WHERE id = ?1").unwrap();
        // let mut rows = statement
        //     .query_and_then(rusqlite::params![id], from_row::<MangedPlan>)
        //     .unwrap();
        let deleted = delete_statement.query([id]);
        match deleted {
            Ok(m) => true,
            Err(_) => false,
        }
    }
    pub fn get_lans(&self, id: u32) -> Vec<MangedPlan> {
        let mut statmnt = self
            .db
            .prepare("SELECT * FROM plans WHERE user_id = ?1")
            .unwrap();
        from_rows::<MangedPlan>(statmnt.query(rusqlite::params![id]).unwrap())
            .filter(|x| x.is_ok())
            .map(|m| m.unwrap())
            .collect::<Vec<MangedPlan>>()
    }
    pub fn getplan(&self, id: u32) -> Vec<MangedPlan> {
        let mut statmnt = self
            .db
            .prepare("SELECT id, plan, last_edit, plan_count, user_id, key, is_paid, payment_id, cus_id FROM plans WHERE id = ?1")
            .unwrap();
        from_rows::<MangedPlan>(statmnt.query(rusqlite::params![id]).unwrap())
            .filter(|x| x.is_ok())
            .map(|m| m.unwrap())
            .collect::<Vec<MangedPlan>>()
    }
    pub fn getplan_by_user(&self, id: u32, planname: String) -> Vec<MangedPlan> {
        let mut statmnt = self
            .db
            .prepare("SELECT id, plan, last_edit, plan_count, user_id, key, is_paid, payment_id, cus_id, last_count FROM plans WHERE user_id = ?1 AND plan = ?2")
            .unwrap();
        from_rows::<MangedPlan>(statmnt.query(rusqlite::params![id, planname]).unwrap())
            .filter(|x| x.is_ok())
            .map(|m| m.unwrap())
            .collect::<Vec<MangedPlan>>()
    }
    pub fn getplan_by_paymentid(&self, id: String) -> Vec<MangedPlan> {
        let mut statmnt = self
            .db
            .prepare("SELECT id, plan, last_edit, plan_count, user_id, key, is_paid, payment_id, cus_id FROM plans WHERE payment_id = ?1")
            .unwrap();
        from_rows::<MangedPlan>(statmnt.query(rusqlite::params![id]).unwrap())
            .filter(|x| x.is_ok())
            .map(|m| m.unwrap())
            .collect::<Vec<MangedPlan>>()
    }
    pub fn updated_plan_payment_id(&self, id: u32, new: String) -> bool {
        let mut statmnt = self
            .db
            .prepare("SELECT payment_id FROM plans WHERE id = ?1")
            .unwrap();
        let mut rows = statmnt.query(rusqlite::params![id]).unwrap();
        let fierst = rows.next();
        match fierst {
            Ok(m) => {
                let __now = chrono::Utc::now().timestamp() as u32;
                match self.db.execute(
                    "
                            UPDATE plans SET last_edit = ?1, payment_id = ?2  WHERE id = ?3",
                    (&__now, &new, &id),
                ) {
                    Ok(s) => true,
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }
    pub fn new_sub(
        &self,
        id: u32,
        is_paid: bool,
        cus_id: String,
        months: u32,
        plan: MangedPlan,
    ) -> bool {
        let mut statmnt = self
            .db
            .prepare("SELECT payment_id FROM plans WHERE id = ?1")
            .unwrap();
        let mut rows = statmnt.query(rusqlite::params![id]).unwrap();
        let fierst = rows.next();
        match fierst {
            Ok(m) => {
                let keygen = ApiGen::new();
                let generatedkey = keygen.generate_by_user(months, id, plan);
                match generatedkey {
                    Ok(key) => {
                        let __now = chrono::Utc::now().timestamp() as u32;
                        match self.db.execute(
                    "UPDATE plans SET last_edit = ?1, key = ?2, is_paid = ?3, cus_id = ?4  WHERE id = ?5",
                    (&__now, &key, &is_paid, &cus_id, &id),
                        ) {
                            Ok(s) => true,
                            Err(_) => false,
                        }
                    }
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }
}
impl AnonymousUsers {
    pub fn default() -> AnonymousUsers {
        let database_path = std::path::Path::new("./AnonymousUsers.sqlite");
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
            CREATE TABLE IF NOT EXISTS users (
            id    INTEGER PRIMARY KEY,
            uuid TEXT NOT NULL,
            lastUsage INTEGER NOT NULL,
            lastcount INTEGER NOT NULL DEFAULT 0
            )",
            (),
        )
        .unwrap();
        AnonymousUsers { db: conn }
    }
    pub fn update_or_create(&self, user: AnonymousUser) -> AnonymousUser {
        // code for creating demo users and count possible usage mitrics
        let mut stmt = self
            .db
            .prepare("SELECT uuid, lastUsage, lastcount FROM users where uuid = ?")
            .unwrap();
        let mut rows = stmt.query(rusqlite::params![user.uuid]).unwrap();
        match rows.next() {
            Ok(Some(row)) => {
                let utc = chrono::Utc::now().timestamp() as u32;
                let last_date: u32 = row.get(1).unwrap();
                let last_count: u32 = row.get(2).unwrap();
                let sub = last_date.abs_diff(utc);
                let _count = match sub > SESSION_DAY {
                    true => 0,
                    false => last_count + 1,
                };
                let _utc = match sub > SESSION_DAY {
                    true => utc,
                    false => last_date,
                };
                let exec_res = self
                    .db
                    .execute(
                        "
                UPDATE users SET lastUsage = ?1, lastcount = ?2  WHERE uuid = ?3
                ",
                        (&_utc, &_count, &user.uuid.as_str()),
                    )
                    .expect("sql requets error!");
                AnonymousUser {
                    uuid: user.uuid,
                    last_usage: _utc,
                    lastcount: _count,
                }
            }
            _ => {
                let utc = chrono::Utc::now().timestamp() as u32;
                let exec_res = self
                    .db
                    .execute(
                        "
                INSERT INTO users (uuid, lastUsage, lastcount) VALUES (?1, ?2, ?3)
                ",
                        (&user.uuid.as_str(), &utc, 0),
                    )
                    .expect("sql requets error!");
                AnonymousUser {
                    uuid: user.uuid,
                    last_usage: utc,
                    lastcount: 0,
                }
            }
        }
    }
}
