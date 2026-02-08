## CDX Generic List

This data type consists of a series of [CDXString](CDXString.md) values.

**In CDX files**, this data type is prefixed by an additional [INT16](CDXNumeric.md) value whose absolute value indicates the total number of values to follow. If the sign of that prefix is negative, then the list actually represents a "not" list.

**In CDXML files**, this data type may be prefixed by an additional "NOT" string. If present, that string indicates that the list actually represents a "not" list.

**Examples:**

| CDX: | 03 00 03 00 00 00 52 03 00 00 00 58 03 00 00 00 41 |
| --- | --- |
| CDXML: | "R X A" |
| logical: | [R, X, A] |
| CDX: | FD FF 03 00 00 00 52 03 00 00 00 58 03 00 00 00 41 |
| CDXML: | "NOT R X A" |
| logical: | NOT [R, X, A] |

See the complete list of CDX data types

---

CDX Documentation index