use crate::cdx::bond::Bond;
use crate::cdx::file::NodePayload;
use crate::cdx::node::Node;
use crate::cdx::values::Point2d;
use crate::modes::{ModeContext, ModeHandler};
use eframe::egui;

/// Default bond length in CDX coordinates
const DEFAULT_BOND_LENGTH: f64 = 30.0;

/// Hit radius for node detection (in screen pixels before zoom scaling)
const NODE_HIT_RADIUS: f32 = 15.0;

/// Hit threshold for bond detection (in screen pixels before zoom scaling)
const BOND_HIT_THRESHOLD: f32 = 8.0;

/// Bond mode for creating and editing bonds
pub struct BondMode {
    /// Source node ID when creating a bond
    source_node: Option<u32>,
    /// Preview end position during bond creation (screen coordinates)
    preview_end: Option<egui::Pos2>,
    /// Whether we're currently in bond creation mode
    is_creating_bond: bool,
    /// Hovered node ID (for highlighting)
    hovered_node: Option<u32>,
    /// Hovered bond ID (for highlighting)
    hovered_bond: Option<u32>,
}

impl BondMode {
    pub fn new() -> Self {
        Self {
            source_node: None,
            preview_end: None,
            is_creating_bond: false,
            hovered_node: None,
            hovered_bond: None,
        }
    }

    /// Get angles of all existing bonds connected to a node
    fn get_connected_bond_angles(&self, ctx: &ModeContext, node_id: u32) -> Vec<f64> {
        let mut angles = Vec::new();
        
        // Get the position of the source node
        let Some(source_pos) = ctx.node_positions.get(&node_id) else {
            return angles;
        };
        
        // Check all bonds to find ones connected to this node
        for (_bond_id, (begin_id, end_id, begin_pos, end_pos)) in ctx.bond_positions.iter() {
            let other_pos = if *begin_id == node_id {
                end_pos
            } else if *end_id == node_id {
                begin_pos
            } else {
                continue;
            };
            
            // Calculate angle from source to other node
            let dx = other_pos.x - source_pos.x;
            let dy = other_pos.y - source_pos.y;
            let angle = dy.atan2(dx);
            angles.push(angle);
        }
        
        angles
    }

    /// Find the best angle for a new bond at 120° from existing bonds
    /// Returns the angle that places the new bond in the most open direction
    fn calculate_best_bond_angle(&self, ctx: &ModeContext, node_id: u32) -> f64 {
        let existing_angles = self.get_connected_bond_angles(ctx, node_id);
        
        if existing_angles.is_empty() {
            // No existing bonds - default to 30° (pointing up-right in CDX coordinates)
            return std::f64::consts::PI / 6.0;
        }
        
        if existing_angles.len() == 1 {
            // Single bond - place new bond at 120° (2π/3) from existing
            // Choose the direction that points more "outward" (prefer upper directions)
            let existing = existing_angles[0];
            let option1 = existing + std::f64::consts::PI * 2.0 / 3.0;
            let option2 = existing - std::f64::consts::PI * 2.0 / 3.0;
            
            // Prefer the option with higher y (more upward in CDX coords where +y is up)
            if option1.sin() >= option2.sin() {
                return Self::normalize_angle(option1);
            } else {
                return Self::normalize_angle(option2);
            }
        }
        
        // Multiple bonds - find the largest gap and place new bond there
        let mut sorted_angles: Vec<f64> = existing_angles.iter().copied().collect();
        sorted_angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Find the largest gap between consecutive angles
        let mut max_gap = 0.0_f64;
        let mut best_mid_angle = 0.0_f64;
        
        for i in 0..sorted_angles.len() {
            let current = sorted_angles[i];
            let next = if i + 1 < sorted_angles.len() {
                sorted_angles[i + 1]
            } else {
                // Wrap around: gap from last to first
                sorted_angles[0] + 2.0 * std::f64::consts::PI
            };
            
            let gap = next - current;
            if gap > max_gap {
                max_gap = gap;
                // Place new bond in the middle of the gap
                best_mid_angle = current + gap / 2.0;
            }
        }
        
        Self::normalize_angle(best_mid_angle)
    }
    
    /// Normalize angle to [-π, π] range
    fn normalize_angle(angle: f64) -> f64 {
        let mut a = angle;
        while a > std::f64::consts::PI {
            a -= 2.0 * std::f64::consts::PI;
        }
        while a < -std::f64::consts::PI {
            a += 2.0 * std::f64::consts::PI;
        }
        a
    }

    /// Create a new node at 120° from existing bond and connect it
    fn create_bond_from_node(&self, ctx: &ModeContext, source_id: u32) -> Option<(u32, u32)> {
        // Get source node position
        let source_pos = ctx.node_positions.get(&source_id)?.clone();
        
        // Calculate the best angle for new bond
        let angle = self.calculate_best_bond_angle(ctx, source_id);
        println!("Creating bond from node {} at angle {:.2}° (rad: {:.4})", source_id, angle.to_degrees(), angle);
        
        // Calculate new node position
        let new_pos = Point2d {
            x: source_pos.x + DEFAULT_BOND_LENGTH * angle.cos(),
            y: source_pos.y + DEFAULT_BOND_LENGTH * angle.sin(),
        };
        println!("Source pos: ({:.2}, {:.2}), New pos: ({:.2}, {:.2})", source_pos.x, source_pos.y, new_pos.x, new_pos.y);
        
        // Create the new node (pass source_id to find correct fragment)
        let new_node_id = self.create_node_near(ctx, new_pos, source_id)?;
        println!("Created new node with id {}", new_node_id);
        
        // Create the bond
        let bond_id = self.create_bond(ctx, source_id, new_node_id)?;
        println!("Created bond {} from {} to {}", bond_id, source_id, new_node_id);
        
        Some((new_node_id, bond_id))
    }

    /// Create a new node at the given CDX position, finding fragment from nearby_node_id
    fn create_node_near(&self, ctx: &ModeContext, position: Point2d, nearby_node_id: u32) -> Option<u32> {
        let mut cdx_borrow = ctx.cdx_file.borrow_mut();
        let cdx_file = cdx_borrow.as_mut()?;
        
        // Get next available ID
        let new_id = cdx_file.next_id();
        
        // Create a new Node
        let mut new_node = Node::new(new_id);
        new_node.position_2d = Some(position);
        new_node.element = Some(6); // Carbon by default
        
        // Find a fragment based on the nearby node
        if let Some(fragment) = cdx_file.find_fragment_for_node(nearby_node_id) {
            let grant = cdx_file.tree.grant_hierarchy_edit().ok()?;
            fragment.create_as_last_child(&grant, NodePayload::Node(new_node));
            return Some(new_id);
        }
        
        println!("Failed to find fragment for node {}", nearby_node_id);
        None
    }

    /// Create a new node at the given CDX position and add it to the tree
    /// This is used when clicking on empty space (no nearby node)
    fn create_node(&self, ctx: &ModeContext, position: Point2d) -> Option<u32> {
        let mut cdx_borrow = ctx.cdx_file.borrow_mut();
        let cdx_file = cdx_borrow.as_mut()?;
        
        // Get next available ID
        let new_id = cdx_file.next_id();
        
        // Create a new Node
        let mut new_node = Node::new(new_id);
        new_node.position_2d = Some(position);
        new_node.element = Some(6); // Carbon by default
        
        // Find any fragment in the tree
        if let Some(fragment) = cdx_file.find_first_fragment() {
            let grant = cdx_file.tree.grant_hierarchy_edit().ok()?;
            fragment.create_as_last_child(&grant, NodePayload::Node(new_node));
            return Some(new_id);
        }
        
        println!("Failed to find any fragment");
        None
    }

    /// Create a bond between two nodes
    fn create_bond(&self, ctx: &ModeContext, begin_id: u32, end_id: u32) -> Option<u32> {
        let mut cdx_borrow = ctx.cdx_file.borrow_mut();
        let cdx_file = cdx_borrow.as_mut()?;
        
        // Get next available ID
        let new_id = cdx_file.next_id();
        
        // Create a new Bond
        let new_bond = Bond::new(new_id, begin_id, end_id);
        
        // Find the fragment containing the begin node
        if let Some(fragment) = cdx_file.find_fragment_for_node(begin_id) {
            let grant = cdx_file.tree.grant_hierarchy_edit().ok()?;
            fragment.create_as_last_child(&grant, NodePayload::Bond(new_bond));
            return Some(new_id);
        }
        
        None
    }

    /// Cycle bond order: 1 -> 2 -> 3 -> 1
    fn cycle_bond_order(&self, ctx: &ModeContext, bond_id: u32) {
        let mut cdx_borrow = ctx.cdx_file.borrow_mut();
        if let Some(cdx_file) = cdx_borrow.as_mut() {
            if let Some(bond_node) = cdx_file.find_node_by_id(bond_id) {
                let mut data = bond_node.borrow_data_mut();
                if let NodePayload::Bond(ref mut bond) = *data {
                    let current_order = bond.bond_order.unwrap_or(1);
                    // Cycle: 1 -> 2 -> 3 -> 1
                    let new_order = match current_order {
                        1 => 2,
                        2 => 3,
                        _ => 1,
                    };
                    bond.bond_order = Some(new_order);
                }
            }
        }
    }

    /// Move a node to a new position
    fn move_node(&self, ctx: &ModeContext, node_id: u32, delta: egui::Vec2) {
        let scale = ctx.zoom * ctx.auto_scale;
        let cdx_delta = Point2d {
            x: (delta.x / scale) as f64,
            y: -(delta.y / scale) as f64, // Y is inverted
        };

        let mut cdx_borrow = ctx.cdx_file.borrow_mut();
        if let Some(cdx_file) = cdx_borrow.as_mut() {
            if let Some(node) = cdx_file.find_node_by_id(node_id) {
                let mut data = node.borrow_data_mut();
                if let NodePayload::Node(ref mut n) = *data {
                    if let Some(ref mut pos) = n.position_2d {
                        pos.x += cdx_delta.x;
                        pos.y += cdx_delta.y;
                    }
                }
            }
        }
    }
}

impl Default for BondMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeHandler for BondMode {
    fn handle_click(&mut self, ctx: &mut ModeContext) {
        // First check if we clicked on a node
        if let Some(node_id) = ctx.hit_test_node(ctx.mouse_pos, NODE_HIT_RADIUS) {
            if self.is_creating_bond {
                // We're in bond creation mode and clicked on a node
                if let Some(source_id) = self.source_node {
                    if source_id != node_id {
                        // Create bond between source_node and clicked node
                        if let Some(bond_id) = self.create_bond(ctx, source_id, node_id) {
                            println!("Created bond {}: {} -> {}", bond_id, source_id, node_id);
                        }
                    }
                }
                // Reset bond creation state
                self.source_node = None;
                self.preview_end = None;
                self.is_creating_bond = false;
            } else {
                // Click on existing node: immediately create a new bond at 120° angle
                println!("Clicked on node {} - attempting to create 120° bond", node_id);
                if let Some((new_node_id, bond_id)) = self.create_bond_from_node(ctx, node_id) {
                    println!("Created node {} and bond {} at 120° from node {}", new_node_id, bond_id, node_id);
                } else {
                    println!("Failed to create bond from node {}", node_id);
                }
            }
            return;
        }

        // Check if we clicked on a bond
        if let Some(bond_id) = ctx.hit_test_bond(ctx.mouse_pos, BOND_HIT_THRESHOLD) {
            // Cycle bond order: 1 -> 2 -> 3 -> 1
            self.cycle_bond_order(ctx, bond_id);
            println!("Cycled bond order for bond {}", bond_id);
            // Reset any pending bond creation
            self.source_node = None;
            self.preview_end = None;
            self.is_creating_bond = false;
            return;
        }

        // Clicked on empty space
        if self.is_creating_bond {
            if let Some(source_id) = self.source_node {
                // Create new node at mouse position and bond to it
                let cdx_pos = ctx.screen_to_cdx(ctx.mouse_pos);
                if let Some(new_node_id) = self.create_node(ctx, cdx_pos) {
                    if let Some(bond_id) = self.create_bond(ctx, source_id, new_node_id) {
                        println!("Created node {} and bond {}", new_node_id, bond_id);
                    }
                }
            }
            // Reset bond creation state
            self.source_node = None;
            self.preview_end = None;
            self.is_creating_bond = false;
        } else {
            // Just clicked on empty space - create new node
            let cdx_pos = ctx.screen_to_cdx(ctx.mouse_pos);
            if let Some(new_node_id) = self.create_node(ctx, cdx_pos) {
                // Start bond creation from this new node
                self.source_node = Some(new_node_id);
                self.is_creating_bond = true;
                self.preview_end = Some(ctx.mouse_pos);
                println!("Created node {}", new_node_id);
            }
        }
    }

    fn handle_drag(&mut self, ctx: &mut ModeContext) {
        if self.is_creating_bond {
            // Update preview end position
            self.preview_end = Some(ctx.mouse_pos);
            
            // Check if we're hovering over a potential target node
            self.hovered_node = ctx.hit_test_node(ctx.mouse_pos, NODE_HIT_RADIUS);
        } else if let Some(node_id) = ctx.hit_test_node(ctx.mouse_pos - ctx.drag_delta, NODE_HIT_RADIUS) {
            // Dragging a node - move it
            self.move_node(ctx, node_id, ctx.drag_delta);
        } else if let Some(bond_id) = ctx.hit_test_bond(ctx.mouse_pos - ctx.drag_delta, BOND_HIT_THRESHOLD) {
            // Dragging a bond - move both nodes
            if let Some((begin_id, end_id, _, _)) = ctx.bond_positions.get(&bond_id).cloned() {
                self.move_node(ctx, begin_id, ctx.drag_delta);
                self.move_node(ctx, end_id, ctx.drag_delta);
            }
        } else {
            // Pan the view
            *ctx.view_offset += ctx.drag_delta;
        }
    }

    fn handle_drag_end(&mut self, ctx: &mut ModeContext) {
        if self.is_creating_bond {
            if let Some(source_id) = self.source_node {
                // Check if we're over a target node
                if let Some(target_id) = ctx.hit_test_node(ctx.mouse_pos, NODE_HIT_RADIUS) {
                    if source_id != target_id {
                        // Create bond between nodes
                        if let Some(bond_id) = self.create_bond(ctx, source_id, target_id) {
                            println!("Created bond {} (drag): {} -> {}", bond_id, source_id, target_id);
                        }
                    }
                } else {
                    // Create new node at mouse position and bond to it
                    let cdx_pos = ctx.screen_to_cdx(ctx.mouse_pos);
                    if let Some(new_node_id) = self.create_node(ctx, cdx_pos) {
                        if let Some(bond_id) = self.create_bond(ctx, source_id, new_node_id) {
                            println!("Created node {} and bond {} (drag)", new_node_id, bond_id);
                        }
                    }
                }
            }
        }

        // Reset state
        self.source_node = None;
        self.preview_end = None;
        self.is_creating_bond = false;
        self.hovered_node = None;
    }

    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter) {
        // Highlight hovered node
        let hovered_node = ctx.hit_test_node(ctx.mouse_pos, NODE_HIT_RADIUS);
        if let Some(node_id) = hovered_node {
            if let Some(pos) = ctx.node_positions.get(&node_id) {
                let screen_pos = ctx.cdx_to_screen(pos);
                let radius = 18.0 * ctx.zoom;
                painter.circle_stroke(
                    screen_pos,
                    radius,
                    egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(100, 200, 100, 200)),
                );
            }
        }

        // Highlight hovered bond
        let hovered_bond = ctx.hit_test_bond(ctx.mouse_pos, BOND_HIT_THRESHOLD);
        if let Some(bond_id) = hovered_bond {
            if let Some((_begin_id, _end_id, begin_pos, end_pos)) = ctx.bond_positions.get(&bond_id) {
                let begin_screen = ctx.cdx_to_screen(begin_pos);
                let end_screen = ctx.cdx_to_screen(end_pos);
                painter.line_segment(
                    [begin_screen, end_screen],
                    egui::Stroke::new(4.0, egui::Color32::from_rgba_unmultiplied(100, 200, 100, 150)),
                );
            }
        }

        // Draw bond creation preview
        if self.is_creating_bond {
            if let Some(source_id) = self.source_node {
                if let Some(source_pos) = ctx.node_positions.get(&source_id) {
                    let source_screen = ctx.cdx_to_screen(source_pos);
                    
                    if let Some(preview_end) = self.preview_end {
                        // Draw preview line
                        let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(80, 160, 255, 180));
                        painter.line_segment([source_screen, preview_end], stroke);
                        
                        // If hovering over a target node, highlight it
                        if let Some(target_id) = self.hovered_node {
                            if target_id != source_id {
                                if let Some(target_pos) = ctx.node_positions.get(&target_id) {
                                    let target_screen = ctx.cdx_to_screen(target_pos);
                                    let radius = 20.0 * ctx.zoom;
                                    painter.circle_stroke(
                                        target_screen,
                                        radius,
                                        egui::Stroke::new(3.0, egui::Color32::from_rgb(80, 200, 80)),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        // Highlight source node during bond creation
        if let Some(source_id) = self.source_node {
            if let Some(source_pos) = ctx.node_positions.get(&source_id) {
                let screen_pos = ctx.cdx_to_screen(source_pos);
                let radius = 22.0 * ctx.zoom;
                painter.circle_stroke(
                    screen_pos,
                    radius,
                    egui::Stroke::new(2.5, egui::Color32::from_rgb(80, 160, 255)),
                );
            }
        }
    }

    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool {
        match key {
            egui::Key::Escape => {
                // Cancel bond creation
                self.source_node = None;
                self.preview_end = None;
                self.is_creating_bond = false;
                self.hovered_node = None;
                self.hovered_bond = None;
                true
            }
            egui::Key::K if ctx.ui.input(|i| i.modifiers.command) => {
                // Ctrl+K: Clean up structure
                let mut cdx_borrow = ctx.cdx_file.borrow_mut();
                if let Some(cdx_file) = cdx_borrow.as_mut() {
                    super::cleanup::cleanup_structure(cdx_file);
                    println!("Structure cleanup completed");
                }
                true
            }
            _ => false,
        }
    }
}
