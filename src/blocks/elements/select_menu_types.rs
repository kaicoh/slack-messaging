use serde::Serializer;
use std::marker::PhantomData;

/// Static option type for select menu element or multi select menu element.
#[derive(Debug, Default, Copy, Clone)]
pub struct StaticOptions;

/// External data source type for select menu element or multi select menu element.
#[derive(Debug, Default, Copy, Clone)]
pub struct ExternalDataSource;

/// User list type for select menu element or multi select menu element.
#[derive(Debug, Default, Copy, Clone)]
pub struct Users;

/// Conversation list type for select menu element or multi select menu element.
#[derive(Debug, Default, Copy, Clone)]
pub struct Conversations;

/// Public channel list type for select menu element or multi select menu element.
#[derive(Debug, Default, Copy, Clone)]
pub struct PublicChannels;

pub trait MultiSelectType {
    fn serialize<S>(ty: &PhantomData<Self>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

macro_rules! impl_multi_select_type {
    ($($ty:tt as $expr:tt),*) => {
        $(
            impl MultiSelectType for $ty {
                fn serialize<S>(_: &PhantomData<$ty>, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str($expr)
                }
            }
        )*
    };
}

impl_multi_select_type! {
    StaticOptions as "multi_static_select",
    ExternalDataSource as "multi_external_select",
    Users as "multi_users_select",
    Conversations as "multi_conversations_select",
    PublicChannels as "multi_channels_select"
}

pub trait SelectType {
    fn serialize<S>(ty: &PhantomData<Self>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

macro_rules! impl_select_type {
    ($($ty:tt as $expr:tt),*) => {
        $(
            impl SelectType for $ty {
                fn serialize<S>(_: &PhantomData<$ty>, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str($expr)
                }
            }
        )*
    };
}

impl_select_type! {
    StaticOptions as "static_select",
    ExternalDataSource as "external_select",
    Users as "users_select",
    Conversations as "conversations_select",
    PublicChannels as "channels_select"
}
