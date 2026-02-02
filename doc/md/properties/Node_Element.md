CDX Format Specification: Node_Element Property
## Node_Element Property

| CDXML Name: | Element |
| --- | --- |
| CDX Constant Name: | kCDXProp_Node_Element |
| CDX Constant Value: | 0x0402 |
| Data Size: | [INT16](/web/20190327222628/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327222628/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**  

The atomic number of the atom representing this node.

This property is irrelevent except for nodes with a [kCDXProp_Node_Type](Node_Type.md) of Element.

**If this property is absent:**  

The node is assumed to be a carbon atom.

---

[CDX Documentation index](/web/20190327222628/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)