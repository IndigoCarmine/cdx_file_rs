CDX Format Specification: Visible Property
## Visible Property

| CDXML Name: | Visible |
| --- | --- |
| CDX Constant Name: | kCDXProp_Visible |
| CDX Constant Value: | 0x0011 |
| Data Size: | [CDXBoolean](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Bond](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Text](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Graphic](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Curve](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Curve.md),[kCDXObj_Table](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_NamedAlternativeGroup](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/NamedAltGroup.md),[kCDXObj_Spectrum](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_TLCPlate](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md),[kCDXObj_TLCLane](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCLane.md),[kCDXObj_TLCSpot](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCSpot.md),[kCDXObj_Arrow](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

The object is visible if non-zero.

Note that ChemDraw does not currently offer a general way for making an object visible again once it is made invisible. As of ChemDraw 7.0, this property is used only for [Object Tag](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md) objects, which can be hidden and reshown.

**If this property is absent:**  

The object will be displayed

---

[CDX Documentation index](/web/20190327225439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)