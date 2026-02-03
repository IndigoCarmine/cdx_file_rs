CDX Format Specification: Atom_GenericList Property
## Atom_GenericList Property

| CDXML Name: | GenericList |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_GenericList |
| CDX Constant Value: | 0x0441 |
| Data Size: | [CDXGenericList](/web/20190327223024/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXGenericList.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327223024/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 10.0 |
| Required? | No |

**Description:**  

A list of generic nicknames.

This property is irrelevent except for nodes with a [kCDXProp_Node_Type](Node_Type.md) of ElementList or ElementListNickname. This property is used in conjunction with the [kCDXProp_Atom_ElementList](Atom_ElementList.md) property. Either or both lists may be populated for a given node of ElementList or ElementListNickname type.

**If this property is absent:**  

The element list values should be derived from the contained [Text](/web/20190327223024/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) object, if present. If no such object is present, the list is empty, which is almost certainly undesired.

---

[CDX Documentation index](/web/20190327223024/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)