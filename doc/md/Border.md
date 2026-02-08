## Border Object

| CDXML Name: | border |
| --- | --- |
| CDX Constant Name: | kCDXObj_Border |
| CDX Constant Value: | 0x8020 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 7.0 |

**Description:**

A collection of information describing one edge of an object. As of ChemDraw 7.0, borders are defined only for rectangular objects, but that restriction may be removed in the future.

Borders have significance in ChemDraw 7.0 only for [Page](Page.md) objects that represent individual cells in a table. In such a case, Pages that represent adjacent cells are expected to have identical objects representing the shared border. If the adjacent pages have contradictory border information, the results are undefined.

**Subobjects:**
*(none)*

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0807 | [kCDXProp_LineWidth](properties/LineWidth.md) | LineWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default line width. |  |  |
| 0x0825 | [kCDXProp_Side](properties/Side.md) | Side | [UINT16](DataType/CDXNumeric.md) |
|  | Required. A specific side of an object (rectangle).
This is an enumerated property. |  |  |
| 0x0A01 | [kCDXProp_Line_Type](properties/Line_Type.md) | LineType | [INT16](DataType/CDXNumeric.md) |
|  | The type of a line object.
This is an enumerated property. |  |  |

---

[CDX Documentation index](index.md)