use crate::cdx::curve::Curve;
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::values::{CDXString, Point3d, Rectangle};
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::curve_tags::*;
use crate::error::CdxError;

impl TaggedObject for Curve {
    const TAG: u16 = CDXOBJ_CURVE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut curve = Curve::new(raw.id);

        // Parse z_order
        if let Some(z_data) = raw.get_property(CDXPROP_Z_ORDER) {
            if z_data.len() >= 2 {
                curve.z_order = Some(i16::from_le_bytes(
                    z_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid z_order data".to_string()))?,
                ));
            }
        }

        // Parse ignore_warnings
        if let Some(ignore_data) = raw.get_property(CDXPROP_IGNORE_WARNINGS) {
            if !ignore_data.is_empty() {
                curve.ignore_warnings = Some(ignore_data[0] != 0);
            }
        }

        // Parse chemical_warning
        if let Some(warning_data) = raw.get_property(CDXPROP_CHEMICAL_WARNING) {
            curve.chemical_warning = CDXString::decode(warning_data).ok();
        }

        // Parse visible
        if let Some(visible_data) = raw.get_property(CDXPROP_VISIBLE) {
            if !visible_data.is_empty() {
                curve.visible = Some(visible_data[0] != 0);
            }
        }

        // Parse bounding_box
        if let Some(bounds_data) = raw.get_property(CDXPROP_BOUNDING_BOX) {
            if bounds_data.len() >= 32 {
                let top = f64::from_le_bytes(
                    bounds_data[0..8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bounding_box data".to_string()))?,
                );
                let left = f64::from_le_bytes(
                    bounds_data[8..16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bounding_box data".to_string()))?,
                );
                let bottom = f64::from_le_bytes(
                    bounds_data[16..24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bounding_box data".to_string()))?,
                );
                let right = f64::from_le_bytes(
                    bounds_data[24..32]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bounding_box data".to_string()))?,
                );
                curve.bounding_box = Some(Rectangle {
                    top,
                    left,
                    bottom,
                    right,
                });
            }
        }

        // Parse foreground_color
        if let Some(color_data) = raw.get_property(CDXPROP_FOREGROUND_COLOR) {
            if color_data.len() >= 2 {
                curve.foreground_color = Some(u16::from_le_bytes(
                    color_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid foreground_color data".to_string()))?,
                ));
            }
        }

        // Parse background_color
        if let Some(color_data) = raw.get_property(CDXPROP_BACKGROUND_COLOR) {
            if color_data.len() >= 2 {
                curve.background_color = Some(i16::from_le_bytes(
                    color_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid background_color data".to_string()))?,
                ));
            }
        }

        // Parse curve_type
        if let Some(type_data) = raw.get_property(CDXPROP_CURVE_TYPE) {
            if type_data.len() >= 2 {
                curve.curve_type = Some(i16::from_le_bytes(
                    type_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid curve_type data".to_string()))?,
                ));
            }
        }

        // Parse arrowhead_size
        if let Some(size_data) = raw.get_property(CDXPROP_ARROWHEAD_SIZE) {
            if size_data.len() >= 2 {
                curve.arrowhead_size = Some(i16::from_le_bytes(
                    size_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid arrowhead_size data".to_string()))?,
                ));
            }
        }

        // Parse curve_points (required, list of Point3d)
        if let Some(points_data) = raw.get_property(CDXPROP_CURVE_POINTS) {
            let mut points = Vec::new();
            let mut i = 0;
            while i + 24 <= points_data.len() {
                let x = f64::from_le_bytes(
                    points_data[i..i + 8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid curve_points data".to_string()))?,
                );
                let y = f64::from_le_bytes(
                    points_data[i + 8..i + 16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid curve_points data".to_string()))?,
                );
                let z = f64::from_le_bytes(
                    points_data[i + 16..i + 24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid curve_points data".to_string()))?,
                );
                points.push(Point3d { x, y, z });
                i += 24;
            }
            if !points.is_empty() {
                curve.curve_points = Some(points);
            }
        }

        // Parse curve_points_3d
        if let Some(points_data) = raw.get_property(CDXPROP_CURVE_POINTS3D) {
            let mut points = Vec::new();
            let mut i = 0;
            while i + 24 <= points_data.len() {
                let x = f64::from_le_bytes(
                    points_data[i..i + 8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid curve_points_3d data".to_string()))?,
                );
                let y = f64::from_le_bytes(
                    points_data[i + 8..i + 16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid curve_points_3d data".to_string()))?,
                );
                let z = f64::from_le_bytes(
                    points_data[i + 16..i + 24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid curve_points_3d data".to_string()))?,
                );
                points.push(Point3d { x, y, z });
                i += 24;
            }
            if !points.is_empty() {
                curve.curve_points_3d = Some(points);
            }
        }

        // Parse arrowhead_type
        if let Some(type_data) = raw.get_property(CDXPROP_ARROWHEAD_TYPE) {
            if !type_data.is_empty() {
                curve.arrowhead_type = Some(i8::from_le_bytes([type_data[0]]));
            }
        }

        // Parse arrowhead_center_size
        if let Some(size_data) = raw.get_property(CDXPROP_ARROWHEAD_CENTER_SIZE) {
            if size_data.len() >= 2 {
                curve.arrowhead_center_size = Some(i16::from_le_bytes(
                    size_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid arrowhead_center_size data".to_string()))?,
                ));
            }
        }

        // Parse arrowhead_width
        if let Some(width_data) = raw.get_property(CDXPROP_ARROWHEAD_WIDTH) {
            if width_data.len() >= 2 {
                curve.arrowhead_width = Some(i16::from_le_bytes(
                    width_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid arrowhead_width data".to_string()))?,
                ));
            }
        }

        // Parse arrow_arrowhead_head
        if let Some(head_data) = raw.get_property(CDXPROP_ARROW_ARROWHEAD_HEAD) {
            if !head_data.is_empty() {
                curve.arrow_arrowhead_head = Some(i8::from_le_bytes([head_data[0]]));
            }
        }

        // Parse arrow_arrowhead_tail
        if let Some(tail_data) = raw.get_property(CDXPROP_ARROW_ARROWHEAD_TAIL) {
            if !tail_data.is_empty() {
                curve.arrow_arrowhead_tail = Some(i8::from_le_bytes([tail_data[0]]));
            }
        }

        // Parse fill_type
        if let Some(fill_data) = raw.get_property(CDXPROP_FILL_TYPE) {
            if !fill_data.is_empty() {
                curve.fill_type = Some(i8::from_le_bytes([fill_data[0]]));
            }
        }

        // Parse closed
        if let Some(closed_data) = raw.get_property(CDXPROP_CLOSED) {
            if !closed_data.is_empty() {
                curve.closed = Some(closed_data[0] != 0);
            }
        }

        // Parse curve_spacing
        if let Some(spacing_data) = raw.get_property(CDXPROP_CURVE_SPACING) {
            if spacing_data.len() >= 2 {
                curve.curve_spacing = Some(i16::from_le_bytes(
                    spacing_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid curve_spacing data".to_string()))?,
                ));
            }
        }

        Ok(curve)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut raw = RawCdxObject::new(Self::TAG, self.id);

        // Write z_order
        if let Some(z_order) = self.z_order {
            raw.add_property(CDXPROP_Z_ORDER, z_order.to_le_bytes().to_vec());
        }

        // Write ignore_warnings
        if let Some(ignore) = self.ignore_warnings {
            raw.add_property(CDXPROP_IGNORE_WARNINGS, vec![ignore as u8]);
        }

        // Write chemical_warning
        if let Some(ref warning) = self.chemical_warning {
            raw.add_property(CDXPROP_CHEMICAL_WARNING, warning.encode()?);
        }

        // Write visible
        if let Some(visible) = self.visible {
            raw.add_property(CDXPROP_VISIBLE, vec![visible as u8]);
        }

        // Write bounding_box
        if let Some(ref bounds) = self.bounding_box {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&bounds.top.to_le_bytes());
            bytes.extend_from_slice(&bounds.left.to_le_bytes());
            bytes.extend_from_slice(&bounds.bottom.to_le_bytes());
            bytes.extend_from_slice(&bounds.right.to_le_bytes());
            raw.add_property(CDXPROP_BOUNDING_BOX, bytes);
        }

        // Write foreground_color
        if let Some(color) = self.foreground_color {
            raw.add_property(CDXPROP_FOREGROUND_COLOR, color.to_le_bytes().to_vec());
        }

        // Write background_color
        if let Some(color) = self.background_color {
            raw.add_property(CDXPROP_BACKGROUND_COLOR, color.to_le_bytes().to_vec());
        }

        // Write curve_type
        if let Some(curve_type) = self.curve_type {
            raw.add_property(CDXPROP_CURVE_TYPE, curve_type.to_le_bytes().to_vec());
        }

        // Write arrowhead_size
        if let Some(size) = self.arrowhead_size {
            raw.add_property(CDXPROP_ARROWHEAD_SIZE, size.to_le_bytes().to_vec());
        }

        // Write curve_points
        if let Some(ref points) = self.curve_points {
            let mut bytes = Vec::new();
            for point in points {
                bytes.extend_from_slice(&point.x.to_le_bytes());
                bytes.extend_from_slice(&point.y.to_le_bytes());
                bytes.extend_from_slice(&point.z.to_le_bytes());
            }
            raw.add_property(CDXPROP_CURVE_POINTS, bytes);
        }

        // Write curve_points_3d
        if let Some(ref points) = self.curve_points_3d {
            let mut bytes = Vec::new();
            for point in points {
                bytes.extend_from_slice(&point.x.to_le_bytes());
                bytes.extend_from_slice(&point.y.to_le_bytes());
                bytes.extend_from_slice(&point.z.to_le_bytes());
            }
            raw.add_property(CDXPROP_CURVE_POINTS3D, bytes);
        }

        // Write arrowhead_type
        if let Some(arrow_type) = self.arrowhead_type {
            raw.add_property(CDXPROP_ARROWHEAD_TYPE, vec![arrow_type as u8]);
        }

        // Write arrowhead_center_size
        if let Some(size) = self.arrowhead_center_size {
            raw.add_property(CDXPROP_ARROWHEAD_CENTER_SIZE, size.to_le_bytes().to_vec());
        }

        // Write arrowhead_width
        if let Some(width) = self.arrowhead_width {
            raw.add_property(CDXPROP_ARROWHEAD_WIDTH, width.to_le_bytes().to_vec());
        }

        // Write arrow_arrowhead_head
        if let Some(head) = self.arrow_arrowhead_head {
            raw.add_property(CDXPROP_ARROW_ARROWHEAD_HEAD, vec![head as u8]);
        }

        // Write arrow_arrowhead_tail
        if let Some(tail) = self.arrow_arrowhead_tail {
            raw.add_property(CDXPROP_ARROW_ARROWHEAD_TAIL, vec![tail as u8]);
        }

        // Write fill_type
        if let Some(fill) = self.fill_type {
            raw.add_property(CDXPROP_FILL_TYPE, vec![fill as u8]);
        }

        // Write closed
        if let Some(closed) = self.closed {
            raw.add_property(CDXPROP_CLOSED, vec![closed as u8]);
        }

        // Write curve_spacing
        if let Some(spacing) = self.curve_spacing {
            raw.add_property(CDXPROP_CURVE_SPACING, spacing.to_le_bytes().to_vec());
        }

        Ok(raw)
    }
}
