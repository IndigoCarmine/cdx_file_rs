## Atom_GenericNickname Property

| CDXML Name: | GenericNickname |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_GenericNickname |
| CDX Constant Value: | 0x0433 |
| Data Size: | [CDXString](/web/20160912060759/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Node](/web/20160912060759/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**

The name of the generic nickname.

This property is irrelevant except for nodes with a [kCDXProp_Node_Type](Node_Type.md) of GenericNickname.

**If this property is absent:**

The name should be derived from the contained [Text](/web/20160912060759/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) object, if present. If no such object is present, the name is considered to be the null string, which is almost certainly undesired.

---

[CDX Documentation index](/web/20160912060759/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)