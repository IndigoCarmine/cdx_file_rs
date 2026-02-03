CDX Format Specification: Atom_AtomNumber Property
## Atom_AtomNumber Property

| CDXML Name: | AtomNumber |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_AtomNumber |
| CDX Constant Value: | 0x0439 |
| Data Size: | [CDXString](/web/20190327222618/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327222618/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

Atom number, as text.

ChemDraw is extremely flexible in what it allows for atom numbers: any string is valid, and this property is not limited to numeric values. For example, not only is "2" a valid atom number, but so are 2', 2a, b, and b'.

**If this property is absent:**  

The node does not have an atom number.

---

[CDX Documentation index](/web/20190327222618/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)