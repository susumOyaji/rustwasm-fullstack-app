use crate::models::{CodeType, SelectorConfig};
use worker::{Headers, Method, Request, RequestInit, Result};

/// Determines the `CodeType` from a given financial code string.
pub fn get_code_type(code: &str) -> CodeType {
    if code.contains("=FX") {
        CodeType::Fx
    } else if code.contains(".O") || code.contains("^N225") { // Nikkei
        CodeType::Nikkei
    } else if code.contains('^') { // Other indices like DJI
        CodeType::Dji
    } else {
        CodeType::Stock
    }
}

/// Provides a default `SelectorConfig` for a given `CodeType`.
pub fn get_default_selectors(code_type: CodeType) -> SelectorConfig {
    match code_type {
        CodeType::Fx => SelectorConfig {
            name_selector: Some("h2[class*=\"_BasePriceBoard__name\"]".to_string()),
            fx_item_selector: Some("div[class*=\"_FxPriceBoard__item\"] > dl".to_string()),
            fx_term_selector: Some("dt".to_string()),
            fx_price_selector: Some("dd span[class*=\"_FxPriceBoard__price\"] span[class*=\"_StyledNumber__value\"]".to_string()),
            ..Default::default()
        },
        CodeType::Dji | CodeType::Nikkei | CodeType::Stock => SelectorConfig {
            name_selector: Some("h2[class*=\"_BasePriceBoard__name\"]".to_string()),
            current_value_selector: Some("span[class*=\"_CommonPriceBoard__price\"] span[class*=\"_StyledNumber__value\"]".to_string()),
            previous_day_change_selector: Some("span[class*=\"__primary\"] span[class*=\"_StyledNumber__value\"]".to_string()),
            change_rate_selector: Some("span[class*=\"__secondary\"]".to_string()),
            update_time_selector: Some("li[class*=\"_CommonPriceBoard__time\"] > time".to_string()),
            ..Default::default()
        },
    }
}

/// Cleans a string by removing whitespace, parentheses, and percent signs.
pub fn clean_text(s: Option<String>) -> Option<String> {
    s.map(|text| {
        text.replace(|c: char| c.is_whitespace() || c == '(' || c == ')' || c == '%' , "")
            .trim()
            .to_string()
    })
}

/// Fetches HTML content from a given URL.
pub async fn fetch_html(url: &str) -> Result<String> {
    let mut headers = Headers::new();
    headers.set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")?;
    let mut request_init = RequestInit::new();
    request_init.with_method(Method::Get).with_headers(headers);
    let request = Request::new_with_init(url, &request_init)?;
    worker::Fetch::Request(request).send().await?.text().await
}

/// Parses a string into an Option<f64>, removing commas.
pub fn parse_f64(s: Option<String>) -> Option<f64> {
    s.and_then(|text| {
        text.replace(",", "").parse::<f64>().ok()
    })
}