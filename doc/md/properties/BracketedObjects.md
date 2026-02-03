CDX Format Specification: BracketedObjects Property
## BracketedObjects Property

| CDXML Name: | BracketedObjectIDs |
| --- | --- |
| CDX Constant Name: | kCDXProp_BracketedObjects |
| CDX Constant Value: | 0x0A27 |
| Data Size: | [CDXObjectIDArray](/web/20160912021256/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_BracketedGroup](/web/20160912021256/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md) |
| First written/read in: | ChemDraw 7.0 / (not read) |
| Required? | No |

**Description:**  

The set of objects contained in a BracketedGroup.

**If this property is absent:**  

The bracketed group is considered not to enclose any objects. Since that is not a particularly useful situation, it is strongly encouraged that this property be present.

---

[CDX Documentation index](/web/20160912021256/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)