use crate::websocket::{lobby::Lobby, ws::WsConn};

use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};

use actix_web_actors::ws;

#[get("/echo/{room_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    room_id: Path<String>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let room_id = room_id.into_inner();

    let ws = WsConn::new(room_id, srv.get_ref().clone());

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}
