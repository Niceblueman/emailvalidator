#![allow(missing_docs)]
#![allow(unused_imports)]

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
