use crate::models::{CodeType, SelectorConfig};
use crate::utils::{fetch_html, get_default_selectors};
use scraper::{Html, Selector, Element};
use std::collections::HashMap;
use worker::kv::KvStore;
use worker::{Result, RouteContext};


/// 指定URLのHTMLをダウンロードし、デフォルトセレクターで要素が取得できるか判定する
pub async fn check_selectors_against_url(url: &str, code_type: CodeType) -> Result<HashMap<String, bool>> {
    // HTMLダウンロード
    let html = fetch_html(url).await?;
    let document = Html::parse_document(&html);
    let selectors = get_default_selectors(code_type);

    // セレクター名とOption<String>のペアを列挙
    let selector_map: Vec<(&str, &Option<String>)> = match code_type {
        CodeType::Fx => vec![
            ("name_selector", &selectors.name_selector),
            ("fx_item_selector", &selectors.fx_item_selector),
            ("fx_term_selector", &selectors.fx_term_selector),
            ("fx_price_selector", &selectors.fx_price_selector),
        ],
        _ => vec![
            ("name_selector", &selectors.name_selector),
            ("current_value_selector", &selectors.current_value_selector),
            ("previous_day_change_selector", &selectors.previous_day_change_selector),
            ("change_rate_selector", &selectors.change_rate_selector),
            ("update_time_selector", &selectors.update_time_selector),
        ],
    };

    // 判定結果
    let mut result = HashMap::new();
    for (name, sel_opt) in selector_map {
        let found = if let Some(sel) = sel_opt {
            if let Ok(selector) = Selector::parse(sel) {
                document.select(&selector).next().is_some()
            } else {
                false
            }
        } else {
            false
        };
        result.insert(name.to_string(), found);
    }
    Ok(result)
}

/// 指定URLのHTMLを取得し、各セレクターが有効か自動判定し、無効な場合は新しいセレクターに書き換える
pub async fn auto_update_invalid_selectors(
    url: &str,
    code_type: CodeType,
    new_selectors: &SelectorConfig,
    kv: &KvStore,
) -> Result<HashMap<String, bool>> {
    let html = fetch_html(url).await?;
    let document = Html::parse_document(&html);
    let mut selectors = get_default_selectors(code_type);

    // セレクター名とOption<String>のペアを列挙
    let selector_map: Vec<(&str, &mut Option<String>, &Option<String>)> = match code_type {
        CodeType::Fx => vec![ 
            ("name_selector", &mut selectors.name_selector, &new_selectors.name_selector),
            ("fx_item_selector", &mut selectors.fx_item_selector, &new_selectors.fx_item_selector),
            ("fx_term_selector", &mut selectors.fx_term_selector, &new_selectors.fx_term_selector),
            ("fx_price_selector", &mut selectors.fx_price_selector, &new_selectors.fx_price_selector),
        ],
        _ => vec![ 
            ("name_selector", &mut selectors.name_selector, &new_selectors.name_selector),
            ("current_value_selector", &mut selectors.current_value_selector, &new_selectors.current_value_selector),
            ("previous_day_change_selector", &mut selectors.previous_day_change_selector, &new_selectors.previous_day_change_selector),
            ("change_rate_selector", &mut selectors.change_rate_selector, &new_selectors.change_rate_selector),
            ("update_time_selector", &mut selectors.update_time_selector, &new_selectors.update_time_selector),
        ],
    };

    let mut result = HashMap::new();
    for (name, sel_mut, new_sel) in selector_map {
        let found = if let Some(sel) = sel_mut {
            if let Ok(selector) = Selector::parse(sel) {
                document.select(&selector).next().is_some()
            } else {
                false
            }
        } else {
            false
        };
        // 無効なら新しいセレクターに書き換え
        if !found {
            *sel_mut = new_sel.clone();
        }
        result.insert(name.to_string(), found);
    }

    // KVストアに新しいセレクターを保存
    kv.put(code_type.as_str(), &selectors)?.execute().await?;

    Ok(result)
}


/// Attempts to heal the selectors for FX by analyzing the page structure.
/// Specifically targets the current_value_selector (Ask price).
/// Returns `true` if a new selector was found and updated in KV.
pub async fn heal_fx_selectors(ctx: &RouteContext<()>) -> Result<bool> {
    worker::console_log!("[Healing] Attempting to heal FX selectors...");
    let url = "https://finance.yahoo.co.jp/quote/USDJPY=FX";

    let html_content = match fetch_html(url).await {
        Ok(content) => content,
        Err(e) => {
            worker::console_error!("[Healing] Failed to fetch HTML: {}", e);
            return Ok(false);
        }
    };
    
    let document = Html::parse_document(&html_content);

    // Keyword for the FX "Ask" price
    let keyword = "Ask（買値）";
    let mut new_selector: Option<String> = None;

    // Heuristic: Find the <dt> element containing the keyword. The price should be in the
    // <span> inside the next sibling <dd> element. This is specific to the current
    // Yahoo Finance layout.
    let dt_selector = Selector::parse("dt").unwrap();
    let span_selector = Selector::parse("span[class*='_FxPriceBoard__price']").unwrap();

    for dt_element in document.select(&dt_selector) {
        if dt_element.text().any(|text| text.contains(keyword)) {
            // Found the right <dt>. Look for the next <dd>.
            if let Some(dd_element) = dt_element.next_sibling_element() {
                if dd_element.value().name() == "dd" {
                    // Find the specific span inside the <dd>.
                    if let Some(span_element) = dd_element.select(&span_selector).next() {
                        let tag = span_element.value().name();
                        let classes = span_element.value().classes().collect::<Vec<_>>();
                        if !classes.is_empty() {
                            // Reconstruct a selector like "span._FxPriceBoard__price.xxxx"
                            let selector_string = format!("{}.{}", tag, classes.join("."));
                            worker::console_log!("[Healing] Found candidate selector: {}", selector_string);
                            new_selector = Some(selector_string);
                            break; // Found it, no need to loop further
                        }
                    }
                }
            }
        }
    }

    if let Some(selector) = new_selector {
        worker::console_log!("[Healing] Found new selector for FX current_value: {}", selector);
        let kv = ctx.env.kv("FIN_SELECTORS")?;
        
        // Get current selectors, or default if none exist
        let mut selectors: SelectorConfig = kv.get("fx").json().await?.unwrap_or_else(|| get_default_selectors(CodeType::Fx));
        
        // Update the specific selector
        selectors.current_value_selector = Some(selector);
        
        // Save back to KV
        kv.put("fx", selectors)?.execute().await?;
        worker::console_log!("[Healing] Successfully updated FX selectors in KV store.");
        Ok(true)
    } else {
        worker::console_log!("[Healing] Could not find a new selector for FX.");
        Ok(false)
    }
}