## Bond_BeginAttach Property

| CDXML Name: | BeginAttach |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_BeginAttach |
| CDX Constant Value: | 0x0608 |
| Data Size: | [UINT8](/web/20160911221524/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160911221524/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

Indicates where within the Bond_Begin node a bond is attached.

The value stored is the zero-based character offset from the beginning of the text representing the label. This value assumes that all characters in the label are single-byte. For example, if a bond attaches to the fifth character of a Japanese label, this value would be 8, not 4.

See also [kCDXProp_Bond_EndAttach](Bond_EndAttach.md).

**If this property is absent:**

The bond is assumed to be attached to the first character in the label.

---

[CDX Documentation index](/web/20160911221524/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)