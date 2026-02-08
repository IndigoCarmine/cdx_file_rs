## Orbital_Type Property

| CDXML Name: | OrbitalType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Orbital_Type |
| CDX Constant Value: | 0x0A05 |
| Data Size: | [INT16](/web/20190326235730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190326235730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The type of orbital object.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | s | s orbital |
| 1 | oval | Oval-shaped orbital |
| 2 | lobe | One lobe of a p orbital |
| 3 | p | Complete p orbital |
| 4 | hybridPlus | hybrid orbital |
| 5 | hybridMinus | hybrid orbital (opposite shading) |
| 6 | dz2Plus | dz2 orbital |
| 7 | dz2Minus | dz2 orbital (opposite shading) |
| 8 | dxy | dxy orbital |
| 256 | sShaded | shaded s orbital |
| 257 | ovalShaded | shaded oval-shaped orbital |
| 258 | lobeShaded | shaded single lobe of a p orbital |
| 259 | pShaded | shaded complete p orbital |
| 512 | sFilled | filled s orbital |
| 513 | ovalFilled | filled oval-shaped orbital |
| 514 | lobeFilled | filled single lobe of a p orbital |
| 515 | pFilled | filled complete p orbital |
| 516 | hybridPlusFilled | filled hybrid orbital |
| 517 | hybridMinusFilled | filled hybrid orbital (opposite shading) |
| 518 | dz2PlusFilled | filled dz2 orbital |
| 519 | dz2MinusFilled | filled dz2 orbital (opposite shading) |
| 520 | dxyFilled | filled dxy orbital |

**If this property is absent:**

The object is treated as a plain s orbital.

---

[CDX Documentation index](/web/20190326235730/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)