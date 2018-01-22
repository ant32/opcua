// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use byte_string::ByteString;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::enums::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use service_types::impls::*;
#[allow(unused_imports)]
use node_ids::ObjectId;
#[allow(unused_imports)]
use status_codes::StatusCode;

/// A mapping between a value of an enumerated type and a name and description.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumValueType {
    pub value: Int64,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
}

impl MessageInfo for EnumValueType {
    fn object_id(&self) -> ObjectId {
        ObjectId::EnumValueType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EnumValueType> for EnumValueType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.value.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.value.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let value = Int64::decode(stream)?;
        let display_name = LocalizedText::decode(stream)?;
        let description = LocalizedText::decode(stream)?;
        Ok(EnumValueType {
            value,
            display_name,
            description,
        })
    }
}
