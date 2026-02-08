## CDX Font Table Data Type

### CDXFontTable:
**In CDX files**, a font table is a variable-length struct that consists of:  

| 0-1 | [UINT16](CDXNumeric.md) | The ID of this font. This must be unique, but need not be sequential |
| --- | --- | --- |
| 2-3 | [UINT16](CDXNumeric.md) | The character set of this font. Acceptable values are shown as part of the discussion of the CDXMLcharsetproperty |
| 4-5 | [UINT16](CDXNumeric.md) | The length of the name of this font |
| 6...n | [Unformatted](Unformatted.md) | The actual name of this font |

| 0-1 | [UINT16](CDXNumeric.md) | The ID of this font. This must be unique, but need not be sequential |
| --- | --- | --- |
| 2-3 | [UINT16](CDXNumeric.md) | The character set of this font. Acceptable values are shown as part of the discussion of the CDXMLcharsetproperty |
| 4-5 | [UINT16](CDXNumeric.md) | The length of the name of this font |
| 6...n | [Unformatted](Unformatted.md) | The actual name of this font |

**In CDXML files**, this data type is represented by a fonttable object.

**Examples:**

| CDX: | 00 02 00 03 00 E4 04 05   ........ 00 41 72 69 61 6C 04 00   .Arial.. E4 04 0F 00 54 69 6D 65   ....Time 73 20 4E 65 77 20 52 6F   s New Ro 6D 61 6E                  man |
| --- | --- |
| CDXML: | `<fonttable> <font id="3" charset="iso-8859-1" name="Arial"/> <font id="4" charset="iso-8859-1" name="Times New Roman"/> </fonttable>` |

See the complete list of CDX data types

---

CDX Documentation index