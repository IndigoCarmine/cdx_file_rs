## Atom_HDash Property

| CDXML Name: | HDash |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_HDash |
| CDX Constant Value: | 0x042F |
| Data Size: | [CDXBooleanImplied](/web/20190327000438/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327000438/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

Signifies the presence of an implicit hydrogen with stereochemistry specified equivalent to an explicit H atom with a hashed bond.

This property has no meaning except on nodes consisting of an unlabeled carbon atom. Atom_HDot and Atom_HDash must not both be specified.

**If this property is absent:**

This node is not considered as an HDash.

---

[CDX Documentation index](/web/20190327000438/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)