CDX Format Specification: Atom_Radical Property
## Atom_Radical Property

| CDXML Name: | Radical |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_Radical |
| CDX Constant Value: | 0x0422 |
| Data Size: | [UINT8](/web/20190327215500/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327215500/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**  

The atomic radical attribute of an atom.

This property is meaningful only for nodes with a [kCDXProp_Node_Type](Node_Type.md) of type Element.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | None | Not a radical |
| 1 | Singlet | Radical singlet (diradical) |
| 2 | Doublet | Radical doublet (monoradical) |
| 3 | Triplet | Radical triplet (diradical) |

**If this property is absent:**  

The node is assumed not be be a radical.

---

[CDX Documentation index](/web/20190327215500/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)