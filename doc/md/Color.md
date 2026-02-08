## Color Object

| CDXML Name: | color |
| --- | --- |
| Contained by objects: | [kCDXProp_ColorTable](ColorTable.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**

An RGB color.

Support for other color types might be added in the future, but note that different color spaces can be interconverted fairly easily. See the Color Space FAQ for details.

This object is used only in CDXML files. In CDX files, color information is stored as part of the [Color Table](properties/ColorTable.md) property directly.

All three properties of this object are required, but it has no required objects.

**Subobjects:**
*(none)*

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [b](properties/b.md) | n/a |
| Required for colors.The blue component of an RGB color, in the range 0.0 ... 1.0. |  |  |
| n/a | n/a | [g](properties/g.md) | n/a |
| Required for colors.The green component of an RGB color, in the range 0.0 ... 1.0. |  |  |
| n/a | n/a | [r](properties/r.md) | n/a |
| Required for colors.The red component of an RGB color, in the range 0.0 ... 1.0. |  |  |

---

[CDX Documentation index](index.md)