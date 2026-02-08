## Bond_DoublePosition Property

| CDXML Name: | DoublePosition |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_DoublePosition |
| CDX Constant Value: | 0x0603 |
| Data Size: | [INT16](/web/20160912170320/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160912170320/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The position of the second line of a double bond.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Center | Double bond is centered, but was positioned automatically by the program |
| 1 | Right | Double bond is on the right (viewing from the "begin" atom to the "end" atom), but was positioned automatically by the program |
| 2 | Left | Double bond is on the left (viewing from the "begin" atom to the "end" atom), but was positioned automatically by the program |
| 256 | Center | Double bond is centered, and was positioned manually by the user |
| 257 | Right | Double bond is on the right (viewing from the "begin" atom to the "end" atom), and was positioned manually by the user |
| 258 | Left | Double bond is on the left (viewing from the "begin" atom to the "end" atom), and was positioned manually by the user |

**If this property is absent:**

The position is treated as unspecified and will be positioned in a way that ChemDraw thinks is best.

---

[CDX Documentation index](/web/20160912170320/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)