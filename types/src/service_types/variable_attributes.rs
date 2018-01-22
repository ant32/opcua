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

/// The attributes for a variable node.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableAttributes {
    pub specified_attributes: UInt32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
    pub write_mask: UInt32,
    pub user_write_mask: UInt32,
    pub value: Variant,
    pub data_type: NodeId,
    pub value_rank: Int32,
    pub array_dimensions: Option<Vec<UInt32>>,
    pub access_level: Byte,
    pub user_access_level: Byte,
    pub minimum_sampling_interval: Double,
    pub historizing: Boolean,
}

impl BinaryEncoder<VariableAttributes> for VariableAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.value.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += byte_len_array(&self.array_dimensions);
        size += self.access_level.byte_len();
        size += self.user_access_level.byte_len();
        size += self.minimum_sampling_interval.byte_len();
        size += self.historizing.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.value.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += write_array(stream, &self.array_dimensions)?;
        size += self.access_level.encode(stream)?;
        size += self.user_access_level.encode(stream)?;
        size += self.minimum_sampling_interval.encode(stream)?;
        size += self.historizing.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let specified_attributes = UInt32::decode(stream)?;
        let display_name = LocalizedText::decode(stream)?;
        let description = LocalizedText::decode(stream)?;
        let write_mask = UInt32::decode(stream)?;
        let user_write_mask = UInt32::decode(stream)?;
        let value = Variant::decode(stream)?;
        let data_type = NodeId::decode(stream)?;
        let value_rank = Int32::decode(stream)?;
        let array_dimensions: Option<Vec<UInt32>> = read_array(stream)?;
        let access_level = Byte::decode(stream)?;
        let user_access_level = Byte::decode(stream)?;
        let minimum_sampling_interval = Double::decode(stream)?;
        let historizing = Boolean::decode(stream)?;
        Ok(VariableAttributes {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            value,
            data_type,
            value_rank,
            array_dimensions,
            access_level,
            user_access_level,
            minimum_sampling_interval,
            historizing,
        })
    }
}
