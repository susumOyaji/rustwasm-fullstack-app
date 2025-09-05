use serde::Deserialize;
// FinancialData構造体を親モジュール(lib.rs)からインポートします。
// これにより、同じデータ構造を共有できます。
use super::FinancialData;

// JSONのネスト構造に合わせたstructを定義します。
// 不要なフィールドは無視するように設定しています。
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PriceBoard {
    name: Option<String>,
    price: Option<String>,
    price_date_time: Option<String>,
    price_change: Option<String>,
    price_change_rate: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MainStocksPriceBoard {
    price_board: PriceBoard,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PreloadedState {
    main_stocks_price_board: MainStocksPriceBoard,
}

/// HTMLに埋め込まれた__PRELOADED_STATE__のJSONから財務データを解析します。
///
/// # Arguments
/// * `html_content` - 解析するHTMLページの完全な文字列。
/// * `code` - 処理対象の銘柄コード。
///
/// # Returns
/// * `Ok(FinancialData)` - 解析が成功した場合。
/// * `Err(String)` - 解析に失敗した場合（JSONが見つからない、パースエラーなど）。
pub fn parse_from_preloaded_state(html_content: &str, code: &str) -> Result<FinancialData, String> {
    // window.__PRELOADED_STATE__ を含む行を探します。
    const SCRIPT_START: &str = "window.__PRELOADED_STATE__ = ";
    let script_line = html_content
        .lines()
        .find(|line| line.trim().starts_with(SCRIPT_START));

    if let Some(line) = script_line {
        // JSON部分を抽出します。
        // 行の末尾に`;`がある可能性を考慮して取り除きます。
        let json_str = line.trim()
            .strip_prefix(SCRIPT_START)
            .and_then(|s| s.strip_suffix(';'))
            .unwrap_or_else(|| &line.trim()[SCRIPT_START.len()..]);

        // JSONをパースします。
        match serde_json::from_str::<PreloadedState>(json_str) {
            Ok(state) => {
                let board = state.main_stocks_price_board.price_board;
                
                // FinancialData構造体にマッピングして返します。
                Ok(FinancialData {
                    name: board.name,
                    code: Some(code.to_string()),
                    update_time: board.price_date_time,
                    current_value: board.price,
                    previous_day_change: board.price_change,
                    change_rate: board.price_change_rate,
                    bid_value: None, // このJSONには含まれていない
                })
            }
            Err(e) => Err(format!("JSON parsing error: {}", e)),
        }
    } else {
        Err("Could not find __PRELOADED_STATE__ in HTML content".to_string())
    }
}
