## LineStarts Property

| CDXML Name: | LineStarts |
| --- | --- |
| CDX Constant Name: | kCDXProp_LineStarts |
| CDX Constant Value: | 0x0704 |
| Data Size: | [INT16ListWithCounts](/web/20190121162220/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/INT16ListWithCounts.md) |
| Property of objects: | [kCDXObj_Text](/web/20190121162220/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The number of lines of a text object followed by that many values indicating the zero-based text position of each line start.

**If this property is absent:**

The line starts are inferred solely from the presence of end-of-line (0x0D) characters in the text.

---

[CDX Documentation index](/web/20190121162220/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)