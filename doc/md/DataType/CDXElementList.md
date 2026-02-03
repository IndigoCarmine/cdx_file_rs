CDX Format Specification: CDX Element List
## CDX Element List

This data type consists of a series of [UINT16](CDXNumeric.md) values.

**In CDX files**, this data type is prefixed by an additional [INT16](CDXNumeric.md) value whose absolute value indicates the total number of values to follow. If the sign of that prefix is negative, then the list actually represents a "not" list.

**In CDXML files**, this data type may be prefixed by an additional "NOT" string. If present, that string indicate that the list actually represents a "not" list.

**Examples:**

| CDX: | 03 00 09 00 11 00 23 00 |
| --- | --- |
| CDXML: | "9 17 35" |
| logical: | [F, Cl, Br] |
| CDX: | FD FF 09 00 11 00 23 00 |
| CDXML: | "NOT 9 17 35" |
| logical: | NOT [F, Cl, Br] |

See the complete list of CDX data types

---

CDX Documentation index