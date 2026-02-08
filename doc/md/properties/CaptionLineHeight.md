## CaptionLineHeight Property

| CDXML Name: | CaptionLineHeight |
| --- | --- |
| CDX Constant Name: | kCDXProp_CaptionLineHeight |
| CDX Constant Value: | 0x0707 |
| Data Size: | [INT16](/web/20190121145809/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Document](/web/20190121145809/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Text](/web/20190121145809/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

Text line height for non-atomlabel text objects

The value stored in this property is in screen units. So, for example, a reasonable line height for 12-point text would be 12 (or maybe a smidge more). There are, additionally, two special values: the value 0 provides Variable line height (determined by the tallest character on each line), and the value 1 provides Automatic line height (determined by the tallest character on any line in the text object).

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190121145809/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet. If no information is found in the Style Sheet either, the line height is treated as Automatic.

---

[CDX Documentation index](/web/20190121145809/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)