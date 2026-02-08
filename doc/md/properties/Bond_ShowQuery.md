## Bond_ShowQuery Property

| CDXML Name: | ShowBondQuery |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_ShowQuery |
| CDX Constant Value: | 0x060C |
| Data Size: | [CDXBoolean](/web/20160912010721/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Document](/web/20160912010721/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Bond](/web/20160912010721/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

Show the query indicator if non-zero.

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20160912010721/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet. If no information is found in the Style Sheet either, this value is treated as true (query indicators are shown).

---

[CDX Documentation index](/web/20160912010721/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)