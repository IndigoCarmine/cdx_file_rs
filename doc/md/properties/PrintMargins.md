## PrintMargins Property

| CDXML Name: | PrintMargins |
| --- | --- |
| CDX Constant Name: | kCDXProp_PrintMargins |
| CDX Constant Value: | 0x0802 |
| Data Size: | [CDXRectangle](/web/20160913174350/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20160913174350/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The outer margins of the Document.

Although stored as a CDXRectangle, this property does not represent a rectangular object. Rather, it stores the linear distances for the margins on the four sides of the page.

**If this property is absent:**

ChemDraw will use the value from its last-used Style Sheet. If no Style Sheet is found, ChemDraw assumes nominal margins of one-half inch on all four sides.

---

[CDX Documentation index](/web/20160913174350/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)