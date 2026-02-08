## Arrow_Type Property

| CDXML Name: | ArrowType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Arrow_Type |
| CDX Constant Value: | 0x0A02 |
| Data Size: | [INT16](/web/20190326232451/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190326232451/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The type of arrow object, which represents line, arrow, arc, rectangle, or orbital.

In CDX files produced by ChemDraw 8.0, this property was mistakenly written as a 1-byte INT8 value. When reading CDX files, if the size of this property is found to be 1 byte instead of the expected 2 bytes, the actual value should be interpreted as actualType = (INT16)savedType. CDX format interpreters that follow the [best practices for reading integer values from CDX files](/web/20190326232451/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) will handle this error automatically. ChemDraw returned to writing INT16 values starting with version 8.0.6.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | NoHead | NoHead |
| 1 | HalfHead | HalfHead |
| 2 | FullHead | FullHead |
| 4 | Resonance | Resonance |
| 8 | Equilibrium | Equilibrium |
| 16 | Hollow | Hollow |
| 32 | RetroSynthetic | RetroSynthetic |
| 64 | NoGo | Failed reaction (crossed-out arrow); may be combined with other types |
| 128 | Dipole | Dipole |

**If this property is absent:**

The arrow is treated as headless.

---

[CDX Documentation index](/web/20190326232451/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)