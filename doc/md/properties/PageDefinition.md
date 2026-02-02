CDX Format Specification: PageDefinition Property
## PageDefinition Property

| CDXML Name: | PageDefinition |
| --- | --- |
| CDX Constant Name: | kCDXProp_PageDefinition |
| CDX Constant Value: | 0x1FF1 |
| Data Size: | [INT8](/web/20100103054725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Page](/web/20100103054725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Page.md),[kCDXObj_Splitter](/web/20100103054725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Splitter.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

A description of the type of formatting used by the page, or by the splitter.

This property is used only for a custom build created for one specific corporation, and not read or written by any public release of ChemDraw.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Undefined |  |
| 1 | Center |  |
| 2 | TL4 |  |
| 3 | IDTerm |  |
| 4 | FlushLeft |  |
| 5 | FlushRight |  |
| 6 | Reaction1 |  |
| 7 | Reaction2 |  |
| 8 | MulticolumnTL4 |  |
| 9 | MulticolumnNonTL4 |  |
| 10 | UserDefined |  |

**If this property is absent:**  

The Page Definition is treated as Undefined.

---

[CDX Documentation index](/web/20100103054725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)