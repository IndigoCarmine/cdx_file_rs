## Style Object

| CDXML Name: | s |
| --- | --- |
| Contained by objects: | [kCDXObj_Text](Text.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**

A string of text in exactly one style. The text itself is stored as #PCDATA.

This object is used only in CDXML files. In CDX files, text information is stored in the [kCDXProp_Text](properties/Text.md) property, and style information is stored in either the [kCDXProp_CaptionStyle](properties/CaptionStyle.md) or [kCDXProp_LabelStyle](properties/LabelStyle.md) properties.

**Subobjects:**
*(none)*

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [face](properties/face.md) | [UINT16](DataType/CDXNumeric.md) |
| The display style of a font. This is a bit-encoded property. |  |  |  |
| n/a | n/a | [font](properties/font.md) | [UINT16](DataType/CDXNumeric.md) |
| The family of a font. |  |  |  |
| n/a | n/a | [size](properties/size.md) | [CDXCoordinate](DataType/CDXCoordinates.md) |
| The size of a font. |  |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
| The foreground color of an object represented as the two-based index into the object's color table. |  |  |  |

---

[CDX Documentation index](index.md)