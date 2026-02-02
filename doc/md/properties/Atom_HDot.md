CDX Format Specification: Atom_HDot Property
## Atom_HDot Property

| CDXML Name: | HDot |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_HDot |
| CDX Constant Value: | 0x042E |
| Data Size: | [CDXBooleanImplied](/web/20190327221009/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327221009/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

Signifies the presence of an implicit hydrogen with stereochemistry specified equivalent to an explicit H atom with a wedged bond.

This property has no meaning except on nodes consisting of an unlabeled carbon atom. Atom_HDot and Atom_HDash must not both be specified.

**If this property is absent:**  

This node is not considered as an HDot.

---

[CDX Documentation index](/web/20190327221009/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)