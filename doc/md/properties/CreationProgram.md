CDX Format Specification: CreationProgram Property
## CreationProgram Property

| CDXML Name: | CreationProgram |
| --- | --- |
| CDX Constant Name: | kCDXProp_CreationProgram |
| CDX Constant Value: | 0x0003 |
| Data Size: | [CDXString](/web/20190911013528/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Document](/web/20190911013528/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 4.0 / (not read) |
| Required? | No |

**Description:**  

The name of the program, including version and platform, that created the associated CDX object. ChemDraw 4.0 uses "ChemDraw 4.0" as the value of CreationProgram.

This attribute is for information only; interpretation of the file based on the value of this attribute should be avoided.

**If this property is absent:**  

This property is not read by ChemDraw. It is written as a courtesy only. There is no consequence to omitting it.

---

[CDX Documentation index](/web/20190911013528/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)