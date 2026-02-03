CDX Format Specification: Frag_ConnectionOrder Property
## Frag_ConnectionOrder Property

| CDXML Name: | ConnectionOrder |
| --- | --- |
| CDX Constant Name: | kCDXProp_Frag_ConnectionOrder |
| CDX Constant Value: | 0x0505 |
| Data Size: | [CDXObjectIDArray](/web/20190912234102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_Fragment](/web/20190912234102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Fragment.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**  

An ordered list of attachment points within a fragment.

Used, for example, to ensure that a fragment is connected in the right head-to-tail order to whatever object contains it. The list of attachment points (that is, [Node](/web/20190912234102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md)s with a [kCDXProp_Node_Type](Node_Type.md) of ExternalConnectionPoint) contained by this property is in the same order as the corresponding list of bonds stored in the containing [Node](/web/20190912234102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md)'s [kCDXProp_Atom_BondOrdering](Atom_BondOrdering.md) property.

This property is meaningless for fragments that contain fewer that two attachment points, and should not be present in those cases.

**If this property is absent:**  

The ordering of attachment points in this fragment is not considered to be significant (or there are fewer than two attachment points).

---

[CDX Documentation index](/web/20190912234102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)