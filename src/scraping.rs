use scraper::{Html, Selector};
use std::collections::HashMap;

pub fn shorten_class_name(full: &str) -> String {
    let mut words: Vec<&str> = full.split_whitespace().collect();

    if words.len() > 3 {
        words.drain(0..3);
    }

    while let Some(last) = words.last() {
        if *last == "Classwork"
            || *last == "Average"
            || last.parse::<f32>().is_ok()
            || (last.len() == 1 && last.chars().all(char::is_alphabetic))
        {
            words.pop();
        } else {
            break;
        }
    }

    words.join(" ")
}

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


pub fn extract_classes(html: &str, short: bool) -> Vec<String> {
    use scraper::{Html, Selector};

    let document = Html::parse_document(html);
    let selector = Selector::parse("div.sg-header").unwrap();

    let mut classes = Vec::new();

    for (i, header) in document.select(&selector).enumerate() {
        if i == 0 {
            continue;
        }

        let text = header.text().collect::<Vec<_>>().join(" ");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");

        if short {
            classes.push(shorten_class_name(&text));
        } else {
            let clean = text
                .split("Classwork")
                .next()
                .unwrap_or(&text)
                .trim()
                .to_string();
            classes.push(clean);
        }
    }

    classes
}

pub fn extract_averages(html: &str, short: bool) -> HashMap<String, String> {
    let document = Html::parse_document(html);
    let assignment_selector = Selector::parse("div.AssignmentClass").unwrap();
    let header_selector = Selector::parse("div.sg-header").unwrap();
    let average_selector = Selector::parse("span.sg-header-heading").unwrap();

    let mut results = HashMap::new();

    for assignment in document.select(&assignment_selector) {
        let header_text = assignment
            .select(&header_selector)
            .next()
            .map(|h| h.text().collect::<Vec<_>>().join(" "))
            .unwrap_or_default();

        let class_name = if short {
            shorten_class_name(&header_text)
        } else {
            header_text
                .split("Classwork")
                .next()
                .unwrap_or(&header_text)
                .trim()
                .to_string()
        };

        let average_text = assignment
            .select(&average_selector)
            .next()
            .map(|a| a.text().collect::<Vec<_>>().join(" "))
            .unwrap_or_default();

        let average_clean = if average_text.len() > 18 {
            average_text[18..].trim().to_string()
        } else {
            average_text.trim().to_string()
        };

        results.insert(class_name, average_clean);
    }

    results
}

