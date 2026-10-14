// @generated
impl serde::Serialize for Block {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.previous_blockhash.is_some() {
            len += 1;
        }
        if !self.merkle_root.is_empty() {
            len += 1;
        }
        if !self.witness_root.is_empty() {
            len += 1;
        }
        if !self.tree_root.is_empty() {
            len += 1;
        }
        if !self.reserved_root.is_empty() {
            len += 1;
        }
        if self.time != 0 {
            len += 1;
        }
        if self.bits != 0 {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if !self.extra_nonce.is_empty() {
            len += 1;
        }
        if !self.mask.is_empty() {
            len += 1;
        }
        if !self.tx.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.handshake.Block", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.previous_blockhash.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("previousBlockhash", pbjson::private::base64::encode(&v).as_str())?;
        }
        if !self.merkle_root.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("merkleRoot", pbjson::private::base64::encode(&self.merkle_root).as_str())?;
        }
        if !self.witness_root.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("witnessRoot", pbjson::private::base64::encode(&self.witness_root).as_str())?;
        }
        if !self.tree_root.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("treeRoot", pbjson::private::base64::encode(&self.tree_root).as_str())?;
        }
        if !self.reserved_root.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("reservedRoot", pbjson::private::base64::encode(&self.reserved_root).as_str())?;
        }
        if self.time != 0 {
            struct_ser.serialize_field("time", &self.time)?;
        }
        if self.bits != 0 {
            struct_ser.serialize_field("bits", &self.bits)?;
        }
        if !self.nonce.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", pbjson::private::base64::encode(&self.nonce).as_str())?;
        }
        if !self.extra_nonce.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("extraNonce", pbjson::private::base64::encode(&self.extra_nonce).as_str())?;
        }
        if !self.mask.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("mask", pbjson::private::base64::encode(&self.mask).as_str())?;
        }
        if !self.tx.is_empty() {
            struct_ser.serialize_field("tx", &self.tx)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Block {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "previous_blockhash",
            "previousBlockhash",
            "merkle_root",
            "merkleRoot",
            "witness_root",
            "witnessRoot",
            "tree_root",
            "treeRoot",
            "reserved_root",
            "reservedRoot",
            "time",
            "bits",
            "nonce",
            "extra_nonce",
            "extraNonce",
            "mask",
            "tx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            PreviousBlockhash,
            MerkleRoot,
            WitnessRoot,
            TreeRoot,
            ReservedRoot,
            Time,
            Bits,
            Nonce,
            ExtraNonce,
            Mask,
            Tx,
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
                            "version" => Ok(GeneratedField::Version),
                            "previousBlockhash" | "previous_blockhash" => Ok(GeneratedField::PreviousBlockhash),
                            "merkleRoot" | "merkle_root" => Ok(GeneratedField::MerkleRoot),
                            "witnessRoot" | "witness_root" => Ok(GeneratedField::WitnessRoot),
                            "treeRoot" | "tree_root" => Ok(GeneratedField::TreeRoot),
                            "reservedRoot" | "reserved_root" => Ok(GeneratedField::ReservedRoot),
                            "time" => Ok(GeneratedField::Time),
                            "bits" => Ok(GeneratedField::Bits),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "extraNonce" | "extra_nonce" => Ok(GeneratedField::ExtraNonce),
                            "mask" => Ok(GeneratedField::Mask),
                            "tx" => Ok(GeneratedField::Tx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Block;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1beta.handshake.Block")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Block, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut previous_blockhash__ = None;
                let mut merkle_root__ = None;
                let mut witness_root__ = None;
                let mut tree_root__ = None;
                let mut reserved_root__ = None;
                let mut time__ = None;
                let mut bits__ = None;
                let mut nonce__ = None;
                let mut extra_nonce__ = None;
                let mut mask__ = None;
                let mut tx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PreviousBlockhash => {
                            if previous_blockhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousBlockhash"));
                            }
                            previous_blockhash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MerkleRoot => {
                            if merkle_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("merkleRoot"));
                            }
                            merkle_root__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::WitnessRoot => {
                            if witness_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("witnessRoot"));
                            }
                            witness_root__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TreeRoot => {
                            if tree_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("treeRoot"));
                            }
                            tree_root__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReservedRoot => {
                            if reserved_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservedRoot"));
                            }
                            reserved_root__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Bits => {
                            if bits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bits"));
                            }
                            bits__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExtraNonce => {
                            if extra_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extraNonce"));
                            }
                            extra_nonce__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Mask => {
                            if mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mask"));
                            }
                            mask__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Block {
                    version: version__.unwrap_or_default(),
                    previous_blockhash: previous_blockhash__,
                    merkle_root: merkle_root__.unwrap_or_default(),
                    witness_root: witness_root__.unwrap_or_default(),
                    tree_root: tree_root__.unwrap_or_default(),
                    reserved_root: reserved_root__.unwrap_or_default(),
                    time: time__.unwrap_or_default(),
                    bits: bits__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    extra_nonce: extra_nonce__.unwrap_or_default(),
                    mask: mask__.unwrap_or_default(),
                    tx: tx__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.handshake.Block", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Covenant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action != 0 {
            len += 1;
        }
        if !self.items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.handshake.Covenant", len)?;
        if self.action != 0 {
            let v = CovenantAction::try_from(self.action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Covenant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action",
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Action,
            Items,
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
                            "action" => Ok(GeneratedField::Action),
                            "items" => Ok(GeneratedField::Items),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Covenant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1beta.handshake.Covenant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Covenant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                let mut items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map_.next_value::<CovenantAction>()? as i32);
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(Covenant {
                    action: action__.unwrap_or_default(),
                    items: items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.handshake.Covenant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CovenantAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COVENANT_ACTION_UNSPECIFIED",
            Self::Claim => "COVENANT_ACTION_CLAIM",
            Self::Open => "COVENANT_ACTION_OPEN",
            Self::Bid => "COVENANT_ACTION_BID",
            Self::Reveal => "COVENANT_ACTION_REVEAL",
            Self::Redeem => "COVENANT_ACTION_REDEEM",
            Self::Register => "COVENANT_ACTION_REGISTER",
            Self::Update => "COVENANT_ACTION_UPDATE",
            Self::Renew => "COVENANT_ACTION_RENEW",
            Self::Transfer => "COVENANT_ACTION_TRANSFER",
            Self::Finalize => "COVENANT_ACTION_FINALIZE",
            Self::Revoke => "COVENANT_ACTION_REVOKE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CovenantAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COVENANT_ACTION_UNSPECIFIED",
            "COVENANT_ACTION_CLAIM",
            "COVENANT_ACTION_OPEN",
            "COVENANT_ACTION_BID",
            "COVENANT_ACTION_REVEAL",
            "COVENANT_ACTION_REDEEM",
            "COVENANT_ACTION_REGISTER",
            "COVENANT_ACTION_UPDATE",
            "COVENANT_ACTION_RENEW",
            "COVENANT_ACTION_TRANSFER",
            "COVENANT_ACTION_FINALIZE",
            "COVENANT_ACTION_REVOKE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CovenantAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COVENANT_ACTION_UNSPECIFIED" => Ok(CovenantAction::Unspecified),
                    "COVENANT_ACTION_CLAIM" => Ok(CovenantAction::Claim),
                    "COVENANT_ACTION_OPEN" => Ok(CovenantAction::Open),
                    "COVENANT_ACTION_BID" => Ok(CovenantAction::Bid),
                    "COVENANT_ACTION_REVEAL" => Ok(CovenantAction::Reveal),
                    "COVENANT_ACTION_REDEEM" => Ok(CovenantAction::Redeem),
                    "COVENANT_ACTION_REGISTER" => Ok(CovenantAction::Register),
                    "COVENANT_ACTION_UPDATE" => Ok(CovenantAction::Update),
                    "COVENANT_ACTION_RENEW" => Ok(CovenantAction::Renew),
                    "COVENANT_ACTION_TRANSFER" => Ok(CovenantAction::Transfer),
                    "COVENANT_ACTION_FINALIZE" => Ok(CovenantAction::Finalize),
                    "COVENANT_ACTION_REVOKE" => Ok(CovenantAction::Revoke),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Transaction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.vin.is_empty() {
            len += 1;
        }
        if !self.vout.is_empty() {
            len += 1;
        }
        if self.locktime != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.witness_hash.is_some() {
            len += 1;
        }
        if self.blockhash.is_some() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.handshake.Transaction", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.vin.is_empty() {
            struct_ser.serialize_field("vin", &self.vin)?;
        }
        if !self.vout.is_empty() {
            struct_ser.serialize_field("vout", &self.vout)?;
        }
        if self.locktime != 0 {
            struct_ser.serialize_field("locktime", &self.locktime)?;
        }
        if !self.hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if let Some(v) = self.witness_hash.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("witnessHash", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.blockhash.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("blockhash", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Transaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "vin",
            "vout",
            "locktime",
            "hash",
            "witness_hash",
            "witnessHash",
            "blockhash",
            "time",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Vin,
            Vout,
            Locktime,
            Hash,
            WitnessHash,
            Blockhash,
            Time,
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
                            "version" => Ok(GeneratedField::Version),
                            "vin" => Ok(GeneratedField::Vin),
                            "vout" => Ok(GeneratedField::Vout),
                            "locktime" => Ok(GeneratedField::Locktime),
                            "hash" => Ok(GeneratedField::Hash),
                            "witnessHash" | "witness_hash" => Ok(GeneratedField::WitnessHash),
                            "blockhash" => Ok(GeneratedField::Blockhash),
                            "time" => Ok(GeneratedField::Time),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Transaction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1beta.handshake.Transaction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Transaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut vin__ = None;
                let mut vout__ = None;
                let mut locktime__ = None;
                let mut hash__ = None;
                let mut witness_hash__ = None;
                let mut blockhash__ = None;
                let mut time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vin => {
                            if vin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vin"));
                            }
                            vin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vout => {
                            if vout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vout"));
                            }
                            vout__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Locktime => {
                            if locktime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locktime"));
                            }
                            locktime__ = 
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
                        GeneratedField::WitnessHash => {
                            if witness_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("witnessHash"));
                            }
                            witness_hash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Blockhash => {
                            if blockhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockhash"));
                            }
                            blockhash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Transaction {
                    version: version__.unwrap_or_default(),
                    vin: vin__.unwrap_or_default(),
                    vout: vout__.unwrap_or_default(),
                    locktime: locktime__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    witness_hash: witness_hash__,
                    blockhash: blockhash__,
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.handshake.Transaction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.txid.is_some() {
            len += 1;
        }
        if self.output_index != 0 {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if !self.witness.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.handshake.TxInput", len)?;
        if let Some(v) = self.txid.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("txid", pbjson::private::base64::encode(&v).as_str())?;
        }
        if self.output_index != 0 {
            struct_ser.serialize_field("outputIndex", &self.output_index)?;
        }
        if self.sequence != 0 {
            struct_ser.serialize_field("sequence", &self.sequence)?;
        }
        if !self.witness.is_empty() {
            struct_ser.serialize_field("witness", &self.witness.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txid",
            "output_index",
            "outputIndex",
            "sequence",
            "witness",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txid,
            OutputIndex,
            Sequence,
            Witness,
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
                            "txid" => Ok(GeneratedField::Txid),
                            "outputIndex" | "output_index" => Ok(GeneratedField::OutputIndex),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "witness" => Ok(GeneratedField::Witness),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1beta.handshake.TxInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txid__ = None;
                let mut output_index__ = None;
                let mut sequence__ = None;
                let mut witness__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OutputIndex => {
                            if output_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputIndex"));
                            }
                            output_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Witness => {
                            if witness__.is_some() {
                                return Err(serde::de::Error::duplicate_field("witness"));
                            }
                            witness__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(TxInput {
                    txid: txid__,
                    output_index: output_index__.unwrap_or_default(),
                    sequence: sequence__.unwrap_or_default(),
                    witness: witness__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.handshake.TxInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxOutput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.covenant.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.handshake.TxOutput", len)?;
        if self.value != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        if !self.address.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        if let Some(v) = self.covenant.as_ref() {
            struct_ser.serialize_field("covenant", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxOutput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "address",
            "covenant",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Address,
            Covenant,
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
                            "address" => Ok(GeneratedField::Address),
                            "covenant" => Ok(GeneratedField::Covenant),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxOutput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1beta.handshake.TxOutput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut address__ = None;
                let mut covenant__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Covenant => {
                            if covenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("covenant"));
                            }
                            covenant__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TxOutput {
                    value: value__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    covenant: covenant__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.handshake.TxOutput", FIELDS, GeneratedVisitor)
    }
}
