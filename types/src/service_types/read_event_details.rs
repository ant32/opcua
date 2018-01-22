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
use service_types::EventFilter;

#[derive(Debug, Clone, PartialEq)]
pub struct ReadEventDetails {
    pub num_values_per_node: UInt32,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub filter: EventFilter,
}

impl BinaryEncoder<ReadEventDetails> for ReadEventDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.num_values_per_node.byte_len();
        size += self.start_time.byte_len();
        size += self.end_time.byte_len();
        size += self.filter.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.num_values_per_node.encode(stream)?;
        size += self.start_time.encode(stream)?;
        size += self.end_time.encode(stream)?;
        size += self.filter.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let num_values_per_node = UInt32::decode(stream)?;
        let start_time = DateTime::decode(stream)?;
        let end_time = DateTime::decode(stream)?;
        let filter = EventFilter::decode(stream)?;
        Ok(ReadEventDetails {
            num_values_per_node,
            start_time,
            end_time,
            filter,
        })
    }
}
