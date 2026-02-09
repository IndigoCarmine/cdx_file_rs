use crate::cdx::annotation::Annotation;
use crate::cdx_tags::anotation_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Annotation, CDXOBJ_ANNOTATION, {
    keyword: CDXPROP_ANNOTATION_KEYWORD,
    content: CDXPROP_ANNOTATION_CONTENT,
});
