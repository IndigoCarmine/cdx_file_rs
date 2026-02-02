CDX Format Specification: Bracket_ComponentOrder Property
## Bracket_ComponentOrder Property

| CDXML Name: | ComponentOrder |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bracket_ComponentOrder |
| CDX Constant Value: | 0x0A29 |
| Data Size: | [INT16](/web/20160912002656/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_BracketedGroup](/web/20160912002656/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md) |
| First written/read in: | ChemDraw 7.0 / (not read) |
| Required? | No |

**Description:**  

The component order associated with a BracketedGroup.

Component Orders should be provided as consecutive positive integers starting at 1 within each [Bracketed Group](/web/20160912002656/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md). A Component Order of zero indicates an unordered component.

This property is relevent only for objects with a [kCDXProp_Bracket_Usage](Bracket_Usage.md) of kCDXBracketUsage_Component.

**If this property is absent:**  

The object is considered to have a component order of zero, meaning that it is considered to be unordered.

---

[CDX Documentation index](/web/20160912002656/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)