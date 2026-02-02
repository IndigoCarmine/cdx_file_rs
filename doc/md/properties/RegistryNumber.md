CDX Format Specification: RegistryNumber Property
## RegistryNumber Property

| CDXML Name: | RegistryNumber |
| --- | --- |
| CDX Constant Name: | kCDXProp_RegistryNumber |
| CDX Constant Value: | 0x000B |
| Data Size: | [CDXString](/web/20190912230650/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_RegistryNumber](/web/20190912230650/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/RegistryNumber.md) |
| First written/read in: | (not written/read) |
| Required? | No |

**Description:**  

A registry or catalog number of a molecule object.

This attribute and other similar attributes for pre-defined registry numbers are intended for chemical structures in a database. Such attributes might be lost when the reader program has structure editing capability.

**If this property is absent:**  

This property is not read (or written) by ChemDraw, but is defined for future compatibility. There is no consequence to omitting it.

---

[CDX Documentation index](/web/20190912230650/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)