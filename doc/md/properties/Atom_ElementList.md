## Atom_ElementList Property

| CDXML Name: | ElementList |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_ElementList |
| CDX Constant Value: | 0x0403 |
| Data Size: | [CDXElementList](/web/20190326232820/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXElementList.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326232820/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**

A list of atomic numbers.

This property is irrelevant except for nodes with a [kCDXProp_Node_Type](Node_Type.md) of ElementList or ElementListNickname. This property is used in conjunction with the [kCDXProp_Atom_GenericList](Atom_GenericList.md) property. Either or both lists may be populated for a given node of ElementList or ElementListNickname type.

**If this property is absent:**

The element list values should be derived from the contained [Text](/web/20190326232820/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) object, if present. If no such object is present, the list is empty, which is almost certainly undesired.

---

[CDX Documentation index](/web/20190326232820/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)