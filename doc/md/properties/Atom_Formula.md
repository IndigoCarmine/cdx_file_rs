## Atom_Formula Property

| CDXML Name: | Formula |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_Formula |
| CDX Constant Value: | 0x0404 |
| Data Size: | [CDXFormula](/web/20190912231049/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXFormula.md) |
| Property of objects: | [kCDXObj_Node](/web/20190912231049/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | (not written/read) |
| Required? | No |

**Description:**

The composition of a node representing a fragment whose composition is known, but whose connectivity is not. For example, C4H9 represents a mixture of the 4 butyl isomers.

It consists of a series of pairs of UINT16s. Each pair represents an element and a count, for example, 6, 4, 1, 9 for C4H9.

**If this property is absent:**

This property is not read (or written) by ChemDraw, but is defined for future compatibility. There is no consequence to omitting it.

---

[CDX Documentation index](/web/20190912231049/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)