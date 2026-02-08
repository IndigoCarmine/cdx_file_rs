## SGDatum Object

| CDXML Name:            | sgdatum                                                         |
| ---------------------- | --------------------------------------------------------------- |
| CDX Constant Name:     | kCDXObj_SGDatum                                                 |
| CDX Constant Value:    | 0x8024                                                          |
| Contained by objects:  | Structural Group–related objects (e.g., SGComponent, Superatom) |
| First written/read in: | ChemDraw (version unknown)                                      |

**Description:**

An SGDatum object represents a single data entry associated with a structural group (SG).
It is used to store typed metadata describing properties of a structural group, including the semantic category of the data, the property classification, and its associated value.
SGDatum objects may optionally be marked as read-only to prevent modification.

There are no explicitly required properties.

**Subobjects:**
*(none)*

**Properties:**

| Value | Name                                                                      | CDXML Name     | Type                   |
| ----- | ------------------------------------------------------------------------- | -------------- | ---------------------- |
| 4608  | SGDataType                                                                | SGDataType     | Unsigned 8-bit Integer |
|       | Specifies the semantic data type of the structural group datum.           |                |                        |
| 4609  | SGPropertyType                                                            | SGPropertyType | Unsigned 8-bit Integer |
|       | Specifies the property category associated with the datum.                |                |                        |
| 4610  | SGDataValue                                                               | SGDataValue    | CDXString              |
|       | Stores the value of the datum as a string representation.                 |                |                        |
| 4614  | IsReadOnly                                                                | IsReadOnly     | Boolean                |
|       | Indicates whether the datum is read-only and protected from modification. |                |                        |

---

[CDX Documentation index](index.md)
