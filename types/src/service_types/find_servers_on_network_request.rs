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

#[derive(Debug, Clone, PartialEq)]
pub struct FindServersOnNetworkRequest {
    pub request_header: RequestHeader,
    pub starting_record_id: UInt32,
    pub max_records_to_return: UInt32,
    pub server_capability_filter: Option<Vec<UAString>>,
}

impl MessageInfo for FindServersOnNetworkRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::FindServersOnNetworkRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<FindServersOnNetworkRequest> for FindServersOnNetworkRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.starting_record_id.byte_len();
        size += self.max_records_to_return.byte_len();
        size += byte_len_array(&self.server_capability_filter);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.starting_record_id.encode(stream)?;
        size += self.max_records_to_return.encode(stream)?;
        size += write_array(stream, &self.server_capability_filter)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let starting_record_id = UInt32::decode(stream)?;
        let max_records_to_return = UInt32::decode(stream)?;
        let server_capability_filter: Option<Vec<UAString>> = read_array(stream)?;
        Ok(FindServersOnNetworkRequest {
            request_header,
            starting_record_id,
            max_records_to_return,
            server_capability_filter,
        })
    }
}
