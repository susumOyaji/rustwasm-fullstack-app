use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

// --- Data Structures ---

/// A generic container for API responses.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

/// Represents the financial data scraped from a web page.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct FinancialData {
    pub name: Option<String>,
    pub code: Option<String>,
    pub update_time: Option<String>,
    pub current_value: Option<f64>,
    pub bid_value: Option<String>,
    pub previous_day_change: Option<f64>,
    pub change_rate: Option<f64>,
}

/// Holds the CSS selectors used for scraping.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct SelectorConfig {
    pub name_selector: Option<String>,
    pub current_value_selector: Option<String>,
    pub previous_day_change_selector: Option<String>,
    pub change_rate_selector: Option<String>,
    pub update_time_selector: Option<String>,
    pub fx_item_selector: Option<String>,
    pub fx_term_selector: Option<String>,
    pub fx_price_selector: Option<String>,
}

/// Represents the type of financial instrument.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum CodeType {
    Fx,
    Nikkei,
    Dji,
    Stock,
}

/// Represents an error that occurred while fetching data for a specific code.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteError {
    pub code: String,
    pub error: String,
}

/// Represents the result of a quote request, containing both successful and failed items.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteResponse {
    pub success: Vec<FinancialData>,
    pub failed: Vec<QuoteError>,
}

/// Represents the request body for the test parser endpoint.
#[derive(Deserialize, Serialize)]
pub struct TestParseRequest {
    pub html_content: String,
    pub code: String,
    pub selectors: SelectorConfig,
}

/// Stores dynamically found JSON paths for data fields.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct JsonPathConfig {
    pub name_path: Option<String>,
    pub current_value_path: Option<String>,
    pub previous_day_change_path: Option<String>,
    pub change_rate_path: Option<String>,
    pub update_time_path: Option<String>,
    pub bid_value_path: Option<String>,
}


// --- Type Conversions ---

impl CodeType {
    /// Returns the string representation of the code type.
    pub fn as_str(&self) -> &'static str {
        match self {
            CodeType::Fx => "fx",
            CodeType::Nikkei => "nikkei",
            CodeType::Dji => "dji",
            CodeType::Stock => "stock",
        }
    }
}

impl FromStr for CodeType {
    type Err = worker::Error;

    /// Parses a string into a `CodeType`.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "fx" => Ok(CodeType::Fx),
            "nikkei" => Ok(CodeType::Nikkei),
            "dji" => Ok(CodeType::Dji),
            "stock" => Ok(CodeType::Stock),
            _ => Err(worker::Error::from(format!("Invalid code_type: {}", s))),
        }
    }
}