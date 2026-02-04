macro_rules! generate_unknown_xxx_struct {
    ( $name:ident, $tag:expr ) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub id: u32,
            pub raw_properties: Vec<(u16, Vec<u8>)>,
        }

        impl $name {
            pub fn new(id: u32) -> Self {
                Self {
                    id,
                    raw_properties: Vec::new(),
                }
            }
        }
    };
}


generate_unknown_xxx_struct!(UnknownObject801D, 0x801D);
generate_unknown_xxx_struct!(UnknownObject802B, 0x802B);
generate_unknown_xxx_struct!(UnknownObject801E, 0x801E);
generate_unknown_xxx_struct!(UnknownObject801F, 0x801F);