CDX Format Specification: Atom_ExternalConnectionType Property
## Atom_ExternalConnectionType Property

| CDXML Name: | ExternalConnectionType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_ExternalConnectionType |
| CDX Constant Value: | 0x0440 |
| Data Size: | [INT8](/web/20190326232206/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326232206/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**  

Type of external connection, for atoms of type kCDXNodeType_ExternalConnectionPoint.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Nothing is specified about the node's isotopic abundance |
| 1 | Diamond | The attachment point is indicated by a black diamond at the end of a bond |
| 2 | Star | The attachment point is indicated by an asterisk at the end of a bond |
| 3 | PolymerBead | The attachment point is indicated by large shaded sphere at the end of a bond |
| 4 | Wavy | The attachment point is indicated by a wavy line perpendicular to the end of a bond |

**If this property is absent:**  

The node is treated as having Unspecified external connection type, which is then treated as a black diamond by ChemDraw.

---

[CDX Documentation index](/web/20190326232206/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)