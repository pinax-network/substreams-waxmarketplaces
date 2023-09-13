// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AnyEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyEvent {
    #[prost(oneof="any_event::Event", tags="1, 2, 3, 4")]
    pub event: ::core::option::Option<any_event::Event>,
}
/// Nested message and enum types in `AnyEvent`.
pub mod any_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        Saleitem(super::SaleEvent),
        #[prost(message, tag="2")]
        Auctionitem(super::AuctionEvent),
        #[prost(message, tag="3")]
        Newbuyoitem(super::NewBuyo),
        #[prost(message, tag="4")]
        Purchasesaleitem(super::PurchaseSale),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaleEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SaleEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaleEvent {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// event type
    #[prost(string, tag="3")]
    pub event: ::prost::alloc::string::String,
    /// payload
    #[prost(string, tag="5")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="6")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="7")]
    pub listing_price: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub settlement_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub maker_marketplace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AuctionEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuctionEvent {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// event type
    #[prost(string, tag="3")]
    pub event: ::prost::alloc::string::String,
    /// payload
    #[prost(string, tag="5")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="6")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="7")]
    pub starting_bid: ::prost::alloc::string::String,
    #[prost(uint32, tag="8")]
    pub duration: u32,
    #[prost(string, tag="9")]
    pub maker_marketplace: ::prost::alloc::string::String,
}
/// Lognewbuyo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBuyo {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// event type
    #[prost(string, tag="3")]
    pub event: ::prost::alloc::string::String,
    /// payload
    #[prost(uint64, tag="4")]
    pub buyoffer_id: u64,
    #[prost(string, tag="5")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="8")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="9")]
    pub memo: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub maker_marketplace: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub collection_fee: ::prost::alloc::string::String,
}
// Acceptbuyo
// Auctionbid
// Createbuyo
// Lognewauct

/// Lognewsale
/// Purchasesale
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseSale {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    /// event type
    #[prost(string, tag="3")]
    pub event: ::prost::alloc::string::String,
    /// payload
    #[prost(string, tag="4")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub sale_id: u64,
    #[prost(uint64, tag="6")]
    pub intended_delphi_median: u64,
    #[prost(string, tag="7")]
    pub taker_marketplace: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
