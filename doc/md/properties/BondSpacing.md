CDX Format Specification: BondSpacing Property
## BondSpacing Property

| CDXML Name: | BondSpacing |
| --- | --- |
| CDX Constant Name: | kCDXProp_BondSpacing |
| CDX Constant Value: | 0x0804 |
| Data Size: | [INT16](/web/20160913174558/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Document](/web/20160913174558/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Bond](/web/20160913174558/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**  

The spacing between segments of a multiple bond, measured relative to bond length.

**In CDX files**, this value is stored as (10 * the bond spacing as a percent of length), so 18% of length = 180.
**In CDXML files**, this value is stored simply as (the bond spacing as a percent of length), so 18% of length = 18.

Note that this property (in combination with [kCDXProp_BondLength](BondLength.md)) is redundant with [kCDXProp_BondSpacingAbs](BondSpacingAbs.md). Only one of the two need be present. If both are present and contradictary, ChemDraw gives preference to the BondSpacingAbs value.

**If this property is absent:**  

If present, the [kCDXProp_BondSpacingAbs](BondSpacingAbs.md) value will be used. If that property is not present, then the value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20160913174558/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20160913174558/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)