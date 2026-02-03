CDX Format Specification: Line_Type Property
## Line_Type Property

| CDXML Name: | LineType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Line_Type |
| CDX Constant Value: | 0x0A01 |
| Data Size: | [INT16](/web/20190326232730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190326232730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Border](/web/20190326232730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Border.md),[kCDXObj_Arrow](/web/20190326232730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The type of a line object.

In CDX files produced by ChemDraw 8.0, this property was mistakenly written as a 1-byte INT8 value. When reading CDX files, if the size of this property is found to be 1 byte instead of the expected 2 bytes, the actual value should be interpreted as actualType = (INT16)savedType. CDX format interpreters that follow the [best practices for reading integer values from CDX files](/web/20190326232730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) will handle this error automatically. ChemDraw returned to writing INT16 values starting with version 8.0.6.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Solid | Solid line |
| 1 | Dashed | Dashed line |
| 2 | Bold | Bold line |
| 4 | Wavy | Wavy line |

**If this property is absent:**  

The line is treated as solid.

---

[CDX Documentation index](/web/20190326232730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)