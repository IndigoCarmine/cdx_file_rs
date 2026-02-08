## Atom_CIPStereochemistry Property

| CDXML Name: | AS |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_CIPStereochemistry |
| CDX Constant Value: | 0x0437 |
| Data Size: | [INT8](/web/20190327221841/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327221841/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**

The node's absolute stereochemistry according to the Cahn-Ingold-Prelog system.

ChemDraw will preserve any value present in a file without confirming its accuracy—as long as the structure is not edited in ChemDraw. Any change to a structure has the potential of changing its stereochemical descriptors, and so ChemDraw will recalculate the structure's chemistry after any change. As a practical matter, we are very confident about the accuracy of ChemDraw's calculation of stereochemical descriptors, so this behavior should rarely cause problems.

The Cahn-Ingold-Prelog system for describing absolute stereochemistry is defined in the following references:

- R.S. Cahn, C.K. Ingold, and V. Prelog, Specification of Molecular Chirality, *Angew. Chem., Int. Ed. Engl.***1966**, *5*, 385-414 (errata: 1966, 5, 511); *Angew. Chem.***1966**, 78, 413-447.
- V. Prelog and G. Helmchen, Basic principals of the CIP-System and Proposals for a Revision, *Angew Chem*. **1982**, *94*, 614-631; *Angew.
Chem., Int. Ed. Engl.***1982**, 21, 567-583.
- P. Mata and A.M. Lobo, The CIP Sequence Rules: Analysis and Proposal for a Revision, *Tetrahedron: Asymmetry.***1993**, *4*,
657-668.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | U | Undetermined |
| 1 | N | Determined to be symmetric |
| 2 | R | Asymmetric: (R) |
| 3 | S | Asymmetric: (S) |
| 4 | r | Pseudoasymmetric: (r) |
| 5 | s | Pseudoasymmetric: (s) |
| 6 | u | Unspecified: The node is not symmetric (might be asymmetric or pseudoasymmetric), but lacks a hash/wedge so absolute configuration cannot be determined |

**If this property is absent:**

The node is treated as having Undetermined stereochemistry.

---

[CDX Documentation index](/web/20190327221841/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)