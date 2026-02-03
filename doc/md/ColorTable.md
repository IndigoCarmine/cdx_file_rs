CDX Format Specification: Color Table Object
## Color Table Object

| CDXML Name: | colortable |
| --- | --- |
| CDX Constant Name: | kCDXProp_ColorTable |
| CDX Constant Value: | 0x0300 |
| Contained by objects: | [kCDXObj_Document](Document.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

The color palette used throughout the document. Color indexes 0 and 1 always correspond to black and white and are not saved in the color table. The first and second RGB values (color indexes 2 and 3) are the default background and foreground colors, and other colors are numbered sequentially.

If no color table is present in the object or any of it's ancestors, only the standard color indexes 0 through 3 may be used. In that case, the background color (2) is assumed to be white, and the foreground color (3) is assumed to be black.

A color table must contain at least one [color](Color.md) object.

This object is used only in CDXML files. In CDX files, a [kCDXProp_ColorTable](properties/ColorTable.md) property is used instead.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| n/a | n/a | [color](Color.md) |  |
|  | An RGB color. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |

---

[CDX Documentation index](index.md)