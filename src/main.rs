#![allow(missing_docs)]
#![allow(unused_imports)]

use check_if_email_exists::{
    check_email, mx, smtp, syntax, CheckEmailInput, CheckEmailInputProxy, CheckEmailOutput,
};
use chrono::{DateTime, Duration, Utc};
use easylog::log_file::{LogFile, LogLevel};
use easylog::log_file_config::LogFileConfig;
use reqwest::header;
use reqwest::*;
use rocket::config::Config;
use rocket::figment::Figment;
use rocket::fs::{relative, FileServer, Options};
use rocket::futures::FutureExt;
use rocket::http::Method;
use rocket::{routes, Route};
use rocket::{catch, catchers, response, response::Responder, Response};
use rocket::{
    figment::providers::{Env, Format, Serialized, Toml},
    Request,
};
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};
use rocket_dyn_templates::{context, Template};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{MediaType, Responses};
use rocket_okapi::okapi::schemars;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi_get_routes, rapidoc::*, swagger_ui::*, OpenApiError};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{format, Display};
use std::str::FromStr;
use dotenv::dotenv;
// --------- All different methods of implementing `OpenApiFromRequest` ------------
// There are a few different ways of doing things.
// And it also depend on the authentication (if any) you want to implement.
// Here are a few different example that cover most of the use cases:
// - No special authentication
// - ApiKey (in http header, query or cookie)
// - HTTP `Authorization` header (inc `basic`, `digest` and `bearer` tokens)
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication#authentication_schemes
// - OAuth 2.0 flows (authorizationCode, implicit, password, clientCredentials)
// - OpenID Connect
// - Just Cookies (for just 1 route/endpoint)
// ---------------------------------------------------------------------------------

mod api_key;
mod tools;

// #[derive(Debug, PartialEq, Deserialize)]
// struct StripeConfig { key: String, secret:String }

fn cors_options() -> CorsOptions {
    let allowed_origins = AllowedOrigins::All;
    // let allowed_origins = AllowedOrigins::All;
    // You can also deserialize this
    rocket_cors::CorsOptions {
        allowed_origins,
        send_wildcard: true,
        // allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        // allow_credentials: true,
        ..Default::default()
    }
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let figment = Config::figment();
    // Set a dummy secret
    // .merge(("secret_key", "226693e9ef34a4233a5e6b316b6d0499".as_bytes()))
    // .merge(("port", 8383))
    // .merge(("address", "0.0.0.0".as_bytes()));
    // let settings = rocket_okapi::settings::OpenApiSettings::new();
    let launch_result = rocket::custom(figment)
        .mount(
            "/",
            openapi_get_routes![
                api_key::check_handler,
                api_key::check_bulk,
                // http_auth::http_auth,
                // oauth2::oauth2_auth_code_get_user,
                // open_id::open_id,
                // cookies::cookie_auth,
            ],
        )
        .mount(
            "/",
            routes![
                // api_key::session,
                api_key::mainpage,
                api_key::register,
                api_key::login,
                api_key::recover,
                api_key::demo_check,
                api_key::profile,
                api_key::getplans,
                api_key::manage_plans,
                api_key::payment_link,
                api_key::pay_back,
                api_key::pay_back_get,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Dark,
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .mount("/static", FileServer::from("./templates/static"))
        .mount("/Background", FileServer::from("./templates/Background"))
        // .mount("/Background", FileServer::from("./templates/Background"))
        .mount("/rss", FileServer::from("./templates/feed"))
        .mount("/assets", FileServer::from("./templates/assets"))
        .mount("/icons", FileServer::from("./templates/icons"))
        // .mount("/Logo", FileServer::from(relative!("templates/Logo")))
        // .mount("/", rocket_cors::catch_all_options_routes()) // mount the catch all routes
        .manage(cors_options().to_cors().expect("To not fail"))
        .attach(Template::fairing())
        .register(
            "/",
            catchers![bad_request, unauthorized, not_found, internal_error],
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

// ----- Catchers -------

/// Error messages returned to user
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct MyError {
    /// The title of the error message
    pub err: String,
    /// The description of the error
    pub msg: Option<String>,
    // HTTP Status Code returned
    #[serde(skip)]
    pub http_status_code: u16,
}

#[catch(400)]
fn bad_request() -> MyError {
    MyError {
        err: "Bad Request".to_owned(),
        msg: Some("The request given is wrongly formatted or data was missing.".to_owned()),
        http_status_code: 400,
    }
}
#[catch(500)]
fn internal_error() -> MyError {
    MyError {
        err: "Bad Request".to_owned(),
        msg: Some("Whoops! Looks like we messed up.".to_owned()),
        http_status_code: 500,
    }
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let default = LogFileConfig::new();
    let mut logfile = LogFile::new(default).unwrap();
    logfile.write(
        LogLevel::INFO,
        format!(
            "origin_ip:{:?} | remote:{:?}",
            req.headers().get_one("cf-connecting-ip"),
            req.remote()
                .into_iter()
                .collect::<Vec<std::net::SocketAddr>>()
        )
        .as_str(),
    );
    let apikey = "PJkAekkIqocvtj5jMsc2CnGIs7ogOWcd";
    let error_str = "The page you requested cannot be found right now";
    Template::render("error", context! { apikey, error_str })
}
#[catch(401)]
fn unauthorized() -> MyError {
    MyError {
        err: "Unauthorized".to_owned(),
        msg: Some("The authentication given was incorrect or insufficient.".to_owned()),
        http_status_code: 401,
    }
}

/// Create my custom response
///
/// Putting this in a separate function somewhere will resolve issues like
/// <https://github.com/GREsau/okapi/issues/57>
pub fn bad_request_response(gen: &mut OpenApiGenerator) -> okapi::openapi3::Response {
    let schema = gen.json_schema::<MyError>();
    okapi::openapi3::Response {
        description: "\
        # 400 Bad Request\n\
        The request given is wrongly formatted or data was missing. \
        "
        .to_owned(),
        content: okapi::map! {
            "application/json".to_owned() => MediaType {
                schema: Some(schema),
                ..Default::default()
            }
        },
        ..Default::default()
    }
}

pub fn unauthorized_response(gen: &mut OpenApiGenerator) -> okapi::openapi3::Response {
    let schema = gen.json_schema::<MyError>();
    okapi::openapi3::Response {
        description: "\
        # 401 Unauthorized\n\
        The authentication given was incorrect or insufficient. \
        "
        .to_owned(),
        content: okapi::map! {
            "application/json".to_owned() => MediaType {
                schema: Some(schema),
                ..Default::default()
            }
        },
        ..Default::default()
    }
}

impl<'r> Responder<'r, 'static> for MyError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .status(rocket::http::Status::new(self.http_status_code))
            .ok()
    }
}

impl OpenApiResponderInner for MyError {
    fn responses(gen: &mut OpenApiGenerator) -> std::result::Result<Responses, OpenApiError> {
        use okapi::openapi3::RefOr;
        Ok(Responses {
            responses: okapi::map! {
                "400".to_owned() => RefOr::Object(bad_request_response(gen)),
                "401".to_owned() => RefOr::Object(unauthorized_response(gen)),
            },
            ..Default::default()
        })
    }
}
/// testing aria .............
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct StripePlan {
    key: String,
    secret: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct StripePlans {
    dev: StripePlan,
    prod: StripePlan,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Plan {
    pub name: String,
    pub requests_per_day: u32,
    pub items_per_req: u32,
    pub price: u32,
}
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Plans {
    pub starter: Plan,
    pub premium: Plan,
    pub demo: Plan,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, time::SystemTime};

    use serde_json::json;

    use crate::tools::{stripe, ApiGen, ApiTokenType, Config, Costumers};

    use super::*;

    //#[test]
    #[tokio::test]
    async fn check() {
        let args: Vec<String> = env::args().collect();
        println!("inputs: {}", args[2]);
        let mut _email = args[2].clone();
        let mut email = CheckEmailInput::new(_email);
        let mut _email = args[2].clone();
        email
            .set_from_email("no-reply@accounts.google.com".into())
            .set_hello_name(_email.split("@").nth(1).unwrap().to_string());
        let result = check_email(&email).await;
        println!("results: {:?}", result);
        assert_eq!(2, 2);
    }
    #[tokio::test]
    async fn customers() {
        let filename = "Stripe.toml";
        let contents = fs::read_to_string(filename).unwrap();
        let data: StripePlans = toml::from_str(&contents).unwrap();
        let dev = data.dev.clone();
        // let prod =  data.prod.clone();
        let _stripe = stripe::Stripe::new(dev.key, dev.secret);
        let data = _stripe.get_customer("cus_MPRh2F5Z8EJfIP".to_string()).await;
        //cus_MPRh2F5Z8EJfIP
        println!("{:?}", data);
        // code here for testing stripe
    }
    #[tokio::test]
    async fn assert_customer() {
        // testing database
        let filename = "Plan.toml";
        let contents = fs::read_to_string(filename).unwrap();
        let plans: Plans = toml::from_str(&contents).unwrap();
        let costomers_db = Costumers::new();
        let now: DateTime<Utc> = Utc::now();
        costomers_db.save_customer(
            uuid::Uuid::new_v4().to_string(),
            "cus_MPRh2F5Z8EJfIP".into(),
            Config {
                cus_id: "cus_MPRh2F5Z8EJfIP".into(),
                email: "example@domain.com".into(),
                items_per_req: plans.starter.items_per_req,
                requests_per_day: plans.starter.items_per_req,
                last_count: 0 as u32,
                last_usage_day: now.to_rfc2822(),
                name: "cus_MPRh2F5Z8EJfIP".into(),
                phone: "+79965043656".into(),
            },
        );
        let retrieved_customer = costomers_db
            .find_customer("cus_MPRh2F5Z8EJfIP".into())
            .unwrap();
        println!("found costumer: {:?}", retrieved_customer);
        assert_eq!(
            costomers_db.check_db_path().to_string_lossy(),
            "./database.sqlite"
        );
    }
    #[tokio::test]
    async fn test_token() {
        // testing token generation and validation
        let filename = "Plan.toml";
        let contents = fs::read_to_string(filename).unwrap();
        let plans: Plans = toml::from_str(&contents).unwrap();
        let keygen = ApiGen::new();
        let month = 1;
        let now: DateTime<Utc> = Utc::now();
        let plan = Config {
            cus_id: "cus_MPRh2F5Z8EJfIP".into(),
            email: "example@domain.com".into(),
            items_per_req: plans.starter.items_per_req,
            requests_per_day: plans.starter.items_per_req,
            last_count: 0 as u32,
            last_usage_day: now.to_rfc2822(),
            name: ApiTokenType::new(plans.starter.name.as_str()).string(),
            phone: "+19999999999".into(),
        };
        let __time = keygen.get_current_timestamp() + Duration::days(30).num_seconds() as u64;
        let key = keygen
            .generate(month, "cus_MPRh2F5Z8EJfIP".into(), plan)
            .expect("Error loading new keygen!");
        println!("Generated ApiKey: {}, {}", __time, key);
    }
    #[tokio::test]
    async fn test_valid_token() {
        // testing the token validation
        let __key = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzUxMiIsImtpZCI6ImN1c19NUFJoMkY1WjhFSmZJUCJ9.eyJpc3MiOiJlbWFpbHZhbGlkYXRvci5rbW96LmRldiIsImNvbXBhbnkiOiJLTU96X0xURCIsImV4cCI6MTY2NjM0NDI2OCwiZGF0YSI6eyJwbGFuIjp7Im5hbWUiOiJTdGFydGVyIiwicmVxdWVzdHNfcGVyX2RheSI6MSwiaXRlbXNfcGVyX3JlcSI6MSwicGhvbmUiOiIrNzk5NjUwNDM2NTYiLCJlbWFpbCI6ImV4YW1wbGVAZG9tYWluLmNvbSIsImN1c19pZCI6ImN1c19NUFJoMkY1WjhFSmZJUCIsImxhc3RfdXNhZ2VfZGF5IjoiV2VkLCAyMSBTZXAgMjAyMiAwOToyNDoyOCArMDAwMCIsImxhc3RfY291bnQiOjB9LCJ1c2VyX2lkIjoiY3VzX01QUmgyRjVaOEVKZklQIn19.S096QfnwQ17EQM-RTdgiV6-sJ7Z_VVDQsNHpFeJdu9gG5Sgni5bf7xiroUlmoxG_T1zzsHZWIo029lfoid96DcVgvce3exbhkYQbqG1auNiQFPVvGQSbLpWU11T2h-V5tQCGYw6XP0RMk7Zo_p485_IkfNpAanEK_DjOY_3SElU1bU1JkpIOKsVLtJaTmHGjbI6bjraXmpUb3QPeR5bl6z4oKfgo0Y4DsZ7N2H9eIq9lbHfjoyxpaFJhBRy9d7aNkuiS6v_ojVZvjPM25SAj1gnvcg7Y8QlG8aP_wrTOkxfl36rDPVuQ24aaOCu5MDbE_vaYtTXerQwn53Erx0zSlg";
        let keygen = ApiGen::new();
        let valid = keygen.validate(__key);
        println!("token id valid:{:?}", valid);
    }
    #[tokio::test]
    async fn test_files() {
        // testing the token validation
        let valid = FileServer::from("/app/static");
        println!("files :{:?}", valid);
    }
    #[tokio::test]
    async fn register() {
        // let mut headers = header::HeaderMap::new();
        // Consider marking security-sensitive headers with `set_sensitive`.
        // let mut auth_value =
        //     header::HeaderValue::from_str(format!("Bearer {}", "ok").as_str())
        //         .unwrap();
        // auth_value.set_sensitive(true);
        // headers.insert(header::AUTHORIZATION, auth_value);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(900))
            .danger_accept_invalid_certs(true)
            // .default_headers(headers)
            .build()
            .expect("client faild to build!");
        let mut _body = HashMap::new();
        _body.insert("username", "username00");
        _body.insert("email", "email@example.com");
        _body.insert("password", "password00");
        let res = client
            .post("https://mailvalidator.dup.company/register")
            .body(
                "{\"username\":\"user0\",\"email\":\"user@email.com\",\"pass\":\"ajkshdkjsdfjk\"}",
            )
            .send()
            .await
            .expect("faild get_customer");
        println!("{:?}", res);
    }
    #[tokio::test]
    async fn sendemail() {}
}
