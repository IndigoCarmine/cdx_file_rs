## Atom_Isotope Property

| CDXML Name: | Isotope |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_Isotope |
| CDX Constant Value: | 0x0420 |
| Data Size: | [INT16](/web/20190326232825/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326232825/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**

The absolute isotopic mass of an atom (2 for deuterium, 14 for carbon-14).

A value of zero indicates natural abundance.

This property is meaningful only for nodes with a [kCDXProp_Node_Type](Node_Type.md) of type Element.

**If this property is absent:**

The atom is assumed to be of natural abundance.

---

[CDX Documentation index](/web/20190326232825/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)