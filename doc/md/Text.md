CDX Format Specification: Text Object
## Text Object

| CDXML Name: | t |
| --- | --- |
| CDX Constant Name: | kCDXObj_Text |
| CDX Constant Value: | 0x8006 |
| Contained by objects: | [kCDXObj_Page](Page.md),[kCDXObj_Group](Group.md),[kCDXObj_Node](Node.md),[kCDXObj_NamedAlternativeGroup](NamedAltGroup.md),[kCDXObj_ObjectTag](ObjectTag.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

An arbitrary block of (possibly styled) text.

An isolated text object has unspecified chemical meaning; it may or may not be interpreted chemically depending on the behavior of the program that reads the file. For example, a text object containing the text "CH3CH2CH3" might be considered to represent propane, or it might be considered simply to be a sequence of 9 characters. If chemical meaning is desired, a [Node](Node.md) object should be created instead, where that node object would itself contain an appropriate Text object, plus an additional Fragment object to describe the chemical meaning.

**In CDX files**, a Text object must contain a kCDXProp_Text property; **in CDXML files** it must contain at least one [s](Style.md) subobject. Until ChemDraw 6.0, a Text object also required a [bounding box](properties/BoundingBox.md).

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |
| n/a | n/a | [s](Style.md) |  |
|  | A string of text in exactly one style. |  |  |

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
| 0x0200 | [kCDXProp_2DPosition](properties/2DPosition.md) | p | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The 2D location (in the order of vertical and horizontal locations) of an object. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | Required for pictures and spectra. Required for graphics and text until 6.0.The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0205 | [kCDXProp_RotationAngle](properties/RotationAngle.md) | RotationAngle | [INT32](DataType/CDXNumeric.md) |
|  | The angular orientation of an object in degrees * 65536. |  |  |
| 0x0700 | [kCDXProp_Text](properties/Text.md) | (not used) | [CDXString](DataType/CDXString.md) |
|  | Required for text objects.The text of a text object. |  |  |
| 0x0701 | [kCDXProp_Justification](properties/Justification.md) | Justification | [INT8](DataType/CDXNumeric.md) |
|  | The horizontal justification of a text object.
This is an enumerated property. |  |  |
| 0x0702 | [kCDXProp_LineHeight](properties/LineHeight.md) | LineHeight | [UINT16](DataType/CDXNumeric.md) |
|  | The line height of a text object. |  |  |
| 0x0703 | [kCDXProp_WordWrapWidth](properties/WordWrapWidth.md) | WordWrapWidth | [INT16](DataType/CDXNumeric.md) |
|  | The word-wrap width of a text object. |  |  |
| 0x0704 | [kCDXProp_LineStarts](properties/LineStarts.md) | LineStarts | [INT16ListWithCounts](DataType/INT16ListWithCounts.md) |
|  | The number of lines of a text object followed by that many values indicating the zero-based text position of each line start. |  |  |
| 0x0705 | [kCDXProp_LabelAlignment](properties/LabelAlignment.md) | LabelAlignment | [INT8](DataType/CDXNumeric.md) |
|  | The alignment of the text with respect to the node position.
This is an enumerated property. |  |  |
| 0x0706 | [kCDXProp_LabelLineHeight](properties/LabelLineHeight.md) | LabelLineHeight | [INT16](DataType/CDXNumeric.md) |
|  | Text line height for atom labels |  |  |
| 0x0707 | [kCDXProp_CaptionLineHeight](properties/CaptionLineHeight.md) | CaptionLineHeight | [INT16](DataType/CDXNumeric.md) |
|  | Text line height for non-atomlabel text objects |  |  |
| 0x0708 | [kCDXProp_InterpretChemically](properties/InterpretChemically.md) | InterpretChemically | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether to the text label should be interpreted chemically (if possible). |  |  |
| 0x080A | [kCDXProp_LabelStyle](properties/LabelStyle.md) | (not used) | [CDXFontStyle](DataType/CDXString.md) |
|  | The default style for atom labels.. |  |  |
| 0x080B | [kCDXProp_CaptionStyle](properties/CaptionStyle.md) | (not used) | [CDXFontStyle](DataType/CDXString.md) |
|  | The default style for non-atomlabel text objects.. |  |  |
| 0x080C | [kCDXProp_CaptionJustification](properties/CaptionJustification.md) | CaptionJustification | [INT8](DataType/CDXNumeric.md) |
|  | The horizontal justification of a caption (non-atomlabel text object)
This is an enumerated property. |  |  |
| 0x081A | [kCDXProp_LabelStyleFont](properties/LabelStyleFont.md) | LabelFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font family for atom labels. |  |  |
| 0x081B | [kCDXProp_CaptionStyleFont](properties/CaptionStyleFont.md) | CaptionFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for captions (non-atom-label text objects). |  |  |
| 0x081C | [kCDXProp_LabelStyleSize](properties/LabelStyleSize.md) | LabelSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for atom labels. |  |  |
| 0x081D | [kCDXProp_CaptionStyleSize](properties/CaptionStyleSize.md) | CaptionSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for captions (non-atom-label text objects). |  |  |
| 0x081E | [kCDXProp_LabelStyleFace](properties/LabelStyleFace.md) | LabelFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for atom labels. |  |  |
| 0x081F | [kCDXProp_CaptionStyleFace](properties/CaptionStyleFace.md) | CaptionFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font face for captions (non-atom-label text objects). |  |  |
| 0x0820 | [kCDXProp_LabelStyleColor](properties/LabelStyleColor.md) | LabelColor | [INT16](DataType/CDXNumeric.md) |
|  | The default color for atom labels |  |  |
| 0x0821 | [kCDXProp_CaptionStyleColor](properties/CaptionStyleColor.md) | CaptionColor | [INT16](DataType/CDXNumeric.md) |
|  | The default color for captions (non-atom-label text objects). |  |  |
| 0x0823 | [kCDXProp_LabelJustification](properties/LabelJustification.md) | LabelJustification | [INT8](DataType/CDXNumeric.md) |
|  | The default justification for atom labels.
This is an enumerated property. |  |  |

---

[CDX Documentation index](index.md)