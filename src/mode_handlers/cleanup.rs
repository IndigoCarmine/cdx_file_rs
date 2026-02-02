//! Structure cleanup algorithm for optimizing chemical structure layouts
//!
//! This module implements a simple force-directed layout algorithm that:
//! 1. Standardizes bond lengths to the default
//! 2. Optimizes bond angles to standard values (120° for sp2, 109.5° for sp3)
//! 3. Resolves atom overlaps

use crate::cdx::file::{CdxFile, NodePayload};
use crate::cdx::values::Point2d;
use std::collections::HashMap;

/// Default bond length in CDX coordinates
const DEFAULT_BOND_LENGTH: f64 = 30.0;

/// Ideal angle for sp2 hybridization (120 degrees)
const SP2_ANGLE: f64 = std::f64::consts::PI * 2.0 / 3.0;

/// Ideal angle for sp3 hybridization (109.5 degrees)  
const SP3_ANGLE: f64 = 109.47 * std::f64::consts::PI / 180.0;

/// Number of iterations for the optimization
const ITERATIONS: usize = 50;

/// Spring constant for bond length optimization
const BOND_SPRING_K: f64 = 0.3;

/// Angular spring constant
const ANGLE_SPRING_K: f64 = 0.1;

/// Repulsion constant for overlap prevention
const REPULSION_K: f64 = 500.0;

/// Minimum distance between atoms
const MIN_DISTANCE: f64 = 20.0;

/// Represents the connectivity of atoms
struct MoleculeGraph {
    /// Node ID to position mapping
    positions: HashMap<u32, Point2d>,
    /// Adjacency list: node_id -> list of connected node_ids
    adjacency: HashMap<u32, Vec<u32>>,
    /// Bond order for each connection
    bond_orders: HashMap<(u32, u32), i16>,
}

impl MoleculeGraph {
    /// Build graph from CdxFile
    fn from_cdx_file(cdx_file: &CdxFile) -> Self {
        let mut positions = HashMap::new();
        let mut adjacency: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut bond_orders = HashMap::new();

        // Collect all nodes and bonds
        let mut queue = vec![cdx_file.tree.root()];
        while let Some(node) = queue.pop() {
            match &*node.borrow_data() {
                NodePayload::Node(n) => {
                    if let Some(pos) = &n.position_2d {
                        positions.insert(n.id, pos.clone());
                        adjacency.entry(n.id).or_insert_with(Vec::new);
                    }
                }
                NodePayload::Bond(b) => {
                    adjacency.entry(b.begin).or_insert_with(Vec::new).push(b.end);
                    adjacency.entry(b.end).or_insert_with(Vec::new).push(b.begin);
                    let order = b.bond_order.unwrap_or(1);
                    bond_orders.insert((b.begin.min(b.end), b.begin.max(b.end)), order);
                }
                _ => {}
            }

            for child in node.children() {
                queue.push(child);
            }
        }

        MoleculeGraph {
            positions,
            adjacency,
            bond_orders,
        }
    }

    /// Apply positions back to CdxFile
    fn apply_to_cdx_file(&self, cdx_file: &mut CdxFile) {
        let mut queue = vec![cdx_file.tree.root()];
        while let Some(node) = queue.pop() {
            {
                let mut data = node.borrow_data_mut();
                if let NodePayload::Node(ref mut n) = *data {
                    if let Some(new_pos) = self.positions.get(&n.id) {
                        n.position_2d = Some(new_pos.clone());
                    }
                }
            }

            for child in node.children() {
                queue.push(child);
            }
        }
    }

    /// Calculate distance between two points
    fn distance(p1: &Point2d, p2: &Point2d) -> f64 {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate angle between three points (p1-center-p2)
    fn angle(p1: &Point2d, center: &Point2d, p2: &Point2d) -> f64 {
        let v1x = p1.x - center.x;
        let v1y = p1.y - center.y;
        let v2x = p2.x - center.x;
        let v2y = p2.y - center.y;

        let dot = v1x * v2x + v1y * v2y;
        let cross = v1x * v2y - v1y * v2x;
        cross.atan2(dot).abs()
    }

    /// Run optimization iterations
    fn optimize(&mut self) {
        for _ in 0..ITERATIONS {
            self.optimize_step();
        }
    }

    /// Single optimization step
    fn optimize_step(&mut self) {
        let mut forces: HashMap<u32, Point2d> = HashMap::new();

        // Initialize forces to zero
        for &id in self.positions.keys() {
            forces.insert(id, Point2d { x: 0.0, y: 0.0 });
        }

        // 1. Bond length forces (spring model)
        let adjacency = self.adjacency.clone();
        for (&node_id, neighbors) in &adjacency {
            let pos_opt = self.positions.get(&node_id).cloned();
            if pos_opt.is_none() { continue; }
            let pos = pos_opt.unwrap();

            for &neighbor_id in neighbors {
                if neighbor_id <= node_id { continue; } // Only process each pair once
                
                let neighbor_pos_opt = self.positions.get(&neighbor_id).cloned();
                if neighbor_pos_opt.is_none() { continue; }
                let neighbor_pos = neighbor_pos_opt.unwrap();

                let dist = Self::distance(&pos, &neighbor_pos);
                if dist < 0.001 { continue; }

                // Force proportional to difference from ideal length
                let diff = dist - DEFAULT_BOND_LENGTH;
                let force_mag = diff * BOND_SPRING_K;

                let dx = (neighbor_pos.x - pos.x) / dist;
                let dy = (neighbor_pos.y - pos.y) / dist;

                // Apply force to both atoms
                if let Some(f) = forces.get_mut(&node_id) {
                    f.x += dx * force_mag;
                    f.y += dy * force_mag;
                }
                if let Some(f) = forces.get_mut(&neighbor_id) {
                    f.x -= dx * force_mag;
                    f.y -= dy * force_mag;
                }
            }
        }

        // 2. Repulsion between non-bonded atoms
        let node_ids: Vec<u32> = self.positions.keys().cloned().collect();
        for i in 0..node_ids.len() {
            for j in (i + 1)..node_ids.len() {
                let id1 = node_ids[i];
                let id2 = node_ids[j];

                // Skip bonded pairs
                if let Some(neighbors) = self.adjacency.get(&id1) {
                    if neighbors.contains(&id2) {
                        continue;
                    }
                }

                let pos1_opt = self.positions.get(&id1).cloned();
                let pos2_opt = self.positions.get(&id2).cloned();
                if pos1_opt.is_none() || pos2_opt.is_none() { continue; }
                let pos1 = pos1_opt.unwrap();
                let pos2 = pos2_opt.unwrap();

                let dist = Self::distance(&pos1, &pos2);
                if dist < 0.001 { continue; }

                // Repulsive force (inverse square law, capped at minimum distance)
                if dist < MIN_DISTANCE * 2.0 {
                    let force_mag = REPULSION_K / (dist * dist);

                    let dx = (pos2.x - pos1.x) / dist;
                    let dy = (pos2.y - pos1.y) / dist;

                    if let Some(f) = forces.get_mut(&id1) {
                        f.x -= dx * force_mag;
                        f.y -= dy * force_mag;
                    }
                    if let Some(f) = forces.get_mut(&id2) {
                        f.x += dx * force_mag;
                        f.y += dy * force_mag;
                    }
                }
            }
        }

        // 3. Angular forces (try to achieve ideal angles)
        for (&node_id, neighbors) in &adjacency {
            if neighbors.len() < 2 { continue; }

            let center_pos_opt = self.positions.get(&node_id).cloned();
            if center_pos_opt.is_none() { continue; }
            let center_pos = center_pos_opt.unwrap();

            // Get ideal angle based on number of neighbors (hybridization)
            let ideal_angle = if neighbors.len() == 2 {
                std::f64::consts::PI // 180° for 2 neighbors (linear or part of chain)
            } else if neighbors.len() == 3 {
                SP2_ANGLE
            } else {
                SP3_ANGLE
            };

            // For each pair of neighbors, apply torque to achieve ideal angle
            for i in 0..neighbors.len() {
                for j in (i + 1)..neighbors.len() {
                    let n1_id = neighbors[i];
                    let n2_id = neighbors[j];

                    let n1_pos_opt = self.positions.get(&n1_id).cloned();
                    let n2_pos_opt = self.positions.get(&n2_id).cloned();
                    if n1_pos_opt.is_none() || n2_pos_opt.is_none() { continue; }
                    let n1_pos = n1_pos_opt.unwrap();
                    let n2_pos = n2_pos_opt.unwrap();

                    let current_angle = Self::angle(&n1_pos, &center_pos, &n2_pos);
                    let angle_diff = current_angle - ideal_angle;

                    // Apply perpendicular force to rotate atoms
                    let force_mag = angle_diff * ANGLE_SPRING_K;

                    // Vector from center to n1
                    let dist1 = Self::distance(&center_pos, &n1_pos);
                    if dist1 < 0.001 { continue; }
                    let dx1 = (n1_pos.x - center_pos.x) / dist1;
                    let dy1 = (n1_pos.y - center_pos.y) / dist1;

                    // Perpendicular direction
                    let px1 = -dy1;
                    let py1 = dx1;

                    if let Some(f) = forces.get_mut(&n1_id) {
                        f.x += px1 * force_mag;
                        f.y += py1 * force_mag;
                    }

                    // Opposite rotation for n2
                    let dist2 = Self::distance(&center_pos, &n2_pos);
                    if dist2 < 0.001 { continue; }
                    let dx2 = (n2_pos.x - center_pos.x) / dist2;
                    let dy2 = (n2_pos.y - center_pos.y) / dist2;
                    let px2 = -dy2;
                    let py2 = dx2;

                    if let Some(f) = forces.get_mut(&n2_id) {
                        f.x -= px2 * force_mag;
                        f.y -= py2 * force_mag;
                    }
                }
            }
        }

        // Apply forces (with damping)
        let damping = 0.5;
        for (&id, force) in &forces {
            if let Some(pos) = self.positions.get_mut(&id) {
                pos.x += force.x * damping;
                pos.y += force.y * damping;
            }
        }
    }
}

/// Clean up the structure in the CdxFile
pub fn cleanup_structure(cdx_file: &mut CdxFile) {
    let mut graph = MoleculeGraph::from_cdx_file(cdx_file);
    graph.optimize();
    graph.apply_to_cdx_file(cdx_file);
}
