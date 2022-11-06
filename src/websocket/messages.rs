// use actix::prelude::{Message, Recipient};

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct WsMessage(pub String);

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Connect {
//     pub addr: Recipient<WsMessage>,
//     pub lobby_id: u32,
//     pub self_id: u32,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Disconnect {
//     pub id: u32,
//     pub room_id: u32,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct ClientActorMessage {
//     pub id: u32,
//     pub msg: String,
//     pub room_id: u32,
// }
