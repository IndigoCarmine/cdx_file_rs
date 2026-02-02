use crate::cdx::bracketed_group::BracketedGroup;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::bracketed_group_tags::*;
use crate::error::CdxError;

impl TaggedObject for BracketedGroup {
    const TAG: u16 = CDXOBJ_BRACKETED_GROUP;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut group = BracketedGroup::new(raw.id);

        // Parse bracket usage
        if let Some(usage_data) = raw.get_property(CDXPROP_BRACKET_USAGE) {
            if !usage_data.is_empty() {
                group.bracket_usage = Some(i8::from_le_bytes([usage_data[0]]));
            }
        }

        // Parse polymer repeat pattern
        if let Some(pattern_data) = raw.get_property(CDXPROP_POLYMER_REPEAT_PATTERN) {
            if !pattern_data.is_empty() {
                group.polymer_repeat_pattern = Some(i8::from_le_bytes([pattern_data[0]]));
            }
        }

        // Parse polymer flip type
        if let Some(flip_data) = raw.get_property(CDXPROP_POLYMER_FLIP_TYPE) {
            if !flip_data.is_empty() {
                group.polymer_flip_type = Some(i8::from_le_bytes([flip_data[0]]));
            }
        }

        // Parse bracketed objects (list of u32 IDs)
        if let Some(objects_data) = raw.get_property(CDXPROP_BRACKETED_OBJECTS) {
            let mut objects = Vec::new();
            let mut i = 0;
            while i + 4 <= objects_data.len() {
                let id = u32::from_le_bytes(
                    objects_data[i..i + 4]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bracketed_objects data".to_string()))?,
                );
                objects.push(id);
                i += 4;
            }
            if !objects.is_empty() {
                group.bracketed_objects = Some(objects);
            }
        }

        // Parse bracket repeat count
        if let Some(count_data) = raw.get_property(CDXPROP_BRACKET_REPEAT_COUNT) {
            if count_data.len() >= 2 {
                group.bracket_repeat_count = Some(i16::from_le_bytes(
                    count_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bracket_repeat_count data".to_string()))?,
                ));
            }
        }

        // Parse bracket component order (list of u32 IDs)
        if let Some(order_data) = raw.get_property(CDXPROP_BRACKET_COMPONENT_ORDER) {
            let mut order = Vec::new();
            let mut i = 0;
            while i + 4 <= order_data.len() {
                let id = u32::from_le_bytes(
                    order_data[i..i + 4]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bracket_component_order data".to_string()))?,
                );
                order.push(id);
                i += 4;
            }
            if !order.is_empty() {
                group.bracket_component_order = Some(order);
            }
        }

        // Parse bracket SRU label (string)
        if let Some(label_data) = raw.get_property(CDXPROP_BRACKET_SRU_LABEL) {
            if !label_data.is_empty() {
                // Skip style runs count (first 2 bytes) and read the string
                let text_start = 2;
                if label_data.len() > text_start {
                    let text_bytes = &label_data[text_start..];
                    // Find null terminator or use all remaining bytes
                    let end = text_bytes.iter().position(|&b| b == 0).unwrap_or(text_bytes.len());
                    let text = String::from_utf8_lossy(&text_bytes[..end]).to_string();
                    if !text.is_empty() {
                        group.bracket_sru_label = Some(text);
                    }
                }
            }
        }

        Ok(group)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        use crate::cdx_parse_impl::raw_nodes::RawCdxProperty;
        let mut properties = Vec::new();

        // Write bracket usage
        if let Some(usage) = self.bracket_usage {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKET_USAGE,
                value: vec![usage as u8],
            });
        }

        // Write polymer repeat pattern
        if let Some(pattern) = self.polymer_repeat_pattern {
            properties.push(RawCdxProperty {
                tag: CDXPROP_POLYMER_REPEAT_PATTERN,
                value: vec![pattern as u8],
            });
        }

        // Write polymer flip type
        if let Some(flip) = self.polymer_flip_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_POLYMER_FLIP_TYPE,
                value: vec![flip as u8],
            });
        }

        // Write bracketed objects
        if let Some(ref objects) = self.bracketed_objects {
            let mut bytes = Vec::new();
            for &id in objects {
                bytes.extend_from_slice(&id.to_le_bytes());
            }
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKETED_OBJECTS,
                value: bytes,
            });
        }

        // Write bracket repeat count
        if let Some(count) = self.bracket_repeat_count {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKET_REPEAT_COUNT,
                value: count.to_le_bytes().to_vec(),
            });
        }

        // Write bracket component order
        if let Some(ref order) = self.bracket_component_order {
            let mut bytes = Vec::new();
            for &id in order {
                bytes.extend_from_slice(&id.to_le_bytes());
            }
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKET_COMPONENT_ORDER,
                value: bytes,
            });
        }

        // Write bracket SRU label
        if let Some(ref label) = self.bracket_sru_label {
            let mut bytes = vec![0, 0]; // Style runs count = 0
            bytes.extend_from_slice(label.as_bytes());
            bytes.push(0); // Null terminator
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKET_SRU_LABEL,
                value: bytes,
            });
        }

        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}
