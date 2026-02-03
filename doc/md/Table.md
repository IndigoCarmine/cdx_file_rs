CDX Format Specification: Table Object
## Table Object

| CDXML Name: | table |
| --- | --- |
| CDX Constant Name: | kCDXObj_Table |
| CDX Constant Value: | 0x8016 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 7.0 |

**Description:**  

Each cell within a Table is stored as an individual [Page](Page.md) object. In ChemDraw 7.0, there are expected to be exactly (rows * colums) number of contained pages, stored rowwise from the top-left. The bounds of the table may be calculated from the union of the [kCDXProp_BoundsInParent](properties/BoundsInParent.md) properties of each contained page, and the positions of each row and column may be derived similarly by looking at the edges of those properties.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8001 | [kCDXObj_Page](Page.md) | page |  |
|  | A drawing space that can contain other objects. |  |  |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x000A | [kCDXProp_ZOrder](properties/ZOrder.md) | Z | [INT16](DataType/CDXNumeric.md) |
|  | Back-to-front ordering index in 2D drawing. |  |  |
| 0x0011 | [kCDXProp_Visible](properties/Visible.md) | Visible | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The object is visible if non-zero. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0806 | [kCDXProp_BoldWidth](properties/BoldWidth.md) | BoldWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default bold bond width. |  |  |
| 0x0807 | [kCDXProp_LineWidth](properties/LineWidth.md) | LineWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default line width. |  |  |
| 0x0808 | [kCDXProp_MarginWidth](properties/MarginWidth.md) | MarginWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default amount of space surrounding atom labels. |  |  |
| 0x081A | [kCDXProp_LabelStyleFont](properties/LabelStyleFont.md) | LabelFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font family for atom labels. |  |  |
| 0x081C | [kCDXProp_LabelStyleSize](properties/LabelStyleSize.md) | LabelSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for atom labels. |  |  |
| 0x081E | [kCDXProp_LabelStyleFace](properties/LabelStyleFace.md) | LabelFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for atom labels. |  |  |

---

[CDX Documentation index](index.md)