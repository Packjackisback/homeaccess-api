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

pub fn extract_assignments(html: &str, short: bool) -> HashMap<String, Vec<Vec<String>>> {
    let document = Html::parse_document(html);
    let class_selector = Selector::parse("div.AssignmentClass").unwrap();
    let header_selector = Selector::parse("div.sg-header").unwrap();
    let link_selector = Selector::parse("a.sg-header-heading").unwrap();
    let table_selector = Selector::parse("table.sg-asp-table").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut ret: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for class_group in document.select(&class_selector) {
        let header = class_group.select(&header_selector).next();
        if header.is_none() {
            continue;
        }
        let mut class_name = header
            .unwrap()
            .select(&link_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        class_name = if class_name.len() > 12 {
            class_name[12..].trim().to_string()
        } else {
            class_name.trim().to_string()
        };

        if short {
            class_name = shorten_class_name(&class_name);
        }

        let mut assignments_for_class: Vec<Vec<String>> = Vec::new();

        for table in class_group.select(&table_selector) {
            for (i, row) in table.select(&row_selector).enumerate() {
                let mut row_data = Vec::new();

                for cell in row.select(&cell_selector) {
                    let mut text = cell.text().collect::<String>();
                    text = text.replace("*", "");
                    text = text.split_whitespace().collect::<Vec<_>>().join(" ");
                    row_data.push(text);
                }

                if i == 0 {
                    continue;
                }

                if !row_data.is_empty() {
                    assignments_for_class.push(row_data);
                }
            }
        }

        if assignments_for_class.len() > 2 {
            let len = assignments_for_class.len();
            assignments_for_class.truncate(len - 2);
        }

        if !assignments_for_class.is_empty() {
            ret.insert(class_name, assignments_for_class);
        }
    }

    ret
}

