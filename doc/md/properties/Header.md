CDX Format Specification: Header Property
## Header Property

| CDXML Name: | Header |
| --- | --- |
| CDX Constant Name: | kCDXProp_Header |
| CDX Constant Value: | 0x0815 |
| Data Size: | [CDXString](/web/20160913174307/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Page](/web/20160913174307/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Page.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**  

The text of the header.

A header, if present, will print at the top of each page (for [Documents](/web/20160913174307/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) with a [DrawingSpaceType](DrawingSpaceType.md) of type Pages) or at the top of the drawing space (for Documents with a DrawingSpaceType of type Pages. It will be offset from the top of the page by the amount specified by the [kCDXProp_HeaderPosition](HeaderPosition.md) property.

In addition to the raw text, the following special sequences may be used:

| &f | File name |
| --- | --- |
| &p | Page number |
| &d | Date printed |
| &t | Time printed |
| &r | Text that follows should be right-justified |
| &c | Text that follows should be centered |
| &l | Text that follows should be left-justified |

**If this property is absent:**  

The document has no header.

---

[CDX Documentation index](/web/20160913174307/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)