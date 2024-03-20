#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "3")]
    pub created_at: i64,
    #[prost(uint32, tag = "4")]
    pub kind: u32,
    #[prost(message, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<TagArray>,
    #[prost(string, tag = "6")]
    pub content: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagArray {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// type = request, meta is event byt, type = sync , type = terminate
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VlcMsg {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub meta: ::prost::alloc::vec::Vec<u8>,
}
