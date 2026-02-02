CDX Format Specification: Rectangle_Type Property
## Rectangle_Type Property

| CDXML Name: | RectangleType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Rectangle_Type |
| CDX Constant Value: | 0x0A03 |
| Data Size: | [INT16](/web/20190327001005/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190327001005/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The type of a rectangle object.

In CDX files produced by ChemDraw 8.0, this property was mistakenly written as a 1-byte INT8 value. When reading CDX files, if the size of this property is found to be 1 byte instead of the expected 2 bytes, the actual value should be interpreted as actualType = (INT16)savedType. CDX format interpreters that follow the [best practices for reading integer values from CDX files](/web/20190327001005/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) will handle this error automatically. ChemDraw returned to writing INT16 values starting with version 8.0.6.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Plain | Plain |
| 1 | RoundEdge | RoundEdge |
| 2 | Shadow | Shadow |
| 4 | Shaded | Shaded |
| 8 | Filled | Filled |
| 16 | Dashed | Dashed |
| 32 | Bold | Bold |

**If this property is absent:**  

The rectangle is treated as plain.

---

[CDX Documentation index](/web/20190327001005/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)