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
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.AnyChainBlock", len)?;
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
                formatter.write_str("struct utxorpc.v1alpha.sync.AnyChainBlock")
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
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.AnyChainBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockRef {
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
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.BlockRef", len)?;
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
impl<'de> serde::Deserialize<'de> for BlockRef {
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
            type Value = BlockRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.BlockRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlockRef, V::Error>
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
                Ok(BlockRef {
                    slot: slot__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.BlockRef", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DumpHistoryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_token.is_some() {
            len += 1;
        }
        if self.max_items != 0 {
            len += 1;
        }
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.DumpHistoryRequest", len)?;
        if let Some(v) = self.start_token.as_ref() {
            struct_ser.serialize_field("startToken", v)?;
        }
        if self.max_items != 0 {
            struct_ser.serialize_field("maxItems", &self.max_items)?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DumpHistoryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_token",
            "startToken",
            "max_items",
            "maxItems",
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartToken,
            MaxItems,
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
                            "startToken" | "start_token" => Ok(GeneratedField::StartToken),
                            "maxItems" | "max_items" => Ok(GeneratedField::MaxItems),
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
            type Value = DumpHistoryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.DumpHistoryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DumpHistoryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_token__ = None;
                let mut max_items__ = None;
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartToken => {
                            if start_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startToken"));
                            }
                            start_token__ = map_.next_value()?;
                        }
                        GeneratedField::MaxItems => {
                            if max_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxItems"));
                            }
                            max_items__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
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
                Ok(DumpHistoryRequest {
                    start_token: start_token__,
                    max_items: max_items__.unwrap_or_default(),
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.DumpHistoryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DumpHistoryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block.is_empty() {
            len += 1;
        }
        if self.next_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.DumpHistoryResponse", len)?;
        if !self.block.is_empty() {
            struct_ser.serialize_field("block", &self.block)?;
        }
        if let Some(v) = self.next_token.as_ref() {
            struct_ser.serialize_field("nextToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DumpHistoryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block",
            "next_token",
            "nextToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Block,
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
                            "block" => Ok(GeneratedField::Block),
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
            type Value = DumpHistoryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.DumpHistoryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DumpHistoryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block__ = None;
                let mut next_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextToken => {
                            if next_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextToken"));
                            }
                            next_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DumpHistoryResponse {
                    block: block__.unwrap_or_default(),
                    next_token: next_token__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.DumpHistoryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FetchBlockRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#ref.is_empty() {
            len += 1;
        }
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.FetchBlockRequest", len)?;
        if !self.r#ref.is_empty() {
            struct_ser.serialize_field("ref", &self.r#ref)?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FetchBlockRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ref",
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ref,
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
                            "ref" => Ok(GeneratedField::Ref),
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
            type Value = FetchBlockRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.FetchBlockRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FetchBlockRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#ref__ = None;
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ref => {
                            if r#ref__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ref"));
                            }
                            r#ref__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FetchBlockRequest {
                    r#ref: r#ref__.unwrap_or_default(),
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.FetchBlockRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FetchBlockResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.FetchBlockResponse", len)?;
        if !self.block.is_empty() {
            struct_ser.serialize_field("block", &self.block)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FetchBlockResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Block,
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
                            "block" => Ok(GeneratedField::Block),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FetchBlockResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.FetchBlockResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FetchBlockResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FetchBlockResponse {
                    block: block__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.FetchBlockResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FollowTipRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.intersect.is_empty() {
            len += 1;
        }
        if self.field_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.FollowTipRequest", len)?;
        if !self.intersect.is_empty() {
            struct_ser.serialize_field("intersect", &self.intersect)?;
        }
        if let Some(v) = self.field_mask.as_ref() {
            struct_ser.serialize_field("fieldMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FollowTipRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "intersect",
            "field_mask",
            "fieldMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Intersect,
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
                            "intersect" => Ok(GeneratedField::Intersect),
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
            type Value = FollowTipRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.FollowTipRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FollowTipRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut intersect__ = None;
                let mut field_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Intersect => {
                            if intersect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intersect"));
                            }
                            intersect__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldMask => {
                            if field_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldMask"));
                            }
                            field_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FollowTipRequest {
                    intersect: intersect__.unwrap_or_default(),
                    field_mask: field_mask__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.FollowTipRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FollowTipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tip.is_some() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.FollowTipResponse", len)?;
        if let Some(v) = self.tip.as_ref() {
            struct_ser.serialize_field("tip", v)?;
        }
        if let Some(v) = self.action.as_ref() {
            match v {
                follow_tip_response::Action::Apply(v) => {
                    struct_ser.serialize_field("apply", v)?;
                }
                follow_tip_response::Action::Undo(v) => {
                    struct_ser.serialize_field("undo", v)?;
                }
                follow_tip_response::Action::Reset(v) => {
                    struct_ser.serialize_field("reset", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FollowTipResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tip",
            "apply",
            "undo",
            "reset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tip,
            Apply,
            Undo,
            Reset,
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
                            "tip" => Ok(GeneratedField::Tip),
                            "apply" => Ok(GeneratedField::Apply),
                            "undo" => Ok(GeneratedField::Undo),
                            "reset" => Ok(GeneratedField::Reset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FollowTipResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.FollowTipResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FollowTipResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tip__ = None;
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tip => {
                            if tip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tip"));
                            }
                            tip__ = map_.next_value()?;
                        }
                        GeneratedField::Apply => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apply"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(follow_tip_response::Action::Apply)
;
                        }
                        GeneratedField::Undo => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("undo"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(follow_tip_response::Action::Undo)
;
                        }
                        GeneratedField::Reset => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reset"));
                            }
                            action__ = map_.next_value::<::std::option::Option<_>>()?.map(follow_tip_response::Action::Reset)
;
                        }
                    }
                }
                Ok(FollowTipResponse {
                    tip: tip__,
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.FollowTipResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadTipRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.ReadTipRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadTipRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadTipRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.ReadTipRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadTipRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReadTipRequest {
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.ReadTipRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadTipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tip.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1alpha.sync.ReadTipResponse", len)?;
        if let Some(v) = self.tip.as_ref() {
            struct_ser.serialize_field("tip", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadTipResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tip",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tip,
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
                            "tip" => Ok(GeneratedField::Tip),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadTipResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1alpha.sync.ReadTipResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadTipResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tip__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tip => {
                            if tip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tip"));
                            }
                            tip__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadTipResponse {
                    tip: tip__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1alpha.sync.ReadTipResponse", FIELDS, GeneratedVisitor)
    }
}
