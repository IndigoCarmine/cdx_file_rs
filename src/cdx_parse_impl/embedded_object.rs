use crate::cdx::embedded_object::EmbeddedObject;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx::values::Rectangle;
use crate::cdx_tags::embedded_object_tags::*;
use crate::error::CdxError;


impl TaggedObject for EmbeddedObject {
    const TAG: u16 = CDXOBJ_EMBEDDED_OBJECT;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut obj = EmbeddedObject::new(raw.id);

        // Parse z_order
        if let Some(z_data) = raw.get_property(CDXPROP_Z_ORDER) {
            if z_data.len() >= 2 {
                obj.z_order = Some(i16::from_le_bytes(
                    z_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid z_order data".to_string()))?,
                ));
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
                obj.bounding_box = Some(Rectangle {
                    top,
                    left,
                    bottom,
                    right,
                });
            }
        }

        // Parse rotation_angle
        if let Some(angle_data) = raw.get_property(CDXPROP_ROTATION_ANGLE) {
            if angle_data.len() >= 4 {
                obj.rotation_angle = Some(i32::from_le_bytes(
                    angle_data[0..4]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid rotation_angle data".to_string()))?,
                ));
            }
        }

        // Parse foreground_color
        if let Some(color_data) = raw.get_property(CDXPROP_FOREGROUND_COLOR) {
            if color_data.len() >= 2 {
                obj.foreground_color = Some(u16::from_le_bytes(
                    color_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid foreground_color data".to_string()))?,
                ));
            }
        }

        // Parse background_color
        if let Some(color_data) = raw.get_property(CDXPROP_BACKGROUND_COLOR) {
            if color_data.len() >= 2 {
                obj.background_color = Some(i16::from_le_bytes(
                    color_data[0..2]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid background_color data".to_string()))?,
                ));
            }
        }

        // Parse image data properties
        if let Some(data) = raw.get_property(CDXPROP_PICTURE_EDITION) {
            obj.picture_edition = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_PICTURE_EDITION_ALIAS) {
            obj.picture_edition_alias = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_MAC_PICT) {
            obj.mac_pict = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_WINDOWS_METAFILE) {
            obj.windows_metafile = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_OLE_OBJECT) {
            obj.ole_object = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_ENHANCED_METAFILE) {
            obj.enhanced_metafile = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_GIF) {
            obj.gif = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_TIFF) {
            obj.tiff = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_PNG) {
            obj.png = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_JPEG) {
            obj.jpeg = Some(data.clone());
        }
        if let Some(data) = raw.get_property(CDXPROP_BMP) {
            obj.bmp = Some(data.clone());
        }

        Ok(obj)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut raw = RawCdxObject::new(Self::TAG, self.id);

        // Write z_order
        if let Some(z_order) = self.z_order {
            raw.add_property(CDXPROP_Z_ORDER, z_order.to_le_bytes().to_vec());
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

        // Write rotation_angle
        if let Some(angle) = self.rotation_angle {
            raw.add_property(CDXPROP_ROTATION_ANGLE, angle.to_le_bytes().to_vec());
        }

        // Write foreground_color
        if let Some(color) = self.foreground_color {
            raw.add_property(CDXPROP_FOREGROUND_COLOR, color.to_le_bytes().to_vec());
        }

        // Write background_color
        if let Some(color) = self.background_color {
            raw.add_property(CDXPROP_BACKGROUND_COLOR, color.to_le_bytes().to_vec());
        }

        // Write image data properties
        if let Some(ref data) = self.picture_edition {
            raw.add_property(CDXPROP_PICTURE_EDITION, data.clone());
        }
        if let Some(ref data) = self.picture_edition_alias {
            raw.add_property(CDXPROP_PICTURE_EDITION_ALIAS, data.clone());
        }
        if let Some(ref data) = self.mac_pict {
            raw.add_property(CDXPROP_MAC_PICT, data.clone());
        }
        if let Some(ref data) = self.windows_metafile {
            raw.add_property(CDXPROP_WINDOWS_METAFILE, data.clone());
        }
        if let Some(ref data) = self.ole_object {
            raw.add_property(CDXPROP_OLE_OBJECT, data.clone());
        }
        if let Some(ref data) = self.enhanced_metafile {
            raw.add_property(CDXPROP_ENHANCED_METAFILE, data.clone());
        }
        if let Some(ref data) = self.gif {
            raw.add_property(CDXPROP_GIF, data.clone());
        }
        if let Some(ref data) = self.tiff {
            raw.add_property(CDXPROP_TIFF, data.clone());
        }
        if let Some(ref data) = self.png {
            raw.add_property(CDXPROP_PNG, data.clone());
        }
        if let Some(ref data) = self.jpeg {
            raw.add_property(CDXPROP_JPEG, data.clone());
        }
        if let Some(ref data) = self.bmp {
            raw.add_property(CDXPROP_BMP, data.clone());
        }

        Ok(raw)
    }
}
