## Window_Position Property

| CDXML Name: | WindowPosition |
| --- | --- |
| CDX Constant Name: | kCDXProp_Window_Position |
| CDX Constant Value: | 0x0901 |
| Data Size: | [CDXPoint2D](/web/20160913174353/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20160913174353/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The top-left position of the main viewing window.

The position stored by this property is relative to the top-left corner of the application workspace for Windows MDI applications, and relative to the screen for other platforms.

Until ChemDraw 7.0, if this property is present, the [kCDXProp_Window_Size](Window_Size.md) property must also be present.

**If this property is absent:**

The window is located at the best position on the screen, considering any other windows present (for example, it might be offset from the top corner).

---

[CDX Documentation index](/web/20160913174353/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)