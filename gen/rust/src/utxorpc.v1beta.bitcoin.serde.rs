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
        if self.previousblockhash.is_some() {
            len += 1;
        }
        if !self.merkleroot.is_empty() {
            len += 1;
        }
        if self.time != 0 {
            len += 1;
        }
        if self.bits != 0 {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        if !self.tx.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.bitcoin.Block", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.previousblockhash.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("previousblockhash", pbjson::private::base64::encode(&v).as_str())?;
        }
        if !self.merkleroot.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("merkleroot", pbjson::private::base64::encode(&self.merkleroot).as_str())?;
        }
        if self.time != 0 {
            struct_ser.serialize_field("time", &self.time)?;
        }
        if self.bits != 0 {
            struct_ser.serialize_field("bits", &self.bits)?;
        }
        if self.nonce != 0 {
            struct_ser.serialize_field("nonce", &self.nonce)?;
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
            "previousblockhash",
            "merkleroot",
            "time",
            "bits",
            "nonce",
            "tx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Previousblockhash,
            Merkleroot,
            Time,
            Bits,
            Nonce,
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
                            "previousblockhash" => Ok(GeneratedField::Previousblockhash),
                            "merkleroot" => Ok(GeneratedField::Merkleroot),
                            "time" => Ok(GeneratedField::Time),
                            "bits" => Ok(GeneratedField::Bits),
                            "nonce" => Ok(GeneratedField::Nonce),
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
                formatter.write_str("struct utxorpc.v1beta.bitcoin.Block")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Block, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut previousblockhash__ = None;
                let mut merkleroot__ = None;
                let mut time__ = None;
                let mut bits__ = None;
                let mut nonce__ = None;
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
                        GeneratedField::Previousblockhash => {
                            if previousblockhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousblockhash"));
                            }
                            previousblockhash__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Merkleroot => {
                            if merkleroot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("merkleroot"));
                            }
                            merkleroot__ = 
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
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
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
                    previousblockhash: previousblockhash__,
                    merkleroot: merkleroot__.unwrap_or_default(),
                    time: time__.unwrap_or_default(),
                    bits: bits__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    tx: tx__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.bitcoin.Block", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScriptPubKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asm.is_empty() {
            len += 1;
        }
        if !self.hex.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.bitcoin.ScriptPubKey", len)?;
        if !self.asm.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("asm", pbjson::private::base64::encode(&self.asm).as_str())?;
        }
        if !self.hex.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("hex", pbjson::private::base64::encode(&self.hex).as_str())?;
        }
        if !self.r#type.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("type", pbjson::private::base64::encode(&self.r#type).as_str())?;
        }
        if let Some(v) = self.address.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScriptPubKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asm",
            "hex",
            "type",
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Asm,
            Hex,
            Type,
            Address,
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
                            "asm" => Ok(GeneratedField::Asm),
                            "hex" => Ok(GeneratedField::Hex),
                            "type" => Ok(GeneratedField::Type),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScriptPubKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct utxorpc.v1beta.bitcoin.ScriptPubKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ScriptPubKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asm__ = None;
                let mut hex__ = None;
                let mut r#type__ = None;
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Asm => {
                            if asm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asm"));
                            }
                            asm__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hex => {
                            if hex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hex"));
                            }
                            hex__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ScriptPubKey {
                    asm: asm__.unwrap_or_default(),
                    hex: hex__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    address: address__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.bitcoin.ScriptPubKey", FIELDS, GeneratedVisitor)
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
        if self.blockhash.is_some() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.bitcoin.Transaction", len)?;
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
                formatter.write_str("struct utxorpc.v1beta.bitcoin.Transaction")
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
                    blockhash: blockhash__,
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.bitcoin.Transaction", FIELDS, GeneratedVisitor)
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
        if self.script_sig.is_some() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if self.coinbase.is_some() {
            len += 1;
        }
        if !self.txinwitness.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.bitcoin.TxInput", len)?;
        if let Some(v) = self.txid.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("txid", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.script_sig.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("scriptSig", pbjson::private::base64::encode(&v).as_str())?;
        }
        if self.sequence != 0 {
            struct_ser.serialize_field("sequence", &self.sequence)?;
        }
        if let Some(v) = self.coinbase.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("coinbase", pbjson::private::base64::encode(&v).as_str())?;
        }
        if !self.txinwitness.is_empty() {
            struct_ser.serialize_field("txinwitness", &self.txinwitness.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
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
            "scriptSig",
            "sequence",
            "coinbase",
            "txinwitness",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txid,
            ScriptSig,
            Sequence,
            Coinbase,
            Txinwitness,
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
                            "scriptSig" => Ok(GeneratedField::ScriptSig),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "coinbase" => Ok(GeneratedField::Coinbase),
                            "txinwitness" => Ok(GeneratedField::Txinwitness),
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
                formatter.write_str("struct utxorpc.v1beta.bitcoin.TxInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txid__ = None;
                let mut script_sig__ = None;
                let mut sequence__ = None;
                let mut coinbase__ = None;
                let mut txinwitness__ = None;
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
                        GeneratedField::ScriptSig => {
                            if script_sig__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptSig"));
                            }
                            script_sig__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
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
                        GeneratedField::Coinbase => {
                            if coinbase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coinbase"));
                            }
                            coinbase__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Txinwitness => {
                            if txinwitness__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txinwitness"));
                            }
                            txinwitness__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(TxInput {
                    txid: txid__,
                    script_sig: script_sig__,
                    sequence: sequence__.unwrap_or_default(),
                    coinbase: coinbase__,
                    txinwitness: txinwitness__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.bitcoin.TxInput", FIELDS, GeneratedVisitor)
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
        if self.script_pub_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("utxorpc.v1beta.bitcoin.TxOutput", len)?;
        if self.value != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        if let Some(v) = self.script_pub_key.as_ref() {
            struct_ser.serialize_field("scriptPubKey", v)?;
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
            "scriptPubKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            ScriptPubKey,
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
                            "scriptPubKey" => Ok(GeneratedField::ScriptPubKey),
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
                formatter.write_str("struct utxorpc.v1beta.bitcoin.TxOutput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TxOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut script_pub_key__ = None;
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
                        GeneratedField::ScriptPubKey => {
                            if script_pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scriptPubKey"));
                            }
                            script_pub_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TxOutput {
                    value: value__.unwrap_or_default(),
                    script_pub_key: script_pub_key__,
                })
            }
        }
        deserializer.deserialize_struct("utxorpc.v1beta.bitcoin.TxOutput", FIELDS, GeneratedVisitor)
    }
}
