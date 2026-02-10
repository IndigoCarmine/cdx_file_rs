use crate::cdx::seg_datum::SegDatum;
use crate::cdx_tags::segment_datum_tags::*;
use crate::cdx::seg_datum::SegDatumData;
use crate::error::CdxError;
use crate::cdx::binary_codec::BinaryCodec;

impl crate::cdx_parse_impl::tagged_object::TaggedObject for SegDatum {
    const TAG:u16 = CDXOBJ_SEGMENTDATUM;
    fn from_raw(raw:crate::cdx_parse_impl::raw_nodes::RawCdxObject) -> Result<Self,crate::error::CdxError>{
        let mut obj =  <SegDatum>::new(raw.id);
        let val = raw.get_prop(CDXPROP_SEGMENTDATUM_TYPE);
        if let Ok(val) = val {
            obj.sg_data_type = val;
        }else if let Err(e) = val {
            println!("Get error: {:?}",e);
            println!("Warning: Missing property tag=0x{:x} in object tag=0x{:x}",CDXPROP_SEGMENTDATUM_TYPE,raw.tag);
            println!("Raw object properties: {:?}",raw.get_property(CDXPROP_SEGMENTDATUM_TYPE));
        }

        let val = raw.get_prop(CDXPROP_SEGMENTDATUM_PROPERTY_TYPE);
        if let Ok(val) = val {
            obj.sg_property_type = val;
        }else if let Err(e) = val {
            println!("Get error: {:?}",e);
            println!("Warning: Missing property tag=0x{:x} in object tag=0x{:x}",CDXPROP_SEGMENTDATUM_PROPERTY_TYPE,raw.tag);
            println!("Raw object properties: {:?}",raw.get_property(CDXPROP_SEGMENTDATUM_PROPERTY_TYPE));
        }
        

        let val = raw.get_property(CDXPROP_SEGMENTDATUM_VALUE);
        let flat: Vec<u8> = val.iter().flat_map(|v| v.iter()).copied().collect();
                   
        obj.sg_data_value = match obj.sg_data_type {
            Some(4)=>{
                //String
                {
                     String::decode(flat.as_slice()).map(|s|Some(SegDatumData::String(s))).map_err(|e|CdxError::DecodeError(e.to_string()))?
                }
            }
            Some(3) =>{
                //Float64
                f64::decode(flat.as_slice()).map(|f|Some(SegDatumData::Float64(f))).map_err(|e|CdxError::DecodeError(e.to_string()))?
            }
            Some(1) =>{
                //Float64
                f64::decode(flat.as_slice()).map(|f|Some(SegDatumData::Float64(f))).map_err(|e|CdxError::DecodeError(e.to_string()))?
            }
            _=>None,
        };

        
        let val = raw.get_prop(CDXPROP_SEGMENTDATUM_IS_READONLY);
        if let Ok(val) = val {
            obj.is_read_only = val;
        }else if let Err(e) = val {
            println!("Get error: {:?}",e);
            println!("Warning: Missing property tag=0x{:x} in object tag=0x{:x}",CDXPROP_SEGMENTDATUM_IS_READONLY,raw.tag);
            println!("Raw object properties: {:?}",raw.get_property(CDXPROP_SEGMENTDATUM_IS_READONLY));
        }Ok(obj)
    }
    fn to_raw(&self) -> Result<crate::cdx_parse_impl::raw_nodes::RawCdxObject,crate::error::CdxError>{
        let mut raw = crate::cdx_parse_impl::raw_nodes::RawCdxObject::new(Self::TAG,self.id);
        if let Some(val) =  &self.sg_data_type {
            raw.set_prop(CDXPROP_SEGMENTDATUM_TYPE,val)?;
        }if let Some(val) =  &self.sg_data_value {
            raw.set_prop(CDXPROP_SEGMENTDATUM_VALUE,val)?;
        }if let Some(val) =  &self.sg_property_type {
            raw.set_prop(CDXPROP_SEGMENTDATUM_PROPERTY_TYPE,val)?;
        }if let Some(val) =  &self.is_read_only {
            raw.set_prop(CDXPROP_SEGMENTDATUM_IS_READONLY,val)?;
        }Ok(raw)
    }

    }
