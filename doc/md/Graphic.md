CDX Format Specification: Graphic Object
## Graphic Object

| CDXML Name: | graphic |
| --- | --- |
| CDX Constant Name: | kCDXObj_Graphic |
| CDX Constant Value: | 0x8007 |
| Contained by objects: | [kCDXObj_Page](Page.md),[kCDXObj_Group](Group.md),[kCDXObj_Fragment](Fragment.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

A Graphic object represents a (generally non-chemical) graphic object such as a line, arc, circle, or rectangle.

Since a Graphic object is necessarily visual, it always requires a [bounding box](properties/BoundingBox.md) property.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |
| 0x000e | [kCDXProp_RepresentsProperty](Represent.md) | represent |  |
|  | An object used to indicate that its containing object has chemical meaning that is also represented in another object. |  |  |

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
| 0x0012 | [kCDXProp_SupersededBy](properties/SupersededBy.md) | SupersededBy | [CDXObjectID](DataType/CDXObjectID.md) |
|  | The ID of the object that should be read instead of this one. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | Required for pictures and spectra. Required for graphics and text until 6.0.The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0207 | [kCDXProp_3DHead](properties/3DHead.md) | Head3D | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location (in the order of X-, Y-, and Z-locations in right-handed coordinate system) of the head of an object in CDX coordinate units. |  |  |
| 0x0208 | [kCDXProp_3DTail](properties/3DTail.md) | Tail3D | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location (in the order of X-, Y-, and Z-locations in right-handed coordinate system) of the tail of an object in CDX coordinate units. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0806 | [kCDXProp_BoldWidth](properties/BoldWidth.md) | BoldWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default bold bond width. |  |  |
| 0x0807 | [kCDXProp_LineWidth](properties/LineWidth.md) | LineWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default line width. |  |  |
| 0x080B | [kCDXProp_CaptionStyle](properties/CaptionStyle.md) | (not used) | [CDXFontStyle](DataType/CDXString.md) |
|  | The default style for non-atomlabel text objects.. |  |  |
| 0x081B | [kCDXProp_CaptionStyleFont](properties/CaptionStyleFont.md) | CaptionFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for captions (non-atom-label text objects). |  |  |
| 0x081D | [kCDXProp_CaptionStyleSize](properties/CaptionStyleSize.md) | CaptionSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for captions (non-atom-label text objects). |  |  |
| 0x081F | [kCDXProp_CaptionStyleFace](properties/CaptionStyleFace.md) | CaptionFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font face for captions (non-atom-label text objects). |  |  |
| 0x0A00 | [kCDXProp_Graphic_Type](properties/Graphic_Type.md) | GraphicType | [INT16](DataType/CDXNumeric.md) |
|  | The type of graphical object.
This is an enumerated property. |  |  |
| 0x0A01 | [kCDXProp_Line_Type](properties/Line_Type.md) | LineType | [INT16](DataType/CDXNumeric.md) |
|  | The type of a line object.
This is an enumerated property. |  |  |
| 0x0A02 | [kCDXProp_Arrow_Type](properties/Arrow_Type.md) | ArrowType | [INT16](DataType/CDXNumeric.md) |
|  | The type of arrow object, which represents line, arrow, arc, rectangle, or orbital.
This is an enumerated property. |  |  |
| 0x0A03 | [kCDXProp_Rectangle_Type](properties/Rectangle_Type.md) | RectangleType | [INT16](DataType/CDXNumeric.md) |
|  | The type of a rectangle object.
This is an enumerated property. |  |  |
| 0x0A04 | [kCDXProp_Oval_Type](properties/Oval_Type.md) | OvalType | [INT16](DataType/CDXNumeric.md) |
|  | The type of an arrow object that represents a circle or ellipse.
This is an enumerated property. |  |  |
| 0x0A05 | [kCDXProp_Orbital_Type](properties/Orbital_Type.md) | OrbitalType | [INT16](DataType/CDXNumeric.md) |
|  | The type of orbital object.
This is an enumerated property. |  |  |
| 0x0A06 | [kCDXProp_Bracket_Type](properties/Bracket_Type.md) | BracketType | [INT16](DataType/CDXNumeric.md) |
|  | The type of symbol object.
This is an enumerated property. |  |  |
| 0x0A07 | [kCDXProp_Symbol_Type](properties/Symbol_Type.md) | SymbolType | [INT16](DataType/CDXNumeric.md) |
|  | The type of symbol object.
This is an enumerated property. |  |  |
| 0x0A20 | [kCDXProp_Arrowhead_Size](properties/Arrowhead_Size.md) | HeadSize | [INT16](DataType/CDXNumeric.md) |
|  | The size of the arrow's head. |  |  |
| 0x0A21 | [kCDXProp_Arc_AngularSize](properties/Arc_AngularSize.md) | AngularSize | [INT16](DataType/CDXNumeric.md) |
|  | The size of an arc (in degrees * 10, so 90 degrees = 900). |  |  |
| 0x0A22 | [kCDXProp_Bracket_LipSize](properties/Bracket_LipSize.md) | LipSize | [INT16](DataType/CDXNumeric.md) |
|  | The size of a bracket. |  |  |
| 0x0A24 | [kCDXProp_Bracket_Usage](properties/Bracket_Usage.md) | BracketUsage | [INT8](DataType/CDXNumeric.md) |
|  | The syntactical chemical meaning of the bracket (SRU, mer, mon, xlink, etc).
This is an enumerated property. |  |  |
| 0x0A25 | [kCDXProp_Polymer_RepeatPattern](properties/Polymer_RepeatPattern.md) | PolymerRepeatPattern | [INT8](DataType/CDXNumeric.md) |
|  | The head-to-tail connectivity of objects contained within the bracket.
This is an enumerated property. |  |  |
| 0x0A26 | [kCDXProp_Polymer_FlipType](properties/Polymer_FlipType.md) | PolymerFlipType | [INT8](DataType/CDXNumeric.md) |
|  | The flip state of objects contained within the bracket.
This is an enumerated property. |  |  |
| 0x0A3C | [kCDXProp_CornerRadius](properties/CornerRadius.md) | CornerRadius | [INT16](DataType/CDXNumeric.md) |
|  | The radius of the rounded corner of a rounded rectangle. |  |  |
| 0x0A3D | [kCDXProp_Frame_Type](properties/Frame_Type.md) | FrameType | [INT16](DataType/CDXNumeric.md) |
|  | The type of frame on an object.
This is an enumerated property. |  |  |

---

[CDX Documentation index](index.md)