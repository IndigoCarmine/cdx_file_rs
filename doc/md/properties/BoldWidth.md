CDX Format Specification: BoldWidth Property
## BoldWidth Property

| CDXML Name: | BoldWidth |
| --- | --- |
| CDX Constant Name: | kCDXProp_BoldWidth |
| CDX Constant Value: | 0x0806 |
| Data Size: | [CDXCoordinate](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Bond](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Graphic](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Table](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_Spectrum](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_TLCPlate](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md),[kCDXObj_Arrow](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**  

The default bold bond width.

Generally, this property is used to provide a default distance value. ChemDraw uses it to determine the following:

- Thickness of bold bonds (single and double)
- Width of hashed bonds
- Width of wavy bonds
- Wedged bonds (wide end of wedge is 50% wider than the bold width)
- Line thickness in bold arrows, circles, ovals, rectangles, rounded rectangles, and arcs
- Line thickness in bold curves
- Spacing between the two lines in doubled curves

**If this property is absent:**  

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20190327001253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)