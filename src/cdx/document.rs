use crate::cdx::nodes::{CdxNode, CdxObject};
use crate::cdx::tags;
use serde::{Deserialize, Serialize};

/// The entire CDX Document
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CdxDocument {
    pub header: crate::CdxHeader,
    pub root: Vec<CdxNode>,
}

impl CdxDocument {
    pub fn document_object(&self) -> Option<&CdxObject> {
        for node in &self.root {
            if let CdxNode::Object(obj) = node
                && obj.tag == tags::DOCUMENT {
                    return Some(obj);
                }
        }
        None
    }

    pub fn document_object_mut(&mut self) -> Option<&mut CdxObject> {
        for node in &mut self.root {
            if let CdxNode::Object(obj) = node
                && obj.tag == tags::DOCUMENT {
                    return Some(obj);
                }
        }
        None
    }

    pub fn find_object(&self, id: u32) -> Option<&CdxObject> {
        for node in &self.root {
            if let Some(found) = Self::find_in_node(node, id) {
                return Some(found);
            }
        }
        None
    }

    fn find_in_node(node: &CdxNode, id: u32) -> Option<&CdxObject> {
        if let CdxNode::Object(obj) = node {
            if obj.id == id {
                return Some(obj);
            }
            for child in &obj.children {
                if let Some(found) = Self::find_in_node(child, id) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn find_object_mut(&mut self, id: u32) -> Option<&mut CdxObject> {
        for node in &mut self.root {
            if let Some(found) = Self::find_in_node_mut(node, id) {
                return Some(found);
            }
        }
        None
    }

    fn find_in_node_mut(node: &mut CdxNode, id: u32) -> Option<&mut CdxObject> {
        if let CdxNode::Object(obj) = node {
            if obj.id == id {
                return Some(obj);
            }
            for child in &mut obj.children {
                if let Some(found) = Self::find_in_node_mut(child, id) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn max_id(&self) -> u32 {
        let mut max = 0;
        for node in &self.root {
            max = max.max(Self::max_id_node(node));
        }
        max
    }

    fn max_id_node(node: &CdxNode) -> u32 {
        if let CdxNode::Object(obj) = node {
            let mut max = obj.id;
            for child in &obj.children {
                max = max.max(Self::max_id_node(child));
            }
            max
        } else {
            0
        }
    }

    pub fn add_to_parent_of(&mut self, target_id: u32, new_node: CdxNode) -> bool {
        for node in &mut self.root {
            if Self::add_to_parent_recursive(node, target_id, &new_node) {
                return true;
            }
        }
        self.root.push(new_node);
        true
    }

    fn add_to_parent_recursive(node: &mut CdxNode, target_id: u32, new_node: &CdxNode) -> bool {
        if let CdxNode::Object(obj) = node {
            for child in &obj.children {
                if let CdxNode::Object(child_obj) = child
                    && child_obj.id == target_id {
                        obj.children.push(new_node.clone());
                        return true;
                    }
            }
            for child in &mut obj.children {
                if Self::add_to_parent_recursive(child, target_id, new_node) {
                    return true;
                }
            }
        }
        false
    }

    pub fn delete_object(&mut self, id: u32) -> bool {
        let mut found = false;
        self.root.retain(|node| {
            if let CdxNode::Object(obj) = node
                && obj.id == id {
                    found = true;
                    return false;
                }
            true
        });
        if found {
            return true;
        }
        for node in &mut self.root {
            if Self::delete_recursive(node, id) {
                return true;
            }
        }
        false
    }

    fn delete_recursive(node: &mut CdxNode, id: u32) -> bool {
        if let CdxNode::Object(obj) = node {
            let mut found = false;
            obj.children.retain(|child| {
                if let CdxNode::Object(child_obj) = child
                    && child_obj.id == id {
                        found = true;
                        return false;
                    }
                true
            });
            if found {
                return true;
            }
            for child in &mut obj.children {
                if Self::delete_recursive(child, id) {
                    return true;
                }
            }
        }
        false
    }
}
