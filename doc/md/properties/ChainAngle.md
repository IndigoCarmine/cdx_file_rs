CDX Format Specification: ChainAngle Property
## ChainAngle Property

| CDXML Name: | ChainAngle |
| --- | --- |
| CDX Constant Name: | kCDXProp_ChainAngle |
| CDX Constant Value: | 0x0803 |
| Data Size: | [INT32](/web/20160912170234/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Document](/web/20160912170234/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**  

The default chain angle setting in degrees * 65536.

Used in ChemDraw when drawing bonds and chains of bonds.

**If this property is absent:**  

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20160912170234/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20160912170234/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)