CDX Format Specification: Positioning Property
## Positioning Property

| CDXML Name: | PositioningType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Positioning |
| CDX Constant Value: | 0x0D06 |
| Data Size: | [INT8](/web/20190327220850/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_ObjectTag](/web/20190327220850/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md),[kCDXObj_ChemicalProperty](/web/20190327220850/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ChemicalProperty.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

How the object should be positioned with respect to its containing object.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | auto | Indicator is positioned automatically |
| 1 | angle | Indicator is positioned by a specified angle (requires[kCDXProp_PositioningAngle](PositioningAngle.md)) |
| 2 | offset | Indicator is positioned by a specified offset (requires[kCDXProp_PositioningOffset](PositioningOffset.md)) |
| 3 | absolute | Indicator is positioned by a specified offset from the top-left of the page (requires[kCDXProp_PositioningOffset](PositioningOffset.md)) |

**If this property is absent:**  

The object is positioned automatically.

---

[CDX Documentation index](/web/20190327220850/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)