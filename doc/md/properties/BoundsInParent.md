CDX Format Specification: BoundsInParent Property
## BoundsInParent Property

| CDXML Name: | BoundsInParent |
| --- | --- |
| CDX Constant Name: | kCDXProp_BoundsInParent |
| CDX Constant Value: | 0x206 |
| Data Size: | [CDXRectangle](/web/20160913174615/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Page](/web/20160913174615/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Page.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

The rectangle containing a page in the coordinate space of the containing page.

In ChemDraw 7.0, this is used only for pages that represent individual cells within a [Table](/web/20160913174615/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md). In that case, the value stored by this property represents the bounds of a particular cell in the coordinate system of the page containing the table. For tables, it is essential that the BoundsInParent values be consistent: i.e., for two adjacent cells/pages, the coordinates of their common border must be exactly the same in the respective BoundsInParent properties.

**If this property is absent:**  

Although not technically mandatory, if this property is omitted from a page that serves as a cell within a [Table](/web/20160913174615/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md), the bounds of that cell will be treated as the null rectangle (0, 0, 0, 0), which isn't very useful.

---

[CDX Documentation index](/web/20160913174615/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)