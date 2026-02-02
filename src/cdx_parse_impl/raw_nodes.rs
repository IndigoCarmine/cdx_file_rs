//objects for reading as raw structures. It is like a Xml.

use serde::{Deserialize, Serialize};

/// CDX Object: Container for other objects and properties
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawCdxObject {
    // Header is stored with the root object
    pub tag: u16,
    pub id: u32,
    pub properties: Vec<RawCdxProperty>,
    pub children: Vec<RawCdxObject>,
}

/// CDX Property: A value associated with a tag (stored as binary data)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawCdxProperty {
    pub tag: u16,
    pub value: Vec<u8>,
}

impl RawCdxObject {
    pub fn new(tag: u16, id: u32) -> Self {
        Self {
            tag,
            id,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_property(&mut self, tag: u16, value: Vec<u8>) {
        self.properties.push(RawCdxProperty { tag, value });
    }

    pub fn get_property(&self, target_tag: u16) -> Option<&Vec<u8>> {
        for property in &self.properties {
            if property.tag == target_tag {
                return Some(&property.value);
            }
        }
        None
    }

    pub fn get_property_mut(&mut self, target_tag: u16) -> Option<&mut Vec<u8>> {
        for child in &mut self.properties {
            if child.tag == target_tag {
                return Some(&mut child.value);
            }
        }
        None
    }

    pub fn set_property(&mut self, tag: u16, value: Vec<u8>) {
        if let Some(prop) = self.get_property_mut(tag) {
            *prop = value;
        } else {
            self.properties.push(RawCdxProperty { tag, value });
        }
    }

    pub fn find_objects(&self, target_tag: u16) -> Vec<&RawCdxObject> {
        let mut results = Vec::new();
        for child in &self.children {
            if child.tag == target_tag {
                results.push(child);
            }
            results.extend(child.find_objects(target_tag));
        }
        results
    }
}
