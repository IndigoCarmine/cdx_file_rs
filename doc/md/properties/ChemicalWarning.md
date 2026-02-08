## ChemicalWarning Property

| CDXML Name: | Warning |
| --- | --- |
| CDX Constant Name: | kCDXProp_ChemicalWarning |
| CDX Constant Value: | 0x0010 |
| Data Size: | [CDXString](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Bond](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Text](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Graphic](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Curve](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Curve.md),[kCDXObj_NamedAlternativeGroup](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/NamedAltGroup.md),[kCDXObj_Spectrum](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_Arrow](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 6.0 / (not read) |
| Required? | No |

**Description:**

A warning concerning possible chemical problems with this object.

This property stores a textual description of a warning. It is not generally expected that programs would parse this string for meaning, but rather display it verbatim if so desired.

**If this property is absent:**

This property is not read by ChemDraw. It is written as a courtesy only. There is no consequence to omitting it.

---

[CDX Documentation index](/web/20190326232515/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)