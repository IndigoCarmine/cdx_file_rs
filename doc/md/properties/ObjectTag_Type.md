CDX Format Specification: ObjectTag_Type Property
## ObjectTag_Type Property

| CDXML Name: | TagType |
| --- | --- |
| CDX Constant Name: | kCDXProp_ObjectTag_Type |
| CDX Constant Value: | 0x0D00 |
| Data Size: | [INT16](/web/20190327000022/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_ObjectTag](/web/20190327000022/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**  

The tag's data type.

The value of this property determines how the [kCDXProp_ObjectTag_Value](ObjectTag_Value.md) property of the Object Tag should be interpreted.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unknown | The formatting of the kCDXProp_ObjectTag_Value is unknown. Use of this enum value is strongly discouraged |
| 1 | Double | The kCDXProp_ObjectTag_Value is stored as[FLOAT64](/web/20190327000022/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| 2 | Long | The kCDXProp_ObjectTag_Value is stored as[INT32](/web/20190327000022/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| 3 | String | The kCDXProp_ObjectTag_Value is stored as unformatted string (a simple sequence of bytes) |

**If this property is absent:**  

The tag type is treated as Undefined.

---

[CDX Documentation index](/web/20190327000022/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)