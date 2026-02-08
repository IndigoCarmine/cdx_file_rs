## BackgroundColor Property

| CDXML Name: | bgcolor |
| --- | --- |
| CDX Constant Name: | kCDXProp_BackgroundColor |
| CDX Constant Value: | 0x0302 |
| Data Size: | [INT16](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Page](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Page.md),[kCDXObj_Node](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Bond](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Graphic](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Curve](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Curve.md),[kCDXObj_EmbeddedObject](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md),[kCDXObj_Table](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_NamedAlternativeGroup](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/NamedAltGroup.md),[kCDXObj_Spectrum](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_TLCPlate](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md),[kCDXObj_Arrow](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | (not written/read) |
| Required? | No |

**Description:**

The background color of an object represented as the two-based index into the object's color table.

**If this property is absent:**

A value of 2 is assumed, that being defined as the default background color in the [color table](ColorTable.md). This property is not read (or written) by ChemDraw, but is defined for future compatibility. There is no consequence to omitting it.

---

[CDX Documentation index](/web/20190327223006/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)