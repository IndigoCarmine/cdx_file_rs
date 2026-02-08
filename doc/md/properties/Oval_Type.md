## Oval_Type Property

| CDXML Name: | OvalType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Oval_Type |
| CDX Constant Value: | 0x0A04 |
| Data Size: | [INT16](/web/20190327001000/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190327001000/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The type of an arrow object that represents a circle or ellipse.

In CDX files produced by ChemDraw 8.0, this property was mistakenly written as a 1-byte INT8 value. When reading CDX files, if the size of this property is found to be 1 byte instead of the expected 2 bytes, the actual value should be interpreted as actualType = (INT16)savedType. CDX format interpreters that follow the [best practices for reading integer values from CDX files](/web/20190327001000/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) will handle this error automatically. ChemDraw returned to writing INT16 values starting with version 8.0.6.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 1 | Circle | Circle |
| 2 | Shaded | Shaded |
| 4 | Filled | Filled |
| 8 | Dashed | Dashed |
| 16 | Bold | Bold |
| 32 | Shadowed | Shadowed |

**If this property is absent:**

The oval is treated as plain

---

[CDX Documentation index](/web/20190327001000/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)