use crate::cdx::arrow::Arrow;
use crate::cdx::values::{Point3d, Rectangle};
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::arrow_tags::*;
use crate::error::CdxError;

impl TaggedObject for Arrow {
    const TAG: u16 = CDXOBJ_ARROW;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut arrow = Arrow::new(raw.id);

        // Parse bounding box
        if let Some(bounds_data) = raw.get_property(CDXPROP_2D_BOUNDS) {
            if bounds_data.len() >= 32 {
                let left = f64::from_le_bytes(
                    bounds_data[0..8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bounds data".to_string()))?,
                );
                let top = f64::from_le_bytes(
                    bounds_data[8..16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bounds data".to_string()))?,
                );
                let right = f64::from_le_bytes(
                    bounds_data[16..24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bounds data".to_string()))?,
                );
                let bottom = f64::from_le_bytes(
                    bounds_data[24..32]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bounds data".to_string()))?,
                );
                arrow.bounding_box = Some(Rectangle {
                    top,
                    left,
                    bottom,
                    right,
                });
            }
        }

        // Parse Z-order
        if let Some(z_data) = raw.get_property(CDXPROP_Z_ORDER) {
            if z_data.len() >= 2 {
                arrow.z_order =
                    Some(i16::from_le_bytes(z_data[0..2].try_into().map_err(
                        |_| CdxError::Parse("Invalid z_order data".to_string()),
                    )?));
            }
        }

        // Parse fill type
        if let Some(fill_data) = raw.get_property(CDXPROP_FILL_TYPE) {
            if fill_data.len() >= 2 {
                arrow.fill_type =
                    Some(i16::from_le_bytes(fill_data[0..2].try_into().map_err(
                        |_| CdxError::Parse("Invalid fill_type data".to_string()),
                    )?));
            }
        }

        // Parse arrowhead properties
        if let Some(head_data) = raw.get_property(CDXPROP_ARROWHEAD_HEAD) {
            if head_data.len() >= 2 {
                arrow.arrowhead_head =
                    Some(i16::from_le_bytes(head_data[0..2].try_into().map_err(
                        |_| CdxError::Parse("Invalid arrowhead_head data".to_string()),
                    )?));
            }
        }

        if let Some(type_data) = raw.get_property(CDXPROP_ARROWHEAD_TYPE) {
            if type_data.len() >= 2 {
                arrow.arrowhead_type =
                    Some(i16::from_le_bytes(type_data[0..2].try_into().map_err(
                        |_| CdxError::Parse("Invalid arrowhead_type data".to_string()),
                    )?));
            }
        }

        if let Some(size_data) = raw.get_property(CDXPROP_HEAD_SIZE) {
            if size_data.len() >= 2 {
                arrow.head_size =
                    Some(i16::from_le_bytes(size_data[0..2].try_into().map_err(
                        |_| CdxError::Parse("Invalid head_size data".to_string()),
                    )?));
            }
        }

        // Parse 3D coordinates
        if let Some(head_3d) = raw.get_property(CDXPROP_3D_HEAD) {
            if head_3d.len() >= 24 {
                let x = f64::from_le_bytes(
                    head_3d[0..8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D head data".to_string()))?,
                );
                let y = f64::from_le_bytes(
                    head_3d[8..16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D head data".to_string()))?,
                );
                let z = f64::from_le_bytes(
                    head_3d[16..24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D head data".to_string()))?,
                );
                arrow.head_3d = Some(Point3d { x, y, z });
            }
        }

        if let Some(tail_3d) = raw.get_property(CDXPROP_3D_TAIL) {
            if tail_3d.len() >= 24 {
                let x = f64::from_le_bytes(
                    tail_3d[0..8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D tail data".to_string()))?,
                );
                let y = f64::from_le_bytes(
                    tail_3d[8..16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D tail data".to_string()))?,
                );
                let z = f64::from_le_bytes(
                    tail_3d[16..24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D tail data".to_string()))?,
                );
                arrow.tail_3d = Some(Point3d { x, y, z });
            }
        }

        if let Some(center_3d) = raw.get_property(CDXPROP_3D_CENTER) {
            if center_3d.len() >= 24 {
                let x = f64::from_le_bytes(
                    center_3d[0..8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D center data".to_string()))?,
                );
                let y = f64::from_le_bytes(
                    center_3d[8..16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D center data".to_string()))?,
                );
                let z = f64::from_le_bytes(
                    center_3d[16..24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D center data".to_string()))?,
                );
                arrow.center_3d = Some(Point3d { x, y, z });
            }
        }

        if let Some(major_axis_data) = raw.get_property(CDXPROP_3D_MAJOR_AXIS_END) {
            if major_axis_data.len() >= 24 {
                let x = f64::from_le_bytes(
                    major_axis_data[0..8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D major axis data".to_string()))?,
                );
                let y = f64::from_le_bytes(
                    major_axis_data[8..16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D major axis data".to_string()))?,
                );
                let z = f64::from_le_bytes(
                    major_axis_data[16..24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D major axis data".to_string()))?,
                );
                arrow.major_axis_end_3d = Some(Point3d { x, y, z });
            }
        }

        if let Some(minor_axis_data) = raw.get_property(CDXPROP_3D_MINOR_AXIS_END) {
            if minor_axis_data.len() >= 24 {
                let x = f64::from_le_bytes(
                    minor_axis_data[0..8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D minor axis data".to_string()))?,
                );
                let y = f64::from_le_bytes(
                    minor_axis_data[8..16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D minor axis data".to_string()))?,
                );
                let z = f64::from_le_bytes(
                    minor_axis_data[16..24]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid 3D minor axis data".to_string()))?,
                );
                arrow.minor_axis_end_3d = Some(Point3d { x, y, z });
            }
        }

        // Parse color
        if let Some(color_data) = raw.get_property(CDXPROP_COLOR) {
            if color_data.len() >= 2 {
                arrow.foreground_color =
                    Some(u16::from_le_bytes(color_data[0..2].try_into().map_err(
                        |_| CdxError::Parse("Invalid color data".to_string()),
                    )?));
            }
        }

        // Parse background color
        if let Some(bg_color_data) = raw.get_property(CDXPROP_BACKGROUND_COLOR) {
            if bg_color_data.len() >= 2 {
                arrow.background_color =
                    Some(i16::from_le_bytes(bg_color_data[0..2].try_into().map_err(
                        |_| CdxError::Parse("Invalid bg_color data".to_string()),
                    )?));
            }
        }

        // Parse line width
        if let Some(width_data) = raw.get_property(CDXPROP_LINE_WIDTH) {
            if width_data.len() >= 8 {
                arrow.line_width =
                    Some(f64::from_le_bytes(width_data[0..8].try_into().map_err(
                        |_| CdxError::Parse("Invalid line_width data".to_string()),
                    )?));
            }
        }

        Ok(arrow)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut raw = RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties: Vec::new(),
            children: Vec::new(),
        };

        // Serialize bounding box
        if let Some(rect) = &self.bounding_box {
            let mut bounds = Vec::with_capacity(32);
            bounds.extend_from_slice(&rect.left.to_le_bytes());
            bounds.extend_from_slice(&rect.top.to_le_bytes());
            bounds.extend_from_slice(&rect.right.to_le_bytes());
            bounds.extend_from_slice(&rect.bottom.to_le_bytes());
            raw.set_property(CDXPROP_2D_BOUNDS, bounds);
        }

        // Serialize Z-order
        if let Some(z) = self.z_order {
            raw.set_property(CDXPROP_Z_ORDER, z.to_le_bytes().to_vec());
        }

        // Serialize fill type
        if let Some(fill) = self.fill_type {
            raw.set_property(CDXPROP_FILL_TYPE, fill.to_le_bytes().to_vec());
        }

        // Serialize arrowhead properties
        if let Some(head) = self.arrowhead_head {
            raw.set_property(CDXPROP_ARROWHEAD_HEAD, head.to_le_bytes().to_vec());
        }

        if let Some(atype) = self.arrowhead_type {
            raw.set_property(CDXPROP_ARROWHEAD_TYPE, atype.to_le_bytes().to_vec());
        }

        if let Some(size) = self.head_size {
            raw.set_property(CDXPROP_HEAD_SIZE, size.to_le_bytes().to_vec());
        }

        // Serialize 3D coordinates
        if let Some(Point3d { x, y, z }) = self.head_3d {
            let mut coords = Vec::with_capacity(24);
            coords.extend_from_slice(&x.to_le_bytes());
            coords.extend_from_slice(&y.to_le_bytes());
            coords.extend_from_slice(&z.to_le_bytes());
            raw.set_property(CDXPROP_3D_HEAD, coords);
        }

        if let Some(Point3d { x, y, z }) = self.tail_3d {
            let mut coords = Vec::with_capacity(24);
            coords.extend_from_slice(&x.to_le_bytes());
            coords.extend_from_slice(&y.to_le_bytes());
            coords.extend_from_slice(&z.to_le_bytes());
            raw.set_property(CDXPROP_3D_TAIL, coords);
        }

        if let Some(Point3d { x, y, z }) = self.center_3d {
            let mut coords = Vec::with_capacity(24);
            coords.extend_from_slice(&x.to_le_bytes());
            coords.extend_from_slice(&y.to_le_bytes());
            coords.extend_from_slice(&z.to_le_bytes());
            raw.set_property(CDXPROP_3D_CENTER, coords);
        }

        if let Some(Point3d { x, y, z }) = self.major_axis_end_3d {
            let mut coords = Vec::with_capacity(24);
            coords.extend_from_slice(&x.to_le_bytes());
            coords.extend_from_slice(&y.to_le_bytes());
            coords.extend_from_slice(&z.to_le_bytes());
            raw.set_property(CDXPROP_3D_MAJOR_AXIS_END, coords);
        }

        if let Some(Point3d { x, y, z }) = self.minor_axis_end_3d {
            let mut coords = Vec::with_capacity(24);
            coords.extend_from_slice(&x.to_le_bytes());
            coords.extend_from_slice(&y.to_le_bytes());
            coords.extend_from_slice(&z.to_le_bytes());
            raw.set_property(CDXPROP_3D_MINOR_AXIS_END, coords);
        }

        // Serialize colors
        if let Some(color) = self.foreground_color {
            raw.set_property(CDXPROP_COLOR, color.to_le_bytes().to_vec());
        }

        if let Some(bg_color) = self.background_color {
            raw.set_property(CDXPROP_BACKGROUND_COLOR, bg_color.to_le_bytes().to_vec());
        }

        // Serialize line width
        if let Some(width) = self.line_width {
            raw.set_property(CDXPROP_LINE_WIDTH, width.to_le_bytes().to_vec());
        }

        Ok(raw)
    }
}
