use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, Query, WebSocketUpgrade};
use axum::response::{Json, Response};
use axum::{routing::get, Router};
use tokio::net::TcpListener;
// api build
pub trait ApiBuilder {
    fn build_api();
}
//start app
#[tokio::main]
pub async fn app_run(port: u16) {
    let mut rt = Router::new()
        .route("/orders/add/:name", get(orders_add))
        .route("/orders/cancle/:name", get(cancle_order))
        .route("/sub/order/:name", get(sub_order));
    let addr = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(addr, rt).await.unwrap();
}

async fn orders_add(Path(name): Path<String>, req: Json<types::AddReq>) -> Json<types::Response> {
    Json(types::Response {})
}
async fn cancle_order(
    Path(name): Path<String>,
    req: Query<types::CancleReq>,
) -> Json<types::Response> {
    Json(types::Response {})
}

async fn sub_order(upgrade: WebSocketUpgrade) -> Response {
    upgrade.on_upgrade(hand_sub_order)
}
async fn hand_sub_order(mut conn: WebSocket) {
    use internal::*;
    let ob = OrderBook::default();
    let df = defer::new(move || {});
    while let Some(req) = conn.recv().await {
        match req {
            Err(err) => return,
            Ok(data) => {
                let raw: types::Sub = data.try_into().unwrap();
                //action by sub code
                match raw.code {
                    types::SubCode::Check => {
                        //check order status
                    }
                }
            }
        }
    }
}
struct defer<F>
where
    F: Fn(),
{
    f: F,
}
impl<F: Fn()> defer<F> {
    pub fn new(src: F) -> Self {
        Self { f: src }
    }
}
impl<F: Fn()> Drop for defer<F> {
    fn drop(&mut self) {
        (self.f)();
    }
}
mod middleware {

    // pub fn mymiddleware() -> impl Filter<Extract = (), Error = Rejection> + Copy {}
}
mod types {
    use axum::extract::ws::Message;
    use rust_decimal::Decimal;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Response {}
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Head {}
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct AddReq {
        pub order_id: String,
        pub price: Decimal,
        pub amount: Decimal,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct CancleReq {
        pub order_id: String,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default)]
    pub struct Sub {
        pub code: SubCode,
        pub msg: String,
    }
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub enum SubCode {
        Check,
    }
    impl Default for SubCode {
        fn default() -> Self {
            todo!()
        }
    }
    impl TryFrom<Message> for Sub {
        type Error = u8;

        fn try_from(value: Message) -> Result<Self, Self::Error> {
            let msg = value.to_text().unwrap();
            match serde_json::from_str(msg) {
                Err(err) => Ok(Self::default()),
                Ok(dt) => Ok(dt),
            }
        }
    }
}
mod internal {
    #[derive(Default)]
    pub struct OrderBook {}
}
