## FixInplaceExtent Property

| CDXML Name: | FixInPlaceExtent |
| --- | --- |
| CDX Constant Name: | kCDXProp_FixInplaceExtent |
| CDX Constant Value: | 0x0824 |
| Data Size: | [CDXPoint2D](/web/20160912060852/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20160912060852/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

Defines a size for OLE In-Place editing.

When OLE In-Place editing is activated, the host application has the option of informing the server application about the window area assigned (by the host application) for in-place editing. In such a case, the server should try to display its document in that area, and should not request that the host application change the size of that area. Usually, such an area is prescribed only by those hosts that display their information in a form-like environment. For example, FileMaker and Microsoft Access use this option. On the other hand, text editors generally allow the in-place area to expand free as long as it fits into the page. This information is sent to the server application only when it changes, and hence must be stored in the document so that it is available when the document is activated the next time.

This property is meaningful only in an OLE In-Place editing environment. It is not expected that other applications would read or write this value in CDX files. See also the [kCDXProp_FixInPlaceGap](FixInPlaceGap.md) property.

**If this property is absent:**  

The document will behave as if it has no external constraints imposed on the size of the in-place active area.

---

[CDX Documentation index](/web/20160912060852/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)