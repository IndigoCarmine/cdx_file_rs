CDX Format Specification: Font Object
## Font Object

| CDXML Name: | font |
| --- | --- |
| Contained by objects: | [kCDXProp_FontTable](FontTable.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

A logical font definition.

A font object requires the name and id properties, but has no required objects.

**Subobjects:**  
*(none)*

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [name](properties/fontname.md) | [Unformatted](DataType/Unformatted.md) |
|  | Required for fonts.The name of a font. |  |  |
| n/a | n/a | [charset](properties/charset.md) | [INT16](DataType/CDXNumeric.md) |
|  | The character set used by a font. |  |  |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | Required for fonts.A unique identifier for an object, used when other objects refer to it. |  |  |

---

[CDX Documentation index](index.md)