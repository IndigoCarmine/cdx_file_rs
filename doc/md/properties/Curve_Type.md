## Curve_Type Property

| CDXML Name: | CurveType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Curve_Type |
| CDX Constant Value: | 0x0A08 |
| Data Size: | [INT16](/web/20190327000940/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Curve](/web/20190327000940/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Curve.md),[kCDXObj_TLCSpot](/web/20190327000940/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCSpot.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The type of curve object.

Note that not all combinations of these types are supported by ChemDraw. If a curve has arrows of any type, it cannot also be closed, filled, shaded, or doubled (ChemDraw gives preference to the arrow type if conflicting). If it has an arrow at start or end, it cannot also have a half-arrow at the same location. A dashed spline cannot be filled, shaded, or doubled.

This is a bit-encoded property. The values shown in the following list may be combined:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0x0001 | 1 | Closed |
| 0x0002 | 2 | Dashed |
| 0x0004 | 4 | Bold |
| 0x0008 | 8 | Arrow at End |
| 0x0010 | 16 | Arrow at Start |
| 0x0020 | 32 | Half-arrow at End |
| 0x0040 | 64 | Half-arrow at Start |
| 0x0080 | 128 | Filled |
| 0x0100 | 256 | Shaded |
| 0x0200 | 512 | Doubled |
| 0x0000 | 0 | Plain |

**If this property is absent:**

The curve is treated as Plain.

---

[CDX Documentation index](/web/20190327000940/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)