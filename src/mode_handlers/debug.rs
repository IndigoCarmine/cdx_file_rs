use crate::modes::{ModeContext, ModeHandler};
use crate::cdx::file::NodePayload;
use crate::renderer::element_to_symbol;
use eframe::egui;
use dendron::Node;

pub struct DebugMode {
    hovered_object_info: Option<ObjectInfo>,
    selected_object_info: Option<ObjectInfo>,
}

#[derive(Clone, Debug)]
pub struct ObjectInfo {
    pub object_type: String,
    pub id: Option<u32>,
    pub position: String,
    pub details: Vec<(String, String)>,
}

impl DebugMode {
    pub fn new() -> Self {
        DebugMode {
            hovered_object_info: None,
            selected_object_info: None,
        }
    }

    fn get_object_at_position(
        &self,
        ctx: &ModeContext,
        cdx_file: &crate::cdx::file::CdxFile,
    ) -> Option<ObjectInfo> {
        let mouse_screen = ctx.mouse_pos;
        let scale = ctx.renderer.zoom * ctx.renderer.auto_scale;
        
        // Convert screen position back to CDX coordinates
        let center_offset = ctx.renderer.center_offset;
        let offset = ctx.renderer.offset;
        let screen_x = mouse_screen.x - center_offset.x - offset.x;
        let screen_y = mouse_screen.y - center_offset.y - offset.y;
        
        let cdx_x = screen_x / scale;
        let cdx_y = -screen_y / scale;
        
        // Search through nodes to find which is closest to mouse
        let tree = &cdx_file.tree;
        let root = tree.root();
        self.search_node_at_position(root, cdx_x as f64, cdx_y as f64, 20.0) // 20px hit radius
    }

    fn search_node_at_position(
        &self,
        node: Node<NodePayload>,
        cdx_x: f64,
        cdx_y: f64,
        hit_radius: f64,
    ) -> Option<ObjectInfo> {
        let data = node.borrow_data();
        
        // Check if this node is near the cursor
        let mut closest = None;
        let mut closest_distance = hit_radius;

        // Try current node
        if let Some(info) = self.create_object_info(&*data, &node) {
            if let Some((x, y)) = self.get_node_position(&*data) {
                let distance = ((x - cdx_x).powi(2) + (y - cdx_y).powi(2)).sqrt();
                if distance < closest_distance {
                    closest_distance = distance;
                    closest = Some(info);
                }
            }
        }
        
        drop(data); // Release the borrow before searching children

        // Search children
        for child in node.children() {
            if let Some(child_info) = self.search_node_at_position(child, cdx_x, cdx_y, closest_distance) {
                closest = Some(child_info);
            }
        }

        closest
    }

    fn create_object_info(&self, payload: &NodePayload, _node: &Node<NodePayload>) -> Option<ObjectInfo> {
        match payload {
            NodePayload::Node(node_obj) => {
                let mut details = vec![
                    ("Element ID".to_string(), format!("{}", node_obj.id)),
                ];
                
                if let Some(element) = node_obj.element {
                    details.push(("Element".to_string(), element_to_symbol(element)));
                }
                
                if let Some(charge) = node_obj.charge {
                    if charge != 0 {
                        details.push(("Charge".to_string(), format!("{:+}", charge)));
                    }
                }
                
                if let Some(isotope) = node_obj.isotope {
                    details.push(("Isotope".to_string(), format!("{}", isotope)));
                }
                
                let position = if let Some(pos) = &node_obj.position_2d {
                    format!("({:.1}, {:.1})", pos.x, pos.y)
                } else if let Some(pos) = &node_obj.position_3d {
                    format!("({:.1}, {:.1}, {:.1})", pos.x, pos.y, pos.z)
                } else {
                    "Unknown".to_string()
                };
                
                Some(ObjectInfo {
                    object_type: "Node".to_string(),
                    id: Some(node_obj.id),
                    position,
                    details,
                })
            },
            NodePayload::Bond(bond_obj) => {
                let mut details = vec![
                    ("Begin Node".to_string(), format!("{}", bond_obj.begin)),
                    ("End Node".to_string(), format!("{}", bond_obj.end)),
                ];
                
                if let Some(order) = bond_obj.bond_order {
                    details.push(("Bond Order".to_string(), format!("{}", order)));
                }
                
                if let Some(cip) = bond_obj.cip_stereochemistry {
                    details.push(("CIP Stereo".to_string(), format!("{}", cip)));
                }
                
                if let Some(pos) = bond_obj.double_position {
                    let pos_str = match pos % 256 {
                        0 => "Center".to_string(),
                        1 => "Right".to_string(),
                        2 => "Left".to_string(),
                        _ => format!("Unknown({})", pos),
                    };
                    details.push(("Double Position".to_string(), pos_str));
                }
                
                Some(ObjectInfo {
                    object_type: "Bond".to_string(),
                    id: None,
                    position: "N/A".to_string(),
                    details,
                })
            },
            NodePayload::TextObject(text_obj) => {
                let text_preview = text_obj.text.text.chars().take(30).collect::<String>();
                let mut details = vec![
                    ("Text".to_string(), format!("\"{}{}\"", text_preview, 
                        if text_obj.text.text.len() > 30 { "..." } else { "" })),
                ];
                
                if let Some(size) = text_obj.label_size {
                    details.push(("Label Size".to_string(), format!("{}", size)));
                }
                
                let position = if let Some(pos) = &text_obj.position_2d {
                    format!("({:.1}, {:.1})", pos.x, pos.y)
                } else {
                    "Unknown".to_string()
                };
                
                Some(ObjectInfo {
                    object_type: "TextObject".to_string(),
                    id: None,
                    position,
                    details,
                })
            },
            NodePayload::Fragment(frag_obj) => {
                let mut details = vec![
                    ("Fragment ID".to_string(), format!("{}", frag_obj.id)),
                ];
                
                if let Some(weight) = frag_obj.mole_weight {
                    details.push(("Molecular Weight".to_string(), format!("{:.2}", weight)));
                }
                
                Some(ObjectInfo {
                    object_type: "Fragment".to_string(),
                    id: Some(frag_obj.id),
                    position: "Container".to_string(),
                    details,
                })
            },
            NodePayload::Page(page_obj) => {
                let mut details = vec![
                    ("Page ID".to_string(), format!("{}", page_obj.id)),
                ];
                
                if let Some(bounds) = &page_obj.bounds_in_parent {
                    details.push(("Bounds".to_string(), 
                        format!("({:.1}, {:.1}) - ({:.1}, {:.1})", bounds.left, bounds.top, bounds.right, bounds.bottom)));
                }
                
                Some(ObjectInfo {
                    object_type: "Page".to_string(),
                    id: Some(page_obj.id),
                    position: "Container".to_string(),
                    details,
                })
            },
            NodePayload::Document(_) => {
                Some(ObjectInfo {
                    object_type: "Document".to_string(),
                    id: None,
                    position: "Root".to_string(),
                    details: vec![],
                })
            },
            _ => Some(ObjectInfo {
                object_type: format!("{:?}", payload),
                id: None,
                position: "Unknown".to_string(),
                details: vec![],
            }),
        }
    }

    fn get_node_position(&self, payload: &NodePayload) -> Option<(f64, f64)> {
        match payload {
            NodePayload::Node(node_obj) => {
                if let Some(pos) = &node_obj.position_2d {
                    Some((pos.x, pos.y))
                } else if let Some(pos) = &node_obj.position_3d {
                    Some((pos.x, pos.y))
                } else {
                    None
                }
            },
            NodePayload::TextObject(text_obj) => {
                if let Some(pos) = &text_obj.position_2d {
                    Some((pos.x, pos.y))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn draw_bounding_box(&self, ctx: &ModeContext, painter: &egui::Painter, info: &ObjectInfo, color: egui::Color32) {
        // Draw a small circle at the object's position
        // For nodes and text, use their position
        // For bonds, use a circle at the midpoint
        
        // This is a simple representation - in a real implementation,
        // we would need to get the actual bounding box from the object
        let radius = 8.0;
        let stroke = egui::Stroke::new(2.0, color);
        
        // Try to convert position string to coordinates
        if let Some(pos_str) = info.position.strip_prefix('(') {
            if let Some(end) = pos_str.rfind(')') {
                let coords_str = &pos_str[..end];
                let coords: Vec<&str> = coords_str.split(',').collect();
                
                if coords.len() >= 2 {
                    if let (Ok(x), Ok(y)) = (coords[0].trim().parse::<f64>(), coords[1].trim().parse::<f64>()) {
                        let pos = crate::cdx::values::Point2d { x, y };
                        let screen_pos = ctx.cdx_to_screen(&pos);
                        
                        // Draw circle
                        painter.circle_stroke(screen_pos, radius, stroke);
                    }
                }
            }
        }
    }
}

impl ModeHandler for DebugMode {
    fn handle_click(&mut self, ctx: &mut ModeContext) {
        // Select the object under cursor
        let cdx_file = &ctx.renderer.cdx_file;
        if let Some(info) = self.get_object_at_position(ctx, cdx_file) {
            self.selected_object_info = Some(info);
        }
        // Also handle drag like View mode
        *ctx.view_offset += ctx.drag_delta;
    }
    
    fn handle_drag(&mut self, ctx: &mut ModeContext) {
        // Drag to move view like View mode
        *ctx.view_offset += ctx.drag_delta;
    }
    
    fn handle_drag_end(&mut self, _ctx: &mut ModeContext) {}
    
    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter) {
        let cdx_file = &ctx.renderer.cdx_file;
        
        // Get hovered object
        let hovered = self.get_object_at_position(ctx, cdx_file);
        
        // Draw bounding boxes
        if let Some(ref selected) = self.selected_object_info {
            self.draw_bounding_box(ctx, painter, selected, egui::Color32::RED);
        }
        
        if let Some(ref hovered_info) = hovered {
            self.draw_bounding_box(ctx, painter, hovered_info, egui::Color32::YELLOW);
            
            // Draw info panel
            let panel_width = 250.0;
            let panel_height = 150.0;
            let panel_x = ctx.mouse_pos.x + 15.0;
            let panel_y = ctx.mouse_pos.y + 15.0;
            
            let rect = egui::Rect::from_min_size(
                egui::Pos2 { x: panel_x, y: panel_y },
                egui::Vec2 { x: panel_width, y: panel_height },
            );
            
            // Draw background
            painter.rect_filled(rect, 5.0, egui::Color32::from_rgba_unmultiplied(30, 30, 40, 240));
            
            // Draw border
            painter.rect_stroke(rect, 5.0, egui::Stroke::new(1.0, egui::Color32::YELLOW));
            
            // Draw text content
            let mut y = rect.top() + 8.0;
            
            // Object type
            let type_text = &hovered_info.object_type;
            painter.text(
                egui::Pos2 { x: rect.left() + 8.0, y },
                egui::Align2::LEFT_TOP,
                type_text,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
                egui::Color32::YELLOW,
            );
            y += 18.0;
            
            // ID if available
            if let Some(id) = hovered_info.id {
                painter.text(
                    egui::Pos2 { x: rect.left() + 8.0, y },
                    egui::Align2::LEFT_TOP,
                    &format!("ID: {}", id),
                    egui::FontId::new(12.0, egui::FontFamily::Monospace),
                    egui::Color32::LIGHT_GREEN,
                );
                y += 16.0;
            }
            
            // Position
            painter.text(
                egui::Pos2 { x: rect.left() + 8.0, y },
                egui::Align2::LEFT_TOP,
                &format!("Pos: {}", hovered_info.position),
                egui::FontId::new(12.0, egui::FontFamily::Monospace),
                egui::Color32::LIGHT_BLUE,
            );
            y += 16.0;
            
            // Details
            for (key, value) in hovered_info.details.iter().take(3) {
                let text = format!("{}: {}", key, value);
                painter.text(
                    egui::Pos2 { x: rect.left() + 8.0, y },
                    egui::Align2::LEFT_TOP,
                    &text,
                    egui::FontId::new(10.0, egui::FontFamily::Monospace),
                    egui::Color32::WHITE,
                );
                y += 14.0;
                
                if y > rect.bottom() - 8.0 {
                    break;
                }
            }
        }
    }
    
    fn handle_key(&mut self, _ctx: &mut ModeContext, _key: egui::Key) -> bool {
        false
    }
}
