CDX Format Specification: INT16 List With Counts Data Type
## INT16 List With Counts Data Type

This data type consists of a series of [UINT16](CDXNumeric.md) values.

**In CDX files**, this data type is prefixed by an additional [UINT16](CDXNumeric.md) value indicating the total number of values to follow.

**Examples:**

| CDX: | 04 00 01 00 02 00 03 00 04 00 |
| --- | --- |
| CDXML: | "1 2 3 4" |

See the complete list of CDX data types

---

CDX Documentation index