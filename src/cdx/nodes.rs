use crate::cdx::geometry::Point2d;
use crate::cdx::tags;
use crate::cdx::text::StyledString;
use crate::cdx::values::CdxValue;
use serde::{Deserialize, Serialize};

/// Generic CDX Node (either an Object or a Property)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CdxNode {
    Object(CdxObject),
    Property(CdxProperty),
}

/// CDX Object: Container for other objects and properties
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CdxObject {
    pub tag: u16,
    pub id: u32,
    pub children: Vec<CdxNode>,
}

/// CDX Property: A value associated with a tag
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CdxProperty {
    pub tag: u16,
    pub value: CdxValue,
}

impl CdxObject {
    pub fn get_property(&self, target_tag: u16) -> Option<&CdxValue> {
        for child in &self.children {
            if let CdxNode::Property(prop) = child
                && prop.tag == target_tag
            {
                return Some(&prop.value);
            }
        }
        None
    }

    pub fn get_property_mut(&mut self, target_tag: u16) -> Option<&mut CdxValue> {
        for child in &mut self.children {
            if let CdxNode::Property(prop) = child
                && prop.tag == target_tag
            {
                return Some(&mut prop.value);
            }
        }
        None
    }

    pub fn set_property(&mut self, tag: u16, value: CdxValue) {
        if let Some(prop) = self.get_property_mut(tag) {
            *prop = value;
        } else {
            self.children
                .push(CdxNode::Property(CdxProperty { tag, value }));
        }
    }

    pub fn find_objects(&self, target_tag: u16) -> Vec<&CdxObject> {
        let mut results = Vec::new();
        for child in &self.children {
            if let CdxNode::Object(obj) = child {
                if obj.tag == target_tag {
                    results.push(obj);
                }
                results.extend(obj.find_objects(target_tag));
            }
        }
        results
    }

    pub fn get_pos2d(&self) -> Option<&Point2d> {
        self.get_property(tags::POSITION).and_then(|v| {
            if let CdxValue::Point2d(p) = v {
                Some(p)
            } else {
                None
            }
        })
    }

    pub fn get_pos2d_mut(&mut self) -> Option<&mut Point2d> {
        self.get_property_mut(tags::POSITION).and_then(|v| {
            if let CdxValue::Point2d(p) = v {
                Some(p)
            } else {
                None
            }
        })
    }

    pub fn get_text_styled(&self) -> Option<&StyledString> {
        self.get_property(tags::TEXT_STRING)
            .or_else(|| self.get_property(tags::TEXT_STRING_ALT))
            .and_then(|v| {
                if let CdxValue::String(s) = v {
                    Some(s)
                } else {
                    None
                }
            })
    }

    pub fn get_element_id(&self) -> Option<i16> {
        self.get_property(tags::ELEMENT).and_then(|v| {
            if let CdxValue::Int16(e) = v {
                Some(*e)
            } else {
                None
            }
        })
    }

    pub fn get_bond_double_position(&self) -> i16 {
        self.get_property(tags::BOND_DOUBLE_POSITION)
            .and_then(|v| {
                if let CdxValue::Int16(p) = v {
                    Some(*p)
                } else if let CdxValue::Uint16(p) = v {
                    Some(*p as i16)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    pub fn get_bond_order(&self) -> i16 {
        self.get_property(tags::BOND_ORDER)
            .and_then(|v| {
                if let CdxValue::Int16(o) = v {
                    Some(*o)
                } else {
                    None
                }
            })
            .unwrap_or(1)
    }

    pub fn get_bond_endpoints(&self) -> Option<(u32, u32)> {
        let b = self.get_property(tags::BOND_BEGIN).and_then(|v| {
            if let CdxValue::Uint32(id) = v {
                Some(*id)
            } else {
                None
            }
        });
        let e = self.get_property(tags::BOND_END).and_then(|v| {
            if let CdxValue::Uint32(id) = v {
                Some(*id)
            } else {
                None
            }
        });
        if let (Some(b), Some(e)) = (b, e) {
            Some((b, e))
        } else {
            None
        }
    }

    pub fn get_arrow_start_end(&self) -> Option<(Point2d, Point2d)> {
        let s = self
            .get_property(tags::TAIL_3D)
            .and_then(|v| {
                if let CdxValue::Point3d(p) = v {
                    Some(Point2d { x: p.x, y: p.y })
                } else {
                    None
                }
            })
            .or_else(|| self.get_pos2d().cloned());
        let e = self
            .get_property(tags::HEAD_3D)
            .and_then(|v| {
                if let CdxValue::Point3d(p) = v {
                    Some(Point2d { x: p.x, y: p.y })
                } else {
                    None
                }
            })
            .or_else(|| {
                if let (Some(p), Some(CdxValue::Point2d(ext))) =
                    (self.get_pos2d(), self.get_property(tags::EXTENT))
                {
                    Some(Point2d {
                        x: p.x + ext.x,
                        y: p.y + ext.y,
                    })
                } else {
                    None
                }
            });
        if let (Some(s), Some(e)) = (s, e) {
            Some((s, e))
        } else {
            None
        }
    }

    pub fn get_color_index(&self) -> Option<usize> {
        self.get_property(tags::BG_COLOR)
            .and_then(|v| {
                if let CdxValue::Int16(idx) = v {
                    Some(*idx as usize)
                } else {
                    None
                }
            })
            .or_else(|| {
                self.get_property(tags::FG_COLOR).and_then(|v| {
                    if let CdxValue::Uint16(idx) = v {
                        Some(*idx as usize)
                    } else {
                        None
                    }
                })
            })
    }
}
