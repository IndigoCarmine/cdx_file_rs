CDX Format Specification: CaptionStyleColor Property
## CaptionStyleColor Property

| CDXML Name: | CaptionColor |
| --- | --- |
| CDX Constant Name: | kCDXProp_CaptionStyleColor |
| CDX Constant Value: | 0x0821 |
| Data Size: | [INT16](/web/20190121161430/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Document](/web/20190121161430/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Text](/web/20190121161430/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | (not written/read) |
| Required? | No |

**Description:**  

The default color for captions (non-atom-label text objects).

Generally, this property is used to provide a default font color for chemically-*in*significant text.

**If this property is absent:**  

This property is not read (or written) by ChemDraw, but is defined for future compatibility. There is no consequence to omitting it.

---

[CDX Documentation index](/web/20190121161430/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)