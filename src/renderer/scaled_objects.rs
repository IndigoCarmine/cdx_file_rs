macro_rules! impl_object_from_raw {
    (
        $raw:ty => $dst:ty {
            $( $field:ident : $kind:tt ),* $(,)?
        }
    ) => {
        impl From<$raw> for $dst {
            fn from(raw: $raw) -> Self {
                Self {
                    $(
                        $field: impl_object_from_raw!(@field raw.$field, $kind)
                    ),*
                }
            }
        }
    };

    (@field $v:expr, passthrough) => {
        $v
    };

    (@field $v:expr, scaled($scale:expr)) => {
        $v as f32 / $scale
    };
}
