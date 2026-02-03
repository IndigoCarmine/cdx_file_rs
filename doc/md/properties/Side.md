CDX Format Specification: Side Property
## Side Property

| CDXML Name: | Side |
| --- | --- |
| CDX Constant Name: | kCDXProp_Side |
| CDX Constant Value: | 0x0825 |
| Data Size: | [UINT16](/web/20160911224334/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Border](/web/20160911224334/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Border.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | Required |

**Description:**  

A specific side of an object (rectangle).

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | undefined | Undefined side |
| 1 | top | Top side |
| 2 | left | Left side |
| 3 | bottom | Bottom side |
| 4 | right | Right side |

**If this property is absent:**  

The side is treated as Undefined. Since that is not particularly useful, it is recommended that this property not be omitted.

---

[CDX Documentation index](/web/20160911224334/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)