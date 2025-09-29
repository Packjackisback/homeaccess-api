use reqwest::Client;
use chrono::Datelike;
use chrono::Utc;
use scraper::{Html, Selector};
use std::collections::HashMap;
use crate::cache::Cache;

async fn fetch_page(
    client: &Client,
    base_url: &str,
    endpoint: &str,
) -> Result<String, String> {
    let url = format!("{}/HomeAccess/Content/Student/{}", base_url, endpoint);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| format!("Failed to fetch {} page", endpoint))?;

    let body = response
        .text()
        .await
        .map_err(|_| format!("Failed to read {} page body", endpoint))?;

    Ok(body)
}

pub async fn fetch_info_page(
    client: &Client,
    base_url: &str,
    cache: &Cache,
    username: &str,
    no_cache: bool,
) -> Result<String, String> {
    if !no_cache {
        if let Some(cached) = cache.get_page(username, base_url, "Registration.aspx", "").await {
            return Ok(cached);
        }
    }

    let html = fetch_page(client, base_url, "Registration.aspx").await?;
    
    if !no_cache {
        cache.set_page(username, base_url, "Registration.aspx", "", html.clone()).await;
    }
    
    Ok(html)
}

pub async fn fetch_assignments_page(
    client: &Client,
    base_url: &str,
    cache: &Cache,
    username: &str,
    no_cache: bool,
) -> Result<String, String> {
    if !no_cache {
        if let Some(cached) = cache.get_page(username, base_url, "Assignments.aspx", "current").await {
            return Ok(cached);
        }
    }

    let html = fetch_page(client, base_url, "Assignments.aspx").await?;
    
    if !no_cache {
        cache.set_page(username, base_url, "Assignments.aspx", "current", html.clone()).await;
    }
    
    Ok(html)
}

pub async fn fetch_report_page(
    client: &Client,
    base_url: &str,
    cache: &Cache,
    username: &str,
    no_cache: bool,
) -> Result<String, String> {
    if !no_cache {
        if let Some(cached) = cache.get_page(username, base_url, "ReportCards.aspx", "").await {
            return Ok(cached);
        }
    }

    let html = fetch_page(client, base_url, "ReportCards.aspx").await?;
    
    if !no_cache {
        cache.set_page(username, base_url, "ReportCards.aspx", "", html.clone()).await;
    }
    
    Ok(html)
}

pub async fn fetch_progress_page(
    client: &Client,
    base_url: &str,
    cache: &Cache,
    username: &str,
    no_cache: bool,
) -> Result<String, String> {
    if !no_cache {
        if let Some(cached) = cache.get_page(username, base_url, "InterimProgress.aspx", "").await {
            return Ok(cached);
        }
    }

    let html = fetch_page(client, base_url, "InterimProgress.aspx").await?;
    
    if !no_cache {
        cache.set_page(username, base_url, "InterimProgress.aspx", "", html.clone()).await;
    }
    
    Ok(html)
}

pub async fn fetch_transcript_page(
    client: &Client,
    base_url: &str,
    cache: &Cache,
    username: &str,
    no_cache: bool,
) -> Result<String, String> {
    if !no_cache {
        if let Some(cached) = cache.get_page(username, base_url, "Transcript.aspx", "").await {
            return Ok(cached);
        }
    }

    let html = fetch_page(client, base_url, "Transcript.aspx").await?;
    
    if !no_cache {
        cache.set_page(username, base_url, "Transcript.aspx", "", html.clone()).await;
    }
    
    Ok(html)
}

pub async fn fetch_name_page(
    client: &Client,
    base_url: &str,
    cache: &Cache,
    username: &str,
    no_cache: bool,
) -> Result<String, String> {
    if !no_cache {
        if let Some(cached) = cache.get_page(username, base_url, "Classwork", "").await {
            return Ok(cached);
        }
    }

    let url = format!("{}/HomeAccess/Classes/Classwork", base_url);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "Failed to fetch classwork page".to_string())?;

    let html = response
        .text()
        .await
        .map_err(|_| "Failed to read classwork page body".to_string())?;

    if !no_cache {
        cache.set_page(username, base_url, "Classwork", "", html.clone()).await;
    }

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

    let payload = extract_form_data(&body, &adjusted_six_weeks);

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

fn extract_form_data(body: &str, adjusted_six_weeks: &str) -> HashMap<&'static str, String> {
    let document = Html::parse_document(body);

    static VIEWSTATE_SEL: std::sync::OnceLock<Selector> = std::sync::OnceLock::new();
    static GENERATOR_SEL: std::sync::OnceLock<Selector> = std::sync::OnceLock::new();
    static VALIDATION_SEL: std::sync::OnceLock<Selector> = std::sync::OnceLock::new();

    let viewstate_sel = VIEWSTATE_SEL.get_or_init(|| {
        Selector::parse("input[name='__VIEWSTATE']").unwrap()
    });
    let generator_sel = GENERATOR_SEL.get_or_init(|| {
        Selector::parse("input[name='__VIEWSTATEGENERATOR']").unwrap()
    });
    let validation_sel = VALIDATION_SEL.get_or_init(|| {
        Selector::parse("input[name='__EVENTVALIDATION']").unwrap()
    });

    let viewstate = document
        .select(viewstate_sel)
        .next()
        .and_then(|el| el.value().attr("value"))
        .unwrap_or("")
        .to_string();

    let generator = document
        .select(generator_sel)
        .next()
        .and_then(|el| el.value().attr("value"))
        .unwrap_or("")
        .to_string();

    let validation = document
        .select(validation_sel)
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
    form_data.insert("ctl00$plnMain$ddlReportCardRuns", adjusted_six_weeks.to_string());
    form_data.insert("ctl00$plnMain$ddlClasses", "ALL".to_string());
    form_data.insert("ctl00$plnMain$ddlCompetencies", "ALL".to_string());
    form_data.insert("ctl00$plnMain$ddlOrderBy", "Class".to_string());

    form_data
}
