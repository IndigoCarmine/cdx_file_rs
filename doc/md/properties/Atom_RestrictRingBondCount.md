## Atom_RestrictRingBondCount Property

| CDXML Name: | RingBondCount |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_RestrictRingBondCount |
| CDX Constant Value: | 0x0425 |
| Data Size: | [INT8](/web/20190326220442/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326220442/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The number of ring bonds attached to an atom.

If posed as a structure query in some database, this atom should not match only those atoms with the specified number of ring bonds.

The value stored in this property exactly corresponds to the Ring Bond Count property in ISIS.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| -1 | Unspecified | Unspecified number of ring bonds |
| 0 | NoRingBonds | Exactly 0 ring bonds |
| 1 | AsDrawn | Ring bonds as drawn |
| 2 | SimpleRing | Exactly 2 ring bonds |
| 3 | Fusion | Exactly 3 ring bonds |
| 4 | SpiroOrHigher | 4 or more ring bonds |

**If this property is absent:**

The number of ring bonds is unspecified.

---

[CDX Documentation index](/web/20190326220442/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)