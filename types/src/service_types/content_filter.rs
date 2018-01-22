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
use service_types::ContentFilterElement;

#[derive(Debug, Clone, PartialEq)]
pub struct ContentFilter {
    pub elements: Option<Vec<ContentFilterElement>>,
}

impl MessageInfo for ContentFilter {
    fn object_id(&self) -> ObjectId {
        ObjectId::ContentFilter_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ContentFilter> for ContentFilter {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.elements);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.elements)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let elements: Option<Vec<ContentFilterElement>> = read_array(stream)?;
        Ok(ContentFilter {
            elements,
        })
    }
}
