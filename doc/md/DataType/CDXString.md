## CDX String Data Types

### CDXString:

**In CDX files**, a CDXString is a variable-length struct that consists of:

| 0-1 | [UINT16](CDXNumeric.md) | The character at which this style starts |
| --- | --- | --- |
| 2-9 | [Style run](#FontStyleRun) | A struct describing this style run |

| 0-1 | [UINT16](CDXNumeric.md) | The character at which this style starts |
| --- | --- | --- |
| 2-9 | [Style run](#FontStyleRun) | A struct describing this style run |
| Text | The string's text written out. The string's length is implicit and determined by subtracting the length of the first two items from the total length of the CDXText object |

The first two bytes of a CDXString are a UINT16 indicating the number of font style runs. If the number of font style runs is zero, the string is taken to be ISO Latin 1 with no particular font or size specified. In some cases, this may imply the use of a default text style specified elsewhere. If the number of font style runs is not zero, it is followed by that many font style runs (see below), which is then followed by string text.

**In CDXML files**, a CDXString is an ordered sequence of one or more Style objects.

**Examples:** (assuming arbitrary index/id of "3" for the font, 12-point text, arbitrary index/id of "4" for the color)

| Text | CDX: | 00 00 54 65 78 74 |
| --- | --- | --- |
| CDXML: | <s>Text</s> |  |
| Text | CDX: | 01 00 00 00 03 00 01 00 0C 00 04 00 54 65 78 74 |
| CDXML: | <s font="3" size="12" face="1">Text</s> |  |
| Text | CDX: | 03 00 00 00 03 00 01 00 00 00 0C 00 04 00 02 00 03 00 05 00 00 00 0C 00 04 00 03 00 03 00 00 00 0C 00 04 00 54 65 78 74 |
| CDXML: | `<s font="3" size="12" face="1">Te</s> <s font="3" size="12" face="5">x</s> <s font="3" size="12">t</s>` |  |

### Font style run:

**In CDX files**, a font style is an 8-byte struct that consists of:

| 0x00 : | plain |
| --- | --- |
| 0x01 : | bold |
| 0x02 : | italic |
| 0x04 : | underline |
| 0x08 : | outline |
| 0x10 : | shadow |
| 0x20 : | subscript |
| 0x40 : | superscript |
| 0x60 : | formula (style in which subscript or superscript style is selected appropriately depending on the formula label) |

Note that the subscript, superscript, and formula styles are mutually exclusive. The other styles may be combined by OR-ing the type codes.
For example, bold italic is 0x03 (= 0x01 | 0x02).

The outline and shadow styles are only useful on Macintosh computers.

| 0x00 : | plain |
| --- | --- |
| 0x01 : | bold |
| 0x02 : | italic |
| 0x04 : | underline |
| 0x08 : | outline |
| 0x10 : | shadow |
| 0x20 : | subscript |
| 0x40 : | superscript |
| 0x60 : | formula (style in which subscript or superscript style is selected appropriately depending on the formula label) |
| 4-5 | [UINT16](CDXNumeric.md) | Font size, measured in 20ths of a point. Note that this is an integral field, which implies that CDX files cannot store font sizes any more accurately than the nearest 0.05 of a point. |
| 6-7 | [UINT16](CDXNumeric.md) | Font color |

See the complete list of CDX data types

---

CDX Documentation index