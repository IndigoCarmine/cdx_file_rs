CDX Format Specification: Atom_LinkCountLow Property
## Atom_LinkCountLow Property

| CDXML Name: | LinkCountLow |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_LinkCountLow |
| CDX Constant Value: | 0x043D |
| Data Size: | [INT16](/web/20190327222623/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327222623/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

Low end of repeat count for link nodes.

This property is meaningful only for nodes with a [kCDXProp_Node_Type](Node_Type.md) of type LinkNode.

**If this property is absent:**  

The LinkCountLow is assumed to be 1.

---

[CDX Documentation index](/web/20190327222623/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)