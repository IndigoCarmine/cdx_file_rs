## Polymer_RepeatPattern Property

| CDXML Name: | PolymerRepeatPattern |
| --- | --- |
| CDX Constant Name: | kCDXProp_Polymer_RepeatPattern |
| CDX Constant Value: | 0x0A25 |
| Data Size: | [INT8](/web/20190327221016/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190327221016/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_BracketedGroup](/web/20190327221016/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

The head-to-tail connectivity of objects contained within the bracket.

For simple linear polymers, the repeating units might connect head-to-tail or head-to-head (or their repeat pattern might be a mixture or unknown):

Although this property is written to both [Graphic](/web/20190327221016/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) objects and to [Bracketed Groups](/web/20190327221016/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md), as of ChemDraw 7.0 it is read only from Graphic objects.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | HeadToTail | One end of the repeating unit is connected to the other end of the adjacent repeating unit |
| 1 | HeadToHead | One end of the repeating unit is connected to the same end of the adjacent repeating unit |
| 2 | EitherUnknown | A mixture of the above, or an unknown repeat pattern |

**If this property is absent:**

The repeating unit is assumed to connect head-to-tail.

---

[CDX Documentation index](/web/20190327221016/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)