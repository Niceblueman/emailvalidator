//! ------ ApiKey (in http header, query or cookie) ------
#![allow(missing_docs)]
#![allow(unused_qualifications)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::tools::get_id;
use crate::tools::mutithread_check::bulkcheckemails;
use crate::tools::responce;
use crate::tools::sender;
use crate::tools::stripe;
use crate::tools::users::AnonymousUser;
use crate::tools::users::ManagedPlans;
use crate::tools::users::MangedPlan;
use crate::tools::users::User;
use crate::tools::users::Users;
use crate::tools::ApiGen;
use crate::MyError;
use crate::Plans;
use check_if_email_exists::{
    check_email, mx, smtp, syntax, CheckEmailInput, CheckEmailInputProxy, CheckEmailOutput,
};
use owning_ref::BoxRef;
use rand::*;
use regex::Regex;
use rocket::form::{Form, FromForm};
use rocket::futures::future::ok;
use rocket::http::hyper;
use rocket::http::private::cookie::Expiration;
use rocket::http::ContentType;
use rocket::http::Cookie;
use rocket::http::Header;
use rocket::http::SameSite;
use rocket::outcome::IntoOutcome;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use rocket::Request;
use rocket::{
    figment::{
        providers::{Env, Format, Serialized, Toml},
        Figment,
    },
    get,
    http::Status,
    post,
    request::{self, FromRequest, Outcome},
    serde::Deserialize,
    Config,
};
use rocket_cors::AllowedOrigins;
use rocket_dyn_templates::{context, Template};
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{
    Object, Responses, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    openapi,
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use schemars::{schema_for, schema_for_value, JsonSchema};
use serde::Serialize;
use serde_json::value::Index;
use serde_json::{json, Value};
use std::ptr::null;
use std::{fs, string};
use std::{str::FromStr, time::Duration};
use tinkoffpay::TaxNDK;
use tinkoffpay::Taxation;
pub struct ApiKey {
    key: String,
    id: u32,
    is_rapidapi: bool,
}
// Implement the actual checks for the authentication
#[rocket::async_trait]
impl<'a> FromRequest<'a> for ApiKey {
    type Error = String;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        // Get the key from the http header
        // println!("origin {:?}", request.headers());
        let generator = ApiGen::new();
        match request.headers().get_one("x-api-key") {
            Some(key) => match key == "" {
                true => match request.headers().get_one("x-rapidapi-proxy-secret") {
                    Some("6508f1f0-5197-11ed-9d87-cde669edcc78") => Outcome::Success(ApiKey {
                        key: "".into(),
                        id: 0 as u32,
                        is_rapidapi: true,
                    }),
                    Some(_) => Outcome::Failure((Status::Unauthorized, "invalid requets!".into())),
                    None => Outcome::Failure((Status::Unauthorized, "invalid requets!".into())),
                },
                false => {
                    let items = request.content_type();
                    let returned_data = generator.validate_by_user(key, 1);
                    let reason = returned_data.clone().reason;
                    if returned_data.success {
                        Outcome::Success(ApiKey {
                            key: key.to_owned(),
                            id: returned_data.clone().plan_id.unwrap(),
                            is_rapidapi: false,
                        })
                    } else {
                        Outcome::Failure((Status::Unauthorized, reason))
                    }
                }
            },
            None => Outcome::Failure((
                Status::BadRequest,
                "Missing `x-api-key` header.".to_string(),
            )),
        }
        // For more info see: https://rocket.rs/v0.5-rc/guide/state/#within-guards
    }
}

impl<'a> OpenApiFromRequest<'a> for ApiKey {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an API key to access, key is: `demoxxxxxxxxxxxxx`.".to_owned(),
            ),
            // Setup data requirements.
            // This can be part of the `header`, `query` or `cookie`.
            // In this case the header `x-api-key: mykey` needs to be set.
            data: SecuritySchemeData::ApiKey {
                name: "x-api-key".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("ApiKeyAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }

    // Optionally add responses
    // Also see `main.rs` part of this.
    fn get_responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        use rocket_okapi::okapi::openapi3::RefOr;
        // Can switch between to the but both are checked if they compile correctly
        let use_method = "recommended";
        // It can return a "400 BadRequest" and a "401 Unauthorized"
        // In both cases we just return a what we have set in the catches (if any).
        // In our cases this is: `crate::MyError`
        // This depends on you catcher return type.

        // Below are 3 examples, all are similar, first 2 are recommended.
        match use_method {
            "recommended" => Ok(Responses {
                // Recommended and most strait forward.
                // And easy to add or remove new responses.
                responses: okapi::map! {
                    "400".to_owned() => RefOr::Object(crate::bad_request_response(gen)),
                    "401".to_owned() => RefOr::Object(crate::unauthorized_response(gen)),
                },
                ..Default::default()
            }),
            "1st alternative" => {
                // This is same as macro above does, so just depends on what you like more.
                let mut responses = Responses::default();
                responses.responses.insert(
                    "400".to_owned(),
                    RefOr::Object(crate::bad_request_response(gen)),
                );
                responses.responses.insert(
                    "401".to_owned(),
                    RefOr::Object(crate::unauthorized_response(gen)),
                );
                Ok(responses)
            }
            "2nd alternative" => {
                // This not advised because of issue #57.
                // But this does work.
                // https://github.com/GREsau/okapi/issues/57
                // Note: this one does not add the `description` field to the responses.
                // So it is slightly different in output.
                let mut responses = Responses::default();
                let schema = gen.json_schema::<crate::MyError>();
                // Add "400 BadRequest"
                rocket_okapi::util::add_schema_response(
                    &mut responses,
                    400,
                    "application/json",
                    schema.clone(),
                )?;
                // Add "401 Unauthorized"
                rocket_okapi::util::add_schema_response(
                    &mut responses,
                    401,
                    "application/json",
                    schema,
                )?;
                Ok(responses)
            }
            _ => Ok(Responses::default()),
        }
    }
}

/// # ApiKey (in http header, query or cookie)
///
/// The key is: `mykey`
/// This is a common way of checking the authentication.
/// (make sure this is only sent over HTTPS, don't want secrets to leak)
///
/// Using `query` is not recommended for secrets!
/// For more info see:
/// <https://owasp.org/www-community/vulnerabilities/Information_exposure_through_query_strings_in_url>
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(JsonSchema)]
pub struct Email<'r> {
    from_email: &'r str,
    to_email: &'r str,
}
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(JsonSchema)]
pub struct EmailBulk {
    from_email: String,
    to_emails: Vec<String>,
}
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(JsonSchema)]
pub struct Subscription<'r> {
    plan: &'r str,
}
#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Misc {
    pub is_disposable: bool,
    /// Is this email a role-based account?
    pub is_role_account: bool,
}
#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Mx {
    pub accepts_mail: bool,
    /// Is this email a role-based account?
    pub records: Vec<String>,
}
#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Smtp {
    pub can_connect_smtp: bool,
    /// Is this email account's inbox full?
    pub has_full_inbox: bool,
    /// Does this domain have a catch-all email address?
    pub is_catch_all: bool,
    /// Can we send an email to this address?
    pub is_deliverable: bool,
    /// Is the email blocked or disabled by the provider?
    pub is_disabled: bool,
}
#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Syntax {
    pub address: String,
    /// The domain name, after "@". It will be the empty string if the email
    /// address if ill-formed.
    pub domain: String,
    /// Does the email have a valid syntax?
    pub is_valid_syntax: bool,
    /// The username, before "@". It will be the empty string if the email
    /// address if ill-formed.
    pub username: String,
}
#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Reachable {
    /// The email is safe to send.
    Safe,
    /// The email address appears to exist, but has quality issues that may
    /// result in low engagement or a bounce. Emails are classified as risky
    /// when one of the following happens:
    /// - catch-all email,
    /// - disposable email,
    /// - role-based address,
    /// - full inbox.
    Risky,
    /// Emails that don't exist or are syntactically incorrect. Do not send to
    /// these emails.
    Invalid,
    /// We're unable to get a valid response from the recipient's email server.
    Unknown,
}
impl Reachable {
    fn from_str(input: &str) -> Reachable {
        match input {
            "Safe" => Reachable::Safe,
            "Risky" => Reachable::Risky,
            "Invalid" => Reachable::Invalid,
            "Unknown" => Reachable::Unknown,
            _ => Reachable::Unknown,
        }
    }
}

#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
struct Record {
    host: String,    // * `exchange` - Name labels for the mail server
    preference: u16, // * `preference` - weight of this MX record as opposed to others, lower values have the higher preference
}

#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
struct MxRecords {
    records: Vec<Record>,
}
#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EmailOutput {
    input: String,
    misc: Misc,
    smtp_detail: Smtp,
    mx_records: Vec<Record>,
    syntax: Syntax,
    status: Reachable,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct StripeConfig {
    key: String,
    secret: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct StripeConfigs {
    dev: StripeConfig,
    prod: StripeConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]

pub struct ModifiedValue(Value);

impl ModifiedValue {
    /// Similar to `get` method but fast to get deep json Value
    /// ```
    /// # use serde_json::json;
    ///
    /// let object = json!({ "A": { "B": { "C": "foo" } } }); // true
    ///
    /// assert_eq!(*array.get_concurrent("A.B.C").unwrap(), json!("foo")); // true
    /// assert_eq!(*array.get_concurrent("A.B.C.D").unwrap(), json!("foo")); // true also
    /// ```
    pub fn get_concurrent(&self, list: &str) -> Option<&Value> {
        let mut rest: Option<&Value> = Some(&self.0);
        for item in list.split(".").into_iter() {
            rest = match rest.expect("end of pattern").get(item) {
                Some(m) => Some(m),
                None => Some(&self.0),
            };
        }
        rest
    }
}
// #[openapi]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserToCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UserToLogin {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct RecoverEmail {
    pub email: String,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct PaymentRequest {
    pub id: String,
    pub plan_id: u32,
    pub email: String,
    pub phone: String,
}
#[post("/register", format = "application/json", data = "<register>")]
pub async fn register(register: Json<UserToCreate>) -> RegisterResponce {
    // code here !
    let _users = Users::default();
    let _res = _users.register(User {
        username: register.username.clone(),
        password: register.password.clone(),
        email: register.email.clone(),
        ..Default::default()
    });
    match _res.status {
        true => {
            let res = json!({
                "success": true,
                "msg": _res.msg.as_str()
            });
            RegisterResponce {
                status: Status::Ok,
                text: res.to_string(),
            }
        }
        false => {
            let res = json!({
                "success": false,
                "msg": _res.msg.as_str()
            });
            RegisterResponce {
                status: Status::Ok,
                text: res.to_string(),
            }
        }
    }
}
pub struct Login {
    pub text: String,
    pub cookie: Cookie<'static>,
    pub status: Status,
}
pub struct Recover {
    pub text: String,
    pub status: Status,
}
pub struct ProfileResponce {
    pub text: String,
    pub status: Status,
}
pub struct RegisterResponce {
    pub text: String,
    pub status: Status,
}
pub struct CheckDemoResponce {
    pub text: String,
    pub status: Status,
}
impl<'r> Responder<'r, 'static> for Login {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        // println!("Login: {:?}: {:?}", req.headers(), req.cookies());
        rocket::Response::build()
            .header(self.cookie)
            .header(Header::new(
                "Access-Control-Allow-Origin",
                "http://mailvalidator.dup.company, https://mailvalidator.dup.company, http://web.mailvalidator.dup.company",
            ))
            // .header(Header::new("Access-Control-Allow-Origin", "https://mailvalidator.dup.company"))
            // .header(Header::new("Access-Control-Allow-Credentials", "true"))
            .header(ContentType::JSON)
            .status(self.status)
            .sized_body(self.text.len(), std::io::Cursor::new(self.text))
            .ok()
    }
}
impl<'r> Responder<'r, 'static> for Recover {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        // println!("Login: {:?}: {:?}", req.headers(), req.cookies());
        rocket::Response::build()
            .header(Header::new(
                "Access-Control-Allow-Origin",
                "http://mailvalidator.dup.company, https://mailvalidator.dup.company, http://web.mailvalidator.dup.company",
            ))
            // .header(Header::new("Access-Control-Allow-Origin", "https://mailvalidator.dup.company"))
            // .header(Header::new("Access-Control-Allow-Credentials", "true"))
            .header(ContentType::JSON)
            .status(self.status)
            .sized_body(self.text.len(), std::io::Cursor::new(self.text))
            .ok()
    }
}
impl<'r> Responder<'r, 'static> for RegisterResponce {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        // println!("Login: {:?}: {:?}", req.headers(), req.cookies());
        rocket::Response::build()
            .header(Header::new(
                "Access-Control-Allow-Origin",
                "http://mailvalidator.dup.company, https://mailvalidator.dup.company, http://web.mailvalidator.dup.company",
            ))
            // .header(Header::new("Access-Control-Allow-Origin", "https://mailvalidator.dup.company"))
            // .header(Header::new("Access-Control-Allow-Credentials", "true"))
            .header(ContentType::JSON)
            .status(self.status)
            .sized_body(self.text.len(), std::io::Cursor::new(self.text))
            .ok()
    }
}
impl<'r> Responder<'r, 'static> for CheckDemoResponce {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        // println!("Login: {:?}: {:?}", req.headers(), req.cookies());
        rocket::Response::build()
            .header(Header::new(
                "Access-Control-Allow-Origin",
                "http://mailvalidator.dup.company, https://mailvalidator.dup.company, http://web.mailvalidator.dup.company",
            ))
            .header(ContentType::JSON)
            .status(self.status)
            .sized_body(self.text.len(), std::io::Cursor::new(self.text))
            .ok()
    }
}
impl<'r> Responder<'r, 'static> for ProfileResponce {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        // println!("ProfileResponce: {:?}: {:?}", req.headers(), req.cookies());
        rocket::Response::build()
            .header(ContentType::JSON)
            .header(Header::new(
                "Access-Control-Allow-Origin",
                "http://mailvalidator.dup.company, https://mailvalidator.dup.company, http://web.mailvalidator.dup.company",
            ))
            .header(Header::new("Access-Control-Allow-Headers", "*"))
            .header(Header::new("Access-Control-Allow-Credentials", "true"))
            .status(self.status)
            .sized_body(self.text.len(), std::io::Cursor::new(self.text))
            .ok()
    }
}
#[derive(Debug)]
pub struct SiteCookie {
    pub ____ads: Option<String>,
    pub origin: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum PlansActions {
    AddEdit,
    Delete,
    GetPlans,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ManagePlansPost {
    action: PlansActions,
    name: String,
    plan_count: Option<u32>,
    id: Option<u32>,
}
#[derive(Debug, Clone, Default)]
pub struct ManagePlansRequest {
    pub user: Option<User>,
    pub origin: String,
}
#[derive(Debug, Clone, Default)]
pub struct ProtectedRequest {
    pub is_auth: bool,
    pub origin: String,
}
#[derive(Debug, Clone, Default)]
pub struct PaymentStatusRequest {
    pub order_id: Option<String>,
    pub origin: String,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for SiteCookie {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<SiteCookie, ()> {
        // println!("request: {:?}", request.headers());
        // println!("SiteCookie: {:?}", request.query_value::<String>("ads"));
        let host = request.host().unwrap();
        match request.query_value::<String>("ads") {
            Some(Ok(m)) => Outcome::Success(SiteCookie {
                ____ads: Some(m),
                origin: host.to_string(),
            }),
            Some(Err(e)) => Outcome::Success(SiteCookie {
                ____ads: None,
                origin: host.to_string(),
            }),
            None => Outcome::Success(SiteCookie {
                ____ads: None,
                origin: host.to_string(),
            }),
        }
    }
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ManagePlansRequest {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<ManagePlansRequest, ()> {
        let host = request.host().unwrap();
        match request.query_value::<String>("id") {
            Some(Ok(m)) => {
                let __users = Users::default();
                Outcome::Success(ManagePlansRequest {
                    user: __users.get_user(&m.as_str()),
                    origin: host.to_string(),
                })
            }
            Some(Err(e)) => Outcome::Success(ManagePlansRequest {
                user: None,
                origin: host.to_string(),
            }),
            None => Outcome::Success(ManagePlansRequest {
                user: None,
                origin: host.to_string(),
            }),
        }
    }
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for PaymentStatusRequest {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<PaymentStatusRequest, ()> {
        let host = request.host().unwrap();
        match request.query_value::<String>("order_id") {
            Some(Ok(m)) => Outcome::Success(PaymentStatusRequest {
                order_id: Some(m),
                origin: host.to_string(),
            }),
            Some(Err(e)) => Outcome::Success(PaymentStatusRequest {
                order_id: None,
                origin: host.to_string(),
            }),
            None => Outcome::Success(PaymentStatusRequest {
                order_id: None,
                origin: host.to_string(),
            }),
        }
    }
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ProtectedRequest {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<ProtectedRequest, ()> {
        let host = request.host().unwrap();
        match request.query_value::<String>("fm") {
            Some(Ok(m)) => {
                let __users = Users::default();
                Outcome::Success(ProtectedRequest {
                    is_auth: __users.validat_session(m.clone()),
                    origin: host.to_string(),
                })
            }
            Some(Err(e)) => Outcome::Success(ProtectedRequest {
                is_auth: false,
                origin: host.to_string(),
            }),
            None => Outcome::Success(ProtectedRequest {
                is_auth: false,
                origin: host.to_string(),
            }),
        }
    }
}
#[derive(Clone, Debug)]
pub struct PlansResponse {
    pub status: Status,
    pub text: String,
}
impl<'r> Responder<'r, 'static> for PlansResponse {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        // println!("ProfileResponce: {:?}: {:?}", req.headers(), req.cookies());
        rocket::Response::build()
            .header(ContentType::JSON)
            .header(Header::new(
                "Access-Control-Allow-Origin",
                "http://mailvalidator.dup.company, https://mailvalidator.dup.company, http://web.mailvalidator.dup.company",
            ))
            .header(Header::new("Access-Control-Allow-Headers", "*"))
            .header(Header::new("Access-Control-Allow-Credentials", "true"))
            .status(self.status)
            .sized_body(self.text.len(), std::io::Cursor::new(self.text))
            .ok()
    }
}
#[post("/myplans", format = "application/json", data = "<data>")]
pub fn manage_plans(req: ManagePlansRequest, data: Json<ManagePlansPost>) -> PlansResponse {
    let filename = "Plan.toml";
    let contents = fs::read_to_string(filename).unwrap();
    match toml::from_str::<Plans>(&contents) {
        Ok(_plans) => match req.user {
            Some(_user) => {
                // println!("data:{:?}", data);
                let _managed = ManagedPlans::default();
                match data.action {
                    PlansActions::AddEdit => {
                        let selected_plan = match data.name.as_str() {
                            "Demo" => _plans.demo,
                            "Starter" => _plans.starter,
                            "Premium" => _plans.premium,
                            _ => _plans.demo,
                        };
                        let __plans = _managed.create_update(MangedPlan {
                            id: data.id,
                            plan: selected_plan.name,
                            plan_count: data.plan_count,
                            user_id: _user.id.unwrap(),
                            ..Default::default()
                        });
                        match __plans {
                            Some(_m) => {
                                let res = json!({
                                    "success": true,
                                    "list": _m
                                });
                                PlansResponse {
                                    status: Status::Ok,
                                    text: res.to_string(),
                                }
                            }
                            None => {
                                let res = json!({
                                    "success": false,
                                    "code":123,
                                    "list": []
                                });
                                PlansResponse {
                                    status: Status::Ok,
                                    text: res.to_string(),
                                }
                            }
                        }
                    }
                    PlansActions::Delete => {
                        let status = _managed.delete_plan(data.id.unwrap());
                        let res = json!({
                            "success": status,
                        });
                        PlansResponse {
                            status: Status::Ok,
                            text: res.to_string(),
                        }
                    }
                    PlansActions::GetPlans => {
                        let plans = _managed.get_lans(_user.id.unwrap());
                        let res = json!({
                            "success": true,
                            "list": plans
                        });
                        PlansResponse {
                            status: Status::Ok,
                            text: res.to_string(),
                        }
                    }
                }
            }
            None => {
                let res = json!({
                    "success": false,
                    "code":125,
                    "list": []
                });
                PlansResponse {
                    status: Status::Ok,
                    text: res.to_string(),
                }
            }
        },
        Err(_) => {
            let res = json!({
                "success": false,
                "code":126,
                "list": []
            });
            PlansResponse {
                status: Status::Ok,
                text: res.to_string(),
            }
        }
    }
}
#[post("/plans")]
pub fn getplans() -> PlansResponse {
    let filename = "Plan.toml";
    let contents = fs::read_to_string(filename).unwrap();
    match toml::from_str::<Plans>(&contents) {
        Ok(m) => match serde_json::to_value(m) {
            Ok(_m) => PlansResponse {
                status: Status::Ok,
                text: _m.to_string(),
            },
            Err(_) => {
                let res = json!({});
                PlansResponse {
                    status: Status::Ok,
                    text: res.to_string(),
                }
            }
        },
        Err(_) => {
            let res = json!({});
            PlansResponse {
                status: Status::Ok,
                text: res.to_string(),
            }
        }
    }
}
#[get("/profile")]
pub fn profile(cookie: SiteCookie) -> ProfileResponce {
    let _users = Users::default();
    match cookie.____ads {
        Some(_cookie) => match _cookie.find(":") {
            Some(m) => {
                // println!("cookie found!:{}", _cookie);
                let __session = _cookie.split(":").collect::<Vec<&str>>()[0];
                let __id = _cookie.split(":").collect::<Vec<&str>>()[1];
                if __id.len() == 0 || __id == ")-".to_string() {
                    let res = json!({
                       "success": false,
                       "Object": {},
                       "code":103,
                       "reason":"no_session"
                    });
                    return ProfileResponce {
                        text: res.to_string(),
                        status: Status::Ok,
                    };
                }
                if __session.len() == 0 || __session == "-(".to_string() {
                    let res = json!({
                       "success": false,
                       "Object": {},
                       "code":102,
                       "reason":"no_session"
                    });
                    return ProfileResponce {
                        text: res.to_string(),
                        status: Status::Ok,
                    };
                }
                match _users.get_user(__id) {
                    Some(__user) => {
                        let _users = Users::default();
                        if _users.validat_session(__session.to_string()) {
                            let res = json!({
                               "success": true,
                               "Object": {
                                   "username": __user.username,
                                   "email": __user.email,
                                   "customer_id": __user.cus_id,
                               }
                            });
                            return ProfileResponce {
                                text: res.to_string(),
                                status: Status::Ok,
                            };
                        } else {
                            let res = json!({
                               "success": false,
                               "Object": {},
                               "code":100,
                               "reason":"no_valid_session"
                            });
                            return ProfileResponce {
                                text: res.to_string(),
                                status: Status::Ok,
                            };
                        }
                    }
                    None => {
                        let res = json!({
                           "success": false,
                           "Object": {},
                           "code":101,
                           "reason":"no_valid_session"
                        });
                        return ProfileResponce {
                            text: res.to_string(),
                            status: Status::Ok,
                        };
                    }
                }
            }
            None => {
                let res = json!({
                   "success": false,
                   "Object": {},
                   "code":105,
                   "reason":"no_session"
                });
                return ProfileResponce {
                    text: res.to_string(),
                    status: Status::Ok,
                };
            }
        },
        None => {
            let res = json!({
               "success": false,
               "Object": {},
               "code":106,
               "reason":"no_valid_session"
            });
            return ProfileResponce {
                text: res.to_string(),
                status: Status::Ok,
            };
        }
    }
}

// #[options("/login", format = "application/json", data = "<login>")]
#[post("/login", format = "application/json", data = "<login>")]
pub fn login(login: Json<UserToLogin>) -> Login {
    // code here !
    let _users = Users::default();
    let _res = _users.login(login.username.clone(), login.password.clone());
    match _res.status {
        true => {
            let __id = _res.user_id.unwrap();
            let _session = Users::default().generate_session(__id.clone());
            let __id = format!("{}:{}", _session, __id);
            let res = json!({
               "success": true,
               "msg": _res.msg.as_str(),
               "id":__id,
            });
            let __cookie = Cookie::build("____ads", __id)
                .domain(".mailvalidator.dup.company")
                .path("/")
                .max_age(rocket::time::Duration::days(30))
                .same_site(SameSite::Strict)
                .secure(true)
                .expires(Expiration::Session)
                .http_only(true)
                .finish();
            Login {
                text: res.to_string(),
                cookie: __cookie,
                status: Status::Ok,
            }
        }
        false => {
            let res = json!({
                "success": false,
                "msg": _res.msg.as_str()
            });
            let __cookie = Cookie::build("____ads", "-(:)-")
                .domain("*.mailvalidator.dup.company")
                .path("/")
                .secure(true)
                .same_site(SameSite::Strict)
                .expires(Expiration::Session)
                .http_only(true)
                .finish();
            Login {
                text: res.to_string(),
                cookie: __cookie,
                status: Status::Ok,
            }
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Payback {
    Amount: u32,         //Number(10000),
    CardId: u32,         //Number(130550983),
    ErrorCode: String,   //String("0"),
    ExpDate: String,     //String("1122"),
    OrderId: String,     //String("278600"),
    Pan: String,         //String("430000******0777"),
    PaymentId: u32,      //Number(1866123460),
    Status: String,      //String("CONFIRMED"),
    Success: bool,       //Bool(true),
    TerminalKey: String, //String("1639044907391DEMO"),
    Token: String, //String("711cd4fa0df2afa8a69a56884e9d902eb882c9a01af56498fc6bfdefaf9eef8d")
}

#[post("/pay_back", format = "application/json", data = "<pay>")]
pub async fn pay_back(pay: Json<Payback>) -> PlansResponse {
    let _mplans = ManagedPlans::default();
    let plans = _mplans.getplan_by_paymentid(pay.OrderId.clone());
    let CONFIRMED = "CONFIRMED".to_string();
    match plans.first() {
        Some(m) => {
            let is_paid = if pay.Status == CONFIRMED { true } else { false };
            let key = _mplans.new_sub(
                m.id.unwrap(),
                is_paid,
                pay.PaymentId.clone().to_string(),
                m.plan_count.unwrap(),
                m.clone(),
            );
            if key {
                let res = json!({
                   "success": true,
                });
                PlansResponse {
                    text: res.to_string(),
                    status: Status::Ok,
                }
            } else {
                let res = json!({
                   "success": false,
                });
                PlansResponse {
                    text: res.to_string(),
                    status: Status::Ok,
                }
            }
        }
        None => {
            let res = json!({
               "success": false,
            });
            PlansResponse {
                text: res.to_string(),
                status: Status::Ok,
            }
        }
    }
}
#[get("/pay_back")]
pub async fn pay_back_get(req: PaymentStatusRequest) -> Template {
    let filename = "Plan.toml";
    let contents = fs::read_to_string(filename).unwrap();
    match req.order_id {
        Some(m) => {
            let _mplans = ManagedPlans::default();
            let plans = _mplans.getplan_by_paymentid(m.clone());
            let _plans = toml::from_str::<Plans>(&contents).unwrap();
            match plans.first() {
                Some(plan) => match plan.clone().is_paid {
                    Some(true) => {
                        let selected_plan = match plan.plan.as_str() {
                            "Starter" => _plans.starter,
                            "Premium" => _plans.premium,
                            _ => _plans.starter,
                        };
                        let total = plan.plan_count.unwrap() * selected_plan.price;
                        let _date = chrono::Local::now().to_rfc2822();
                        Template::render(
                            "payment_success",
                            context! { PaymentId:plan.clone().payment_id.unwrap(), Status:"CONFIRMED", Amount: total, OrderId:m.clone(), date:_date.clone() },
                        )
                    }
                    Some(false) => {
                        let apikey = "PJkAekkIqocvtj5jMsc2CnGIs7ogOWcd";
                        let error_str = "The page you requested cannot be found right now";
                        Template::render("error", context! { apikey, error_str })
                    }
                    None => {
                        let apikey = "PJkAekkIqocvtj5jMsc2CnGIs7ogOWcd";
                        let error_str = "The page you requested cannot be found right now";
                        Template::render("error", context! { apikey, error_str })
                    }
                },
                None => {
                    let apikey = "PJkAekkIqocvtj5jMsc2CnGIs7ogOWcd";
                    let error_str = "The page you requested cannot be found right now";
                    Template::render("error", context! { apikey, error_str })
                }
            }
        }
        None => {
            let apikey = "PJkAekkIqocvtj5jMsc2CnGIs7ogOWcd";
            let error_str = "The page you requested cannot be found right now";
            Template::render("error", context! { apikey, error_str })
        }
    }
}
#[post("/payment", format = "application/json", data = "<plan>")]
pub async fn payment_link(req: ProtectedRequest, plan: Json<PaymentRequest>) -> PlansResponse {
    //id, plan, last_edit, plan_count, user_id, key, is_paid, payment_id, cus_id
    match req.is_auth {
        true => {
            let _mplans = ManagedPlans::default();
            let found = _mplans.getplan(plan.plan_id);
            let _if_plan = found.first();
            match _if_plan {
                Some(m) => {
                    let filename = "Plan.toml";
                    let contents = fs::read_to_string(filename).unwrap();
                    let _plans = toml::from_str::<Plans>(&contents).unwrap();
                    let selected_plan = match m.plan.as_str() {
                        "Starter" => _plans.starter,
                        "Premium" => _plans.premium,
                        _ => _plans.starter,
                    };
                    let __id = get_id();
                    if !__id.is_empty() {
                        _mplans.updated_plan_payment_id(m.id.unwrap().clone(), __id.clone());
                    }
                    let _count = match m.plan_count {
                        Some(m) => m,
                        None => 1,
                    };
                    let payment_url = tinkoffpay::Payments::default()
                        .build(
                            "1639044907391",
                            "https://mailvalidator.dup.company/pay_back",
                            __id.as_str(),
                            &format!(
                                "Подписка {} на {} месяца(ы) (mailvalidator.dup.company API) за {}.00 руб",
                                m.plan,
                                _count,
                                selected_plan.price * _count
                            )
                            .as_str(),
                        )
                        .set_amount(selected_plan.price * _count) // in ruble
                        .set_customer(&plan.phone.as_str(), &plan.email.as_str())
                        .set_receipt(
                            tinkoffpay::Receipt::default()
                                .build(
                                    &plan.email.as_str(),
                                    &plan.phone.as_str(),
                                    "contact@oldi.dev",
                                    Taxation::OSN,
                                )
                                .add_item(
                                    &format!("Ежемесячная подписка {} (mailvalidator.dup.company API)", m.plan)
                                        .as_str(),
                                    _count,
                                    selected_plan.price, // in ruble
                                    TaxNDK::None,
                                ),
                        )
                        .to_url()
                        .await;
                    match payment_url.clone() {
                        Ok(m) => {
                            if m.success {
                                let res = json!({
                                   "success": true,
                                   "msg": "payment_url_ready",
                                   "url": m.payment_url
                                });
                                PlansResponse {
                                    text: res.to_string(),
                                    status: Status::Ok,
                                }
                            } else {
                                let res = json!({
                                   "success": false,
                                   "msg": "con_not_complete",
                                });
                                PlansResponse {
                                    text: res.to_string(),
                                    status: Status::Ok,
                                }
                            }
                        }
                        Err(err) => {
                            let res = json!({
                               "success": false,
                               "msg": "con_not_complete",
                            });
                            PlansResponse {
                                text: res.to_string(),
                                status: Status::Ok,
                            }
                        }
                    }
                }
                None => {
                    let res = json!({
                       "success": false,
                       "msg": "plan_not_found",
                    });
                    PlansResponse {
                        text: res.to_string(),
                        status: Status::Ok,
                    }
                }
            }
        }
        false => {
            let res = json!({
               "success": false,
               "msg": "not_authorised",
            });
            PlansResponse {
                text: res.to_string(),
                status: Status::Ok,
            }
        }
    }
}
// #[options("/login", format = "application/json", data = "<login>")]
#[post("/recover", format = "application/json", data = "<email>")]
pub async fn recover(email: Json<RecoverEmail>) -> Recover {
    let _users = Users::default();
    let _res = _users.recover(email.email.clone());
    match _res.status {
        true => {
            match sender::send_email_smtp(
                _res.from.as_str(),
                _res.to.as_str(),
                _res.subject.as_str(),
                _res.body,
            ) {
                Ok(m) => {
                    if m {
                        let res = json!({
                           "success": true,
                           "msg": "check_your_inbox_if_email_exit",
                        });
                        Recover {
                            text: res.to_string(),
                            status: Status::Ok,
                        }
                    } else {
                        let res = json!({
                           "success": false,
                           "msg": "server_error_email",
                        });
                        Recover {
                            text: res.to_string(),
                            status: Status::Ok,
                        }
                    }
                }
                Err(_) => {
                    let res = json!({
                       "success": false,
                       "msg": "server_error_email",
                    });
                    Recover {
                        text: res.to_string(),
                        status: Status::Ok,
                    }
                }
            }
        }
        false => {
            let res = json!({
                "success": true,
                "msg": "check_your_inbox_if_email_exit"
            });
            Recover {
                text: res.to_string(),
                status: Status::Ok,
            }
        }
    }
}

#[post("/session", format = "application/json", data = "<session>")]
pub async fn session(session: Json<ModifiedValue>) -> Result<Json<ModifiedValue>, crate::MyError> {
    // Use api key
    let filename = "Stripe.toml";
    let contents = fs::read_to_string(filename).unwrap();
    let data: StripeConfigs = toml::from_str(&contents).unwrap();
    // .extract::<StripeConfig>().unwrap();
    println!("session {:?}", session);
    let cus_id = session
        .get_concurrent("data.object.customer")
        .unwrap()
        .clone()
        .to_string()
        .replace("\\", "")
        .replace("\"", "");
    let _stripe = stripe::Stripe::new(data.dev.key.clone(), data.dev.secret.clone());
    let _type = session
        .get_concurrent("type")
        .unwrap()
        .to_string()
        .replace("\\", "")
        .replace("\"", "");
    if !cus_id.is_empty() && _type == "payment_intent.succeeded" {
        let get_cust = _stripe.get_customer(cus_id).await;
        match get_cust {
            Ok(sub) => {
                print!("CUST_GET: {:?}", sub.clone());
                // Object {
                //     "address": Object {
                //            "city": Null,
                //            "country": String("RU"),
                //            "line1": Null,
                //            "line2": Null,
                //            "postal_code": Null,
                //            "state": Null},
                //            "balance": Number(0),
                //            "created": Number(1662969800),
                //            "currency": Null,
                //            "default_currency": Null,
                //            "default_source": Null,
                //            "delinquent": Bool(false),
                //            "description": Null,
                //            "discount": Null,
                //            "email": String("kimo@oldi.dev"),
                //            "id": String("cus_MPxXDKhYsF8itW"),
                //            "invoice_prefix": String("6056FEDE"),
                //            "invoice_settings": Object {
                //              "custom_fields": Null,
                //              "default_payment_method": Null,
                //              "footer": Null, "rendering_options": Null
                //              },
                //            "livemode": Bool(false),
                //            "metadata": Object {},
                //            "name": String("abdelkarim ouazmir"),
                //            "next_invoice_sequence": Number(1),
                //            "object": String("customer"),
                //            "phone": String("+79965043656"),
                //            "preferred_locales": Array [String("en-US"), String("en")],
                //            "shipping": Null,
                //            "tax_exempt": String("none"),
                //            "test_clock": Null
                //            }

                Ok(Json(ModifiedValue(json!({
                    "code": 200,
                    "success": true,
                    "result": "thanks you for ordering!"
                }))))
            }
            Err(err) => Ok(Json(ModifiedValue(json!({
            "code": 500,
            "success": false,
            "reason": "no customer id"
            })))),
        }
    } else {
        Ok(Json(ModifiedValue(json!({
            "code": 500,
            "success": false,
            "reason": "no customer id"
        }))))
    }
}
#[openapi]
#[post("/check", format = "application/json", data = "<email>")]
pub async fn check_handler(
    key: ApiKey,
    email: Json<Email<'_>>,
) -> Result<Json<EmailOutput>, crate::MyError> {
    // Use api key
    let emailinput = email.to_email;
    let mut input = CheckEmailInput::new(emailinput.into());
    input
        .set_from_email(email.from_email.into()) // Used in the `MAIL FROM:` command
        .set_hello_name(emailinput.split("@").nth(1).unwrap().into()); // Used in the `EHLO` command
    let myresult = check_email(&input).await;
    let can_connect = match myresult.smtp.as_ref() {
        Ok(m) => m.can_connect_smtp,
        Err(_) => false,
    };
    let has_full_inbox = match myresult.smtp.as_ref() {
        Ok(m) => m.has_full_inbox,
        Err(_) => false,
    };
    let is_catch_all = match myresult.smtp.as_ref() {
        Ok(m) => m.is_catch_all,
        Err(_) => false,
    };
    let is_deliverable = match myresult.smtp.as_ref() {
        Ok(m) => m.is_deliverable,
        Err(_) => true,
    };
    let is_disabled = match myresult.smtp.as_ref() {
        Ok(m) => m.is_disabled,
        Err(_) => false,
    };
    let responce = EmailOutput {
        input: myresult.input,
        status: Reachable::from_str(format!("{:?}", myresult.is_reachable).as_str()),
        misc: Misc {
            is_disposable: myresult.misc.as_ref().unwrap().is_disposable,
            is_role_account: myresult.misc.as_ref().unwrap().is_role_account,
        },
        mx_records: MxRecords {
            records: myresult
                .mx
                .unwrap()
                .lookup
                .unwrap()
                .iter()
                .map(|record| Record {
                    host: record.exchange().clone().to_string(),
                    preference: record.preference().clone(),
                })
                .collect::<Vec<Record>>(),
        }
        .records,
        smtp_detail: Smtp {
            can_connect_smtp: can_connect,
            has_full_inbox,
            is_catch_all,
            is_deliverable,
            is_disabled,
        },
        syntax: Syntax {
            address: myresult.syntax.address.as_ref().unwrap().to_string(),
            domain: myresult.syntax.domain,
            is_valid_syntax: myresult.syntax.is_valid_syntax,
            username: myresult.syntax.username,
        },
    };
    Ok(Json(responce))
}

// #[openapi]
// #[post("/check_bulk", format = "application/json", data = "<emails>")]
// pub async fn check_bulk(
//     key: ApiKey,
//     emails: Json<EmailBulk>,
// ) -> Result<Json<Vec<EmailOutput>>, crate::MyError> {
//     // Use api key
//     let filename = "Plan.toml";
//     let contents = fs::read_to_string(filename).unwrap();
//     let all_plans = toml::from_str::<Plans>(&contents).unwrap();
//     let _plans = ManagedPlans::default();
//     let _plan = _plans.getplan(key.id);
//     let his_plan_exp = _plan.first();
//     if his_plan_exp.is_none() {
//         return Err(MyError {
//             err: "Rejected Request".to_owned(),
//             msg: Some("Whoops! Looks you have no correct Api Plan".to_owned()),
//             //exeeded emails limit by request.
//             http_status_code: 200,
//         });
//     }
//     let selected_plan = match his_plan_exp.unwrap().plan.as_str() {
//         "Demo" => all_plans.demo,
//         "Starter" => all_plans.starter,
//         "Premium" => all_plans.premium,
//         _ => all_plans.demo,
//     };
//     let allowed_max = selected_plan.items_per_req as usize;
//     if emails.to_emails.len() > allowed_max {
//         return Err(MyError {
//             err: "Rejected Request".to_owned(),
//             msg: Some("Whoops! Looks you have exeeded emails limit by request.".to_owned()),
//             http_status_code: 200,
//         });
//     }
//     let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
//     let cleaned = emails
//         .to_emails
//         .iter()
//         .filter(|s| return re.is_match(s.to_owned().as_str()))
//         .map(|f| format!("{}", f).to_string())
//         .collect::<Vec<String>>();
//     let responce = bulkcheckemails(emails.from_email.clone(), cleaned, Some(900 as u64)).await;
//     let res = responce
//         .iter()
//         .map(|myresult| {
//             let can_connect = match myresult.smtp.as_ref() {
//                 Ok(m) => m.can_connect_smtp,
//                 Err(_) => false,
//             };
//             let has_full_inbox = match myresult.smtp.as_ref() {
//                 Ok(m) => m.has_full_inbox,
//                 Err(_) => false,
//             };
//             let is_catch_all = match myresult.smtp.as_ref() {
//                 Ok(m) => m.is_catch_all,
//                 Err(_) => false,
//             };
//             let is_deliverable = match myresult.smtp.as_ref() {
//                 Ok(m) => m.is_deliverable,
//                 Err(_) => true,
//             };
//             let is_disabled = match myresult.smtp.as_ref() {
//                 Ok(m) => m.is_disabled,
//                 Err(_) => false,
//             };
//             EmailOutput {
//                 input: myresult.clone().input.clone(),
//                 status: Reachable::from_str(
//                     format!("{:?}", myresult.clone().is_reachable).as_str(),
//                 ),
//                 misc: Misc {
//                     is_disposable: myresult.clone().misc.as_ref().unwrap().is_disposable,
//                     is_role_account: myresult.clone().misc.as_ref().unwrap().is_role_account,
//                 },
//                 mx_records: MxRecords {
//                     records: myresult
//                         .mx
//                         .as_ref()
//                         .unwrap()
//                         .lookup
//                         .as_ref()
//                         .unwrap()
//                         .iter()
//                         .map(|record| Record {
//                             host: record.exchange().clone().to_string(),
//                             preference: record.preference().clone(),
//                         })
//                         .collect::<Vec<Record>>(),
//                 }.records,
//                 smtp_detail: Smtp {
//                     can_connect_smtp: can_connect,
//                     has_full_inbox,
//                     is_catch_all,
//                     is_deliverable,
//                     is_disabled,
//                 },
//                 syntax: Syntax {
//                     address: myresult.syntax.address.as_ref().unwrap().to_string(),
//                     domain: myresult.syntax.domain.clone(),
//                     is_valid_syntax: myresult.syntax.is_valid_syntax,
//                     username: myresult.syntax.username.clone(),
//                 },
//             }
//         })
//         .collect::<Vec<EmailOutput>>();
//     Ok(Json(res))
// }

#[post("/demoCheck", format = "application/json", data = "<email>")]
pub async fn demo_check(user: AnonymousUser, email: Json<Email<'_>>) -> CheckDemoResponce {
    // Use api key
    // let proxies =  vec![
    //     // ("205.251.66.56",7497),
    //     ("37.99.224.225", 7497), // fast and working
    //     ("192.99.101.142", 7497), //allow
    //     ("socks5.kmoz.dev", 9150), //allow
    //     // ("103.53.228.217",7497),
    //     // ("205.251.66.56",7497),
    //     // ("170.238.79.2",7497),
    //     ];
    match user.lastcount > 50 {
        false => {
            let emailinput = email.to_email;
            // let index = (rand::random::<f32>() * proxies.len() as f32).floor() as usize;
            // let myproxy = proxies[index];
            let mut input = CheckEmailInput::new(emailinput.into());
            // Optionally, we can also tweak the configuration parameters used in the
            // verification.
            // println!("{}",email);
            input
                .set_from_email(email.from_email.into()) // Used in the `MAIL FROM:` command
                .set_hello_name(emailinput.split("@").nth(1).unwrap().into()); // Used in the `EHLO` command
                                                                               // .set_proxy(CheckEmailInputProxy {         // Use a SOCKS5 proxy to verify the email
                                                                               //     host: myproxy.0.into(),
                                                                               //     port: myproxy.1,
                                                                               //     password: Some("".into()),
                                                                               //     username: Some("".into())
                                                                               // })

            // Verify this email, using async/await syntax.
            let myresult = check_email(&input).await;
            // let responce = EmailOutput {
            //     input: myresult.input,
            //     is_reachable: Reachable::from_str(format!("{:?}", myresult.is_reachable).as_str()),
            //     misc: Misc {
            //         is_disposable: myresult.misc.as_ref().unwrap().is_disposable,
            //         is_role_account: myresult.misc.as_ref().unwrap().is_role_account,
            //     },
            //     mx: MxRecords {
            //         records: myresult
            //             .mx
            //             .unwrap()
            //             .lookup
            //             .unwrap()
            //             .iter()
            //             .map(|record| Record {
            //                 host: record.exchange().clone().to_string(),
            //                 preference: record.preference().clone(),
            //             })
            //             .collect::<Vec<Record>>(),
            //     },
            //     // .clone().unwrap().lookup  .as_ref()
            //     //     .expect("If lookup is error, we already returned. qed.")
            //     //     .iter().map(|record| {
            //     //         // println!("{} {}", record.preference(), record.exchange());
            //     //         Record {
            //     //         host: record.exchange().to_string(),
            //     //         preference: record.preference(),
            //     //     }
            //     //.collect::<Vec<Record>>()
            //     smtp: Smtp {
            //         can_connect_smtp: myresult.smtp.as_ref().unwrap().can_connect_smtp,
            //         has_full_inbox: myresult.smtp.as_ref().unwrap().has_full_inbox,
            //         is_catch_all: myresult.smtp.as_ref().unwrap().is_catch_all,
            //         is_deliverable: myresult.smtp.as_ref().unwrap().is_deliverable,
            //         is_disabled: myresult.smtp.as_ref().unwrap().is_disabled,
            //     },
            //     syntax: Syntax {
            //         address: myresult.syntax.address.as_ref().unwrap().to_string(),
            //         domain: myresult.syntax.domain,
            //         is_valid_syntax: myresult.syntax.is_valid_syntax,
            //         username: myresult.syntax.username,
            //     },
            // };
            // let output = Box::new(responce);
            let can_connect = match myresult.smtp.as_ref() {
                Ok(m) => m.can_connect_smtp,
                Err(_) => false,
            };
            let has_full_inbox = match myresult.smtp.as_ref() {
                Ok(m) => m.has_full_inbox,
                Err(_) => false,
            };
            let is_catch_all = match myresult.smtp.as_ref() {
                Ok(m) => m.is_catch_all,
                Err(_) => false,
            };
            let is_deliverable = match myresult.smtp.as_ref() {
                Ok(m) => m.is_deliverable,
                Err(_) => true,
            };
            let is_disabled = match myresult.smtp.as_ref() {
                Ok(m) => m.is_disabled,
                Err(_) => false,
            };
            let mx_records = myresult
                .mx
                .unwrap()
                .lookup
                .unwrap()
                .iter()
                .map(|record| Record {
                    host: record.exchange().clone().to_string(),
                    preference: record.preference().clone(),
                })
                .filter(|x| x.host != "mx.kmoz.dev.".to_string())
                .collect::<Vec<Record>>();
            let res = json!({
                "status":true,
                "data":{
                    "email":myresult.input,
                    "status": Reachable::from_str(format!("{:?}", myresult.is_reachable).as_str()),
                    "temporary_email":myresult.misc.as_ref().unwrap().is_disposable,
                    "restricted":myresult.misc.as_ref().unwrap().is_role_account,
                    "email_syntax_valid": myresult.syntax.is_valid_syntax,
                    "mx_records": mx_records,
                    "smtp_live": can_connect,
                    "smtp_has_full_inbox": has_full_inbox,
                    "smtp_can_catch_all": is_catch_all,
                    "smtp_deliverable": is_deliverable,
                    "smtp_disabled": is_disabled,

                },
                "free_request": user.lastcount
            });
            CheckDemoResponce {
                text: res.to_string(),
                status: Status::Ok,
            }
        }
        true => {
            let res = json!({
                "status":false,
                "reason":"free_pack_exeeded",
                "free_request": user.lastcount
            });
            CheckDemoResponce {
                text: res.to_string(),
                status: Status::Ok,
            }
        }
    }
}

#[openapi(tag = "MainPage")]
#[get("/")]
pub fn mainpage() -> Template {
    Template::render("index", context! { name: "ok" })
}
