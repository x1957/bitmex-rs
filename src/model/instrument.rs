use super::GeneralRequest;
use chrono::{DateTime, Utc};

pub type GetInstrumentRequest = GeneralRequest;

impl Default for GetInstrumentRequest {
    fn default() -> Self {
        GetInstrumentRequest {
            symbol: None,
            columns: None,
            filter: None,
            count: 100,
            start: None,
            reverse: None,
            start_time: None,
            end_time: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentResponse {
    pub symbol: Option<String>,
    pub root_symbol: Option<String>,
    pub state: Option<String>,
    pub typ: Option<String>,
    pub listing: Option<String>,
    pub front: Option<String>,
    pub expiry: Option<String>,
    pub settle: Option<String>,
    pub relist_interval: Option<String>,
    pub inverse_leg: Option<String>,
    pub sell_leg: Option<String>,
    pub buy_leg: Option<String>,
    pub option_strike_pcnt: Option<f64>,
    pub option_strike_round: Option<f64>,
    pub option_strike_price: Option<f64>,
    pub option_multiplier: Option<f64>,
    pub position_currency: Option<String>,
    pub underlying: Option<String>,
    pub quote_currency: Option<String>,
    pub underlying_symbol: Option<String>,
    pub reference: Option<String>,
    pub reference_symbol: Option<String>,
    pub calc_interval: Option<String>,
    pub publish_interval: Option<String>,
    pub publish_time: Option<String>,
    pub max_order_qty: Option<f64>,
    pub max_price: Option<f64>,
    pub lot_size: Option<f64>,
    pub tick_size: Option<f64>,
    pub multiplier: Option<f64>,
    pub settl_currency: Option<String>,
    pub underlying_to_position_multiplier: Option<f64>,
    pub underlying_to_settle_multiplier: Option<f64>,
    pub quote_to_settle_multiplier: Option<f64>,
    pub is_quanto: Option<bool>,
    pub is_inverse: Option<bool>,
    pub init_margin: Option<f64>,
    pub maint_margin: Option<f64>,
    pub risk_limit: Option<f64>,
    pub risk_step: Option<f64>,
    pub limit: Option<f64>,
    pub capped: Option<bool>,
    pub taxed: Option<bool>,
    pub deleverage: Option<bool>,
    pub maker_fee: Option<f64>,
    pub taker_fee: Option<f64>,
    pub settlement_fee: Option<f64>,
    pub insurance_fee: Option<f64>,
    pub funding_base_symbol: Option<String>,
    pub funding_quote_symbol: Option<String>,
    pub funding_premium_symbol: Option<String>,
    pub funding_timestamp: Option<String>,
    pub funding_interval: Option<String>,
    pub funding_rate: Option<f64>,
    pub indicative_funding_rate: Option<f64>,
    pub rebalance_timestamp: Option<String>,
    pub rebalance_interval: Option<String>,
    pub opening_timestamp: Option<String>,
    pub closing_timestamp: Option<String>,
    pub session_interval: Option<String>,
    pub prev_close_price: Option<f64>,
    pub limit_down_price: Option<f64>,
    pub limit_up_price: Option<f64>,
    pub bankrupt_limit_down_price: Option<f64>,
    pub bankrupt_limit_up_price: Option<f64>,
    pub prev_total_volume: Option<f64>,
    pub total_volume: Option<f64>,
    pub volume: Option<f64>,
    pub volume24h: Option<f64>,
    pub prev_total_turnover: Option<f64>,
    pub total_turnover: Option<f64>,
    pub turnover: Option<f64>,
    pub turnover24h: Option<f64>,
    pub home_notional24h: Option<f64>,
    pub foreign_notional24h: Option<f64>,
    pub prev_price24h: Option<f64>,
    pub vwap: Option<f64>,
    pub high_price: Option<f64>,
    pub low_price: Option<f64>,
    pub last_price: Option<f64>,
    pub last_price_protected: Option<f64>,
    pub last_tick_direction: Option<String>,
    pub last_change_pcnt: Option<f64>,
    pub bid_price: Option<f64>,
    pub mid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub impact_bid_price: Option<f64>,
    pub impact_mid_price: Option<f64>,
    pub impact_ask_price: Option<f64>,
    pub has_liquidity: Option<bool>,
    pub open_interest: Option<f64>,
    pub open_value: Option<f64>,
    pub fair_method: Option<String>,
    pub fair_basis_rate: Option<f64>,
    pub fair_basis: Option<f64>,
    pub fair_price: Option<f64>,
    pub mark_method: Option<String>,
    pub mark_price: Option<f64>,
    pub indicative_tax_rate: Option<f64>,
    pub indicative_settle_price: Option<f64>,
    pub option_underlying_price: Option<f64>,
    pub settled_price: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

pub type GetInstrumentActiveResponse = GetInstrumentResponse;
pub type GetInstrumentActiveAndIndicesResponse = GetInstrumentResponse;
pub type GetInstrumentIndicesResponse = GetInstrumentResponse;

#[derive(Clone, Debug, Deserialize)]
pub struct GetInstrumentActiveIntervalsResponse {
    pub intervals: Vec<String>,
    pub symbols: Vec<String>,
}

pub type GetInstrumentCompositeIndexRequest = GetInstrumentRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentCompositeIndexResponse {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub index_symbol: String,
    pub reference: String,
    pub last_price: f64,
    pub weight: f64,
    pub logged: DateTime<Utc>,
}
