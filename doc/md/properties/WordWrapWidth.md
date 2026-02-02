CDX Format Specification: WordWrapWidth Property
## WordWrapWidth Property

| CDXML Name: | WordWrapWidth |
| --- | --- |
| CDX Constant Name: | kCDXProp_WordWrapWidth |
| CDX Constant Value: | 0x0703 |
| Data Size: | [INT16](/web/20190121155331/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Text](/web/20190121155331/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The word-wrap width of a text object.

This property stores a value measured in screen pixels at 100% magnification.

**If this property is absent:**  

The wrapping of the text is undefined. In ChemDraw, that means that the text will wrap only when it reaches the edge(s) of the containing [Page](/web/20190121155331/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Page.md).

---

[CDX Documentation index](/web/20190121155331/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)