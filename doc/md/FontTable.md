## Font Table Object

| CDXML Name: | fonttable |
| --- | --- |
| CDX Constant Name: | kCDXProp_FontTable |
| CDX Constant Value: | 0x0100 |
| Contained by objects: | [kCDXObj_Document](Document.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**

A list of fonts used in the document. Other objects may use a font's ID to refer to individual entries in the font table.

A font table is required if fonts are used elsewhere in the document.

A font table must contain at least one [font](Font.md) object.

This object is used only in CDXML files. In CDX files, a [kCDXProp_FontTable](properties/FontTable.md) property is used instead.

**Subobjects:**

| Value | Name | CDXML Name |
| --- | --- | --- |
| n/a | n/a | [font](Font.md) |
|  | A logical font definition. |  |

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |

---

[CDX Documentation index](index.md)