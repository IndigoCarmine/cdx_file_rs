## Bond_Display Property

| CDXML Name: | Display |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_Display |
| CDX Constant Value: | 0x0601 |
| Data Size: | [INT16](/web/20160913174300/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160913174300/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The display type of a bond object.

ChemDraw does not support all display values (unsupported values are provided for future compatibility). For single bonds, all but Dot and DashDot are supported. For double bonds, Solid, Dash, and Bold are supported, and Wavy is treated as a double-either (crossed) bond. For triple and quadruple bonds, only Solid is supported.

See also [kCDXProp_Bond_Display2](Bond_Display2.md).

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Solid | Solid bond |
| 1 | Dash | Dashed bond |
| 2 | Hash | Hashed bond |
| 3 | WedgedHashBegin | Wedged hashed bond with the narrow end on the "begin" atom |
| 4 | WedgedHashEnd | Wedged hashed bond with the narrow end on the "end" atom |
| 5 | Bold | Bold bond |
| 6 | WedgeBegin | Wedged solid bond with the narrow end on the "begin" atom |
| 7 | WedgeEnd | Wedged solid bond with the narrow end on the "end" atom |
| 8 | Wavy | Wavy bond |
| 9 | HollowWedgeBegin | Wedged hollow bond with the narrow end on the "begin" atom |
| 10 | HollowWedgeEnd | Wedged hollow bond with the narrow end on the "end" atom |
| 11 | WavyWedgeBegin | Wedged wavy bond with the narrow end on the "begin" atom |
| 12 | WavyWedgeEnd | Wedged wavy bond with the narrow end on the "end" atom |
| 13 | Dot | Dotted bond |
| 14 | DashDot | Dashed-and-dotted bond |

**If this property is absent:**

The bond is treated as Solid. Additionally, any unsupported type will also be treated as Solid.

---

[CDX Documentation index](/web/20160913174300/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)