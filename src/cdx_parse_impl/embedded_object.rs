use crate::cdx::embedded_object::EmbeddedObject;
use crate::cdx_tags::embedded_object_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(EmbeddedObject, CDXOBJ_EMBEDDED_OBJECT, {
    z_order: CDXPROP_Z_ORDER,
    bounding_box: CDXPROP_BOUNDING_BOX,
    rotation_angle: CDXPROP_ROTATION_ANGLE,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
    picture_edition: CDXPROP_PICTURE_EDITION,
    picture_edition_alias: CDXPROP_PICTURE_EDITION_ALIAS,
    mac_pict: CDXPROP_MAC_PICT,
    windows_metafile: CDXPROP_WINDOWS_METAFILE,
    ole_object: CDXPROP_OLE_OBJECT,
    enhanced_metafile: CDXPROP_ENHANCED_METAFILE,
    gif: CDXPROP_GIF,
    tiff: CDXPROP_TIFF,
    png: CDXPROP_PNG,
    jpeg: CDXPROP_JPEG,
    bmp: CDXPROP_BMP,
});
