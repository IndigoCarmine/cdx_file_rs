CDX Format Specification: Atom_RestrictImplicitHydrogens Property
## Atom_RestrictImplicitHydrogens Property

| CDXML Name: | ImplicitHydrogens |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_RestrictImplicitHydrogens |
| CDX Constant Value: | 0x0424 |
| Data Size: | [CDXBooleanImplied](/web/20190327000345/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327000345/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

Signifies that implicit hydrogens are not allowed on this atom.

This property is used when making queries. If implicit hydrogens are not allowed, the node may not match a target node that has any attached hydrogens. This property is redundant with the [kCDXProp_Atom_RestrictSubstituentsUpTo](Atom_RestrictSubstituentsUpTo.md) and [kCDXProp_Atom_RestrictSubstituentsExactly](Atom_RestrictSubstituentsExactly.md) properties.

The value stored in this property exactly corresponds to the H0 property in ISIS.

**If this property is absent:**  

Implicit hydrogens are allowed.

---

[CDX Documentation index](/web/20190327000345/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)