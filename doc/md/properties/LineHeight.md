## LineHeight Property

| CDXML Name: | LineHeight |
| --- | --- |
| CDX Constant Name: | kCDXProp_LineHeight |
| CDX Constant Value: | 0x0702 |
| Data Size: | [UINT16](/web/20190816083716/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Text](/web/20190816083716/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The line height of a text object.

The value stored in this property is in screen units. So, for example, a reasonable line height for 12-point text would be 12 (or maybe a smidge more). There are, additionally, two special values: the value 0 provides Variable line height (determined by the tallest character on each line), and the value 1 provides Automatic line height (determined by the tallest character on any line in the text object).

This property is considered obsolete starting with ChemDraw 7.0. Values stored in the [kCDXProp_CaptionLineHeight](CaptionLineHeight.md) and [kCDXProp_LabelLineHeight](LabelLineHeight.md) properties are used preferentially, if present. ChemDraw will continue to write this property for backward-compatibility, however.

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190816083716/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet. If no information is found in the Style Sheet either, then atom labels have a Variable line height, and non-atomlabel text has an Automatic line height.

---

[CDX Documentation index](/web/20190816083716/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)