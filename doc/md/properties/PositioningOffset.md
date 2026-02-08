## PositioningOffset Property

| CDXML Name: | PositioningOffset |
| --- | --- |
| CDX Constant Name: | kCDXProp_PositioningOffset |
| CDX Constant Value: | 0x0D08 |
| Data Size: | [CDXPoint2D](/web/20190326220823/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_ObjectTag](/web/20190326220823/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md),[kCDXObj_ChemicalProperty](/web/20190326220823/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ChemicalProperty.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

Offset positioning.

This object should be positioned at a given offset from the relevant point (usually the center) of its containing object (for [kCDXProp_Positioning](Positioning.md) = kCDXPositioningType_Offset) or from the top left of the page (for [kCDXProp_Positioning](Positioning.md) = kCDXPositioningType_Absolute).

**If this property is absent:**

This object will be positioned with zero offset from the relevant point of its containing object (that is, at the center of the object).

---

[CDX Documentation index](/web/20190326220823/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)