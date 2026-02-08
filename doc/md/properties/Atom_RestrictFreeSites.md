## Atom_RestrictFreeSites Property

| CDXML Name: | FreeSites |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_RestrictFreeSites |
| CDX Constant Value: | 0x0423 |
| Data Size: | [UINT8](/web/20190327000155/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327000155/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

Indicates that up to the specified number of additional substituents are permitted on this atom.

A substituent is defined as some other node bonded to this one. It is strictly a count of attached bonds, and *not* a count of bond orders.

A value of zero indicates that the atom must be matched as drawn.

**If this property is absent:**

The node is treated as unrestricted in terms of free sites.

---

[CDX Documentation index](/web/20190327000155/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)