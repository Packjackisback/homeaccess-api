use scraper::{Html, Selector};
use std::collections::HashMap;


pub fn extract_name(body: &str) -> Option<String> {
    let doc = Html::parse_document(body);
    let sel = Selector::parse("div.sg-banner-menu-container span").ok()?;
    doc.select(&sel)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(""))
}

pub fn extract_info(html: &str) -> Option<HashMap<String, String>> {
    let document = Html::parse_document(html);
    let mut info = HashMap::new();

    let selectors = vec![
        ("name", "#plnMain_lblRegStudentName"),
        ("grade", "#plnMain_lblGrade"),
        ("school", "#plnMain_lblBuildingName"),
        ("dob", "#plnMain_lblBirthDate"),
        ("counselor", "#plnMain_lblCounselor"),
        ("language", "#plnMain_lblLanguage"),
        ("cohort_year", "#plnMain_lblCohortYear"),
    ];
    for (key, sel) in selectors {
        let selector = Selector::parse(sel).ok()?;
        if let Some(element) = document.select(&selector).next() {
            info.insert(key.to_string(), element.text()
                .collect::<String>().trim().to_string());
        }
    }

    if info.is_empty() { None } else { Some(info) }
}

pub fn extract_classes(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let mut classes = Vec::new();

    let container_selector = Selector::parse("div.AssignmentClass").unwrap();
    let header_selector = Selector::parse("div.sg-header").unwrap();

    for container in document.select(&container_selector) {
        if let Some(header) = container.select(&header_selector).next() {
            let text = header.text().collect::<Vec<_>>().join(" ");
            let mut words: Vec<&str> = text.split_whitespace().collect();
            if words.len() <= 3 {
                continue;
            }
            words.drain(0..3);
            while let Some(last) = words.last() {
                if last == &"Classwork" || last == &"Average" || last.parse::<f32>().is_ok() || last.len() == 1 && last.chars().all(char::is_alphabetic) {
                    words.pop();
                } else {
                    break;
                }
            }

            let class_name = words.join(" ");
            if !class_name.is_empty() {
                classes.push(class_name);
            }
        }
    }
    classes
}

