use crate::models::{CodeType, FinancialData, SelectorConfig};
use crate::utils::{clean_text, get_code_type};
use scraper::{Html, Selector};
use worker::Result;

pub fn parse_with_selectors(html_content: &str, code: &str, selectors: &SelectorConfig) -> Result<FinancialData> {
    let document = Html::parse_document(html_content);
    let get_text = |sel_str: &Option<String>| -> Result<Option<String>> {
        if let Some(s) = sel_str {
            let selector = Selector::parse(s).map_err(|e| worker::Error::from(format!("Selector parse error: {}", e)))?;
            Ok(document.select(&selector).next().map(|e| e.text().collect::<String>().trim().to_string()))
        } else {
            Ok(None)
        }
    };

    let mut data = FinancialData {
        code: Some(code.to_string()),
        ..Default::default()
    };

    let code_type = get_code_type(code);
    if code_type == CodeType::Fx {
        data.name = get_text(&selectors.name_selector)?;
        if let (Some(item_sel), Some(term_sel), Some(price_sel)) =
            (&selectors.fx_item_selector, &selectors.fx_term_selector, &selectors.fx_price_selector) {
            let item_selector = Selector::parse(item_sel).map_err(|e| worker::Error::from(format!("{}",e)))?;
            let term_selector = Selector::parse(term_sel).map_err(|e| worker::Error::from(format!("{}",e)))?;
            let price_selector = Selector::parse(price_sel).map_err(|e| worker::Error::from(format!("{}",e)))?;

            for item in document.select(&item_selector) {
                let term = item.select(&term_selector).next().map(|e| e.text().collect::<String>());
                let value = item.select(&price_selector).next().map(|e| e.text().collect::<String>());
                if let Some(term) = term {
                    match term.trim() {
                        "Bid（売値）" => data.bid_value = value,
                        "Ask（買値）" => data.current_value = value,
                        "Change（始値比）" => data.change_rate = clean_text(value),
                        _ => (),
                    }
                }
            }
        }
    } else { // dji, nikkei, stock
        data.name = get_text(&selectors.name_selector)?;
        data.current_value = get_text(&selectors.current_value_selector)?;
        data.previous_day_change = get_text(&selectors.previous_day_change_selector)?;
        data.change_rate = clean_text(get_text(&selectors.change_rate_selector)?);
        data.update_time = get_text(&selectors.update_time_selector)?;
    }

    Ok(data)
}