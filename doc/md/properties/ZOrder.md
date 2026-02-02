CDX Format Specification: ZOrder Property
## ZOrder Property

| CDXML Name: | Z |
| --- | --- |
| CDX Constant Name: | kCDXProp_ZOrder |
| CDX Constant Value: | 0x000A |
| Data Size: | [INT16](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Bond](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Text](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Graphic](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Curve](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Curve.md),[kCDXObj_EmbeddedObject](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md),[kCDXObj_Table](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_NamedAlternativeGroup](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/NamedAltGroup.md),[kCDXObj_Spectrum](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_TLCPlate](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md),[kCDXObj_Arrow](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

Back-to-front ordering index in 2D drawing.

Items with a lower value will be ordered behind items with a higher value. ChemDraw uses this property to determine, for example, which bond is in front when two bonds cross. Note that this is a graphical property only; it can be safely omitted if a file is only intended to have chemical meaning and the exact visual display is unimportant. It can also be omitted in files where it is known that no objects overlap.

**If this property is absent:**  

The object will be treated as if the property was present with a value of zero. That is, the object will be arranged with all other objects having a Z-ordering index of zero (those objects will be arranged among themselves randomly), and all other objects will be arranged in order, on top of them.

---

[CDX Documentation index](/web/20190326232735/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)