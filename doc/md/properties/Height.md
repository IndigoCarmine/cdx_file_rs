CDX Format Specification: Height Property
## Height Property

| CDXML Name: | Height |
| --- | --- |
| CDX Constant Name: | kCDXProp_Height |
| CDX Constant Value: | 0x0813 |
| Data Size: | [CDXCoordinate](/web/20160912011219/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Page](/web/20160912011219/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Page.md),[kCDXObj_TLCSpot](/web/20160912011219/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCSpot.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | Until ChemDraw 7.0 |

**Description:**  

The height of an object in CDX coordinate units, possibly in a rotated or skewed frame.

When used as a property of a document, this property is meaningful only for documents with a [DrawingSpaceType](DrawingSpaceType.md) of type Poster. This value overrides he default document width will be used as specified in the [kCDXProp_MacPrintInfo](MacPrintInfo.md) property.

**If this property is absent:**  

When used as a property of a document, the default document width will be used as specified in the [kCDXProp_MacPrintInfo](MacPrintInfo.md) property. When used for other objects, the width will be assumed to be zero.

---

[CDX Documentation index](/web/20160912011219/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)