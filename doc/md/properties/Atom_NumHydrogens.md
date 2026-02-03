CDX Format Specification: Atom_NumHydrogens Property
## Atom_NumHydrogens Property

| CDXML Name: | NumHydrogens |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_NumHydrogens |
| CDX Constant Value: | 0x042B |
| Data Size: | [UINT16](/web/20190326231201/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326231201/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**  

The number of (explicit) hydrogens in a labeled atom consisting of one heavy atom and (optionally) the symbol H (e.g., CH3).

As for an example, an atom with the label "Si" with bonds to CH3, CH3, H, and H has no explicit hydrogen (the Hs are represented as distinct nodes), and this attribute is not required. On the other hand, the same element node has two explicit hydrogens if it is labeled "SiH2" with two bonds to CH3 and CH3.

This property is meaningful only for nodes with a [kCDXProp_Node_Type](Node_Type.md) of type Element.

**If this property is absent:**  

The number of hydrogens is assumed to be the minimum value necessary to satisfy the valence requirements for the atom.

---

[CDX Documentation index](/web/20190326231201/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)