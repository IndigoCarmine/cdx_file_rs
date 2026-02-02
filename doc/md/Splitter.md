CDX Format Specification: Splitter Object
## Splitter Object

| CDXML Name: | splitter |
| --- | --- |
| CDX Constant Name: | kCDXObj_Splitter |
| CDX Constant Value: | 0x8015 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 7.0 |

**Description:**  

This object was defined for future compatibility and is not read or written by any public release of ChemDraw

There are no required properties or objects.

**Subobjects:**  
*(none)*

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| 0x0200 | [kCDXProp_2DPosition](properties/2DPosition.md) | p | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The 2D location (in the order of vertical and horizontal locations) of an object. |  |  |
| 0x1FF1 | [kCDXProp_PageDefinition](properties/PageDefinition.md) | PageDefinition | [INT8](DataType/CDXNumeric.md) |
|  | A description of the type of formatting used by the page, or by the splitter.
This is an enumerated property. |  |  |

---

[CDX Documentation index](index.md)