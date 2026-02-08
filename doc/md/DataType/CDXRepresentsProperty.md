## CDX RepresentsProperty Data Type

**In CDX files**, this data type consists of a series of [CDXObjectID](CDXObjectID.md) followed by a [UINT16](CDXNumeric.md). The first value indicates which other object is relevant, and the second value indicates which property of that other object is referenced by this object.

**In CDXML files**, this data type is represented by a `represent` object.

**Examples:**

| CDX: | 06 00 00 00 21 04 |
| --- | --- |
| CDXML: | `<represent attribute="Charge" object="6"/>` |

See the complete list of CDX data types

---

CDX Documentation index