CDX Format Specification: Atom_Charge Property
## Atom_Charge Property

| CDXML Name: | Charge |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_Charge |
| CDX Constant Value: | 0x0421 |
| Data Size: | [INT8](/web/20190326232621/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326232621/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 / 6.0 |
| Required? | No |

**Description:**  

The atomic charge of an atom.

Fractional values for charge are not supported.

In CDX files produced by ChemDraw 8.0, this property was mistakenly written as a 4-byte Fixed value. When reading CDX files, if the size of this property is found to be 4 bytes instead of the expected 1 byte, the actual value should be interpreted as actualCharge = (INT32)savedCharge >> 24. ChemDraw returned to writing INT8 values starting with version 8.0.6.

**If this property is absent:**  

The atom is assumed to be uncharged.

---

[CDX Documentation index](/web/20190326232621/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)