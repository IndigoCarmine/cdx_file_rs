CDX Format Specification: Atom_Translation Property
## Atom_Translation Property

| CDXML Name: | Translation |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_Translation |
| CDX Constant Value: | 0x0438 |
| Data Size: | [INT8](/web/20190326235740/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326235740/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**  

Provides for restrictions on whether a given node may match other more- or less-general nodes.

This property corresponds directly to the Translation attribute of Markush DARC.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Equal | Generic nickname can match only an identical generic nickname; specific atoms can match only the identical specific atoms |
| 1 | Broad | Node may match any identical or more-general node, including more-general generic nicknames |
| 2 | Narrow | Generic nickname must match specific atoms or groups of atoms (cannot match another generic nickname) |
| 3 | Any | Generic nicknames may match other more-general generic nicknames, or may match specific atoms or groups of atoms |

**If this property is absent:**  

The node is treated as having Equal translation.

---

[CDX Documentation index](/web/20190326235740/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)