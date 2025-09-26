use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub async fn login_handler(username: &str, password: &str, link: &str) -> Result<Client, String> {
    let client = Client::builder()
        .cookie_store(true)
        .build()
        .map_err(|e| format!("Client build error: {}", e))?;

    let login_url = format!("{}/HomeAccess/Account/LogOn", link.trim_end_matches('/'));
    let resp = client
        .get(&login_url)
        .send()
        .await
        .map_err(|e| format!("Failed to GET login page: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Failed to read login page HTML: {}", e))?;

    let token = {
        let doc = Html::parse_document(&resp);
        let selector = Selector::parse("input[name='__RequestVerificationToken']").unwrap();
        doc.select(&selector)
            .next()
            .and_then(|e| e.value().attr("value"))
            .map(|s| s.to_string())
            .ok_or("No __RequestVerificationToken found")?
    };

    let mut form = HashMap::new();
    form.insert("__RequestVerificationToken", token);
    form.insert("SCKTY00328510CustomEnabled", "True".to_string());
    form.insert("SCKTY00436568CustomEnabled", "True".to_string());
    form.insert("Database", "10".to_string());
    form.insert("VerificationOption", "UsernamePassword".to_string());
    form.insert("LogOnDetails.UserName", username.to_string());
    form.insert("tempUN", "".to_string());
    form.insert("tempPW", "".to_string());
    form.insert("LogOnDetails.Password", password.to_string());

    let post_resp = client
        .post(&login_url)
        .form(&form)
        .send()
        .await
        .map_err(|e| format!("Failed to POST login: {}", e))?;

    let final_url = post_resp.url().to_string();
    if final_url.contains("LogOn") {
        Err("Invalid username or password".to_string())
    } else {
        Ok(client)
    }
}
