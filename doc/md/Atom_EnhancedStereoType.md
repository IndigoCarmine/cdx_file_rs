CDX Format Specification: Atom_EnhancedStereoType Property
## Atom_EnhancedStereoType Property

| CDXML Name: | EnhancedStereoType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_EnhancedStereoType |
| CDX Constant Value: | 0x0446 |
| Data Size: | UINT8 |
| Property of objects: | kCDXObj_Node |
| First written/read in: | ChemDraw 10.0 |
| Required? | No |

**Description:**  

The type of enhanced stereochemistry present on this atom.

And and Or types additionally require a [kCDXProp_Atom_EnhancedStereoGroupNum](Atom_EnhancedStereoGroupNum.md).

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Unspecified |
| 1 | None | None |
| 2 | Absolute | Absolute |
| 3 | Or | Or (requires[kCDXProp_Atom_EnhancedStereoGroupNum](Atom_EnhancedStereoGroupNum.md)) |
| 4 | And | And (requires[kCDXProp_Atom_EnhancedStereoGroupNum](Atom_EnhancedStereoGroupNum.md)) |

**If this property is absent:**  

The node is treated as unspecified in terms of enhanced stereochemistry.

---

CDX Documentation index