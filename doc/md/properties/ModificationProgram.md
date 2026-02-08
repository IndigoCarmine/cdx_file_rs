## ModificationProgram Property

| CDXML Name: | ModificationProgram |
| --- | --- |
| CDX Constant Name: | kCDXProp_ModificationProgram |
| CDX Constant Value: | 0x0006 |
| Data Size: | [CDXString](/web/20160911223343/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Document](/web/20160911223343/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | (not written/read) |
| Required? | No |

**Description:**

The name of the program, including version and platform, of the last program to perform a modification. ChemDraw 4.0 uses "ChemDraw 4.0" as the value of CreationProgram.

This attribute is for information only; interpretation of the file based on the value of this attribute should be avoided.

**If this property is absent:**

This property is not read (or written) by ChemDraw, but is defined for future compatibility. There is no consequence to omitting it.

---

[CDX Documentation index](/web/20160911223343/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)