CDX Format Specification: Atom_RestrictSubstituentsExactly Property
## Atom_RestrictSubstituentsExactly Property

| CDXML Name: | SubstituentsExactly |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_RestrictSubstituentsExactly |
| CDX Constant Value: | 0x0436 |
| Data Size: | [UINT8](/web/20190326232830/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326232830/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

Indicates that exactly the specified number of substituents must be present.

A substituent is defined as some other non-hydrogen node bonded to this one. It is strictly a count of attached bonds (to non-hydrogen atoms), and *not* a sum of bond orders. This definition of "substituent" exactly matches the definition used by ISIS.

Note that it is possible to assign impossible values, for example by saying that one of the atoms in benzene has exactly one substituent (when it already has two, by nature of being part of the benzene ring). Impossibilities of that sort are not forbidden by this specification, but it is assumed that such a structure would match nothing else if posed as a structure query in some database.

The value stored in this property exactly matches the value stored in ISIS's Substitution Count property, with the exception that ISIS accepts a maximum value of 6.

**If this property is absent:**  

The node is treated as unrestricted in terms of substitution.

---

[CDX Documentation index](/web/20190326232830/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)