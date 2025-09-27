use scraper::{Html, Selector};
use std::collections::HashMap;
use serde_json::{Value, Map};



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

fn normalize_class_name(name: &str, short: bool) -> String {
    let name = name.trim();
    let name = name.split("Classwork").next().unwrap_or(name).trim();
    if short {
        shorten_class_name(name)
    } else {
        name.to_string()
    }
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

        let class_name = normalize_class_name(&header_text, short);

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

pub fn extract_weightings(html: &str, short: bool) -> HashMap<String, Vec<Vec<String>>> {
    let document = Html::parse_document(html);
    let class_selector = Selector::parse("div.AssignmentClass").unwrap();
    let table_selector = Selector::parse("table.sg-asp-table").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    let header_selector = Selector::parse("div.sg-header").unwrap();
    let heading_selector = Selector::parse("a.sg-header-heading").unwrap();

    let mut weightings: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for class_group in document.select(&class_selector) {
        let header = match class_group.select(&header_selector).next() {
            Some(h) => h,
            None => continue,
        };

        
        let header_text = header
            .select(&heading_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let class_name = normalize_class_name(&header_text, short);


        let mut class_weightings: Vec<Vec<String>> = Vec::new();

        for table in class_group.select(&table_selector) {
            if let Some(id) = table.value().attr("id") {
                if id.contains("CourseCategories") {
                    let rows: Vec<_> = table.select(&row_selector).collect();
                    if rows.len() <= 1 { continue; }

                    let rows_to_process = &rows[1..rows.len().saturating_sub(1)];

                    for row in rows_to_process {
                        let row_data: Vec<String> = row.select(&cell_selector)
                            .map(|c| c.text().collect::<String>().trim().to_string())
                            .collect();

                        if row_data.is_empty() { continue; }
                        class_weightings.push(row_data);
                    }
                }
            }
        }

        if !class_weightings.is_empty() {
            weightings.insert(class_name, class_weightings);
        }
    }

    weightings
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
        let header = match class_group.select(&header_selector).next() {
            Some(h) => h,
            None => continue,
        };

        let header_text = header
            .select(&link_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let class_name = normalize_class_name(&header_text, short);

        let mut assignments_for_class: Vec<Vec<String>> = Vec::new();

        for table in class_group.select(&table_selector) {
            if let Some(id) = table.value().attr("id") {
                if id.contains("CourseAssignments") {
                    let rows: Vec<_> = table.select(&row_selector).collect();
                    if rows.len() <= 2 { continue; }
                    

                    let rows_to_process = &rows[1..rows.len().saturating_sub(2)];

                    for row in rows_to_process {
                        let row_data: Vec<String> = row.select(&cell_selector)
                            .map(|cell| cell.text().collect::<String>()
                                .replace("*", "")
                                .split_whitespace()
                                .collect::<Vec<_>>()
                                .join(" "))
                            .collect();

                        if row_data.is_empty() { continue; }

                        if let Some(first) = row_data.first() {
                            if ["Major", "Minor", "Other", "Total"].contains(&first.as_str()) {
                                continue;
                            }
                        }

                        assignments_for_class.push(row_data);
                    }
                }
            }
        }

        if !assignments_for_class.is_empty() {
            ret.insert(class_name, assignments_for_class);
        }
    }

    ret
}

pub fn extract_gradebook(html: &str, short: bool) -> Map<String, Value> {
    let averages = extract_averages(html, short);
    let assignments = extract_assignments(html, short);
    let weightings = extract_weightings(html, short);
    
    let mut combined = Map::new();
    
    let mut all_classes: std::collections::HashSet<String> = std::collections::HashSet::new();
    all_classes.extend(averages.keys().cloned());
    all_classes.extend(assignments.keys().cloned());
    all_classes.extend(weightings.keys().cloned());
    
    for class_name in all_classes {
        let mut class_obj = Map::new();
        
        class_obj.insert(
            "average".to_string(),
            Value::String(averages.get(&class_name).unwrap_or(&String::new()).clone())
        );
        
        class_obj.insert(
            "assignments".to_string(),
            Value::Array(
                assignments.get(&class_name)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|row| Value::Array(
                        row.iter().map(|cell| Value::String(cell.clone())).collect()
                    ))
                    .collect()
            )
        );
        
        class_obj.insert(
            "weightings".to_string(),
            Value::Array(
                weightings.get(&class_name)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|row| Value::Array(
                        row.iter().map(|cell| Value::String(cell.clone())).collect()
                    ))
                    .collect()
            )
        );
        
        combined.insert(class_name, Value::Object(class_obj));
    }
    
    combined
}

pub fn extract_report_cards(html: &str) -> Vec<Vec<String>> {
    use scraper::{Html, Selector};

    let document = Html::parse_document(html);
    let td_selector = Selector::parse("td").unwrap();

    let mut all_cells: Vec<String> = Vec::new();

    for (i, td) in document.select(&td_selector).enumerate() {
        if i >= 32 { 
            let text = td.text().collect::<Vec<_>>().join(" ").trim().to_string();
            all_cells.push(text);
        }
    }

    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();

    for cell in all_cells {
        current_row.push(cell);
        if current_row.len() == 32 {
            rows.push(current_row);
            current_row = Vec::new();
        }
    }

    for row in &mut rows {
        if row.len() >= 32 {
            row.drain(23..32);
        }

        if row.len() >= 7 {
            row.drain(5..7);
        }
    }

    rows
}

pub fn extract_progress(html: &str) -> Vec<Vec<String>> {
    let document = Html::parse_document(html);
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut data: Vec<Vec<String>> = Vec::new();

    for (i, row) in document.select(&row_selector).enumerate() {
        let row_data: Vec<String> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .collect();

        if row_data.is_empty() {
            continue;
        }

        if i != 0 {
            data.push(row_data);
        }
    }

    data 
}

