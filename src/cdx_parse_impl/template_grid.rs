use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::template_grid::TemplateGrid;
use crate::cdx::values::Point2d;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::template_grid_tags::*;
use crate::error::CdxError;

impl TaggedObject for TemplateGrid {
    const TAG: u16 = CDXOBJ_TEMPLATE_GRID;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut template = TemplateGrid::new(raw.id);

        // Parse 2D extent (Required - CDXPoint2D)
        template.extent_2d = raw
            .get_property(CDXPROP_2D_EXTENT)
            .and_then(|v| Point2d::decode(v).ok());

        // Parse pane height (Required - CDXCoordinate as f64)
        template.template_pane_height = raw
            .get_property(CDXPROP_TEMPLATE_PANE_HEIGHT)
            .and_then(|v| f64::decode(v).ok());

        // Parse number of rows (Required - INT16)
        template.template_num_rows = raw
            .get_property(CDXPROP_TEMPLATE_NUM_ROWS)
            .and_then(|v| i16::decode(v).ok());

        // Parse number of columns (Required - INT16)
        template.template_num_columns = raw
            .get_property(CDXPROP_TEMPLATE_NUM_COLUMNS)
            .and_then(|v| i16::decode(v).ok());

        Ok(template)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        if let Some(ref v) = self.extent_2d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_2D_EXTENT,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.template_pane_height {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TEMPLATE_PANE_HEIGHT,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.template_num_rows {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TEMPLATE_NUM_ROWS,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.template_num_columns {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TEMPLATE_NUM_COLUMNS,
                value: v.encode()?,
            });
        }

        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}
