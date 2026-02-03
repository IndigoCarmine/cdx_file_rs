CDX Format Specification: Window_Size Property
## Window_Size Property

| CDXML Name: | WindowSize |
| --- | --- |
| CDX Constant Name: | kCDXProp_Window_Size |
| CDX Constant Value: | 0x0902 |
| Data Size: | [CDXPoint2D](/web/20160912003643/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20160912003643/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

Height and width of the document window.

Until ChemDraw 7.0, if this property is present, if this property is present, the [kCDXProp_Window_Position](Window_Position.md) property must also be present

**If this property is absent:**  

The window is sized appropriately for the screen.

---

[CDX Documentation index](/web/20160912003643/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)