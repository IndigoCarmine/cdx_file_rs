CDX Format Specification: PositioningAngle Property
## PositioningAngle Property

| CDXML Name: | PositioningAngle |
| --- | --- |
| CDX Constant Name: | kCDXProp_PositioningAngle |
| CDX Constant Value: | 0x0D07 |
| Data Size: | [INT32](/web/20190327001600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_ObjectTag](/web/20190327001600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md),[kCDXObj_ChemicalProperty](/web/20190327001600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ChemicalProperty.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

Angular positioning, in degrees * 65536.

This object should be positioned along a vector at the appropriate angle, with the zero-degree position being the positive X axis and increasing clockwise.

**If this property is absent:**  

This object will be positioned at the zero degree position (that is, directly to the right of its containing object)

---

[CDX Documentation index](/web/20190327001600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)