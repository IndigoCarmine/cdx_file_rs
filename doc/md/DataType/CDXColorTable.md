## CDX Color Table Data Type

**In CDX files**, this data type consists of a [UINT16](CDXNumeric.md) count, followed by that many [UINT16](CDXNumeric.md) triples. Each of those triples consists of the red, green, and blue components (in that order) of the color value, scaled to a range of 0...65535.

**In CDXML files**, this data type is represented by a `colortable` object.

**Examples:**

| CDX: | 08 00 FF FF FF FF FF FF 00 00 00 00 00 00 FF FF 00 00 00 00 FF FF FF FF 00 00 00 00 FF FF 00 00 00 00 FF FF FF FF 00 00 00 00 FF FF FF FF 00 00 FF FF |
| --- | --- |
| CDXML: | `<colortable> <color r="1" g="1" b="1"/> <color r="0" g="0" b="0"/> <color r="1" g="0" b="0"/> <color r="1" g="1" b="0"/> <color r="0" g="1" b="0"/> <color r="0" g="1" b="1"/> <color r="0" g="0" b="1"/> <color r="1" g="0" b="1"/> </colortable>` |

See the complete list of CDX data types

---

CDX Documentation index