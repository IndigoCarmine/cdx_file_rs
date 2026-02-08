## IgnoreWarnings Property

| CDXML Name: | IgnoreWarnings |
| --- | --- |
| CDX Constant Name: | kCDXProp_IgnoreWarnings |
| CDX Constant Value: | 0x000F |
| Data Size: | [CDXBooleanImplied](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Bond](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Text](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Graphic](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Curve](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Curve.md),[kCDXObj_NamedAlternativeGroup](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/NamedAltGroup.md),[kCDXObj_Spectrum](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_Arrow](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**

Signifies whether chemical warnings should be suppressed on this object.

ChemDraw will display chemical warnings (in the form of red boxes) on objects that have various types of unusual chemistry. If this property is present, no warnings will be displayed on the object, even if they normally would have been.

**If this property is absent:**

Warnings will be displayed if appropriate.

---

[CDX Documentation index](/web/20190326221708/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)