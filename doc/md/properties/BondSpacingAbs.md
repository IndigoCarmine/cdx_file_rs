## BondSpacingAbs Property

| CDXML Name: | BondSpacingAbs |
| --- | --- |
| CDX Constant Name: | kCDXProp_BondSpacingAbs |
| CDX Constant Value: | 0x0822 |
| Data Size: | [CDXCoordinate](/web/20160912170249/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160912170249/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

The absolute distance between segments of a multiple bond.

Note that this property is redundant with [kCDXProp_BondSpacing](BondSpacing.md) (in combination with [kCDXProp_BondLength](BondLength.md)). Only one of the two need be present. If both are present and contradictory, ChemDraw gives preference to the BondSpacingAbs value.

**If this property is absent:**

If present, the [kCDXProp_BondSpacing](BondSpacing.md) value will be used. If that property is not present, then the value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20160912170249/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20160912170249/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)