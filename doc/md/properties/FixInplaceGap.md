CDX Format Specification: FixInplaceGap Property
## FixInplaceGap Property

| CDXML Name: | FixInPlaceGap |
| --- | --- |
| CDX Constant Name: | kCDXProp_FixInplaceGap |
| CDX Constant Value: | 0x0826 |
| Data Size: | [CDXPoint2D](/web/20160911232548/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20160911232548/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**  

Defines a padding for OLE In-Place editing.

When OLE In-Place editing is activated, the host application has the option of informing the server application about the window area assigned (by the host application) for in-place editing. In such a case, the server should try to display its document in that area. If the document contents are smaller than the specified size, there is some flexibility over exactly where within the assigned area those contents should be displayed. This property specifies an exact amount of padding (whitespace) between the right and bottom edges of the document content and the right and bottom edges of the OLE frame.

This property is meaningful only in an OLE In-Place editing environment. It is not expected that other applications would read or write this value in CDX files. See also the [kCDXProp_FixInPlaceExtent](FixInPlaceExtent.md) property.

**If this property is absent:**  

The document will behave as if there is no padding between the right and bottom edges of the contained objects and the right and bottom edges of the OLE frame.

---

[CDX Documentation index](/web/20160911232548/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)