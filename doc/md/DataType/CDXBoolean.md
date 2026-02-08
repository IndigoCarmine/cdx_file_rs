## CDX Boolean Data Types

### CDXBoolean:

**In CDX files**, an [INT8](CDXNumeric.md) value representing True or Yes if non-zero, and False or No if zero.

**In CDXML files**, a enumerated value that may be either yes or no.

Note that this data type actually has a third implied value, 'unknown'. Since CDX and CDXML are both tagged formats, any given property may be omitted altogether from the file. A missing property of this type cannot be assumed to be either true or false.

### CDXBooleanImplied:

**In CDX files**, an [INT8](CDXNumeric.md) value representing True or Yes if present, and False or No if absent.

Note that properties of this type have zero length in CDX files: the only thing that matters is whether the property is or isn't present in the file. In contrast to the CDXBoolean data type (above), if a property of this type is missing from the CDX file, its value *must* be assumed to be False or No.

**In CDXML files**, a enumerated value that may be either yes or no.

See the complete list of CDX data types

---

CDX Documentation index