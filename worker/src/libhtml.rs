use serde::Deserialize;
use super::FinancialData;

// --- Structs for Individual Stocks ---
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StockPriceBoard {
    name: Option<String>,
    price: Option<String>,
    price_date_time: Option<String>,
    price_change: Option<String>,
    price_change_rate: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MainStocksPriceBoard {
    price_board: StockPriceBoard,
}

// --- Structs for Domestic Indices (e.g., Nikkei) ---
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct IndexPrices {
    name: Option<String>,
    price: Option<String>,
    #[serde(rename = "changePrice")]
    change_price: Option<String>,
    #[serde(rename = "changePriceRate")]
    change_price_rate: Option<String>,
    #[serde(rename = "japanUpdateTime")]
    update_time: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MainDomesticIndexPriceBoard {
    index_prices: IndexPrices,
}


// --- Top-level State Struct ---
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PreloadedState {
    main_stocks_price_board: Option<MainStocksPriceBoard>,
    main_domestic_index_price_board: Option<MainDomesticIndexPriceBoard>,
}

/// Parses financial data from the __PRELOADED_STATE__ JSON embedded in HTML.
pub fn parse_from_preloaded_state(html_content: &str, code: &str) -> Result<FinancialData, String> {
    const SCRIPT_START: &str = "window.__PRELOADED_STATE__ = ";
    let script_line = html_content
        .lines()
        .find(|line| line.trim().starts_with(SCRIPT_START));

    if let Some(line) = script_line {
        let json_str = line.trim()
            .strip_prefix(SCRIPT_START)
            .and_then(|s| s.strip_suffix(';'))
            .unwrap_or_else(|| &line.trim()[SCRIPT_START.len()..]);

        match serde_json::from_str::<PreloadedState>(json_str) {
            Ok(state) => {
                if let Some(stock_board_wrapper) = state.main_stocks_price_board {
                    let board = stock_board_wrapper.price_board;
                    Ok(FinancialData {
                        name: board.name,
                        code: Some(code.to_string()),
                        update_time: board.price_date_time,
                        current_value: board.price,
                        previous_day_change: board.price_change,
                        change_rate: board.price_change_rate,
                        ..Default::default()
                    })
                } else if let Some(index_board_wrapper) = state.main_domestic_index_price_board {
                    let prices = index_board_wrapper.index_prices;
                    Ok(FinancialData {
                        name: prices.name,
                        code: Some(code.to_string()),
                        update_time: prices.update_time,
                        current_value: prices.price,
                        previous_day_change: prices.change_price,
                        change_rate: prices.change_price_rate,
                        ..Default::default()
                    })
                } else {
                    Err("No relevant price board found in __PRELOADED_STATE__".to_string())
                }
            }
            Err(e) => Err(format!("JSON parsing error: {}", e)),
        }
    } else {
        Err("Could not find __PRELOADED_STATE__ in HTML content".to_string())
    }
}
