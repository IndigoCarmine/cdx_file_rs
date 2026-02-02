CDX Format Specification: TLC_SolventFrontFraction Property
## TLC_SolventFrontFraction Property

| CDXML Name: | SolventFrontFraction |
| --- | --- |
| CDX Constant Name: | kCDXProp_TLC_SolventFrontFraction |
| CDX Constant Value: | 0x0AA1 |
| Data Size: | [FLOAT64](/web/20160912060729/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_TLCPlate](/web/20160912060729/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**  

The distance of the solvent front from the top of a TLC Plate, as a fraction of the total height of the plate.

The Solvent Front Fraction should be a decimal value between zero and one. Values less than zero or greater than one have no meaning.

**If this property is absent:**  

The solvent front fraction will be assigned to a default value, possibly specified by the user.

---

[CDX Documentation index](/web/20160912060729/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)