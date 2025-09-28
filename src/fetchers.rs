use reqwest::Client;
use chrono::Datelike;
use chrono::Utc;
use scraper::{Html, Selector};
use std::collections::HashMap;


pub async fn fetch_info_page(client: &Client, base_url: &str) -> Result<String, String> {
    let url = format!("{}/HomeAccess/Content/Student/Registration.aspx", base_url);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch registration page".to_string())?;

    let body = response
        .text()
        .await
        .map_err(|_| "Failed to read registration page body".to_string())?;

    Ok(body)
}

pub async fn fetch_assignments_page(
    client: &Client,
    base_url: &str,
) -> Result<String, String> {
    let url = format!("{}/HomeAccess/Content/Student/Assignments.aspx", base_url);

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch assignments page".to_string())?;

    let html = resp
        .text()
        .await
        .map_err(|_| "Failed to read assignments page".to_string())?;

    Ok(html)
}


fn format_six_weeks_param(input: &str) -> String {
    if input.eq_ignore_ascii_case("ALL") {
        return "ALL".to_string();
    }

    let week: u8 = match input.parse() {
        Ok(w) => w,
        Err(_) => return "ALL".to_string(),
    };

    let now = Utc::now();
    let year = if now.month() >= 7 {
        now.year() + 1
    } else {
        now.year()
    };


    format!("{}-{}", week, year)
}


pub async fn fetch_assignments_page_for_six_weeks(
    client: &Client,
    base_url: &str,
    six_weeks: &str,
) -> Result<String, String> {
    let assignments_url = format!("{}/HomeAccess/Content/Student/Assignments.aspx", base_url);
    let adjusted_six_weeks = format_six_weeks_param(six_weeks);

    let response = client
        .get(&assignments_url)
        .send()
        .await
        .map_err(|_| "Failed to fetch assignments page".to_string())?;

    let body = response
        .text()
        .await
        .map_err(|_| "Failed to read assignments page".to_string())?;

    let payload = {
        let document = Html::parse_document(&body);

        let viewstate = document
            .select(&Selector::parse("input[name='__VIEWSTATE']").unwrap())
            .next()
            .and_then(|el| el.value().attr("value"))
            .unwrap_or("")
            .to_string();

        let generator = document
            .select(&Selector::parse("input[name='__VIEWSTATEGENERATOR']").unwrap())
            .next()
            .and_then(|el| el.value().attr("value"))
            .unwrap_or("")
            .to_string();

        let validation = document
            .select(&Selector::parse("input[name='__EVENTVALIDATION']").unwrap())
            .next()
            .and_then(|el| el.value().attr("value"))
            .unwrap_or("")
            .to_string();

        let mut form_data: HashMap<&str, String> = HashMap::new();
        form_data.insert("__EVENTTARGET", "ctl00$plnMain$btnRefreshView".to_string());
        form_data.insert("__EVENTARGUMENT", "".to_string());
        form_data.insert("__LASTFOCUS", "".to_string());
        form_data.insert("__VIEWSTATE", viewstate);
        form_data.insert("__VIEWSTATEGENERATOR", generator);
        form_data.insert("__EVENTVALIDATION", validation);
        form_data.insert("ctl00$plnMain$ddlReportCardRuns", adjusted_six_weeks);
        form_data.insert("ctl00$plnMain$ddlClasses", "ALL".to_string());
        form_data.insert("ctl00$plnMain$ddlCompetencies", "ALL".to_string());
        form_data.insert("ctl00$plnMain$ddlOrderBy", "Class".to_string());

        form_data
    }; 

    let post_resp = client
        .post(&assignments_url)
        .form(&payload)
        .send()
        .await
        .map_err(|_| "Failed to post assignments request".to_string())?;

    let post_body = post_resp
        .text()
        .await
        .map_err(|_| "Failed to read assignments response".to_string())?;

    Ok(post_body)
}
pub async fn fetch_name_page(client: &Client, base_url: &str) -> Result<String, String> {
    let url = format!("{}/HomeAccess/Classes/Classwork", base_url);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch classwork page".to_string())?;

    let body = response
        .text()
        .await
        .map_err(|_| "Failed to read classwork page body".to_string())?;

    Ok(body)
}

pub async fn fetch_report_page(client: &reqwest::Client, base_url: &str) -> Result<String, String> {
    let url = format!("{}/HomeAccess/Content/Student/ReportCards.aspx", base_url);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch report page".to_string())?;

    let body = response
        .text()
        .await
        .map_err(|_| "Failed to read report page body".to_string())?;

    Ok(body)
}

pub async fn fetch_progress_page(client: &Client, base_url: &str) -> Result<String, String> {
    let url = format!("{}/HomeAccess/Content/Student/InterimProgress.aspx", base_url);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch progress page".to_string())?;

    let body = response
        .text()
        .await
        .map_err(|_| "Failed to read progress page body".to_string())?;

    Ok(body)
}

pub async fn fetch_transcript_page(client: &Client, base_url: &str) -> Result<String, String> {
    let url = format!("{}/HomeAccess/Content/Student/Transcript.aspx", base_url);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch transcript page".to_string())?;

    let body = response
        .text()
        .await
        .map_err(|_| "Failed to read transcript page body".to_string())?;

    Ok(body)
}
