use prost::Message;

// message WebSocketRequestMessage {
//     optional string verb    = 1;
//     optional string path    = 2;
//     optional bytes  body    = 3;
//     repeated string headers = 5;
//     optional uint64 id      = 4;
// }

#[derive(Clone, PartialEq, Message)]
pub struct WebSocketRequestMessage {
    #[prost(string, optional, tag = "1")]
    pub verb: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub path: Option<String>,
    #[prost(bytes, optional, tag = "3")]
    pub body: Option<Vec<u8>>,
    #[prost(string, repeated, tag = "5")]
    pub headers: Vec<String>,
    #[prost(uint64, optional, tag = "4")]
    pub id: Option<u64>,
}

// message WebSocketResponseMessage {
//     optional uint64 id      = 1;
//     optional uint32 status  = 2;
//     optional string message = 3;
//     repeated string headers = 5;
//     optional bytes  body    = 4;
// }

#[derive(Clone, PartialEq, Message)]
pub struct WebSocketResponseMessage {
    #[prost(uint64, optional, tag = "1")]
    pub id: Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub status: Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub message: Option<String>,
    #[prost(string, repeated, tag = "5")]
    pub headers: Vec<String>,
    #[prost(bytes, optional, tag = "4")]
    pub body: Option<Vec<u8>>,
}

// message WebSocketMessage {
//     enum Type {
//         UNKNOWN  = 0;
//         REQUEST  = 1;
//         RESPONSE = 2;
//     }

//     optional Type                     type     = 1;
//     optional WebSocketRequestMessage  request  = 2;
//     optional WebSocketResponseMessage response = 3;
// }

#[derive(Clone, PartialEq, Message)]
pub struct WebSocketMessage {
    #[prost(enumeration = "webSocketMessage::Type", optional, tag = "1")]
    pub r#type: Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub request: Option<WebSocketRequestMessage>,
    #[prost(message, optional, tag = "3")]
    pub response: Option<WebSocketResponseMessage>,
}

#[allow(non_snake_case)]
pub mod webSocketMessage {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        UNKNOWN = 0,
        REQUEST = 1,
        RESPONSE = 2,
    }
}
