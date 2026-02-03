CDX Format Specification: TLC Plate Object
## TLC Plate Object

| CDXML Name: | tlcplate |
| --- | --- |
| CDX Constant Name: | kCDXObj_TLCPlate |
| CDX Constant Value: | 0x8023 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 8.0 |

**Description:**  

Each plate contains a series of [Lanes](TLCLane.md). Those lanes should be arranged on the plate from left to right in the order that they appear in the cdx file.

TLC Plates should not be assumed to be positioned vertically. The actual orientation can be determined from the four corner properties ([kCDXProp_TopLeft](properties/TopLeft.md), etc). Similarly, they should not be assumed to be orthogonal, although in most practical cases there will be 90° angles at each corner.

TLC Plate objects technically have no required properties or objects, but if the four corner properties are omitted, the plate will be assumed to have a zero-by-zero size, which isn't very useful. Similarly, at least one [Lane](TLCLane.md) would be helpful, and that lane should probably contain at least one [Spot](TLCSpot.md).

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |
| 0x8024 | [kCDXObj_TLCLane](TLCLane.md) | tlclane |  |
|  | A logical object representing a series of spots arranged vertically on a TLC plate. |  |  |

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
| 0x0209 | [kCDXProp_TopLeft](properties/TopLeft.md) | TopLeft | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The location of the top-left corner of a quadrilateral object, possibly in a rotated or skewed frame. |  |  |
| 0x020A | [kCDXProp_TopRight](properties/TopRight.md) | TopRight | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The location of the top-right corner of a quadrilateral object, possibly in a rotated or skewed frame. |  |  |
| 0x020B | [kCDXProp_BottomRight](properties/BottomRight.md) | BottomRight | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The location of the bottom-right corner of a quadrilateral object, possibly in a rotated or skewed frame. |  |  |
| 0x020C | [kCDXProp_BottomLeft](properties/BottomLeft.md) | BottomLeft | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The location of the bottom-left corner of a quadrilateral object, possibly in a rotated or skewed frame. |  |  |
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
| 0x0AA0 | [kCDXProp_TLC_OriginFraction](properties/TLC_OriginFraction.md) | OriginFraction | [FLOAT64](DataType/CDXNumeric.md) |
|  | The distance of the origin line from the bottom of a TLC Plate, as a fraction of the total height of the plate. |  |  |
| 0x0AA1 | [kCDXProp_TLC_SolventFrontFraction](properties/TLC_SolventFrontFraction.md) | SolventFrontFraction | [FLOAT64](DataType/CDXNumeric.md) |
|  | The distance of the solvent front from the top of a TLC Plate, as a fraction of the total height of the plate. |  |  |
| 0x0AA2 | [kCDXProp_TLC_ShowOrigin](properties/TLC_ShowOrigin.md) | ShowOrigin | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the origin line near the base of the TLC Plate if non-zero. |  |  |
| 0x0AA3 | [kCDXProp_TLC_ShowSolventFront](properties/TLC_ShowSolventFront.md) | ShowSolventFront | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the solvent front line near the top of the TLC Plate if non-zero. |  |  |
| 0x0AA4 | [kCDXProp_TLC_ShowBorders](properties/TLC_ShowBorders.md) | ShowBorders | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show borders around the edges of the TLC Plate if non-zero. |  |  |

---

[CDX Documentation index](index.md)