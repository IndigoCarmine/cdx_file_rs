CDX Format Specification: Symbol_Type Property
## Symbol_Type Property

| CDXML Name: | SymbolType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Symbol_Type |
| CDX Constant Value: | 0x0A07 |
| Data Size: | [INT16](/web/20190327000028/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190327000028/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The type of symbol object.

In CDX files produced by ChemDraw 8.0, this property was mistakenly written as a 1-byte INT8 value. When reading CDX files, if the size of this property is found to be 1 byte instead of the expected 2 bytes, the actual value should be interpreted as actualType = (INT16)savedType. CDX format interpreters that follow the [best practices for reading integer values from CDX files](/web/20190327000028/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) will handle this error automatically. ChemDraw returned to writing INT16 values starting with version 8.0.6.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | LonePair | LonePair |
| 1 | Electron | Electron |
| 2 | RadicalCation | RadicalCation |
| 3 | RadicalAnion | RadicalAnion |
| 4 | CirclePlus | CirclePlus |
| 5 | CircleMinus | CircleMinus |
| 6 | Dagger | Dagger |
| 7 | DoubleDagger | DoubleDagger |
| 8 | Plus | Plus |
| 9 | Minus | Minus |
| 10 | Racemic | Racemic |
| 11 | Absolute | Absolute |
| 12 | Relative | Relative |
| 13 | LonePair | LonePair, drawn as a solid bar rather than as two electron dots |

**If this property is absent:**  

The object is treated as a lone pair.

---

[CDX Documentation index](/web/20190327000028/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)