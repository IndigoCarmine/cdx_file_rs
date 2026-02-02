CDX Format Specification: ForegroundColor Property
## ForegroundColor Property

| CDXML Name: | color |
| --- | --- |
| CDX Constant Name: | kCDXProp_ForegroundColor |
| CDX Constant Value: | 0x0301 |
| Data Size: | [UINT16](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Bond](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Graphic](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Curve](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Curve.md),[kCDXObj_EmbeddedObject](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md),[kCDXObj_Table](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_NamedAlternativeGroup](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/NamedAltGroup.md),[kCDXObj_Spectrum](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_Border](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Border.md),[kCDXObj_Geometry](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md),[kCDXObj_Constraint](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md),[kCDXObj_TLCPlate](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md),[s](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Style.md),[kCDXObj_Arrow](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The foreground color of an object represented as the two-based index into the object's color table.

A value indicating the 2-based index of a color in the [color table](ColorTable.md) (a value of 2 indicates the first item, a value of 3 indicates the second item, etc.).

**If this property is absent:**  

A value of 3 is assumed, that being defined as the default foreground color in the [color table](ColorTable.md).

---

[CDX Documentation index](/web/20190327224706/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)