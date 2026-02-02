CDX Format Specification: Bond_Display2 Property
## Bond_Display2 Property

| CDXML Name: | Display2 |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_Display2 |
| CDX Constant Value: | 0x0602 |
| Data Size: | [INT16](/web/20160912170439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160912170439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The display type for the second line of a double bond.

ChemDraw supports only Dashed types for the second line of a double bond (unsupport values are provided for future compatibility).

See also [kCDXProp_Bond_Display](Bond_Display.md).

This is an enumerated property. Acceptible values are shown in the following list:

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

The second line of a double bond is treated as the same as the first line. Additionally, any unsupported type will also be treated as Solid.

---

[CDX Documentation index](/web/20160912170439/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)