use super::position_status::PositionStatus;
use super::timestamp::Timestamp;

#[derive(Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
  pub bot_id: String,
  pub entry_at: Box<Timestamp>,
  pub exit_at: Option<Box<Timestamp>>,
  pub id: String,
  pub profit_amount: String,
  pub profit_percent: String,
  pub status: Box<PositionStatus>,
  pub symbol: String,
  pub trading_amount: String,
  pub valuation: String,
}