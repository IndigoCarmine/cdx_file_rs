CDX Format Specification: Atom_Geometry Property
## Atom_Geometry Property

| CDXML Name: | Geometry |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_Geometry |
| CDX Constant Value: | 0x0430 |
| Data Size: | [INT8](/web/20190326221653/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326221653/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.5 / 6.0 |
| Required? | No |

**Description:**  

The geometry of the bonds about this atom.

The number of bonds and implicit and explicit hydrogens must equal the number of attachments given in the table for the specified geometry.

Most of the values listed in the table below are provided for future expansion. The only value that ChemDraw reads or writes is Tetrahedral. For Tetrahedral geometry, the bonds attached to this node are listed in the [kCDXProp_Atom_BondOrdering](Atom_BondOrdering.md) property in an order so that, when the node is viewed from the first-listed bond, the other bonds appear in a counter-clockwise orientation. This corresponds exactly to the '@@' chirality type in SMILES.

This property is used by ChemDraw only when reading files where no nodes have the [kCDXProp_2DPosition.htm](2DPosition.md) property. In such a case, ChemDraw will use the values in this property and in the [kCDXProp_Atom_BondOrdering](Atom_BondOrdering.md) property to generate appropriate hashes and wedges on the attached bonds. If the generating program is saving positions, it is assumed that it will also be responsible for generating hashes and wedges.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unknown | Unknown |
| 1 | 1 | 1 ligand |
| 2 | Linear | 2 ligands: linear |
| 3 | Bent | 2 ligands: bent |
| 4 | TrigonalPlanar | 3 ligands: trigonal planar |
| 5 | TrigonalPyramidal | 3 ligands: trigonal pyramidal |
| 6 | SquarePlanar | 4 ligands: square planar |
| 7 | Tetrahedral | 4 ligands: tetrahedral |
| 8 | TrigonalBipyramidal | 5 ligands: trigonal bipyramidal |
| 9 | SquarePyramidal | 5 ligands: square pyramidal |
| 10 | 5 | 5 ligands: unspecified |
| 11 | Octahedral | 6 ligands: octahedral |
| 12 | 6 | 6 ligands: unspecified |
| 13 | 7 | 7 ligands: unspecified |
| 14 | 8 | 8 ligands: unspecified |
| 15 | 9 | 9 ligands: unspecified |
| 16 | 10 | 10 ligands: unspecified |

**If this property is absent:**  

The geometry is treated as Unknown.

---

[CDX Documentation index](/web/20190326221653/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)