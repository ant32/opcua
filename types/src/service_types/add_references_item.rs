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

/// A request to add a reference to the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct AddReferencesItem {
    pub source_node_id: NodeId,
    pub reference_type_id: NodeId,
    pub is_forward: Boolean,
    pub target_server_uri: UAString,
    pub target_node_id: ExpandedNodeId,
    pub target_node_class: NodeClass,
}

impl MessageInfo for AddReferencesItem {
    fn object_id(&self) -> ObjectId {
        ObjectId::AddReferencesItem_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AddReferencesItem> for AddReferencesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.source_node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.is_forward.byte_len();
        size += self.target_server_uri.byte_len();
        size += self.target_node_id.byte_len();
        size += self.target_node_class.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.source_node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_forward.encode(stream)?;
        size += self.target_server_uri.encode(stream)?;
        size += self.target_node_id.encode(stream)?;
        size += self.target_node_class.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let source_node_id = NodeId::decode(stream)?;
        let reference_type_id = NodeId::decode(stream)?;
        let is_forward = Boolean::decode(stream)?;
        let target_server_uri = UAString::decode(stream)?;
        let target_node_id = ExpandedNodeId::decode(stream)?;
        let target_node_class = NodeClass::decode(stream)?;
        Ok(AddReferencesItem {
            source_node_id,
            reference_type_id,
            is_forward,
            target_server_uri,
            target_node_id,
            target_node_class,
        })
    }
}
