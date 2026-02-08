## Atom_IsotopicAbundance Property

| CDXML Name: | IsotopicAbundance |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_IsotopicAbundance |
| CDX Constant Value: | 0x043F |
| Data Size: | [INT8](/web/20190327225236/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327225236/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**

Isotopic abundance of this atom's isotope.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Nothing is specified about the node's isotopic abundance |
| 1 | Any | Node may match any node of the same element type, regardless of isotopy |
| 2 | Natural | Node's element exists explicitly in natural abundance. Not valid when the node also has a specific isotopy |
| 3 | Enriched | Node is enriched in the specified isotope. Not valid unless the node also has a specific isotopy |
| 4 | Deficient | Node is deficient in the specified isotope. Not valid unless the node also has a specific isotopy |
| 5 | Nonnatural | Node has a non-natural abundance of the specified isotope (may be either enriched or deficient). Not valid when the node also has a specific isotopy |

**If this property is absent:**

The node is treated as having Unspecified isotopic abundance.

---

[CDX Documentation index](/web/20190327225236/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)