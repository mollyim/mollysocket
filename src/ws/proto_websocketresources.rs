#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebSocketRequestMessage {
    #[prost(string, optional, tag = "1")]
    pub verb: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub body: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "5")]
    pub headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "4")]
    pub id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebSocketResponseMessage {
    #[prost(uint64, optional, tag = "1")]
    pub id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub status: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub body: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebSocketMessage {
    #[prost(enumeration = "web_socket_message::Type", optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub request: ::core::option::Option<WebSocketRequestMessage>,
    #[prost(message, optional, tag = "3")]
    pub response: ::core::option::Option<WebSocketResponseMessage>,
}
/// Nested message and enum types in `WebSocketMessage`.
pub mod web_socket_message {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Unknown = 0,
        Request = 1,
        Response = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "UNKNOWN",
                Type::Request => "REQUEST",
                Type::Response => "RESPONSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "REQUEST" => Some(Self::Request),
                "RESPONSE" => Some(Self::Response),
                _ => None,
            }
        }
    }
}
