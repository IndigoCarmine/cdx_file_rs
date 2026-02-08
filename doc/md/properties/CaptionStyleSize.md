## CaptionStyleSize Property

| CDXML Name: | CaptionSize |
| --- | --- |
| CDX Constant Name: | kCDXProp_CaptionStyleSize |
| CDX Constant Value: | 0x081D |
| Data Size: | [INT16](/web/20190326232725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Document](/web/20190326232725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Text](/web/20190326232725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Graphic](/web/20190326232725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Arrow](/web/20190326232725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

The default font size for captions (non-atom-label text objects).

Generally, this property is used to provide a default size for chemically-*in*significant text.

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190326232725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20190326232725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)