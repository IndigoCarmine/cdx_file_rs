## Polymer_FlipType Property

| CDXML Name: | PolymerFlipType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Polymer_FlipType |
| CDX Constant Value: | 0x0A26 |
| Data Size: | [INT8](/web/20190327000258/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190327000258/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_BracketedGroup](/web/20190327000258/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

The flip state of objects contained within the bracket.

For ladder-type polymers (those polymers with two connecting bonds on each side), there is a potential ambiguity in the order of the connections. The Flip Type property is designed to resolve that ambiguity. This property is appropriate *only* for brackets with exactly two crossing bonds.

Although this property is written to both [Graphic](/web/20190327000258/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) objects and to [Bracketed Groups](/web/20190327000258/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md), as of ChemDraw 7.0 it is read only from Graphic objects.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Unspecified flip type |
| 1 | NoFlip | The orientation of the repeating unit does not change between adjacent units |
| 2 | Flip | The orientation of the repeating unit does change between adjacent units |

**If this property is absent:**

The flip type is treated as Unspecified.

---

[CDX Documentation index](/web/20190327000258/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)