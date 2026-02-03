CDX Format Specification: Bond_Order Property
## Bond_Order Property

| CDXML Name: | Order |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_Order |
| CDX Constant Value: | 0x0600 |
| Data Size: | [INT16](/web/20160913174134/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160913174134/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The order of a bond object.

Dative and Ionic bonds are specified with the [Bond_Begin](Bond_Begin.md) as the positive end (e.g. the N in pyridine N-oxide). Hydrogen bonds are specified with Bond_Begin as the hydrogen. Three center bonds are specified with two Bond objects, each with Bond_Begin as the center atom.

Bond order values may be combined. For example, a value of 3 would represent a "single or double" bond, which is useful for substructure queries.

Ionic, hydrogen, and three-center bonds are not currently supported by ChemDraw, but are defined for future compatibility.

This is a bit-encoded property. The values shown in the following list may be combined:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0xFFFF |  | Unspecified order (omitted entirely in CDXML) |
| 0x0001 | 1 | Single bond |
| 0x0002 | 2 | Double bond |
| 0x0004 | 3 | Triple bond |
| 0x0008 | 4 | Quadruple bond (used for some inorganic complexes) |
| 0x0010 | 5 | Quintuple bond (used for some inorganic complexes) |
| 0x0020 | 6 | Hextuple bond (used for some inorganic complexes) |
| 0x0040 | 0.5 | Bond of order one-half |
| 0x0080 | 1.5 | Bond of order one and one-half (an aromatic bond) |
| 0x0100 | 2.5 | Bond of order two and one-half (in benzyne, for example) |
| 0x0200 | 3.5 | Bond of order three and one-half (used for some inorganic complexes) |
| 0x0400 | 4.5 | Bond of order four and one-half (used for some inorganic complexes) |
| 0x0800 | 5.5 | Bond of order five and one-half (used for some inorganic complexes) |
| 0x1000 | dative | Dative bond, from the "begin" atom to the "end" atom |
| 0x2000 | ionic | Ionic bond |
| 0x4000 | hydrogen | Hydrogen bond |
| 0x8000 | threecenter | Three-center-bond (in boranes, for example) |

**If this property is absent:**  

The bond is treated as a single bond.

---

[CDX Documentation index](/web/20160913174134/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)