use crate::cdx::bracket_attachment::BracketAttachment;
use crate::cdx_tags::bracket_attachment_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(BracketAttachment, CDXOBJ_BRACKET_ATTACHMENT, {
    bracket_graphic_id: CDXPROP_BRACKET_GRAPHIC_ID,
});
