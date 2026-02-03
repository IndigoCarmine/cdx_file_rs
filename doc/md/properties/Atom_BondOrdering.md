CDX Format Specification: Atom_BondOrdering Property
## Atom_BondOrdering Property

| CDXML Name: | BondOrdering |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_BondOrdering |
| CDX Constant Value: | 0x0431 |
| Data Size: | [CDXObjectIDArray](/web/20190819212409/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_Node](/web/20190819212409/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**  

An ordering of the bonds to this node, used for stereocenters, fragments, and named alternative groups with more than one attachment.

An array of bond IDs. For stereocenters, it defines the stereochemistry about this atom. The interpretation depends on the [kCDXProp_Atom_Geometry](Atom_Geometry.md). For nodes with a [kCDXProp_Node_Type](Node_Type.md) of NamedAlternativeGroup or Fragment, it defines the correspondence between the bonds to this node and the external connection points defined for the fragments.

**If this property is absent:**  

The order of the bonds to this node is not considered to be significant.

---

[CDX Documentation index](/web/20190819212409/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)