// @generated
impl serde::Serialize for AnyChainBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.native_bytes.is_empty() {
            len += 1;
        }
        if self.chain.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.AnyChainBlock", len)?;
        if !self.native_bytes.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nativeBytes", pbjson::private::base64::encode(&self.native_bytes).as_str())?;
        }
        if let Some(v) = self.chain.as_ref() {
            match v {
                any_chain_block::Chain::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyChainBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "native_bytes",
            "nativeBytes",
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NativeBytes,
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nativeBytes" | "native_bytes" => Ok(GeneratedField::NativeBytes),
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyChainBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.AnyChainBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyChainBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut native_bytes__ = None;
                let mut chain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NativeBytes => {
                            if native_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nativeBytes"));
                            }
                            native_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cardano => {
                            if chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            chain__ = map_.next_value::<::std::option::Option<_>>()?.map(any_chain_block::Chain::Cardano)
;
                        }
                    }
                }
                Ok(AnyChainBlock {
                    native_bytes: native_bytes__.unwrap_or_default(),
                    chain: chain__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.AnyChainBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnyChainDatum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.native_bytes.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if self.parsed_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.AnyChainDatum", len)?;
        if !self.native_bytes.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nativeBytes", pbjson::private::base64::encode(&self.native_bytes).as_str())?;
        }
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if let Some(v) = self.parsed_state.as_ref() {
            match v {
                any_chain_datum::ParsedState::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyChainDatum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "native_bytes",
            "nativeBytes",
            "key",
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NativeBytes,
            Key,
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nativeBytes" | "native_bytes" => Ok(GeneratedField::NativeBytes),
                            "key" => Ok(GeneratedField::Key),
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyChainDatum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.AnyChainDatum")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyChainDatum, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut native_bytes__ = None;
                let mut key__ = None;
                let mut parsed_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NativeBytes => {
                            if native_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nativeBytes"));
                            }
                            native_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cardano => {
                            if parsed_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            parsed_state__ = map_.next_value::<::std::option::Option<_>>()?.map(any_chain_datum::ParsedState::Cardano)
;
                        }
                    }
                }
                Ok(AnyChainDatum {
                    native_bytes: native_bytes__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    parsed_state: parsed_state__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.AnyChainDatum", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnyChainParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.AnyChainParams", len)?;
        if let Some(v) = self.params.as_ref() {
            match v {
                any_chain_params::Params::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyChainParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyChainParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.AnyChainParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyChainParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cardano => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            params__ = map_.next_value::<::std::option::Option<_>>()?.map(any_chain_params::Params::Cardano)
;
                        }
                    }
                }
                Ok(AnyChainParams {
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.AnyChainParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnyChainStateData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.AnyChainStateData", len)?;
        if let Some(v) = self.result.as_ref() {
            match v {
                any_chain_state_data::Result::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyChainStateData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyChainStateData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.AnyChainStateData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyChainStateData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cardano => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(any_chain_state_data::Result::Cardano)
;
                        }
                    }
                }
                Ok(AnyChainStateData {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.AnyChainStateData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnyChainStateQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.query.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.AnyChainStateQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                any_chain_state_query::Query::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyChainStateQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyChainStateQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.AnyChainStateQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyChainStateQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cardano => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(any_chain_state_query::Query::Cardano)
;
                        }
                    }
                }
                Ok(AnyChainStateQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.AnyChainStateQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnyChainTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.native_bytes.is_empty() {
            len += 1;
        }
        if self.block_ref.is_some() {
            len += 1;
        }
        if self.chain.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.AnyChainTx", len)?;
        if !self.native_bytes.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nativeBytes", pbjson::private::base64::encode(&self.native_bytes).as_str())?;
        }
        if let Some(v) = self.block_ref.as_ref() {
            struct_ser.serialize_field("blockRef", v)?;
        }
        if let Some(v) = self.chain.as_ref() {
            match v {
                any_chain_tx::Chain::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyChainTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "native_bytes",
            "nativeBytes",
            "block_ref",
            "blockRef",
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NativeBytes,
            BlockRef,
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nativeBytes" | "native_bytes" => Ok(GeneratedField::NativeBytes),
                            "blockRef" | "block_ref" => Ok(GeneratedField::BlockRef),
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyChainTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.AnyChainTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyChainTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut native_bytes__ = None;
                let mut block_ref__ = None;
                let mut chain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NativeBytes => {
                            if native_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nativeBytes"));
                            }
                            native_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockRef => {
                            if block_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockRef"));
                            }
                            block_ref__ = map_.next_value()?;
                        }
                        GeneratedField::Cardano => {
                            if chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            chain__ = map_.next_value::<::std::option::Option<_>>()?.map(any_chain_tx::Chain::Cardano)
;
                        }
                    }
                }
                Ok(AnyChainTx {
                    native_bytes: native_bytes__.unwrap_or_default(),
                    block_ref: block_ref__,
                    chain: chain__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.AnyChainTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnyUtxoData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.native_bytes.is_empty() {
            len += 1;
        }
        if self.txo_ref.is_some() {
            len += 1;
        }
        if self.block_ref.is_some() {
            len += 1;
        }
        if self.parsed_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.AnyUtxoData", len)?;
        if !self.native_bytes.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nativeBytes", pbjson::private::base64::encode(&self.native_bytes).as_str())?;
        }
        if let Some(v) = self.txo_ref.as_ref() {
            struct_ser.serialize_field("txoRef", v)?;
        }
        if let Some(v) = self.block_ref.as_ref() {
            struct_ser.serialize_field("blockRef", v)?;
        }
        if let Some(v) = self.parsed_state.as_ref() {
            match v {
                any_utxo_data::ParsedState::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyUtxoData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "native_bytes",
            "nativeBytes",
            "txo_ref",
            "txoRef",
            "block_ref",
            "blockRef",
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NativeBytes,
            TxoRef,
            BlockRef,
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nativeBytes" | "native_bytes" => Ok(GeneratedField::NativeBytes),
                            "txoRef" | "txo_ref" => Ok(GeneratedField::TxoRef),
                            "blockRef" | "block_ref" => Ok(GeneratedField::BlockRef),
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyUtxoData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.AnyUtxoData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyUtxoData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut native_bytes__ = None;
                let mut txo_ref__ = None;
                let mut block_ref__ = None;
                let mut parsed_state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NativeBytes => {
                            if native_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nativeBytes"));
                            }
                            native_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TxoRef => {
                            if txo_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txoRef"));
                            }
                            txo_ref__ = map_.next_value()?;
                        }
                        GeneratedField::BlockRef => {
                            if block_ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockRef"));
                            }
                            block_ref__ = map_.next_value()?;
                        }
                        GeneratedField::Cardano => {
                            if parsed_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            parsed_state__ = map_.next_value::<::std::option::Option<_>>()?.map(any_utxo_data::ParsedState::Cardano)
;
                        }
                    }
                }
                Ok(AnyUtxoData {
                    native_bytes: native_bytes__.unwrap_or_default(),
                    txo_ref: txo_ref__,
                    block_ref: block_ref__,
                    parsed_state: parsed_state__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.AnyUtxoData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnyUtxoPattern {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.utxo_pattern.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.AnyUtxoPattern", len)?;
        if let Some(v) = self.utxo_pattern.as_ref() {
            match v {
                any_utxo_pattern::UtxoPattern::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyUtxoPattern {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyUtxoPattern;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.AnyUtxoPattern")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyUtxoPattern, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut utxo_pattern__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cardano => {
                            if utxo_pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            utxo_pattern__ = map_.next_value::<::std::option::Option<_>>()?.map(any_utxo_pattern::UtxoPattern::Cardano)
;
                        }
                    }
                }
                Ok(AnyUtxoPattern {
                    utxo_pattern: utxo_pattern__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.AnyUtxoPattern", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChainPoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.slot != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ChainPoint", len)?;
        if self.slot != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("slot", ToString::to_string(&self.slot).as_str())?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChainPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "slot",
            "hash",
            "height",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Slot,
            Hash,
            Height,
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "slot" => Ok(GeneratedField::Slot),
                            "hash" => Ok(GeneratedField::Hash),
                            "height" => Ok(GeneratedField::Height),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChainPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ChainPoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChainPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut slot__ = None;
                let mut hash__ = None;
                let mut height__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Slot => {
                            if slot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slot"));
                            }
                            slot__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ChainPoint {
                    slot: slot__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ChainPoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadDataRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadDataRequest", len)?;
        if !self.key.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadDataRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            FieldMask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadDataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadDataRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadDataRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadDataRequest {
                    key: key__.unwrap_or_default(),
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadDataRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadDataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        if self.ledger_tip.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadDataResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.ledger_tip.as_ref() {
            struct_ser.serialize_field("ledgerTip", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadDataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "ledger_tip",
            "ledgerTip",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            LedgerTip,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "ledgerTip" | "ledger_tip" => Ok(GeneratedField::LedgerTip),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadDataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadDataResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadDataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut ledger_tip__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::LedgerTip => {
                            if ledger_tip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledgerTip"));
                            }
                            ledger_tip__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadDataResponse {
                    value: value__,
                    ledger_tip: ledger_tip__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadDataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadEraSummaryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadEraSummaryRequest", len)?;
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadEraSummaryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldMask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadEraSummaryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadEraSummaryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadEraSummaryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadEraSummaryRequest {
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadEraSummaryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadEraSummaryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.summary.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadEraSummaryResponse", len)?;
        if let Some(v) = self.summary.as_ref() {
            match v {
                read_era_summary_response::Summary::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadEraSummaryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadEraSummaryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadEraSummaryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadEraSummaryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut summary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cardano => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            summary__ = map_.next_value::<::std::option::Option<_>>()?.map(read_era_summary_response::Summary::Cardano)
;
                        }
                    }
                }
                Ok(ReadEraSummaryResponse {
                    summary: summary__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadEraSummaryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadGenesisRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadGenesisRequest", len)?;
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadGenesisRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldMask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadGenesisRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadGenesisRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadGenesisRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadGenesisRequest {
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadGenesisRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadGenesisResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.genesis.is_empty() {
            len += 1;
        }
        if !self.caip2.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadGenesisResponse", len)?;
        if !self.genesis.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("genesis", pbjson::private::base64::encode(&self.genesis).as_str())?;
        }
        if !self.caip2.is_empty() {
            struct_ser.serialize_field("caip2", &self.caip2)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                read_genesis_response::Config::Cardano(v) => {
                    struct_ser.serialize_field("cardano", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadGenesisResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "genesis",
            "caip2",
            "cardano",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Genesis,
            Caip2,
            Cardano,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "genesis" => Ok(GeneratedField::Genesis),
                            "caip2" => Ok(GeneratedField::Caip2),
                            "cardano" => Ok(GeneratedField::Cardano),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadGenesisResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadGenesisResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadGenesisResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut genesis__ = None;
                let mut caip2__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Genesis => {
                            if genesis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genesis"));
                            }
                            genesis__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Caip2 => {
                            if caip2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caip2"));
                            }
                            caip2__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cardano => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cardano"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(read_genesis_response::Config::Cardano)
;
                        }
                    }
                }
                Ok(ReadGenesisResponse {
                    genesis: genesis__.unwrap_or_default(),
                    caip2: caip2__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadGenesisResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadParamsRequest", len)?;
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldMask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadParamsRequest {
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadParamsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.values.is_some() {
            len += 1;
        }
        if self.ledger_tip.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadParamsResponse", len)?;
        if let Some(v) = self.values.as_ref() {
            struct_ser.serialize_field("values", v)?;
        }
        if let Some(v) = self.ledger_tip.as_ref() {
            struct_ser.serialize_field("ledgerTip", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
            "ledger_tip",
            "ledgerTip",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
            LedgerTip,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            "ledgerTip" | "ledger_tip" => Ok(GeneratedField::LedgerTip),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                let mut ledger_tip__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = map_.next_value()?;
                        }
                        GeneratedField::LedgerTip => {
                            if ledger_tip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledgerTip"));
                            }
                            ledger_tip__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadParamsResponse {
                    values: values__,
                    ledger_tip: ledger_tip__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadParamsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.query.is_some() {
            len += 1;
        }
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadStateRequest", len)?;
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadStateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            FieldMask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "query" => Ok(GeneratedField::Query),
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadStateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadStateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadStateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadStateRequest {
                    query: query__,
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadStateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        if self.ledger_tip.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadStateResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        if let Some(v) = self.ledger_tip.as_ref() {
            struct_ser.serialize_field("ledgerTip", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
            "ledger_tip",
            "ledgerTip",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
            LedgerTip,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "result" => Ok(GeneratedField::Result),
                            "ledgerTip" | "ledger_tip" => Ok(GeneratedField::LedgerTip),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadStateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadStateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadStateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                let mut ledger_tip__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map_.next_value()?;
                        }
                        GeneratedField::LedgerTip => {
                            if ledger_tip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledgerTip"));
                            }
                            ledger_tip__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadStateResponse {
                    result: result__,
                    ledger_tip: ledger_tip__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadStateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadTxRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadTxRequest", len)?;
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadTxRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            FieldMask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadTxRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadTxRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadTxRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadTxRequest {
                    hash: hash__.unwrap_or_default(),
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadTxRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadTxResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tx.is_some() {
            len += 1;
        }
        if self.ledger_tip.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadTxResponse", len)?;
        if let Some(v) = self.tx.as_ref() {
            struct_ser.serialize_field("tx", v)?;
        }
        if let Some(v) = self.ledger_tip.as_ref() {
            struct_ser.serialize_field("ledgerTip", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadTxResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx",
            "ledger_tip",
            "ledgerTip",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tx,
            LedgerTip,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tx" => Ok(GeneratedField::Tx),
                            "ledgerTip" | "ledger_tip" => Ok(GeneratedField::LedgerTip),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadTxResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadTxResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadTxResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tx__ = None;
                let mut ledger_tip__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = map_.next_value()?;
                        }
                        GeneratedField::LedgerTip => {
                            if ledger_tip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledgerTip"));
                            }
                            ledger_tip__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadTxResponse {
                    tx: tx__,
                    ledger_tip: ledger_tip__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadTxResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadUtxosRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadUtxosRequest", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadUtxosRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
            FieldMask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keys" => Ok(GeneratedField::Keys),
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadUtxosRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadUtxosRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadUtxosRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadUtxosRequest {
                    keys: keys__.unwrap_or_default(),
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadUtxosRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadUtxosResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if self.ledger_tip.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.ReadUtxosResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.ledger_tip.as_ref() {
            struct_ser.serialize_field("ledgerTip", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadUtxosResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "ledger_tip",
            "ledgerTip",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            LedgerTip,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            "ledgerTip" | "ledger_tip" => Ok(GeneratedField::LedgerTip),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadUtxosResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.ReadUtxosResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadUtxosResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut ledger_tip__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LedgerTip => {
                            if ledger_tip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledgerTip"));
                            }
                            ledger_tip__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadUtxosResponse {
                    items: items__.unwrap_or_default(),
                    ledger_tip: ledger_tip__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.ReadUtxosResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchUtxosRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.predicate.is_some() {
            len += 1;
        }
        if self.field_mask.is_some() {
            len += 1;
        }
        if self.max_items.is_some() {
            len += 1;
        }
        if self.start_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.SearchUtxosRequest", len)?;
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        if let Some(v) = self.max_items.as_ref() {
            struct_ser.serialize_field("maxItems", v)?;
        }
        if let Some(v) = self.start_token.as_ref() {
            struct_ser.serialize_field("startToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchUtxosRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate",
            "field_mask",
            "fieldMask",
            "max_items",
            "maxItems",
            "start_token",
            "startToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Predicate,
            FieldMask,
            MaxItems,
            StartToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "predicate" => Ok(GeneratedField::Predicate),
                            "fieldMask" | "field_mask" => Ok(GeneratedField::FieldMask),
                            "maxItems" | "max_items" => Ok(GeneratedField::MaxItems),
                            "startToken" | "start_token" => Ok(GeneratedField::StartToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchUtxosRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.SearchUtxosRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchUtxosRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut predicate__ = None;
                let mut field_mask__ = None;
                let mut max_items__ = None;
                let mut start_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Predicate => {
                            if predicate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate__ = map_.next_value()?;
                        }
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                        GeneratedField::MaxItems => {
                            if max_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxItems"));
                            }
                            max_items__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::StartToken => {
                            if start_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startToken"));
                            }
                            start_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SearchUtxosRequest {
                    predicate: predicate__,
                    field_mask: field_mask__,
                    max_items: max_items__,
                    start_token: start_token__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.SearchUtxosRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchUtxosResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if self.ledger_tip.is_some() {
            len += 1;
        }
        if self.next_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.SearchUtxosResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.ledger_tip.as_ref() {
            struct_ser.serialize_field("ledgerTip", v)?;
        }
        if let Some(v) = self.next_token.as_ref() {
            struct_ser.serialize_field("nextToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchUtxosResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "ledger_tip",
            "ledgerTip",
            "next_token",
            "nextToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            LedgerTip,
            NextToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "items" => Ok(GeneratedField::Items),
                            "ledgerTip" | "ledger_tip" => Ok(GeneratedField::LedgerTip),
                            "nextToken" | "next_token" => Ok(GeneratedField::NextToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchUtxosResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.SearchUtxosResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchUtxosResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut ledger_tip__ = None;
                let mut next_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LedgerTip => {
                            if ledger_tip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledgerTip"));
                            }
                            ledger_tip__ = map_.next_value()?;
                        }
                        GeneratedField::NextToken => {
                            if next_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextToken"));
                            }
                            next_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SearchUtxosResponse {
                    items: items__.unwrap_or_default(),
                    ledger_tip: ledger_tip__,
                    next_token: next_token__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.SearchUtxosResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxoRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.TxoRef", len)?;
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxoRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "index",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Index,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "index" => Ok(GeneratedField::Index),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxoRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.TxoRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxoRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TxoRef {
                    hash: hash__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.TxoRef", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UtxoPredicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#match.is_some() {
            len += 1;
        }
        if !self.not.is_empty() {
            len += 1;
        }
        if !self.all_of.is_empty() {
            len += 1;
        }
        if !self.any_of.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1.query.UtxoPredicate", len)?;
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if !self.not.is_empty() {
            struct_ser.serialize_field("not", &self.not)?;
        }
        if !self.all_of.is_empty() {
            struct_ser.serialize_field("allOf", &self.all_of)?;
        }
        if !self.any_of.is_empty() {
            struct_ser.serialize_field("anyOf", &self.any_of)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UtxoPredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "match",
            "not",
            "all_of",
            "allOf",
            "any_of",
            "anyOf",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Match,
            Not,
            AllOf,
            AnyOf,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "match" => Ok(GeneratedField::Match),
                            "not" => Ok(GeneratedField::Not),
                            "allOf" | "all_of" => Ok(GeneratedField::AllOf),
                            "anyOf" | "any_of" => Ok(GeneratedField::AnyOf),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UtxoPredicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1.query.UtxoPredicate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UtxoPredicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#match__ = None;
                let mut not__ = None;
                let mut all_of__ = None;
                let mut any_of__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map_.next_value()?;
                        }
                        GeneratedField::Not => {
                            if not__.is_some() {
                                return Err(serde::de::Error::duplicate_field("not"));
                            }
                            not__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllOf => {
                            if all_of__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allOf"));
                            }
                            all_of__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnyOf => {
                            if any_of__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anyOf"));
                            }
                            any_of__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UtxoPredicate {
                    r#match: r#match__,
                    not: not__.unwrap_or_default(),
                    all_of: all_of__.unwrap_or_default(),
                    any_of: any_of__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1.query.UtxoPredicate", FIELDS, GeneratedVisitor)
    }
}
