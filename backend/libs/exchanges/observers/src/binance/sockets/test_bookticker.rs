use ::futures::StreamExt;

use ::round_robin_client::entities::WSMessageDetail;
use ::rug::Float;
use ::test_utils::websocket::setup_server;

use crate::binance::entities::{BookTicker, WebsocketPayload};

use super::BookTickerSocket;

fn setup_bookticker_fixture() -> Vec<BookTicker<Float>> {
  return vec![
    BookTicker {
      id: 1,
      symbol: "BTCUSDT".to_string(),
      bid_price: Float::with_val(32, 1.2345678),
      bid_qty: Float::with_val(32, 2.3456789),
      ask_price: Float::with_val(32, 3.4567890),
      ask_qty: Float::with_val(32, 4.5678901),
    },
    BookTicker {
      id: 2,
      symbol: "BTCUSDT".to_string(),
      bid_price: Float::with_val(32, 5.6789012),
      bid_qty: Float::with_val(32, 6.7890123),
      ask_price: Float::with_val(32, 7.8901234),
      ask_qty: Float::with_val(32, 8.9012345),
    },
    BookTicker {
      id: 3,
      symbol: "BTCUSDT".to_string(),
      bid_price: Float::with_val(32, 9.0123456),
      bid_qty: Float::with_val(32, 0.1234567),
      ask_price: Float::with_val(32, 12.345678),
      ask_qty: Float::with_val(32, 23.456789),
    },
  ];
}

fn setup_payload_fixture(
  bookticker_fixture: &[BookTicker<Float>],
) -> Vec<WebsocketPayload> {
  return bookticker_fixture
    .iter()
    .map(|bookticker| {
      let bookticker: BookTicker<String> = bookticker.clone().into();
      WebsocketPayload::BookTicker(bookticker)
    })
    .collect();
}

#[::tokio::test]
async fn test_bookticker_socket_receive() {
  let correct = setup_bookticker_fixture();
  let fixture = setup_payload_fixture(&correct);
  let (sig, port) = setup_server(&fixture);
  let mut socket =
    BookTickerSocket::test_new(format!("ws://127.0.0.1:{}", port))
      .await
      .unwrap();
  let mut result: Vec<BookTicker<Float>> = Vec::new();
  while let Some(payload) = socket.next().await {
    match payload {
      WSMessageDetail::EntityReceived(payload) => {
        result.push(payload);
      }
      WSMessageDetail::Continue => {}
      WSMessageDetail::Disconnected => break,
    }
    if result.len() >= fixture.len() {
      break;
    }
  }
  let _ = sig.send(());
  assert_eq!(result, correct);
}
