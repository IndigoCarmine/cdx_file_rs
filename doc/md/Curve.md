## Curve Object

| CDXML Name: | curve |
| --- | --- |
| CDX Constant Name: | kCDXObj_Curve |
| CDX Constant Value: | 0x8008 |
| Contained by objects: | [kCDXObj_Page](Page.md),[kCDXObj_Group](Group.md),[kCDXObj_Fragment](Fragment.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**

A Bézier curve. Bézier curves are a standard mathematical curve type, and are defined in a number of places, including [here](here), [here](here), [here](here), [here](here), and [here](here).

A Curve object necessarily requires a `CurvePoints` property, but it has no required objects.

**Subobjects:**

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x000A | [kCDXProp_ZOrder](properties/ZOrder.md) | Z | [INT16](DataType/CDXNumeric.md) |
|  | Back-to-front ordering index in 2D drawing. |  |  |
| 0x000F | [kCDXProp_IgnoreWarnings](properties/IgnoreWarnings.md) | IgnoreWarnings | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether chemical warnings should be suppressed on this object. |  |  |
| 0x0010 | [kCDXProp_ChemicalWarning](properties/ChemicalWarning.md) | Warning | [CDXString](DataType/CDXString.md) |
|  | A warning concerning possible chemical problems with this object. |  |  |
| 0x0011 | [kCDXProp_Visible](properties/Visible.md) | Visible | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The object is visible if non-zero. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0A08 | [kCDXProp_Curve_Type](properties/Curve_Type.md) | CurveType | [INT16](DataType/CDXNumeric.md) |
|  | The type of curve object.
This is a bit-encoded property. |  |  |
| 0x0A20 | [kCDXProp_Arrowhead_Size](properties/Arrowhead_Size.md) | HeadSize | [INT16](DataType/CDXNumeric.md) |
|  | The size of the arrow's head. |  |  |
| 0x0A23 | [kCDXProp_Curve_Points](properties/Curve_Points.md) | CurvePoints | [CDXCurvePoints](DataType/CDXCurvePoints.md) |
|  | Required for curves. The Bézier curve's control point locations. |  |  |
| 0x0A2E | [kCDXProp_Curve_Points3D](properties/Curve_Points3D.md) | CurvePoints3D | [CDXCurvePoints3D](DataType/CDXCurvePoints3D.md) |
|  | The Bézier curve's control point locations in 3D space. |  |  |
| 0x0A2F | [kCDXProp_Arrowhead_Type](properties/Arrowhead_Type.md) | ArrowHeadType | [INT16](DataType/CDXNumeric.md) |
|  | The type of arrowhead.
This is an enumerated property. |  |  |
| 0x0A30 | [kCDXProp_Arrowhead_CenterSize](properties/Arrowhead_CenterSize.md) | HeadCenterSize | [UINT16](DataType/CDXNumeric.md) |
|  | The size of the arrow's head from the tip to the back of the head. |  |  |
| 0x0A31 | [kCDXProp_Arrowhead_Width](properties/Arrowhead_Width.md) | HeadWidth | [UINT16](DataType/CDXNumeric.md) |
|  | The half-width of the arrow's head. |  |  |
| 0x0A35 | [kCDXProp_Arrow_ArrowHead_Head](properties/Arrow_ArrowHead_Head.md) | ArrowHeadHead | [INT16](DataType/CDXNumeric.md) |
|  | The type of arrowhead at the head of the arrow.
This is an enumerated property. |  |  |
| 0x0A36 | [kCDXProp_Arrow_ArrowHead_Tail](properties/Arrow_ArrowHead_Tail.md) | ArrowHeadTail | [INT16](DataType/CDXNumeric.md) |
|  | The type of arrowhead at the tail of the arrow.
This is an enumerated property. |  |  |
| 0x0A37 | [kCDXProp_Fill_Type](properties/Fill_Type.md) | FillType | [INT16](DataType/CDXNumeric.md) |
|  | The type of the fill, for objects that can be filled.
This is an enumerated property. |  |  |
| 0x0A38 | [kCDXProp_Closed](properties/Closed.md) | Closed | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Signifies whether object is closed. |  |  |
| 0x0A38 | [kCDXProp_Curve_Spacing](properties/Curve_Spacing.md) | CurveSpacing | [UINT16](DataType/CDXNumeric.md) |
|  | The width of the space between a Doubled curve. |  |  |

---

[CDX Documentation index](index.md)