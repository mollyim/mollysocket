#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Envelope {
    #[prost(enumeration = "envelope::Type", optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub source_service_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "7")]
    pub source_device: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "13")]
    pub destination_service_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "5")]
    pub timestamp: ::core::option::Option<u64>,
    /// Contains an encrypted Content
    #[prost(bytes = "vec", optional, tag = "8")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "9")]
    pub server_guid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "10")]
    pub server_timestamp: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "14", default = "true")]
    pub urgent: ::core::option::Option<bool>,
    /// MOLLY: Used by linked device if primary device changes phone number
    #[prost(string, optional, tag = "15")]
    pub updated_pni: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "16")]
    pub story: ::core::option::Option<bool>,
    /// NEXT ID: 18
    #[prost(bytes = "vec", optional, tag = "17")]
    pub reporting_token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `Envelope`.
pub mod envelope {
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
        Ciphertext = 1,
        KeyExchange = 2,
        PrekeyBundle = 3,
        Receipt = 5,
        UnidentifiedSender = 6,
        PlaintextContent = 8,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "UNKNOWN",
                Type::Ciphertext => "CIPHERTEXT",
                Type::KeyExchange => "KEY_EXCHANGE",
                Type::PrekeyBundle => "PREKEY_BUNDLE",
                Type::Receipt => "RECEIPT",
                Type::UnidentifiedSender => "UNIDENTIFIED_SENDER",
                Type::PlaintextContent => "PLAINTEXT_CONTENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "CIPHERTEXT" => Some(Self::Ciphertext),
                "KEY_EXCHANGE" => Some(Self::KeyExchange),
                "PREKEY_BUNDLE" => Some(Self::PrekeyBundle),
                "RECEIPT" => Some(Self::Receipt),
                "UNIDENTIFIED_SENDER" => Some(Self::UnidentifiedSender),
                "PLAINTEXT_CONTENT" => Some(Self::PlaintextContent),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    #[prost(message, optional, tag = "1")]
    pub data_message: ::core::option::Option<DataMessage>,
    #[prost(message, optional, tag = "2")]
    pub sync_message: ::core::option::Option<SyncMessage>,
    #[prost(message, optional, tag = "3")]
    pub call_message: ::core::option::Option<CallMessage>,
    #[prost(message, optional, tag = "4")]
    pub null_message: ::core::option::Option<NullMessage>,
    #[prost(message, optional, tag = "5")]
    pub receipt_message: ::core::option::Option<ReceiptMessage>,
    #[prost(message, optional, tag = "6")]
    pub typing_message: ::core::option::Option<TypingMessage>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub sender_key_distribution_message: ::core::option::Option<
        ::prost::alloc::vec::Vec<u8>,
    >,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub decryption_error_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "9")]
    pub story_message: ::core::option::Option<StoryMessage>,
    #[prost(message, optional, tag = "10")]
    pub pni_signature_message: ::core::option::Option<PniSignatureMessage>,
    #[prost(message, optional, tag = "11")]
    pub edit_message: ::core::option::Option<EditMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallMessage {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<call_message::Offer>,
    #[prost(message, optional, tag = "2")]
    pub answer: ::core::option::Option<call_message::Answer>,
    #[prost(message, repeated, tag = "3")]
    pub ice_update: ::prost::alloc::vec::Vec<call_message::IceUpdate>,
    #[prost(message, optional, tag = "4")]
    pub legacy_hangup: ::core::option::Option<call_message::Hangup>,
    #[prost(message, optional, tag = "5")]
    pub busy: ::core::option::Option<call_message::Busy>,
    #[prost(message, optional, tag = "7")]
    pub hangup: ::core::option::Option<call_message::Hangup>,
    #[prost(uint32, optional, tag = "9")]
    pub destination_device_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "10")]
    pub opaque: ::core::option::Option<call_message::Opaque>,
}
/// Nested message and enum types in `CallMessage`.
pub mod call_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Offer {
        #[prost(uint64, optional, tag = "1")]
        pub id: ::core::option::Option<u64>,
        /// Legacy/deprecated; replaced by 'opaque'
        #[prost(string, optional, tag = "2")]
        pub sdp: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(enumeration = "offer::Type", optional, tag = "3")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(bytes = "vec", optional, tag = "4")]
        pub opaque: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    /// Nested message and enum types in `Offer`.
    pub mod offer {
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
            OfferAudioCall = 0,
            OfferVideoCall = 1,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::OfferAudioCall => "OFFER_AUDIO_CALL",
                    Type::OfferVideoCall => "OFFER_VIDEO_CALL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "OFFER_AUDIO_CALL" => Some(Self::OfferAudioCall),
                    "OFFER_VIDEO_CALL" => Some(Self::OfferVideoCall),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Answer {
        #[prost(uint64, optional, tag = "1")]
        pub id: ::core::option::Option<u64>,
        /// Legacy/deprecated; replaced by 'opaque'
        #[prost(string, optional, tag = "2")]
        pub sdp: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub opaque: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IceUpdate {
        #[prost(uint64, optional, tag = "1")]
        pub id: ::core::option::Option<u64>,
        /// Legacy/deprecated; remove when old clients are gone.
        #[prost(string, optional, tag = "2")]
        pub mid: ::core::option::Option<::prost::alloc::string::String>,
        /// Legacy/deprecated; remove when old clients are gone.
        #[prost(uint32, optional, tag = "3")]
        pub line: ::core::option::Option<u32>,
        /// Legacy/deprecated; replaced by 'opaque'
        #[prost(string, optional, tag = "4")]
        pub sdp: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "5")]
        pub opaque: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Busy {
        #[prost(uint64, optional, tag = "1")]
        pub id: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Hangup {
        #[prost(uint64, optional, tag = "1")]
        pub id: ::core::option::Option<u64>,
        #[prost(enumeration = "hangup::Type", optional, tag = "2")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3")]
        pub device_id: ::core::option::Option<u32>,
    }
    /// Nested message and enum types in `Hangup`.
    pub mod hangup {
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
            HangupNormal = 0,
            HangupAccepted = 1,
            HangupDeclined = 2,
            HangupBusy = 3,
            HangupNeedPermission = 4,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::HangupNormal => "HANGUP_NORMAL",
                    Type::HangupAccepted => "HANGUP_ACCEPTED",
                    Type::HangupDeclined => "HANGUP_DECLINED",
                    Type::HangupBusy => "HANGUP_BUSY",
                    Type::HangupNeedPermission => "HANGUP_NEED_PERMISSION",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "HANGUP_NORMAL" => Some(Self::HangupNormal),
                    "HANGUP_ACCEPTED" => Some(Self::HangupAccepted),
                    "HANGUP_DECLINED" => Some(Self::HangupDeclined),
                    "HANGUP_BUSY" => Some(Self::HangupBusy),
                    "HANGUP_NEED_PERMISSION" => Some(Self::HangupNeedPermission),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Opaque {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(enumeration = "opaque::Urgency", optional, tag = "2")]
        pub urgency: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `Opaque`.
    pub mod opaque {
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
        pub enum Urgency {
            Droppable = 0,
            HandleImmediately = 1,
        }
        impl Urgency {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Urgency::Droppable => "DROPPABLE",
                    Urgency::HandleImmediately => "HANDLE_IMMEDIATELY",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DROPPABLE" => Some(Self::Droppable),
                    "HANDLE_IMMEDIATELY" => Some(Self::HandleImmediately),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BodyRange {
    #[prost(uint32, optional, tag = "1")]
    pub start: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub length: ::core::option::Option<u32>,
    #[prost(oneof = "body_range::AssociatedValue", tags = "3, 4")]
    pub associated_value: ::core::option::Option<body_range::AssociatedValue>,
}
/// Nested message and enum types in `BodyRange`.
pub mod body_range {
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
    pub enum Style {
        None = 0,
        Bold = 1,
        Italic = 2,
        Spoiler = 3,
        Strikethrough = 4,
        Monospace = 5,
    }
    impl Style {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Style::None => "NONE",
                Style::Bold => "BOLD",
                Style::Italic => "ITALIC",
                Style::Spoiler => "SPOILER",
                Style::Strikethrough => "STRIKETHROUGH",
                Style::Monospace => "MONOSPACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "BOLD" => Some(Self::Bold),
                "ITALIC" => Some(Self::Italic),
                "SPOILER" => Some(Self::Spoiler),
                "STRIKETHROUGH" => Some(Self::Strikethrough),
                "MONOSPACE" => Some(Self::Monospace),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AssociatedValue {
        #[prost(string, tag = "3")]
        MentionAci(::prost::alloc::string::String),
        #[prost(enumeration = "Style", tag = "4")]
        Style(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataMessage {
    #[prost(string, optional, tag = "1")]
    pub body: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub attachments: ::prost::alloc::vec::Vec<AttachmentPointer>,
    #[prost(message, optional, tag = "15")]
    pub group_v2: ::core::option::Option<GroupContextV2>,
    #[prost(uint32, optional, tag = "4")]
    pub flags: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub expire_timer: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub profile_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "7")]
    pub timestamp: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "8")]
    pub quote: ::core::option::Option<data_message::Quote>,
    #[prost(message, repeated, tag = "9")]
    pub contact: ::prost::alloc::vec::Vec<data_message::Contact>,
    #[prost(message, repeated, tag = "10")]
    pub preview: ::prost::alloc::vec::Vec<Preview>,
    #[prost(message, optional, tag = "11")]
    pub sticker: ::core::option::Option<data_message::Sticker>,
    #[prost(uint32, optional, tag = "12")]
    pub required_protocol_version: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "14")]
    pub is_view_once: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "16")]
    pub reaction: ::core::option::Option<data_message::Reaction>,
    #[prost(message, optional, tag = "17")]
    pub delete: ::core::option::Option<data_message::Delete>,
    #[prost(message, repeated, tag = "18")]
    pub body_ranges: ::prost::alloc::vec::Vec<BodyRange>,
    #[prost(message, optional, tag = "19")]
    pub group_call_update: ::core::option::Option<data_message::GroupCallUpdate>,
    #[prost(message, optional, tag = "20")]
    pub payment: ::core::option::Option<data_message::Payment>,
    #[prost(message, optional, tag = "21")]
    pub story_context: ::core::option::Option<data_message::StoryContext>,
    #[prost(message, optional, tag = "22")]
    pub gift_badge: ::core::option::Option<data_message::GiftBadge>,
}
/// Nested message and enum types in `DataMessage`.
pub mod data_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Quote {
        #[prost(uint64, optional, tag = "1")]
        pub id: ::core::option::Option<u64>,
        #[prost(string, optional, tag = "5")]
        pub author_aci: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub text: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "4")]
        pub attachments: ::prost::alloc::vec::Vec<quote::QuotedAttachment>,
        #[prost(message, repeated, tag = "6")]
        pub body_ranges: ::prost::alloc::vec::Vec<super::BodyRange>,
        #[prost(enumeration = "quote::Type", optional, tag = "7")]
        pub r#type: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `Quote`.
    pub mod quote {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QuotedAttachment {
            #[prost(string, optional, tag = "1")]
            pub content_type: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "2")]
            pub file_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(message, optional, tag = "3")]
            pub thumbnail: ::core::option::Option<super::super::AttachmentPointer>,
        }
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
            Normal = 0,
            GiftBadge = 1,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Normal => "NORMAL",
                    Type::GiftBadge => "GIFT_BADGE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NORMAL" => Some(Self::Normal),
                    "GIFT_BADGE" => Some(Self::GiftBadge),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contact {
        #[prost(message, optional, tag = "1")]
        pub name: ::core::option::Option<contact::Name>,
        #[prost(message, repeated, tag = "3")]
        pub number: ::prost::alloc::vec::Vec<contact::Phone>,
        #[prost(message, repeated, tag = "4")]
        pub email: ::prost::alloc::vec::Vec<contact::Email>,
        #[prost(message, repeated, tag = "5")]
        pub address: ::prost::alloc::vec::Vec<contact::PostalAddress>,
        #[prost(message, optional, tag = "6")]
        pub avatar: ::core::option::Option<contact::Avatar>,
        #[prost(string, optional, tag = "7")]
        pub organization: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `Contact`.
    pub mod contact {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Name {
            #[prost(string, optional, tag = "1")]
            pub given_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "2")]
            pub family_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "3")]
            pub prefix: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "4")]
            pub suffix: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "5")]
            pub middle_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "6")]
            pub display_name: ::core::option::Option<::prost::alloc::string::String>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Phone {
            #[prost(string, optional, tag = "1")]
            pub value: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(enumeration = "phone::Type", optional, tag = "2")]
            pub r#type: ::core::option::Option<i32>,
            #[prost(string, optional, tag = "3")]
            pub label: ::core::option::Option<::prost::alloc::string::String>,
        }
        /// Nested message and enum types in `Phone`.
        pub mod phone {
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
                Home = 1,
                Mobile = 2,
                Work = 3,
                Custom = 4,
            }
            impl Type {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Type::Home => "HOME",
                        Type::Mobile => "MOBILE",
                        Type::Work => "WORK",
                        Type::Custom => "CUSTOM",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "HOME" => Some(Self::Home),
                        "MOBILE" => Some(Self::Mobile),
                        "WORK" => Some(Self::Work),
                        "CUSTOM" => Some(Self::Custom),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Email {
            #[prost(string, optional, tag = "1")]
            pub value: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(enumeration = "email::Type", optional, tag = "2")]
            pub r#type: ::core::option::Option<i32>,
            #[prost(string, optional, tag = "3")]
            pub label: ::core::option::Option<::prost::alloc::string::String>,
        }
        /// Nested message and enum types in `Email`.
        pub mod email {
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
                Home = 1,
                Mobile = 2,
                Work = 3,
                Custom = 4,
            }
            impl Type {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Type::Home => "HOME",
                        Type::Mobile => "MOBILE",
                        Type::Work => "WORK",
                        Type::Custom => "CUSTOM",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "HOME" => Some(Self::Home),
                        "MOBILE" => Some(Self::Mobile),
                        "WORK" => Some(Self::Work),
                        "CUSTOM" => Some(Self::Custom),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PostalAddress {
            #[prost(enumeration = "postal_address::Type", optional, tag = "1")]
            pub r#type: ::core::option::Option<i32>,
            #[prost(string, optional, tag = "2")]
            pub label: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "3")]
            pub street: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "4")]
            pub pobox: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "5")]
            pub neighborhood: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "6")]
            pub city: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "7")]
            pub region: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "8")]
            pub postcode: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "9")]
            pub country: ::core::option::Option<::prost::alloc::string::String>,
        }
        /// Nested message and enum types in `PostalAddress`.
        pub mod postal_address {
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
                Home = 1,
                Work = 2,
                Custom = 3,
            }
            impl Type {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Type::Home => "HOME",
                        Type::Work => "WORK",
                        Type::Custom => "CUSTOM",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "HOME" => Some(Self::Home),
                        "WORK" => Some(Self::Work),
                        "CUSTOM" => Some(Self::Custom),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Avatar {
            #[prost(message, optional, tag = "1")]
            pub avatar: ::core::option::Option<super::super::AttachmentPointer>,
            #[prost(bool, optional, tag = "2")]
            pub is_profile: ::core::option::Option<bool>,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sticker {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub pack_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub pack_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(uint32, optional, tag = "3")]
        pub sticker_id: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "4")]
        pub data: ::core::option::Option<super::AttachmentPointer>,
        #[prost(string, optional, tag = "5")]
        pub emoji: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reaction {
        #[prost(string, optional, tag = "1")]
        pub emoji: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "2")]
        pub remove: ::core::option::Option<bool>,
        #[prost(string, optional, tag = "4")]
        pub target_author_aci: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint64, optional, tag = "5")]
        pub target_sent_timestamp: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Delete {
        #[prost(uint64, optional, tag = "1")]
        pub target_sent_timestamp: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupCallUpdate {
        #[prost(string, optional, tag = "1")]
        pub era_id: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StoryContext {
        #[prost(string, optional, tag = "1")]
        pub author_aci: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint64, optional, tag = "2")]
        pub sent_timestamp: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Payment {
        #[prost(oneof = "payment::Item", tags = "1, 2")]
        pub item: ::core::option::Option<payment::Item>,
    }
    /// Nested message and enum types in `Payment`.
    pub mod payment {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Amount {
            #[prost(oneof = "amount::Amount", tags = "1")]
            pub amount: ::core::option::Option<amount::Amount>,
        }
        /// Nested message and enum types in `Amount`.
        pub mod amount {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct MobileCoin {
                #[prost(uint64, optional, tag = "1")]
                pub pico_mob: ::core::option::Option<u64>,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Amount {
                #[prost(message, tag = "1")]
                MobileCoin(MobileCoin),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Notification {
            #[prost(string, optional, tag = "2")]
            pub note: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(oneof = "notification::Transaction", tags = "1")]
            pub transaction: ::core::option::Option<notification::Transaction>,
        }
        /// Nested message and enum types in `Notification`.
        pub mod notification {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct MobileCoin {
                #[prost(bytes = "vec", optional, tag = "1")]
                pub receipt: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Transaction {
                #[prost(message, tag = "1")]
                MobileCoin(MobileCoin),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Activation {
            #[prost(enumeration = "activation::Type", optional, tag = "1")]
            pub r#type: ::core::option::Option<i32>,
        }
        /// Nested message and enum types in `Activation`.
        pub mod activation {
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
                Request = 0,
                Activated = 1,
            }
            impl Type {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Type::Request => "REQUEST",
                        Type::Activated => "ACTIVATED",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "REQUEST" => Some(Self::Request),
                        "ACTIVATED" => Some(Self::Activated),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Item {
            #[prost(message, tag = "1")]
            Notification(Notification),
            #[prost(message, tag = "2")]
            Activation(Activation),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GiftBadge {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub receipt_credential_presentation: ::core::option::Option<
            ::prost::alloc::vec::Vec<u8>,
        >,
    }
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
    pub enum Flags {
        EndSession = 1,
        ExpirationTimerUpdate = 2,
        ProfileKeyUpdate = 4,
    }
    impl Flags {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Flags::EndSession => "END_SESSION",
                Flags::ExpirationTimerUpdate => "EXPIRATION_TIMER_UPDATE",
                Flags::ProfileKeyUpdate => "PROFILE_KEY_UPDATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "END_SESSION" => Some(Self::EndSession),
                "EXPIRATION_TIMER_UPDATE" => Some(Self::ExpirationTimerUpdate),
                "PROFILE_KEY_UPDATE" => Some(Self::ProfileKeyUpdate),
                _ => None,
            }
        }
    }
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
    pub enum ProtocolVersion {
        Initial = 0,
        MessageTimers = 1,
        ViewOnce = 2,
        ViewOnceVideo = 3,
        Reactions = 4,
        CdnSelectorAttachments = 5,
        Mentions = 6,
        Payments = 7,
    }
    impl ProtocolVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProtocolVersion::Initial => "INITIAL",
                ProtocolVersion::MessageTimers => "MESSAGE_TIMERS",
                ProtocolVersion::ViewOnce => "VIEW_ONCE",
                ProtocolVersion::ViewOnceVideo => "VIEW_ONCE_VIDEO",
                ProtocolVersion::Reactions => "REACTIONS",
                ProtocolVersion::CdnSelectorAttachments => "CDN_SELECTOR_ATTACHMENTS",
                ProtocolVersion::Mentions => "MENTIONS",
                ProtocolVersion::Payments => "PAYMENTS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INITIAL" => Some(Self::Initial),
                "MESSAGE_TIMERS" => Some(Self::MessageTimers),
                "VIEW_ONCE" => Some(Self::ViewOnce),
                "VIEW_ONCE_VIDEO" => Some(Self::ViewOnceVideo),
                "REACTIONS" => Some(Self::Reactions),
                "CDN_SELECTOR_ATTACHMENTS" => Some(Self::CdnSelectorAttachments),
                "MENTIONS" => Some(Self::Mentions),
                "PAYMENTS" => Some(Self::Payments),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullMessage {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub padding: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiptMessage {
    #[prost(enumeration = "receipt_message::Type", optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub timestamp: ::prost::alloc::vec::Vec<u64>,
}
/// Nested message and enum types in `ReceiptMessage`.
pub mod receipt_message {
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
        Delivery = 0,
        Read = 1,
        Viewed = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Delivery => "DELIVERY",
                Type::Read => "READ",
                Type::Viewed => "VIEWED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DELIVERY" => Some(Self::Delivery),
                "READ" => Some(Self::Read),
                "VIEWED" => Some(Self::Viewed),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypingMessage {
    #[prost(uint64, optional, tag = "1")]
    pub timestamp: ::core::option::Option<u64>,
    #[prost(enumeration = "typing_message::Action", optional, tag = "2")]
    pub action: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub group_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `TypingMessage`.
pub mod typing_message {
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
    pub enum Action {
        Started = 0,
        Stopped = 1,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Started => "STARTED",
                Action::Stopped => "STOPPED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STARTED" => Some(Self::Started),
                "STOPPED" => Some(Self::Stopped),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoryMessage {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub profile_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<GroupContextV2>,
    #[prost(bool, optional, tag = "5")]
    pub allows_replies: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "6")]
    pub body_ranges: ::prost::alloc::vec::Vec<BodyRange>,
    #[prost(oneof = "story_message::Attachment", tags = "3, 4")]
    pub attachment: ::core::option::Option<story_message::Attachment>,
}
/// Nested message and enum types in `StoryMessage`.
pub mod story_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Attachment {
        #[prost(message, tag = "3")]
        FileAttachment(super::AttachmentPointer),
        #[prost(message, tag = "4")]
        TextAttachment(super::TextAttachment),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Preview {
    #[prost(string, optional, tag = "1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub image: ::core::option::Option<AttachmentPointer>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "5")]
    pub date: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAttachment {
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "text_attachment::Style", optional, tag = "2")]
    pub text_style: ::core::option::Option<i32>,
    /// integer representation of hex color
    #[prost(uint32, optional, tag = "3")]
    pub text_foreground_color: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub text_background_color: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub preview: ::core::option::Option<Preview>,
    #[prost(oneof = "text_attachment::Background", tags = "6, 7")]
    pub background: ::core::option::Option<text_attachment::Background>,
}
/// Nested message and enum types in `TextAttachment`.
pub mod text_attachment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Gradient {
        /// deprecated: this field will be removed in a future release.
        #[prost(uint32, optional, tag = "1")]
        pub start_color: ::core::option::Option<u32>,
        /// deprecated: this field will be removed in a future release.
        #[prost(uint32, optional, tag = "2")]
        pub end_color: ::core::option::Option<u32>,
        /// degrees
        #[prost(uint32, optional, tag = "3")]
        pub angle: ::core::option::Option<u32>,
        #[prost(uint32, repeated, packed = "false", tag = "4")]
        pub colors: ::prost::alloc::vec::Vec<u32>,
        /// percent from 0 to 1
        #[prost(float, repeated, packed = "false", tag = "5")]
        pub positions: ::prost::alloc::vec::Vec<f32>,
    }
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
    pub enum Style {
        Default = 0,
        Regular = 1,
        Bold = 2,
        Serif = 3,
        Script = 4,
        Condensed = 5,
    }
    impl Style {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Style::Default => "DEFAULT",
                Style::Regular => "REGULAR",
                Style::Bold => "BOLD",
                Style::Serif => "SERIF",
                Style::Script => "SCRIPT",
                Style::Condensed => "CONDENSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "REGULAR" => Some(Self::Regular),
                "BOLD" => Some(Self::Bold),
                "SERIF" => Some(Self::Serif),
                "SCRIPT" => Some(Self::Script),
                "CONDENSED" => Some(Self::Condensed),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Background {
        #[prost(message, tag = "6")]
        Gradient(Gradient),
        #[prost(uint32, tag = "7")]
        Color(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Verified {
    #[prost(string, optional, tag = "5")]
    pub destination_aci: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub identity_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "verified::State", optional, tag = "3")]
    pub state: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub null_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `Verified`.
pub mod verified {
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
    pub enum State {
        Default = 0,
        Verified = 1,
        Unverified = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Default => "DEFAULT",
                State::Verified => "VERIFIED",
                State::Unverified => "UNVERIFIED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "VERIFIED" => Some(Self::Verified),
                "UNVERIFIED" => Some(Self::Unverified),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncMessage {
    #[prost(message, optional, tag = "1")]
    pub sent: ::core::option::Option<sync_message::Sent>,
    #[prost(message, optional, tag = "2")]
    pub contacts: ::core::option::Option<sync_message::Contacts>,
    #[prost(message, optional, tag = "4")]
    pub request: ::core::option::Option<sync_message::Request>,
    #[prost(message, repeated, tag = "5")]
    pub read: ::prost::alloc::vec::Vec<sync_message::Read>,
    #[prost(message, optional, tag = "6")]
    pub blocked: ::core::option::Option<sync_message::Blocked>,
    #[prost(message, optional, tag = "7")]
    pub verified: ::core::option::Option<Verified>,
    #[prost(message, optional, tag = "9")]
    pub configuration: ::core::option::Option<sync_message::Configuration>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub padding: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "10")]
    pub sticker_pack_operation: ::prost::alloc::vec::Vec<
        sync_message::StickerPackOperation,
    >,
    #[prost(message, optional, tag = "11")]
    pub view_once_open: ::core::option::Option<sync_message::ViewOnceOpen>,
    #[prost(message, optional, tag = "12")]
    pub fetch_latest: ::core::option::Option<sync_message::FetchLatest>,
    #[prost(message, optional, tag = "13")]
    pub keys: ::core::option::Option<sync_message::Keys>,
    #[prost(message, optional, tag = "14")]
    pub message_request_response: ::core::option::Option<
        sync_message::MessageRequestResponse,
    >,
    #[prost(message, optional, tag = "15")]
    pub outgoing_payment: ::core::option::Option<sync_message::OutgoingPayment>,
    #[prost(message, repeated, tag = "16")]
    pub viewed: ::prost::alloc::vec::Vec<sync_message::Viewed>,
    #[prost(message, optional, tag = "18")]
    pub pni_change_number: ::core::option::Option<sync_message::PniChangeNumber>,
    #[prost(message, optional, tag = "19")]
    pub call_event: ::core::option::Option<sync_message::CallEvent>,
    #[prost(message, optional, tag = "20")]
    pub call_link_update: ::core::option::Option<sync_message::CallLinkUpdate>,
    #[prost(message, optional, tag = "21")]
    pub call_log_event: ::core::option::Option<sync_message::CallLogEvent>,
}
/// Nested message and enum types in `SyncMessage`.
pub mod sync_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sent {
        #[prost(string, optional, tag = "1")]
        pub destination_e164: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "7")]
        pub destination_service_id: ::core::option::Option<
            ::prost::alloc::string::String,
        >,
        #[prost(uint64, optional, tag = "2")]
        pub timestamp: ::core::option::Option<u64>,
        #[prost(message, optional, tag = "3")]
        pub message: ::core::option::Option<super::DataMessage>,
        #[prost(uint64, optional, tag = "4")]
        pub expiration_start_timestamp: ::core::option::Option<u64>,
        #[prost(message, repeated, tag = "5")]
        pub unidentified_status: ::prost::alloc::vec::Vec<
            sent::UnidentifiedDeliveryStatus,
        >,
        #[prost(bool, optional, tag = "6", default = "false")]
        pub is_recipient_update: ::core::option::Option<bool>,
        #[prost(message, optional, tag = "8")]
        pub story_message: ::core::option::Option<super::StoryMessage>,
        #[prost(message, repeated, tag = "9")]
        pub story_message_recipients: ::prost::alloc::vec::Vec<
            sent::StoryMessageRecipient,
        >,
        #[prost(message, optional, tag = "10")]
        pub edit_message: ::core::option::Option<super::EditMessage>,
    }
    /// Nested message and enum types in `Sent`.
    pub mod sent {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UnidentifiedDeliveryStatus {
            #[prost(string, optional, tag = "3")]
            pub destination_service_id: ::core::option::Option<
                ::prost::alloc::string::String,
            >,
            #[prost(bool, optional, tag = "2")]
            pub unidentified: ::core::option::Option<bool>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StoryMessageRecipient {
            #[prost(string, optional, tag = "1")]
            pub destination_service_id: ::core::option::Option<
                ::prost::alloc::string::String,
            >,
            #[prost(string, repeated, tag = "2")]
            pub distribution_list_ids: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            #[prost(bool, optional, tag = "3")]
            pub is_allowed_to_reply: ::core::option::Option<bool>,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contacts {
        #[prost(message, optional, tag = "1")]
        pub blob: ::core::option::Option<super::AttachmentPointer>,
        #[prost(bool, optional, tag = "2", default = "false")]
        pub complete: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Blocked {
        #[prost(string, repeated, tag = "1")]
        pub numbers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "3")]
        pub acis: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub group_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(enumeration = "request::Type", optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
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
            Contacts = 1,
            ///       GROUPS        = 2;
            Blocked = 3,
            Configuration = 4,
            Keys = 5,
            PniIdentity = 6,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unknown => "UNKNOWN",
                    Type::Contacts => "CONTACTS",
                    Type::Blocked => "BLOCKED",
                    Type::Configuration => "CONFIGURATION",
                    Type::Keys => "KEYS",
                    Type::PniIdentity => "PNI_IDENTITY",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "CONTACTS" => Some(Self::Contacts),
                    "BLOCKED" => Some(Self::Blocked),
                    "CONFIGURATION" => Some(Self::Configuration),
                    "KEYS" => Some(Self::Keys),
                    "PNI_IDENTITY" => Some(Self::PniIdentity),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Read {
        #[prost(string, optional, tag = "3")]
        pub sender_aci: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint64, optional, tag = "2")]
        pub timestamp: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Viewed {
        #[prost(string, optional, tag = "3")]
        pub sender_aci: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint64, optional, tag = "2")]
        pub timestamp: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Configuration {
        #[prost(bool, optional, tag = "1")]
        pub read_receipts: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "2")]
        pub unidentified_delivery_indicators: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "3")]
        pub typing_indicators: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "5")]
        pub provisioning_version: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "6")]
        pub link_previews: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StickerPackOperation {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub pack_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub pack_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(enumeration = "sticker_pack_operation::Type", optional, tag = "3")]
        pub r#type: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `StickerPackOperation`.
    pub mod sticker_pack_operation {
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
            Install = 0,
            Remove = 1,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Install => "INSTALL",
                    Type::Remove => "REMOVE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "INSTALL" => Some(Self::Install),
                    "REMOVE" => Some(Self::Remove),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ViewOnceOpen {
        #[prost(string, optional, tag = "3")]
        pub sender_aci: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint64, optional, tag = "2")]
        pub timestamp: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FetchLatest {
        #[prost(enumeration = "fetch_latest::Type", optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `FetchLatest`.
    pub mod fetch_latest {
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
            LocalProfile = 1,
            StorageManifest = 2,
            SubscriptionStatus = 3,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unknown => "UNKNOWN",
                    Type::LocalProfile => "LOCAL_PROFILE",
                    Type::StorageManifest => "STORAGE_MANIFEST",
                    Type::SubscriptionStatus => "SUBSCRIPTION_STATUS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "LOCAL_PROFILE" => Some(Self::LocalProfile),
                    "STORAGE_MANIFEST" => Some(Self::StorageManifest),
                    "SUBSCRIPTION_STATUS" => Some(Self::SubscriptionStatus),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Keys {
        /// @deprecated
        #[prost(bytes = "vec", optional, tag = "1")]
        pub storage_service: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub master: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MessageRequestResponse {
        #[prost(string, optional, tag = "2")]
        pub thread_aci: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub group_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(enumeration = "message_request_response::Type", optional, tag = "4")]
        pub r#type: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `MessageRequestResponse`.
    pub mod message_request_response {
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
            Accept = 1,
            Delete = 2,
            Block = 3,
            BlockAndDelete = 4,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unknown => "UNKNOWN",
                    Type::Accept => "ACCEPT",
                    Type::Delete => "DELETE",
                    Type::Block => "BLOCK",
                    Type::BlockAndDelete => "BLOCK_AND_DELETE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "ACCEPT" => Some(Self::Accept),
                    "DELETE" => Some(Self::Delete),
                    "BLOCK" => Some(Self::Block),
                    "BLOCK_AND_DELETE" => Some(Self::BlockAndDelete),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutgoingPayment {
        #[prost(string, optional, tag = "1")]
        pub recipient_service_id: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub note: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(oneof = "outgoing_payment::PaymentDetail", tags = "3")]
        pub payment_detail: ::core::option::Option<outgoing_payment::PaymentDetail>,
    }
    /// Nested message and enum types in `OutgoingPayment`.
    pub mod outgoing_payment {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MobileCoin {
            #[prost(bytes = "vec", optional, tag = "1")]
            pub recipient_address: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            /// @required
            #[prost(uint64, optional, tag = "2")]
            pub amount_pico_mob: ::core::option::Option<u64>,
            /// @required
            #[prost(uint64, optional, tag = "3")]
            pub fee_pico_mob: ::core::option::Option<u64>,
            #[prost(bytes = "vec", optional, tag = "4")]
            pub receipt: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(uint64, optional, tag = "5")]
            pub ledger_block_timestamp: ::core::option::Option<u64>,
            /// @required
            #[prost(uint64, optional, tag = "6")]
            pub ledger_block_index: ::core::option::Option<u64>,
            #[prost(bytes = "vec", repeated, tag = "7")]
            pub spent_key_images: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
            #[prost(bytes = "vec", repeated, tag = "8")]
            pub output_public_keys: ::prost::alloc::vec::Vec<
                ::prost::alloc::vec::Vec<u8>,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PaymentDetail {
            #[prost(message, tag = "3")]
            MobileCoin(MobileCoin),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PniChangeNumber {
        /// Serialized libsignal-client IdentityKeyPair
        #[prost(bytes = "vec", optional, tag = "1")]
        pub identity_key_pair: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        /// Serialized libsignal-client SignedPreKeyRecord
        #[prost(bytes = "vec", optional, tag = "2")]
        pub signed_pre_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        /// Serialized libsignal-client KyberPreKeyRecord
        #[prost(bytes = "vec", optional, tag = "5")]
        pub last_resort_kyber_pre_key: ::core::option::Option<
            ::prost::alloc::vec::Vec<u8>,
        >,
        #[prost(uint32, optional, tag = "3")]
        pub registration_id: ::core::option::Option<u32>,
        /// The e164 we have changed our number to
        #[prost(string, optional, tag = "4")]
        pub new_e164: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallEvent {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub conversation_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(uint64, optional, tag = "2")]
        pub id: ::core::option::Option<u64>,
        #[prost(uint64, optional, tag = "3")]
        pub timestamp: ::core::option::Option<u64>,
        #[prost(enumeration = "call_event::Type", optional, tag = "4")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(enumeration = "call_event::Direction", optional, tag = "5")]
        pub direction: ::core::option::Option<i32>,
        #[prost(enumeration = "call_event::Event", optional, tag = "6")]
        pub event: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `CallEvent`.
    pub mod call_event {
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
            UnknownType = 0,
            AudioCall = 1,
            VideoCall = 2,
            GroupCall = 3,
            AdHocCall = 4,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::UnknownType => "UNKNOWN_TYPE",
                    Type::AudioCall => "AUDIO_CALL",
                    Type::VideoCall => "VIDEO_CALL",
                    Type::GroupCall => "GROUP_CALL",
                    Type::AdHocCall => "AD_HOC_CALL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN_TYPE" => Some(Self::UnknownType),
                    "AUDIO_CALL" => Some(Self::AudioCall),
                    "VIDEO_CALL" => Some(Self::VideoCall),
                    "GROUP_CALL" => Some(Self::GroupCall),
                    "AD_HOC_CALL" => Some(Self::AdHocCall),
                    _ => None,
                }
            }
        }
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
        pub enum Direction {
            UnknownDirection = 0,
            Incoming = 1,
            Outgoing = 2,
        }
        impl Direction {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Direction::UnknownDirection => "UNKNOWN_DIRECTION",
                    Direction::Incoming => "INCOMING",
                    Direction::Outgoing => "OUTGOING",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN_DIRECTION" => Some(Self::UnknownDirection),
                    "INCOMING" => Some(Self::Incoming),
                    "OUTGOING" => Some(Self::Outgoing),
                    _ => None,
                }
            }
        }
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
        pub enum Event {
            UnknownAction = 0,
            Accepted = 1,
            NotAccepted = 2,
            Delete = 3,
        }
        impl Event {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Event::UnknownAction => "UNKNOWN_ACTION",
                    Event::Accepted => "ACCEPTED",
                    Event::NotAccepted => "NOT_ACCEPTED",
                    Event::Delete => "DELETE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN_ACTION" => Some(Self::UnknownAction),
                    "ACCEPTED" => Some(Self::Accepted),
                    "NOT_ACCEPTED" => Some(Self::NotAccepted),
                    "DELETE" => Some(Self::Delete),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallLinkUpdate {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub root_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub admin_pass_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallLogEvent {
        #[prost(enumeration = "call_log_event::Type", optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(uint64, optional, tag = "2")]
        pub timestamp: ::core::option::Option<u64>,
    }
    /// Nested message and enum types in `CallLogEvent`.
    pub mod call_log_event {
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
            Clear = 0,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Clear => "CLEAR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CLEAR" => Some(Self::Clear),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachmentPointer {
    #[prost(string, optional, tag = "2")]
    pub content_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "4")]
    pub size: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub thumbnail: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub digest: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "18")]
    pub incremental_mac: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "17")]
    pub incremental_mac_chunk_size: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "7")]
    pub file_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "8")]
    pub flags: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub height: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "11")]
    pub caption: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub blur_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "13")]
    pub upload_timestamp: ::core::option::Option<u64>,
    /// Next ID: 19
    #[prost(uint32, optional, tag = "14")]
    pub cdn_number: ::core::option::Option<u32>,
    #[prost(oneof = "attachment_pointer::AttachmentIdentifier", tags = "1, 15")]
    pub attachment_identifier: ::core::option::Option<
        attachment_pointer::AttachmentIdentifier,
    >,
}
/// Nested message and enum types in `AttachmentPointer`.
pub mod attachment_pointer {
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
    pub enum Flags {
        VoiceMessage = 1,
        Borderless = 2,
        Gif = 4,
    }
    impl Flags {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Flags::VoiceMessage => "VOICE_MESSAGE",
                Flags::Borderless => "BORDERLESS",
                Flags::Gif => "GIF",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VOICE_MESSAGE" => Some(Self::VoiceMessage),
                "BORDERLESS" => Some(Self::Borderless),
                "GIF" => Some(Self::Gif),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AttachmentIdentifier {
        #[prost(fixed64, tag = "1")]
        CdnId(u64),
        #[prost(string, tag = "15")]
        CdnKey(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupContext {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "group_context::Type", optional, tag = "2")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub members_e164: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub members: ::prost::alloc::vec::Vec<group_context::Member>,
    #[prost(message, optional, tag = "5")]
    pub avatar: ::core::option::Option<AttachmentPointer>,
}
/// Nested message and enum types in `GroupContext`.
pub mod group_context {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Member {
        #[prost(string, optional, tag = "2")]
        pub e164: ::core::option::Option<::prost::alloc::string::String>,
    }
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
        Update = 1,
        Deliver = 2,
        Quit = 3,
        RequestInfo = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "UNKNOWN",
                Type::Update => "UPDATE",
                Type::Deliver => "DELIVER",
                Type::Quit => "QUIT",
                Type::RequestInfo => "REQUEST_INFO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "UPDATE" => Some(Self::Update),
                "DELIVER" => Some(Self::Deliver),
                "QUIT" => Some(Self::Quit),
                "REQUEST_INFO" => Some(Self::RequestInfo),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupContextV2 {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub master_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "2")]
    pub revision: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub group_change: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactDetails {
    #[prost(string, optional, tag = "1")]
    pub number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub aci: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub avatar: ::core::option::Option<contact_details::Avatar>,
    #[prost(string, optional, tag = "4")]
    pub color: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub verified: ::core::option::Option<Verified>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub profile_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "7")]
    pub blocked: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "8")]
    pub expire_timer: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub inbox_position: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "11")]
    pub archived: ::core::option::Option<bool>,
}
/// Nested message and enum types in `ContactDetails`.
pub mod contact_details {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Avatar {
        #[prost(string, optional, tag = "1")]
        pub content_type: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub length: ::core::option::Option<u32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub members_e164: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "9")]
    pub members: ::prost::alloc::vec::Vec<group_details::Member>,
    #[prost(message, optional, tag = "4")]
    pub avatar: ::core::option::Option<group_details::Avatar>,
    #[prost(bool, optional, tag = "5", default = "true")]
    pub active: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "6")]
    pub expire_timer: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "7")]
    pub color: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "8")]
    pub blocked: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "10")]
    pub inbox_position: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "11")]
    pub archived: ::core::option::Option<bool>,
}
/// Nested message and enum types in `GroupDetails`.
pub mod group_details {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Avatar {
        #[prost(string, optional, tag = "1")]
        pub content_type: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub length: ::core::option::Option<u32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Member {
        #[prost(string, optional, tag = "2")]
        pub e164: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAddress {
    #[prost(oneof = "payment_address::Address", tags = "1")]
    pub address: ::core::option::Option<payment_address::Address>,
}
/// Nested message and enum types in `PaymentAddress`.
pub mod payment_address {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MobileCoinAddress {
        #[prost(bytes = "vec", optional, tag = "1")]
        pub address: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Address {
        #[prost(message, tag = "1")]
        MobileCoinAddress(MobileCoinAddress),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptionErrorMessage {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub ratchet_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "2")]
    pub timestamp: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub device_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PniSignatureMessage {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub pni: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditMessage {
    #[prost(uint64, optional, tag = "1")]
    pub target_sent_timestamp: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "2")]
    pub data_message: ::core::option::Option<DataMessage>,
}
