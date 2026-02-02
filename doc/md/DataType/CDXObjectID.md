CDX Format Specification: CDX ObjectID Data Types
## CDX ObjectID Data Types

### CDXObjectID:

One object may refer to another object via the other object's ObjectID, a [UINT32](CDXNumeric.md) value. Object IDs should be unique within a href="../CDXML file, but in a case where there is ambiguity, a CDXObjectID refers to the closest object with that ID. For example, an object sharing the same containing object is closer than one that does not share the same containing object.

### CDXObjectIDArray:

This data type is used if one object needs to refer to more than one other object. It is a series of [UINT32](CDXNumeric.md) values. The length of the array is implicit from the size of the data.

**Examples:**

| CDX: | 01 00 00 00 02 00 00 00 03 00 00 00 04 00 00 00 |
| --- | --- |
| CDXML: | "1 2 3 4" |

### CDXObjectIDArrayWithCounts:

This data type is used if one object needs to refer to more than one other object. It is a series of [UINT32](CDXNumeric.md) values.

**In CDX files**, this data type lists the size of the array explicitly, unlike with the CDXObjectIDArray type, with an a leading [UINT16](CDXNumeric.md) value before the series of CDXObjectIDs.

**In CDXML files**, this data type is equivalent to the CDXObjectIDArray type.

**Examples:**

| CDX: | 04 00 01 00 00 00 02 00 00 00 03 00 00 00 04 00 00 00 |
| --- | --- |
| CDXML: | "1 2 3 4" |

See the complete list of CDX data types

---

CDX Documentation index