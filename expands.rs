#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod solana {
    pub mod actions {
        mod create_associated_token_account {
            use std::{str::FromStr, sync::Arc};
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use solana_sdk::pubkey::Pubkey;
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `create_associated_token_account` action.
            pub struct CreateAssociatedTokenAccountArgs {
                #[schemars(description = "Wallet public key")]
                wallet: String,
                #[schemars(description = "Mint public key")]
                mint_pubkey: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateAssociatedTokenAccountArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "wallet" => _serde::__private::Ok(__Field::__field0),
                                    "mint_pubkey" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"wallet" => _serde::__private::Ok(__Field::__field0),
                                    b"mint_pubkey" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<
                                CreateAssociatedTokenAccountArgs,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateAssociatedTokenAccountArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateAssociatedTokenAccountArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CreateAssociatedTokenAccountArgs with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CreateAssociatedTokenAccountArgs with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateAssociatedTokenAccountArgs {
                                    wallet: __field0,
                                    mint_pubkey: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("wallet"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "mint_pubkey",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("wallet")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("mint_pubkey")?
                                    }
                                };
                                _serde::__private::Ok(CreateAssociatedTokenAccountArgs {
                                    wallet: __field0,
                                    mint_pubkey: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "wallet",
                            "mint_pubkey",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateAssociatedTokenAccountArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    CreateAssociatedTokenAccountArgs,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateAssociatedTokenAccountArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateAssociatedTokenAccountArgs",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "wallet",
                            &self.wallet,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mint_pubkey",
                            &self.mint_pubkey,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for CreateAssociatedTokenAccountArgs {
                    fn schema_name() -> std::string::String {
                        "CreateAssociatedTokenAccountArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::create_associated_token_account::CreateAssociatedTokenAccountArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "wallet",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Wallet public key",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "mint_pubkey",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Mint public key",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `create_associated_token_account` action.",
                        )
                    }
                }
            };
            /// Response for the `create_associated_token_account` action.
            pub struct CreateAssociatedTokenAccountResponse {
                #[schemars(description = "Associated token account public key")]
                ata_pubkey: String,
                #[schemars(description = "Transaction signature")]
                signature: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de>
                for CreateAssociatedTokenAccountResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "ata_pubkey" => _serde::__private::Ok(__Field::__field0),
                                    "signature" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"ata_pubkey" => _serde::__private::Ok(__Field::__field0),
                                    b"signature" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<
                                CreateAssociatedTokenAccountResponse,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateAssociatedTokenAccountResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateAssociatedTokenAccountResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CreateAssociatedTokenAccountResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CreateAssociatedTokenAccountResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateAssociatedTokenAccountResponse {
                                    ata_pubkey: __field0,
                                    signature: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "ata_pubkey",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "signature",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("ata_pubkey")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("signature")?
                                    }
                                };
                                _serde::__private::Ok(CreateAssociatedTokenAccountResponse {
                                    ata_pubkey: __field0,
                                    signature: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "ata_pubkey",
                            "signature",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateAssociatedTokenAccountResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    CreateAssociatedTokenAccountResponse,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateAssociatedTokenAccountResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateAssociatedTokenAccountResponse",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "ata_pubkey",
                            &self.ata_pubkey,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "signature",
                            &self.signature,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for CreateAssociatedTokenAccountResponse {
                    fn schema_name() -> std::string::String {
                        "CreateAssociatedTokenAccountResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::create_associated_token_account::CreateAssociatedTokenAccountResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "ata_pubkey",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Associated token account public key",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "signature",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Transaction signature",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `create_associated_token_account` action.",
                        )
                    }
                }
            };
            pub struct CreateAssociatedTokenAccount {
                context: Arc<SolanaRpcClient>,
            }
            impl CreateAssociatedTokenAccount {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    args: CreateAssociatedTokenAccountArgs,
                ) -> Result<CreateAssociatedTokenAccountResponse, yart::ToolError> {
                    {
                        let mint = Pubkey::from_str(&args.mint_pubkey)
                            .map_err(|e| ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Invalid mint pubkey: {0}", e),
                                    );
                                    res
                                }),
                            ))?;
                        let wallet = Pubkey::from_str(&args.wallet)
                            .map_err(|e| ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Invalid wallet pubkey: {0}", e),
                                    );
                                    res
                                }),
                            ))?;
                        let (signature, ata_pubkey) = ctx
                            .create_associated_token_account(&wallet, &mint)
                            .await?;
                        Ok(CreateAssociatedTokenAccountResponse {
                            signature: signature.to_string(),
                            ata_pubkey: ata_pubkey.to_string(),
                        })
                    }
                }
            }
            impl rig::tool::Tool for CreateAssociatedTokenAccount {
                const NAME: &'static str = "create_associated_token_account";
                type Error = yart::ToolError;
                type Args = CreateAssociatedTokenAccountArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Create an associated token account for a mint"
                            .to_string(),
                        parameters: yart::derive_parameters::<
                            CreateAssociatedTokenAccountArgs,
                        >(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            CreateAssociatedTokenAccount::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod create_mint {
            use std::{str::FromStr, sync::Arc};
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use solana_sdk::pubkey::Pubkey;
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `create_mint` action.
            pub struct CreateMintArgs {
                #[schemars(
                    description = "Number of decimal places (0-255)",
                    with = "i32"
                )]
                decimals: u8,
                #[schemars(description = "Optional mint authority public key")]
                authority: Option<String>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateMintArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "decimals" => _serde::__private::Ok(__Field::__field0),
                                    "authority" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"decimals" => _serde::__private::Ok(__Field::__field0),
                                    b"authority" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<CreateMintArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateMintArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateMintArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    u8,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CreateMintArgs with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CreateMintArgs with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateMintArgs {
                                    decimals: __field0,
                                    authority: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Option<String>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "decimals",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "authority",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<String>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("decimals")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("authority")?
                                    }
                                };
                                _serde::__private::Ok(CreateMintArgs {
                                    decimals: __field0,
                                    authority: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "decimals",
                            "authority",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateMintArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CreateMintArgs>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateMintArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateMintArgs",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "decimals",
                            &self.decimals,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "authority",
                            &self.authority,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for CreateMintArgs {
                    fn schema_name() -> std::string::String {
                        "CreateMintArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::create_mint::CreateMintArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        i32,
                                    >(
                                        object_validation,
                                        "decimals",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<i32>(),
                                            "Number of decimal places (0-255)",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        Option<String>,
                                    >(
                                        object_validation,
                                        "authority",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<Option<String>>(),
                                            "Optional mint authority public key",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `create_mint` action.",
                        )
                    }
                }
            };
            /// Response for the `create_mint` action.
            pub struct CreateMintResponse {
                #[schemars(description = "Public key of the created mint")]
                mint_pubkey: String,
                #[schemars(description = "Transaction signature")]
                signature: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CreateMintResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "mint_pubkey" => _serde::__private::Ok(__Field::__field0),
                                    "signature" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"mint_pubkey" => _serde::__private::Ok(__Field::__field0),
                                    b"signature" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<CreateMintResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateMintResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateMintResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct CreateMintResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CreateMintResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateMintResponse {
                                    mint_pubkey: __field0,
                                    signature: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "mint_pubkey",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "signature",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("mint_pubkey")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("signature")?
                                    }
                                };
                                _serde::__private::Ok(CreateMintResponse {
                                    mint_pubkey: __field0,
                                    signature: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "mint_pubkey",
                            "signature",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateMintResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    CreateMintResponse,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateMintResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateMintResponse",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mint_pubkey",
                            &self.mint_pubkey,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "signature",
                            &self.signature,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for CreateMintResponse {
                    fn schema_name() -> std::string::String {
                        "CreateMintResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::create_mint::CreateMintResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "mint_pubkey",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Public key of the created mint",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "signature",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Transaction signature",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `create_mint` action.",
                        )
                    }
                }
            };
            pub struct CreateMint {
                context: Arc<SolanaRpcClient>,
            }
            impl CreateMint {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    args: CreateMintArgs,
                ) -> Result<CreateMintResponse, yart::ToolError> {
                    {
                        let authority = match args.authority {
                            Some(m) => {
                                Some(
                                    Pubkey::from_str(&m)
                                        .map_err(|e| ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Invalid mint authority: {0}", e),
                                                );
                                                res
                                            }),
                                        ))?,
                                )
                            }
                            None => None,
                        };
                        let (signature, mint_pubkey) = ctx
                            .create_mint(args.decimals, authority)
                            .await?;
                        Ok(CreateMintResponse {
                            signature: signature.to_string(),
                            mint_pubkey: mint_pubkey.to_string(),
                        })
                    }
                }
            }
            impl rig::tool::Tool for CreateMint {
                const NAME: &'static str = "create_mint";
                type Error = yart::ToolError;
                type Args = CreateMintArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Create a new SPL token mint".to_string(),
                        parameters: yart::derive_parameters::<CreateMintArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            CreateMint::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod get_balance {
            use std::{str::FromStr, sync::Arc};
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use solana_sdk::pubkey::Pubkey;
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `get_balance` action.
            pub struct GetBalanceArgs {
                #[schemars(
                    description = "Solana public key of mint account, null for SOL"
                )]
                mint_pubkey: Option<String>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetBalanceArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "mint_pubkey" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"mint_pubkey" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetBalanceArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetBalanceArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetBalanceArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetBalanceArgs with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetBalanceArgs {
                                    mint_pubkey: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    Option<String>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "mint_pubkey",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<String>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("mint_pubkey")?
                                    }
                                };
                                _serde::__private::Ok(GetBalanceArgs {
                                    mint_pubkey: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["mint_pubkey"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetBalanceArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<GetBalanceArgs>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetBalanceArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetBalanceArgs",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mint_pubkey",
                            &self.mint_pubkey,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetBalanceArgs {
                    fn schema_name() -> std::string::String {
                        "GetBalanceArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_balance::GetBalanceArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        Option<String>,
                                    >(
                                        object_validation,
                                        "mint_pubkey",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<Option<String>>(),
                                            "Solana public key of mint account, null for SOL",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `get_balance` action.",
                        )
                    }
                }
            };
            /// Response for the `get_balance` action.
            pub struct GetBalanceResponse {
                #[schemars(description = "Balance amount")]
                amount: String,
                #[schemars(description = "Number of decimal places", with = "i32")]
                decimal: u8,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetBalanceResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "amount" => _serde::__private::Ok(__Field::__field0),
                                    "decimal" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"amount" => _serde::__private::Ok(__Field::__field0),
                                    b"decimal" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetBalanceResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetBalanceResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetBalanceResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetBalanceResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    u8,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct GetBalanceResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetBalanceResponse {
                                    amount: __field0,
                                    decimal: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<u8> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "decimal",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("amount")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("decimal")?
                                    }
                                };
                                _serde::__private::Ok(GetBalanceResponse {
                                    amount: __field0,
                                    decimal: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["amount", "decimal"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetBalanceResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetBalanceResponse,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetBalanceResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetBalanceResponse",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "amount",
                            &self.amount,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "decimal",
                            &self.decimal,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetBalanceResponse {
                    fn schema_name() -> std::string::String {
                        "GetBalanceResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_balance::GetBalanceResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "amount",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Balance amount",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        i32,
                                    >(
                                        object_validation,
                                        "decimal",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<i32>(),
                                            "Number of decimal places",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `get_balance` action.",
                        )
                    }
                }
            };
            pub struct GetBalance {
                context: Arc<SolanaRpcClient>,
            }
            impl GetBalance {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    args: GetBalanceArgs,
                ) -> Result<GetBalanceResponse, yart::ToolError> {
                    {
                        let mint_pubkey = match args.mint_pubkey {
                            Some(m) => {
                                Some(
                                    Pubkey::from_str(&m)
                                        .map_err(|e| ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Invalid mint pubkey: {0}", e),
                                                );
                                                res
                                            }),
                                        ))?,
                                )
                            }
                            None => None,
                        };
                        let token_amount = ctx.get_balance(mint_pubkey).await?;
                        Ok(GetBalanceResponse {
                            amount: token_amount.amount.to_string(),
                            decimal: token_amount.decimals,
                        })
                    }
                }
            }
            impl rig::tool::Tool for GetBalance {
                const NAME: &'static str = "get_balance";
                type Error = yart::ToolError;
                type Args = GetBalanceArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Get wallet balance for SOL or SPL token"
                            .to_string(),
                        parameters: yart::derive_parameters::<GetBalanceArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            GetBalance::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod get_block_hash {
            use std::sync::Arc;
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `get_blockhash` action (empty).
            pub struct GetBlockhashArgs {}
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetBlockhashArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetBlockhashArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetBlockhashArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetBlockhashArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(GetBlockhashArgs {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(GetBlockhashArgs {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetBlockhashArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<GetBlockhashArgs>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetBlockhashArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetBlockhashArgs",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetBlockhashArgs {
                    fn schema_name() -> std::string::String {
                        "GetBlockhashArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_block_hash::GetBlockhashArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `get_blockhash` action (empty).",
                        )
                    }
                }
            };
            /// Response for the `get_blockhash` action.
            pub struct GetBlockhashResponse {
                #[schemars(description = "Latest blockhash")]
                blockhash: String,
                #[schemars(description = "Last valid block height")]
                last_valid_block_height: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetBlockhashResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "blockhash" => _serde::__private::Ok(__Field::__field0),
                                    "last_valid_block_height" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"blockhash" => _serde::__private::Ok(__Field::__field0),
                                    b"last_valid_block_height" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetBlockhashResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetBlockhashResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetBlockhashResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetBlockhashResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct GetBlockhashResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetBlockhashResponse {
                                    blockhash: __field0,
                                    last_valid_block_height: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "blockhash",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "last_valid_block_height",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("blockhash")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field(
                                            "last_valid_block_height",
                                        )?
                                    }
                                };
                                _serde::__private::Ok(GetBlockhashResponse {
                                    blockhash: __field0,
                                    last_valid_block_height: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "blockhash",
                            "last_valid_block_height",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetBlockhashResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetBlockhashResponse,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetBlockhashResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetBlockhashResponse",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "blockhash",
                            &self.blockhash,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "last_valid_block_height",
                            &self.last_valid_block_height,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetBlockhashResponse {
                    fn schema_name() -> std::string::String {
                        "GetBlockhashResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_block_hash::GetBlockhashResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "blockhash",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Latest blockhash",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "last_valid_block_height",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Last valid block height",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `get_blockhash` action.",
                        )
                    }
                }
            };
            pub struct GetBlockhash {
                context: Arc<SolanaRpcClient>,
            }
            impl GetBlockhash {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    _args: GetBlockhashArgs,
                ) -> Result<GetBlockhashResponse, yart::ToolError> {
                    {
                        let (hash, block_height) = ctx.get_block_hash().await?;
                        Ok(GetBlockhashResponse {
                            blockhash: hash.to_string(),
                            last_valid_block_height: block_height.to_string(),
                        })
                    }
                }
            }
            impl rig::tool::Tool for GetBlockhash {
                const NAME: &'static str = "get_blockhash";
                type Error = yart::ToolError;
                type Args = GetBlockhashArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Get the latest blockhash".to_string(),
                        parameters: yart::derive_parameters::<GetBlockhashArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            GetBlockhash::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod get_token_accounts {
            use std::{str::FromStr, sync::Arc};
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use solana_sdk::pubkey::Pubkey;
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `get_token_accounts` action.
            pub struct GetTokenAccountsArgs {
                #[schemars(description = "Optional mint public key to filter by")]
                mint_pubkey: Option<String>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetTokenAccountsArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "mint_pubkey" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"mint_pubkey" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetTokenAccountsArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetTokenAccountsArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetTokenAccountsArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetTokenAccountsArgs with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetTokenAccountsArgs {
                                    mint_pubkey: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    Option<String>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "mint_pubkey",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<String>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("mint_pubkey")?
                                    }
                                };
                                _serde::__private::Ok(GetTokenAccountsArgs {
                                    mint_pubkey: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["mint_pubkey"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetTokenAccountsArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetTokenAccountsArgs,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetTokenAccountsArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetTokenAccountsArgs",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mint_pubkey",
                            &self.mint_pubkey,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetTokenAccountsArgs {
                    fn schema_name() -> std::string::String {
                        "GetTokenAccountsArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_token_accounts::GetTokenAccountsArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        Option<String>,
                                    >(
                                        object_validation,
                                        "mint_pubkey",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<Option<String>>(),
                                            "Optional mint public key to filter by",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `get_token_accounts` action.",
                        )
                    }
                }
            };
            /// Details of a token account.
            pub struct TokenAccount {
                #[schemars(description = "Token account public key")]
                pubkey: String,
                #[schemars(description = "Mint public key")]
                mint: String,
                #[schemars(description = "Token balance")]
                amount: String,
                #[schemars(description = "Number of decimal places", with = "i32")]
                decimal: u8,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for TokenAccount {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "pubkey" => _serde::__private::Ok(__Field::__field0),
                                    "mint" => _serde::__private::Ok(__Field::__field1),
                                    "amount" => _serde::__private::Ok(__Field::__field2),
                                    "decimal" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"pubkey" => _serde::__private::Ok(__Field::__field0),
                                    b"mint" => _serde::__private::Ok(__Field::__field1),
                                    b"amount" => _serde::__private::Ok(__Field::__field2),
                                    b"decimal" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<TokenAccount>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = TokenAccount;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct TokenAccount",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct TokenAccount with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct TokenAccount with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct TokenAccount with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field3 = match _serde::de::SeqAccess::next_element::<
                                    u8,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                3usize,
                                                &"struct TokenAccount with 4 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(TokenAccount {
                                    pubkey: __field0,
                                    mint: __field1,
                                    amount: __field2,
                                    decimal: __field3,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field3: _serde::__private::Option<u8> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("pubkey"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("mint"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "decimal",
                                                    ),
                                                );
                                            }
                                            __field3 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("pubkey")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("mint")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("amount")?
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("decimal")?
                                    }
                                };
                                _serde::__private::Ok(TokenAccount {
                                    pubkey: __field0,
                                    mint: __field1,
                                    amount: __field2,
                                    decimal: __field3,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "pubkey",
                            "mint",
                            "amount",
                            "decimal",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "TokenAccount",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<TokenAccount>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for TokenAccount {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "TokenAccount",
                            false as usize + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "pubkey",
                            &self.pubkey,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mint",
                            &self.mint,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "amount",
                            &self.amount,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "decimal",
                            &self.decimal,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for TokenAccount {
                    fn schema_name() -> std::string::String {
                        "TokenAccount".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_token_accounts::TokenAccount",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "pubkey",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Token account public key",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "mint",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Mint public key",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "amount",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Token balance",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        i32,
                                    >(
                                        object_validation,
                                        "decimal",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<i32>(),
                                            "Number of decimal places",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Details of a token account.",
                        )
                    }
                }
            };
            /// Response for the `get_token_accounts` action.
            pub struct GetTokenAccountsResponse {
                #[schemars(description = "List of token accounts")]
                accounts: Vec<TokenAccount>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetTokenAccountsResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "accounts" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"accounts" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<
                                GetTokenAccountsResponse,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetTokenAccountsResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetTokenAccountsResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    Vec<TokenAccount>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetTokenAccountsResponse with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetTokenAccountsResponse {
                                    accounts: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    Vec<TokenAccount>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "accounts",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Vec<TokenAccount>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("accounts")?
                                    }
                                };
                                _serde::__private::Ok(GetTokenAccountsResponse {
                                    accounts: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["accounts"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetTokenAccountsResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetTokenAccountsResponse,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetTokenAccountsResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetTokenAccountsResponse",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "accounts",
                            &self.accounts,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetTokenAccountsResponse {
                    fn schema_name() -> std::string::String {
                        "GetTokenAccountsResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_token_accounts::GetTokenAccountsResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        Vec<TokenAccount>,
                                    >(
                                        object_validation,
                                        "accounts",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<Vec<TokenAccount>>(),
                                            "List of token accounts",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `get_token_accounts` action.",
                        )
                    }
                }
            };
            pub struct GetTokenAccounts {
                context: Arc<SolanaRpcClient>,
            }
            impl GetTokenAccounts {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    args: GetTokenAccountsArgs,
                ) -> Result<GetTokenAccountsResponse, yart::ToolError> {
                    {
                        let mint_pubkey = match args.mint_pubkey {
                            Some(m) => {
                                Some(
                                    Pubkey::from_str(&m)
                                        .map_err(|e| ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Invalid mint pubkey: {0}", e),
                                                );
                                                res
                                            }),
                                        ))?,
                                )
                            }
                            None => None,
                        };
                        let token_accounts = ctx.get_token_accounts(mint_pubkey).await?;
                        Ok(GetTokenAccountsResponse {
                            accounts: token_accounts
                                .iter()
                                .map(|a| TokenAccount {
                                    pubkey: a.pubkey.to_string(),
                                    mint: a.mint.to_string(),
                                    amount: a.amount.to_string(),
                                    decimal: a.decimals,
                                })
                                .collect(),
                        })
                    }
                }
            }
            impl rig::tool::Tool for GetTokenAccounts {
                const NAME: &'static str = "get_token_accounts";
                type Error = yart::ToolError;
                type Args = GetTokenAccountsArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Get token accounts owned by the wallet"
                            .to_string(),
                        parameters: yart::derive_parameters::<GetTokenAccountsArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            GetTokenAccounts::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod get_transaction {
            use std::{str::FromStr, sync::Arc};
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use solana_sdk::signature::Signature;
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `get_transaction` action.
            pub struct GetTransactionArgs {
                #[schemars(description = "Transaction signature")]
                signature: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetTransactionArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "signature" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"signature" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetTransactionArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetTransactionArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetTransactionArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetTransactionArgs with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetTransactionArgs {
                                    signature: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "signature",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("signature")?
                                    }
                                };
                                _serde::__private::Ok(GetTransactionArgs {
                                    signature: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["signature"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetTransactionArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetTransactionArgs,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetTransactionArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetTransactionArgs",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "signature",
                            &self.signature,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetTransactionArgs {
                    fn schema_name() -> std::string::String {
                        "GetTransactionArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_transaction::GetTransactionArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "signature",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Transaction signature",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `get_transaction` action.",
                        )
                    }
                }
            };
            /// Response for the `get_transaction` action.
            pub struct GetTransactionResponse {
                #[schemars(description = "Transaction status (e.g., Confirmed, Failed)")]
                status: String,
                #[schemars(description = "Slot number")]
                slot: String,
                #[schemars(
                    description = "Block time (Unix timestamp), null if unavailable"
                )]
                block_time: Option<String>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetTransactionResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "status" => _serde::__private::Ok(__Field::__field0),
                                    "slot" => _serde::__private::Ok(__Field::__field1),
                                    "block_time" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"status" => _serde::__private::Ok(__Field::__field0),
                                    b"slot" => _serde::__private::Ok(__Field::__field1),
                                    b"block_time" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<
                                GetTransactionResponse,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetTransactionResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetTransactionResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct GetTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct GetTransactionResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetTransactionResponse {
                                    status: __field0,
                                    slot: __field1,
                                    block_time: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<
                                    Option<String>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("slot"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "block_time",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<String>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("status")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("slot")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("block_time")?
                                    }
                                };
                                _serde::__private::Ok(GetTransactionResponse {
                                    status: __field0,
                                    slot: __field1,
                                    block_time: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "status",
                            "slot",
                            "block_time",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetTransactionResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetTransactionResponse,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetTransactionResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetTransactionResponse",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "status",
                            &self.status,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "slot",
                            &self.slot,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "block_time",
                            &self.block_time,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetTransactionResponse {
                    fn schema_name() -> std::string::String {
                        "GetTransactionResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_transaction::GetTransactionResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "status",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Transaction status (e.g., Confirmed, Failed)",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "slot",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Slot number",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        Option<String>,
                                    >(
                                        object_validation,
                                        "block_time",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<Option<String>>(),
                                            "Block time (Unix timestamp), null if unavailable",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `get_transaction` action.",
                        )
                    }
                }
            };
            pub struct GetTransaction {
                context: Arc<SolanaRpcClient>,
            }
            impl GetTransaction {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    args: GetTransactionArgs,
                ) -> Result<GetTransactionResponse, yart::ToolError> {
                    {
                        let signature = Signature::from_str(&args.signature)
                            .map_err(|e| ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Invalid signature: {0}", e),
                                    );
                                    res
                                }),
                            ))?;
                        let tx = ctx.get_transaction(&signature).await?;
                        Ok(GetTransactionResponse {
                            status: tx.status,
                            slot: tx.slot.to_string(),
                            block_time: tx
                                .block_time
                                .map_or(None, |b| Some(b.to_string())),
                        })
                    }
                }
            }
            impl rig::tool::Tool for GetTransaction {
                const NAME: &'static str = "get_transaction";
                type Error = yart::ToolError;
                type Args = GetTransactionArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Get transaction status and details".to_string(),
                        parameters: yart::derive_parameters::<GetTransactionArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            GetTransaction::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod get_wallet_address {
            use std::sync::Arc;
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `get_wallet_address` action (empty).
            pub struct GetWalletAddressArgs {}
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetWalletAddressArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<GetWalletAddressArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetWalletAddressArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetWalletAddressArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(GetWalletAddressArgs {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(GetWalletAddressArgs {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetWalletAddressArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetWalletAddressArgs,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetWalletAddressArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetWalletAddressArgs",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetWalletAddressArgs {
                    fn schema_name() -> std::string::String {
                        "GetWalletAddressArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_wallet_address::GetWalletAddressArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `get_wallet_address` action (empty).",
                        )
                    }
                }
            };
            /// Response for the `get_wallet_address` action.
            pub struct GetWalletAddressResponse {
                #[schemars(description = "Wallet public key")]
                address: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for GetWalletAddressResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "address" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"address" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<
                                GetWalletAddressResponse,
                            >,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = GetWalletAddressResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct GetWalletAddressResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct GetWalletAddressResponse with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(GetWalletAddressResponse {
                                    address: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "address",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("address")?
                                    }
                                };
                                _serde::__private::Ok(GetWalletAddressResponse {
                                    address: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["address"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "GetWalletAddressResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<
                                    GetWalletAddressResponse,
                                >,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for GetWalletAddressResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "GetWalletAddressResponse",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "address",
                            &self.address,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for GetWalletAddressResponse {
                    fn schema_name() -> std::string::String {
                        "GetWalletAddressResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::get_wallet_address::GetWalletAddressResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "address",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Wallet public key",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `get_wallet_address` action.",
                        )
                    }
                }
            };
            pub struct GetWalletAddress {
                context: Arc<SolanaRpcClient>,
            }
            impl GetWalletAddress {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    _args: GetWalletAddressArgs,
                ) -> Result<GetWalletAddressResponse, yart::ToolError> {
                    {
                        let pubkey = ctx.get_wallet_pubkey();
                        Ok(GetWalletAddressResponse {
                            address: pubkey.to_string(),
                        })
                    }
                }
            }
            impl rig::tool::Tool for GetWalletAddress {
                const NAME: &'static str = "get_wallet_address";
                type Error = yart::ToolError;
                type Args = GetWalletAddressArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Get wallet public key".to_string(),
                        parameters: yart::derive_parameters::<GetWalletAddressArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            GetWalletAddress::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod mint_to {
            use std::{str::FromStr, sync::Arc};
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use solana_sdk::pubkey::Pubkey;
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `mint_to` action.
            pub struct MintToArgs {
                #[schemars(description = "Recipient wallet public key")]
                to_wallet: String,
                #[schemars(description = "Mint public key")]
                mint_pubkey: String,
                #[schemars(
                    description = "Amount to mint (in UI units, e.g., 100.5 tokens)"
                )]
                amount: f64,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for MintToArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "to_wallet" => _serde::__private::Ok(__Field::__field0),
                                    "mint_pubkey" => _serde::__private::Ok(__Field::__field1),
                                    "amount" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"to_wallet" => _serde::__private::Ok(__Field::__field0),
                                    b"mint_pubkey" => _serde::__private::Ok(__Field::__field1),
                                    b"amount" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<MintToArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = MintToArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct MintToArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct MintToArgs with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct MintToArgs with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    f64,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct MintToArgs with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(MintToArgs {
                                    to_wallet: __field0,
                                    mint_pubkey: __field1,
                                    amount: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<f64> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "to_wallet",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "mint_pubkey",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("to_wallet")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("mint_pubkey")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("amount")?
                                    }
                                };
                                _serde::__private::Ok(MintToArgs {
                                    to_wallet: __field0,
                                    mint_pubkey: __field1,
                                    amount: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "to_wallet",
                            "mint_pubkey",
                            "amount",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "MintToArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<MintToArgs>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for MintToArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "MintToArgs",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "to_wallet",
                            &self.to_wallet,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mint_pubkey",
                            &self.mint_pubkey,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "amount",
                            &self.amount,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for MintToArgs {
                    fn schema_name() -> std::string::String {
                        "MintToArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::mint_to::MintToArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "to_wallet",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Recipient wallet public key",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "mint_pubkey",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Mint public key",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        f64,
                                    >(
                                        object_validation,
                                        "amount",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<f64>(),
                                            "Amount to mint (in UI units, e.g., 100.5 tokens)",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `mint_to` action.",
                        )
                    }
                }
            };
            /// Response for the `mint_to` action.
            pub struct MintToResponse {
                #[schemars(description = "Transaction signature")]
                signature: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for MintToResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "signature" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"signature" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<MintToResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = MintToResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct MintToResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct MintToResponse with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(MintToResponse {
                                    signature: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "signature",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("signature")?
                                    }
                                };
                                _serde::__private::Ok(MintToResponse {
                                    signature: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["signature"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "MintToResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<MintToResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for MintToResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "MintToResponse",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "signature",
                            &self.signature,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for MintToResponse {
                    fn schema_name() -> std::string::String {
                        "MintToResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::mint_to::MintToResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "signature",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Transaction signature",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `mint_to` action.",
                        )
                    }
                }
            };
            pub struct MintTo {
                context: Arc<SolanaRpcClient>,
            }
            impl MintTo {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    args: MintToArgs,
                ) -> Result<MintToResponse, yart::ToolError> {
                    {
                        let to_wallet = Pubkey::from_str(&args.to_wallet)
                            .map_err(|e| ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Invalid wallet pubkey: {0}", e),
                                    );
                                    res
                                }),
                            ))?;
                        let mint = Pubkey::from_str(&args.mint_pubkey)
                            .map_err(|e| ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Invalid mint pubkey: {0}", e),
                                    );
                                    res
                                }),
                            ))?;
                        if args.amount <= 0.0 {
                            return Err(
                                ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            format_args!("Mint amount must be positive"),
                                        );
                                        error
                                    })
                                    .into(),
                            );
                        }
                        let signature = ctx
                            .mint_to(&mint, &to_wallet, args.amount)
                            .await?;
                        Ok(MintToResponse {
                            signature: signature.to_string(),
                        })
                    }
                }
            }
            impl rig::tool::Tool for MintTo {
                const NAME: &'static str = "mint_to";
                type Error = yart::ToolError;
                type Args = MintToArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Mint tokens to a wallet's associated token account"
                            .to_string(),
                        parameters: yart::derive_parameters::<MintToArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            MintTo::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod request_airdrop {
            use std::sync::Arc;
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `request_airdrop` action.
            pub struct AirdropArgs {
                #[schemars(description = "Amount of SOL to request (maximum 5 SOL)")]
                amount: f64,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for AirdropArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "amount" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"amount" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<AirdropArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = AirdropArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct AirdropArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    f64,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct AirdropArgs with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(AirdropArgs { amount: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<f64> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("amount")?
                                    }
                                };
                                _serde::__private::Ok(AirdropArgs { amount: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["amount"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "AirdropArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<AirdropArgs>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for AirdropArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "AirdropArgs",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "amount",
                            &self.amount,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for AirdropArgs {
                    fn schema_name() -> std::string::String {
                        "AirdropArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::request_airdrop::AirdropArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        f64,
                                    >(
                                        object_validation,
                                        "amount",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<f64>(),
                                            "Amount of SOL to request (maximum 5 SOL)",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `request_airdrop` action.",
                        )
                    }
                }
            };
            /// Response for the `request_airdrop` action.
            pub struct AirdropResponse {
                #[schemars(description = "Transaction signature")]
                signature: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for AirdropResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "signature" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"signature" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<AirdropResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = AirdropResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct AirdropResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct AirdropResponse with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(AirdropResponse {
                                    signature: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "signature",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("signature")?
                                    }
                                };
                                _serde::__private::Ok(AirdropResponse {
                                    signature: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["signature"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "AirdropResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<AirdropResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for AirdropResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "AirdropResponse",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "signature",
                            &self.signature,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for AirdropResponse {
                    fn schema_name() -> std::string::String {
                        "AirdropResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::request_airdrop::AirdropResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "signature",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Transaction signature",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `request_airdrop` action.",
                        )
                    }
                }
            };
            pub struct RequestAirdrop {
                context: Arc<SolanaRpcClient>,
            }
            impl RequestAirdrop {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    args: AirdropArgs,
                ) -> Result<AirdropResponse, yart::ToolError> {
                    {
                        if args.amount <= 0.0 {
                            return Err(
                                ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            format_args!("Airdrop amount must be positive"),
                                        );
                                        error
                                    })
                                    .into(),
                            );
                        }
                        if args.amount > 5.0 {
                            return Err(
                                ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            format_args!("Airdrop amount exceeds maximum of 5 SOL"),
                                        );
                                        error
                                    })
                                    .into(),
                            );
                        }
                        let signature = ctx.request_airdrop(args.amount).await?;
                        Ok(AirdropResponse {
                            signature: signature.to_string(),
                        })
                    }
                }
            }
            impl rig::tool::Tool for RequestAirdrop {
                const NAME: &'static str = "request_airdrop";
                type Error = yart::ToolError;
                type Args = AirdropArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Request SOL airdrop for devnet".to_string(),
                        parameters: yart::derive_parameters::<AirdropArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            RequestAirdrop::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        mod transfer {
            use std::{str::FromStr, sync::Arc};
            use anyhow::Result;
            use schemars::JsonSchema;
            use serde::{Deserialize, Serialize};
            use solana_sdk::pubkey::Pubkey;
            use crate::solana::solana_rpc_client::SolanaRpcClient;
            /// Arguments for the `transfer` action.
            pub struct TransferArgs {
                #[schemars(description = "Recipient's public key")]
                to_wallet: String,
                #[schemars(
                    description = "Amount to transfer (in UI units, e.g., 1.5 SOL or tokens)"
                )]
                amount: f64,
                #[schemars(
                    description = "Solana public key of mint account, null for SOL"
                )]
                mint_pubkey: Option<String>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for TransferArgs {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "to_wallet" => _serde::__private::Ok(__Field::__field0),
                                    "amount" => _serde::__private::Ok(__Field::__field1),
                                    "mint_pubkey" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"to_wallet" => _serde::__private::Ok(__Field::__field0),
                                    b"amount" => _serde::__private::Ok(__Field::__field1),
                                    b"mint_pubkey" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<TransferArgs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = TransferArgs;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct TransferArgs",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct TransferArgs with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    f64,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct TransferArgs with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct TransferArgs with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(TransferArgs {
                                    to_wallet: __field0,
                                    amount: __field1,
                                    mint_pubkey: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<f64> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<
                                    Option<String>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "to_wallet",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("amount"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "mint_pubkey",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<String>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("to_wallet")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("amount")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("mint_pubkey")?
                                    }
                                };
                                _serde::__private::Ok(TransferArgs {
                                    to_wallet: __field0,
                                    amount: __field1,
                                    mint_pubkey: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "to_wallet",
                            "amount",
                            "mint_pubkey",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "TransferArgs",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<TransferArgs>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for TransferArgs {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "TransferArgs",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "to_wallet",
                            &self.to_wallet,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "amount",
                            &self.amount,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mint_pubkey",
                            &self.mint_pubkey,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for TransferArgs {
                    fn schema_name() -> std::string::String {
                        "TransferArgs".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::transfer::TransferArgs",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "to_wallet",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Recipient's public key",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        f64,
                                    >(
                                        object_validation,
                                        "amount",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<f64>(),
                                            "Amount to transfer (in UI units, e.g., 1.5 SOL or tokens)",
                                        ),
                                    );
                                }
                                {
                                    schemars::_private::insert_object_property::<
                                        Option<String>,
                                    >(
                                        object_validation,
                                        "mint_pubkey",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<Option<String>>(),
                                            "Solana public key of mint account, null for SOL",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Arguments for the `transfer` action.",
                        )
                    }
                }
            };
            /// Response for the `transfer` action.
            pub struct TransferResponse {
                #[schemars(description = "Transaction signature")]
                signature: String,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for TransferResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "signature" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"signature" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<TransferResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = TransferResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct TransferResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct TransferResponse with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(TransferResponse {
                                    signature: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "signature",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("signature")?
                                    }
                                };
                                _serde::__private::Ok(TransferResponse {
                                    signature: __field0,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["signature"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "TransferResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<TransferResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for TransferResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "TransferResponse",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "signature",
                            &self.signature,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            const _: () = {
                #[automatically_derived]
                #[allow(unused_braces)]
                impl schemars::JsonSchema for TransferResponse {
                    fn schema_name() -> std::string::String {
                        "TransferResponse".to_owned()
                    }
                    fn schema_id() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(
                            "solana_tools::solana::actions::transfer::TransferResponse",
                        )
                    }
                    fn json_schema(
                        generator: &mut schemars::gen::SchemaGenerator,
                    ) -> schemars::schema::Schema {
                        schemars::_private::metadata::add_description(
                            {
                                let mut schema_object = schemars::schema::SchemaObject {
                                    instance_type: Some(
                                        schemars::schema::InstanceType::Object.into(),
                                    ),
                                    ..Default::default()
                                };
                                let object_validation = schema_object.object();
                                {
                                    schemars::_private::insert_object_property::<
                                        String,
                                    >(
                                        object_validation,
                                        "signature",
                                        false,
                                        false,
                                        schemars::_private::metadata::add_description(
                                            generator.subschema_for::<String>(),
                                            "Transaction signature",
                                        ),
                                    );
                                }
                                schemars::schema::Schema::Object(schema_object)
                            },
                            "Response for the `transfer` action.",
                        )
                    }
                }
            };
            pub struct Transfer {
                context: Arc<SolanaRpcClient>,
            }
            impl Transfer {
                pub fn new(ctx: Arc<SolanaRpcClient>) -> Self {
                    Self { context: ctx }
                }
                async fn internal_call(
                    ctx: Arc<SolanaRpcClient>,
                    args: TransferArgs,
                ) -> Result<TransferResponse, yart::ToolError> {
                    {
                        let to_wallet = Pubkey::from_str(&args.to_wallet)
                            .map_err(|e| ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Invalid recipient pubkey: {0}", e),
                                    );
                                    res
                                }),
                            ))?;
                        if args.amount <= 0.0 {
                            return Err(
                                ::anyhow::__private::must_use({
                                        let error = ::anyhow::__private::format_err(
                                            format_args!("Transfer amount must be positive"),
                                        );
                                        error
                                    })
                                    .into(),
                            );
                        }
                        let mint_pubkey = match args.mint_pubkey {
                            Some(m) => {
                                Some(
                                    Pubkey::from_str(&m)
                                        .map_err(|e| ::anyhow::Error::msg(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Invalid mint pubkey: {0}", e),
                                                );
                                                res
                                            }),
                                        ))?,
                                )
                            }
                            None => None,
                        };
                        let signature = ctx
                            .transfer(&to_wallet, args.amount, mint_pubkey)
                            .await?;
                        Ok(TransferResponse {
                            signature: signature.to_string(),
                        })
                    }
                }
            }
            impl rig::tool::Tool for Transfer {
                const NAME: &'static str = "transfer";
                type Error = yart::ToolError;
                type Args = TransferArgs;
                type Output = yart::ToolOutput;
                fn name(&self) -> String {
                    Self::NAME.to_string()
                }
                async fn definition(
                    &self,
                    _prompt: String,
                ) -> rig::completion::ToolDefinition {
                    rig::completion::ToolDefinition {
                        name: Self::NAME.to_string(),
                        description: "Transfer SOL or SPL token to another wallet"
                            .to_string(),
                        parameters: yart::derive_parameters::<TransferArgs>(),
                    }
                }
                async fn call(
                    &self,
                    args: Self::Args,
                ) -> Result<Self::Output, Self::Error> {
                    let ctx = self.context.clone();
                    let result = yart::wrap_unsafe(move || async move {
                            Transfer::internal_call(ctx, args)
                                .await
                                .map_err(|e| ::anyhow::__private::must_use({
                                    use ::anyhow::__private::kind::*;
                                    let error = match e.to_string() {
                                        error => (&error).anyhow_kind().new(error),
                                    };
                                    error
                                }))
                        })
                        .await?;
                    let serialized_result = serde_json::to_value(result)
                        .map_err(|e| yart::ToolError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Serialization error: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    Ok(yart::ToolOutput {
                        result: serialized_result,
                    })
                }
            }
        }
        pub use create_associated_token_account::*;
        pub use create_mint::*;
        pub use get_balance::*;
        pub use get_block_hash::*;
        pub use get_token_accounts::*;
        pub use get_transaction::*;
        pub use get_wallet_address::*;
        pub use mint_to::*;
        pub use request_airdrop::*;
        pub use transfer::*;
    }
    pub mod solana_rpc_client {
        use std::time::Duration;
        use std::{collections::HashSet, str::FromStr};
        use anyhow::Result;
        use solana_account_decoder::{UiAccountData, parse_token::UiTokenAccount};
        use solana_client::{
            nonblocking::rpc_client::RpcClient, rpc_request::TokenAccountsFilter,
        };
        use solana_sdk::{
            commitment_config::CommitmentConfig, hash::Hash, instruction::Instruction,
            program_pack::{IsInitialized, Pack},
            pubkey::Pubkey, signature::{Keypair, Signature},
            signer::Signer, system_instruction, transaction::Transaction,
        };
        use solana_transaction_status_client_types::UiTransactionEncoding;
        use spl_associated_token_account::{
            get_associated_token_address_with_program_id,
            instruction::create_associated_token_account,
        };
        use spl_token::{
            instruction::{initialize_mint, mint_to},
            state::Mint, ui_amount_to_amount,
        };
        use tokio::time::{sleep, timeout};
        use crate::solana::types::TokenAmount;
        use super::types::{TokenAccountDetails, TransactionDetails};
        /// A client for interacting with the Solana network using an RPC endpoint and a wallet keypair.
        pub struct SolanaRpcClient {
            pub rpc_client: RpcClient,
            pub wallet: Keypair,
        }
        impl SolanaRpcClient {
            /// Creates a new `SolanaRpcClient` with the provided RPC client and wallet keypair.
            pub fn new(rpc_client: RpcClient, wallet: Keypair) -> Self {
                Self { rpc_client, wallet }
            }
            /// Returns the public key of the wallet associated with this client.
            pub fn get_wallet_pubkey(&self) -> Pubkey {
                self.wallet.pubkey()
            }
            /// Processes a single instruction on the Solana network.
            ///
            /// # Arguments
            /// * `instruction` - The instruction to process.
            /// * `signers` - A vector of keypairs that will sign the transaction.
            /// * `payer` - Optional pubkey of the account paying for transaction fees. Defaults to the wallet's pubkey.
            ///
            /// # Returns
            /// A `Result` containing the transaction signature or an error if the transaction fails.
            pub async fn process_instruction(
                &self,
                instruction: Instruction,
                signers: &Vec<&Keypair>,
                payer: Option<&Pubkey>,
            ) -> Result<Signature> {
                self.process_instructions(&[instruction], signers, payer).await
            }
            /// Processes multiple instructions in a single transaction on the Solana network.
            ///
            /// # Arguments
            /// * `instructions` - A slice of instructions to process.
            /// * `signers` - A vector of keypairs that will sign the transaction.
            /// * `payer` - Optional pubkey of the account paying for transaction fees. Defaults to the wallet's pubkey.
            ///
            /// # Returns
            /// A `Result` containing the transaction signature or an error if the transaction fails.
            pub async fn process_instructions(
                &self,
                instructions: &[Instruction],
                signers: &Vec<&Keypair>,
                payer: Option<&Pubkey>,
            ) -> Result<Signature> {
                let recent_blockhash = self
                    .rpc_client
                    .get_latest_blockhash()
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to get recent blockhash: {0}", e),
                            );
                            res
                        }),
                    ))?;
                let wallet_pubkey = self.get_wallet_pubkey();
                let actual_payer = payer.unwrap_or(&wallet_pubkey);
                let mut unique_signers: Vec<&Keypair> = ::alloc::vec::Vec::new();
                if actual_payer == &wallet_pubkey {
                    unique_signers.push(&self.wallet);
                }
                let signer_pubkeys: HashSet<Pubkey> = unique_signers
                    .iter()
                    .map(|s| s.pubkey())
                    .collect();
                for signer in signers {
                    if !signer_pubkeys.contains(&signer.pubkey()) {
                        unique_signers.push(signer);
                    }
                }
                let tx = Transaction::new_signed_with_payer(
                    instructions,
                    Some(actual_payer),
                    &unique_signers,
                    recent_blockhash,
                );
                let signature = self
                    .rpc_client
                    .send_and_confirm_transaction(&tx)
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to send transaction: {0}", e),
                            );
                            res
                        }),
                    ))?;
                Ok(signature)
            }
            /// Retrieves and unpacks an account's data into a specified type.
            ///
            /// # Arguments
            /// * `pubkey` - The public key of the account to query.
            ///
            /// # Returns
            /// A `Result` containing the unpacked account data or an error if the account doesn't exist or unpacking fails.
            pub async fn get_packed_account<T: Pack + IsInitialized>(
                &self,
                pubkey: &Pubkey,
            ) -> Result<T> {
                let account = self
                    .rpc_client
                    .get_account(pubkey)
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to get account {0}: {1}", pubkey, e),
                            );
                            res
                        }),
                    ))?;
                T::unpack(&account.data[..])
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to unpack account {0}: {1}", pubkey, e),
                            );
                            res
                        }),
                    ))
            }
            /// Retrieves and unpacks a mint account's data.
            ///
            /// # Arguments
            /// * `mint_pubkey` - The public key of the mint account.
            ///
            /// # Returns
            /// A `Result` containing the mint data and its program ID, or an error if the account doesn't exist or unpacking fails.
            pub async fn get_mint_account(
                &self,
                mint_pubkey: &Pubkey,
            ) -> Result<(Mint, Pubkey)> {
                let account = self
                    .rpc_client
                    .get_account(mint_pubkey)
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to get mint account {0}: {1}",
                                    mint_pubkey,
                                    e,
                                ),
                            );
                            res
                        }),
                    ))?;
                let mint = Mint::unpack(&account.data[..])
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to unpack mint account {0}: {1}",
                                    mint_pubkey,
                                    e,
                                ),
                            );
                            res
                        }),
                    ))?;
                Ok((mint, account.owner))
            }
            /// Retrieves the balance of the wallet for native SOL or a specific SPL token.
            ///
            /// # Arguments
            /// * `mint_pubkey` - Optional public key of the token mint. If `None`, returns the SOL balance.
            ///
            /// # Returns
            /// A `Result` containing the balance as a `TokenAmount` or an error if the query fails.
            pub async fn get_balance(
                &self,
                mint_pubkey: Option<Pubkey>,
            ) -> Result<TokenAmount> {
                let wallet_pubkey = self.get_wallet_pubkey();
                let balance = if let Some(mint) = mint_pubkey {
                    let (_, token_program_id) = self.get_mint_account(&mint).await?;
                    let token_account_pubkey = get_associated_token_address_with_program_id(
                        &wallet_pubkey,
                        &mint,
                        &token_program_id,
                    );
                    let balance = self
                        .rpc_client
                        .get_token_account(&token_account_pubkey)
                        .await
                        .map_err(|e| {
                            ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get token account {0}: {1}",
                                            token_account_pubkey,
                                            e,
                                        ),
                                    );
                                    res
                                }),
                            )
                        })?;
                    if let Some(ta) = balance {
                        (ta.token_amount.amount.parse()?, ta.token_amount.decimals)
                    } else {
                        return Err(
                            ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Token account {0} does not exist",
                                            token_account_pubkey,
                                        ),
                                    );
                                    res
                                }),
                            ),
                        );
                    }
                } else {
                    let balance = self
                        .rpc_client
                        .get_balance(&wallet_pubkey)
                        .await
                        .map_err(|e| {
                            ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get SOL balance for {0}: {1}",
                                            wallet_pubkey,
                                            e,
                                        ),
                                    );
                                    res
                                }),
                            )
                        })?;
                    (balance, 9)
                };
                Ok(TokenAmount {
                    amount: balance.0,
                    decimals: balance.1,
                })
            }
            /// Requests an airdrop of SOL to the wallet.
            ///
            /// # Arguments
            /// * `amount` - The amount of SOL to request.
            ///
            /// # Returns
            /// A `Result` containing the transaction signature or an error if the airdrop fails or times out.
            pub async fn request_airdrop(&self, amount: f64) -> Result<Signature> {
                if amount <= 0.0 {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("Airdrop amount must be positive"),
                            );
                            error
                        }),
                    );
                }
                let lamports = ui_amount_to_amount(amount, 9);
                let wallet_pubkey = self.get_wallet_pubkey();
                let signature = self
                    .rpc_client
                    .request_airdrop(&wallet_pubkey, lamports)
                    .await
                    .map_err(|e| {
                        ::anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to request airdrop for {0}: {1}",
                                        wallet_pubkey,
                                        e,
                                    ),
                                );
                                res
                            }),
                        )
                    })?;
                let confirmed = timeout(
                        Duration::from_secs(30),
                        async {
                            loop {
                                let confirmed = self
                                    .rpc_client
                                    .confirm_transaction(&signature)
                                    .await
                                    .map_err(|e| ::anyhow::Error::msg(
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("Failed to confirm airdrop: {0}", e),
                                            );
                                            res
                                        }),
                                    ));
                                if confirmed.is_ok() && confirmed.unwrap() {
                                    break true;
                                }
                                sleep(Duration::from_millis(500)).await;
                            }
                        },
                    )
                    .await
                    .map_err(|_| {
                        ::anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Airdrop confirmation timed out for signature {0}",
                                        signature,
                                    ),
                                );
                                res
                            }),
                        )
                    })?;
                if !confirmed {
                    return Err(
                        ::anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Airdrop failed to confirm for signature {0}",
                                        signature,
                                    ),
                                );
                                res
                            }),
                        ),
                    );
                }
                Ok(signature)
            }
            /// Transfers SOL or SPL tokens to another wallet.
            ///
            /// # Arguments
            /// * `to_wallet_pubkey` - The recipient's public key.
            /// * `amount` - The amount to transfer (in UI amount, e.g., 1.5 SOL or tokens).
            /// * `mint_pubkey` - Optional public key of the token mint. If `None`, transfers SOL.
            ///
            /// # Returns
            /// A `Result` containing the transaction signature or an error if the transfer fails.
            pub async fn transfer(
                &self,
                to_wallet_pubkey: &Pubkey,
                amount: f64,
                mint_pubkey: Option<Pubkey>,
            ) -> Result<Signature> {
                if amount <= 0.0 {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("Transfer amount must be positive"),
                            );
                            error
                        }),
                    );
                }
                let signature = if let Some(mint) = mint_pubkey {
                    let (mint_account, token_program_id) = self
                        .get_mint_account(&mint)
                        .await?;
                    let from_wallet_pubkey = self.get_wallet_pubkey();
                    let token_account_pubkey = get_associated_token_address_with_program_id(
                        &from_wallet_pubkey,
                        &mint,
                        &token_program_id,
                    );
                    let source_balance = self.get_balance(Some(mint)).await?;
                    let amount_lamports = ui_amount_to_amount(
                        amount,
                        mint_account.decimals,
                    );
                    if source_balance.amount < amount_lamports {
                        return Err(
                            ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Insufficient balance: {0} available, {1} required",
                                            source_balance.amount,
                                            amount_lamports,
                                        ),
                                    );
                                    res
                                }),
                            ),
                        );
                    }
                    let to_token_account_pubkey = get_associated_token_address_with_program_id(
                        to_wallet_pubkey,
                        &mint,
                        &token_program_id,
                    );
                    let mut ixs = if self
                        .rpc_client
                        .get_account(&to_token_account_pubkey)
                        .await
                        .is_err()
                    {
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                create_associated_token_account(
                                    &from_wallet_pubkey,
                                    to_wallet_pubkey,
                                    &mint,
                                    &token_program_id,
                                ),
                            ]),
                        )
                    } else {
                        ::alloc::vec::Vec::new()
                    };
                    ixs.push(
                        spl_token::instruction::transfer(
                            &token_program_id,
                            &token_account_pubkey,
                            &to_token_account_pubkey,
                            &from_wallet_pubkey,
                            &[],
                            amount_lamports,
                        )?,
                    );
                    self.process_instructions(&ixs, &::alloc::vec::Vec::new(), None)
                        .await?
                } else {
                    let lamports = ui_amount_to_amount(amount, 9);
                    let ix = system_instruction::transfer(
                        &self.get_wallet_pubkey(),
                        to_wallet_pubkey,
                        lamports,
                    );
                    self.process_instruction(ix, &::alloc::vec::Vec::new(), None).await?
                };
                Ok(signature)
            }
            /// Creates a new SPL token mint.
            ///
            /// # Arguments
            /// * `decimals` - The number of decimal places for the token.
            /// * `mint_authority` - Optional public key of the mint authority. Defaults to the wallet's pubkey.
            ///
            /// # Returns
            /// A `Result` containing the transaction signature and the mint pubkey, or an error if the operation fails.
            pub async fn create_mint(
                &self,
                decimals: u8,
                mint_authority: Option<Pubkey>,
            ) -> Result<(Signature, Pubkey)> {
                let wallet_pubkey = self.get_wallet_pubkey();
                let mint_authority = mint_authority.unwrap_or(wallet_pubkey);
                let mint_keypair = Keypair::new();
                let mint_pubkey = mint_keypair.pubkey();
                let rent_exempt_balance = self
                    .rpc_client
                    .get_minimum_balance_for_rent_exemption(Mint::LEN)
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to get rent-exempt balance: {0}", e),
                            );
                            res
                        }),
                    ))?;
                let instructions = <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        system_instruction::create_account(
                            &wallet_pubkey,
                            &mint_pubkey,
                            rent_exempt_balance,
                            Mint::LEN as u64,
                            &spl_token::id(),
                        ),
                        initialize_mint(
                            &spl_token::id(),
                            &mint_pubkey,
                            &mint_authority,
                            None,
                            decimals,
                        )?,
                    ]),
                );
                let signature = self
                    .process_instructions(
                        &instructions,
                        &<[_]>::into_vec(::alloc::boxed::box_new([&mint_keypair])),
                        None,
                    )
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to create mint {0}: {1}",
                                    mint_pubkey,
                                    e,
                                ),
                            );
                            res
                        }),
                    ))?;
                Ok((signature, mint_pubkey))
            }
            /// Mints tokens to a specified wallet's associated token account.
            ///
            /// # Arguments
            /// * `mint` - The public key of the token mint.
            /// * `to_wallet` - The public key of the recipient wallet.
            /// * `amount` - The amount of tokens to mint (in UI amount, e.g., 100.5 tokens).
            ///
            /// # Returns
            /// A `Result` containing the transaction signature, or an error if the operation fails.
            pub async fn mint_to(
                &self,
                mint: &Pubkey,
                to_wallet: &Pubkey,
                amount: f64,
            ) -> Result<Signature> {
                if amount <= 0.0 {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("Mint amount must be positive"),
                            );
                            error
                        }),
                    );
                }
                let (mint_account, token_program_id) = self
                    .get_mint_account(mint)
                    .await?;
                let wallet_pubkey = self.get_wallet_pubkey();
                let to_token_account = get_associated_token_address_with_program_id(
                    to_wallet,
                    mint,
                    &token_program_id,
                );
                let mut instructions = if self
                    .rpc_client
                    .get_account(&to_token_account)
                    .await
                    .is_err()
                {
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            create_associated_token_account(
                                &wallet_pubkey,
                                to_wallet,
                                mint,
                                &token_program_id,
                            ),
                        ]),
                    )
                } else {
                    ::alloc::vec::Vec::new()
                };
                let amount_lamports = ui_amount_to_amount(amount, mint_account.decimals);
                instructions
                    .push(
                        mint_to(
                            &token_program_id,
                            mint,
                            &to_token_account,
                            &wallet_pubkey,
                            &[],
                            amount_lamports,
                        )?,
                    );
                let signature = self
                    .process_instructions(&instructions, &::alloc::vec::Vec::new(), None)
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to mint to {0}: {1}",
                                    to_token_account,
                                    e,
                                ),
                            );
                            res
                        }),
                    ))?;
                Ok(signature)
            }
            /// Creates an associated token account for a wallet and mint.
            ///
            /// # Arguments
            /// * `wallet` - The public key of the wallet to create the ATA for.
            /// * `mint` - The public key of the token mint.
            ///
            /// # Returns
            /// A `Result` containing the transaction signature and the ATA pubkey, or an error if the operation fails.
            pub async fn create_associated_token_account(
                &self,
                wallet: &Pubkey,
                mint: &Pubkey,
            ) -> Result<(Signature, Pubkey)> {
                let account = self.rpc_client.get_account(mint).await?;
                let token_program_id = account.owner;
                let ata_pubkey = get_associated_token_address_with_program_id(
                    wallet,
                    mint,
                    &token_program_id,
                );
                let wallet_pubkey = self.get_wallet_pubkey();
                if self.rpc_client.get_account(&ata_pubkey).await.is_ok() {
                    return Err(
                        ::anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Associated token account {0} already exists",
                                        ata_pubkey,
                                    ),
                                );
                                res
                            }),
                        ),
                    );
                }
                let instruction = create_associated_token_account(
                    &wallet_pubkey,
                    wallet,
                    mint,
                    &token_program_id,
                );
                let signature = self
                    .process_instruction(instruction, &::alloc::vec::Vec::new(), None)
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to create ATA {0}: {1}", ata_pubkey, e),
                            );
                            res
                        }),
                    ))?;
                Ok((signature, ata_pubkey))
            }
            /// Retrieves the latest blockhash and its last valid block height.
            ///
            /// # Returns
            /// A `Result` containing the blockhash and last valid block height, or an error if the query fails.
            pub async fn get_block_hash(&self) -> Result<(Hash, u64)> {
                let (blockhash, last_valid_block_height) = self
                    .rpc_client
                    .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to get blockhash: {0}", e),
                            );
                            res
                        }),
                    ))?;
                Ok((blockhash, last_valid_block_height))
            }
            /// Retrieves all token accounts owned by the wallet, optionally filtered by mint.
            ///
            /// # Arguments
            /// * `mint_pubkey` - Optional public key of the token mint to filter by.
            ///
            /// # Returns
            /// A `Result` containing a vector of token account details, or an error if the query fails.
            pub async fn get_token_accounts(
                &self,
                mint_pubkey: Option<Pubkey>,
            ) -> Result<Vec<TokenAccountDetails>> {
                let wallet_pubkey = self.get_wallet_pubkey();
                let token_accounts = if let Some(mint) = mint_pubkey {
                    self.rpc_client
                        .get_token_accounts_by_owner(
                            &wallet_pubkey,
                            TokenAccountsFilter::Mint(mint),
                        )
                        .await
                        .map_err(|e| {
                            ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get token accounts for mint {0}: {1}",
                                            mint,
                                            e,
                                        ),
                                    );
                                    res
                                }),
                            )
                        })?
                } else {
                    self.rpc_client
                        .get_token_accounts_by_owner(
                            &wallet_pubkey,
                            TokenAccountsFilter::ProgramId(spl_token::id()),
                        )
                        .await
                        .map_err(|e| {
                            ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to get token accounts for wallet {0}: {1}",
                                            wallet_pubkey,
                                            e,
                                        ),
                                    );
                                    res
                                }),
                            )
                        })?
                };
                let mut details = ::alloc::vec::Vec::new();
                for account in token_accounts {
                    let UiAccountData::Json(json_data) = &account.account.data else {
                        return Err(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("non-JSON token account data returned"),
                                );
                                error
                            }),
                        );
                    };
                    let info = json_data
                        .parsed
                        .get("info")
                        .ok_or(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("missing \'info\' field"),
                                );
                                error
                            }),
                        )?;
                    let token_account = serde_json::from_value::<
                        UiTokenAccount,
                    >(info.clone())?;
                    let amount = token_account
                        .token_amount
                        .amount
                        .parse()
                        .map_err(|e| ::anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Failed to parse token amount: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    let mint = Pubkey::from_str(&token_account.mint)
                        .map_err(|e| ::anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Failed to parse mint pubkey: {0}", e),
                                );
                                res
                            }),
                        ))?;
                    let (mint_account, _) = self.get_mint_account(&mint).await?;
                    details
                        .push(TokenAccountDetails {
                            pubkey: Pubkey::from_str(&account.pubkey)
                                .map_err(|e| ::anyhow::Error::msg(
                                    ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("Failed to parse token amount: {0}", e),
                                        );
                                        res
                                    }),
                                ))?,
                            mint,
                            amount,
                            decimals: mint_account.decimals,
                        });
                }
                Ok(details)
            }
            /// Retrieves details of a transaction by its signature.
            ///
            /// # Arguments
            /// * `signature` - The transaction signature.
            ///
            /// # Returns
            /// A `Result` containing the transaction details, or an error if the query fails.
            pub async fn get_transaction(
                &self,
                signature: &Signature,
            ) -> Result<TransactionDetails> {
                let transaction = self
                    .rpc_client
                    .get_transaction_with_config(
                        signature,
                        solana_client::rpc_config::RpcTransactionConfig {
                            commitment: Some(CommitmentConfig::confirmed()),
                            max_supported_transaction_version: Some(0),
                            encoding: Some(UiTransactionEncoding::Json),
                        },
                    )
                    .await
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to get transaction {0}: {1}",
                                    signature,
                                    e,
                                ),
                            );
                            res
                        }),
                    ))?;
                let status = match transaction.transaction.meta {
                    Some(meta) => {
                        if meta.err.is_some() {
                            "Failed".to_string()
                        } else {
                            "Confirmed".to_string()
                        }
                    }
                    None => "Unknown".to_string(),
                };
                Ok(TransactionDetails {
                    status,
                    slot: transaction.slot,
                    block_time: transaction.block_time,
                })
            }
        }
    }
    pub mod types {
        use solana_sdk::pubkey::Pubkey;
        pub struct TokenAmount {
            pub amount: u64,
            pub decimals: u8,
        }
        /// Details of a token account.
        pub struct TokenAccountDetails {
            pub pubkey: Pubkey,
            pub mint: Pubkey,
            pub amount: u64,
            pub decimals: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TokenAccountDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "TokenAccountDetails",
                    "pubkey",
                    &self.pubkey,
                    "mint",
                    &self.mint,
                    "amount",
                    &self.amount,
                    "decimals",
                    &&self.decimals,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TokenAccountDetails {
            #[inline]
            fn clone(&self) -> TokenAccountDetails {
                TokenAccountDetails {
                    pubkey: ::core::clone::Clone::clone(&self.pubkey),
                    mint: ::core::clone::Clone::clone(&self.mint),
                    amount: ::core::clone::Clone::clone(&self.amount),
                    decimals: ::core::clone::Clone::clone(&self.decimals),
                }
            }
        }
        /// Details of a transaction.
        pub struct TransactionDetails {
            pub status: String,
            pub slot: u64,
            pub block_time: Option<i64>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TransactionDetails {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "TransactionDetails",
                    "status",
                    &self.status,
                    "slot",
                    &self.slot,
                    "block_time",
                    &&self.block_time,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TransactionDetails {
            #[inline]
            fn clone(&self) -> TransactionDetails {
                TransactionDetails {
                    status: ::core::clone::Clone::clone(&self.status),
                    slot: ::core::clone::Clone::clone(&self.slot),
                    block_time: ::core::clone::Clone::clone(&self.block_time),
                }
            }
        }
    }
}
pub use solana::*;
