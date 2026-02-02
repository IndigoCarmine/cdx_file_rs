CDX Format Specification: ObjectTag_Tracking Property
## ObjectTag_Tracking Property

| CDXML Name: | Tracking |
| --- | --- |
| CDX Constant Name: | kCDXProp_ObjectTag_Tracking |
| CDX Constant Value: | 0x0D03 |
| Data Size: | [CDXBoolean](/web/20190326220453/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_ObjectTag](/web/20190326220453/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**  

The tag will participate in tracking if non-zero.

The exact nature of 'participate in tracking' is currently undefined, and this property is not currently used in any way by ChemDraw. It is provided for future expansion only. Applications creating CDX files should omit this value or leave it at its default (false) value. Applications reading CDX files should ignore this value until its precise meaning is defined.

**If this property is absent:**  

The tag will not participate in tracking.

---

[CDX Documentation index](/web/20190326220453/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)