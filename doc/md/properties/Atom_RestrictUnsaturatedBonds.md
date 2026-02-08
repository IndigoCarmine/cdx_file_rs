## Atom_RestrictUnsaturatedBonds Property

| CDXML Name: | UnsaturatedBonds |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_RestrictUnsaturatedBonds |
| CDX Constant Value: | 0x0426 |
| Data Size: | [INT8](/web/20190912231552/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190912231552/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

Indicates whether unsaturation should be present or absent.

For this and similar attributes, it is the responsibility of the reader to deal with conflicting values (for example, an atom attached to a double bond, but which has an "unsaturation must be absent" attribute) gracefully and not to cause failure to read entire document, although the writer has the first-hand responsibility to provide valid information.

The value stored in this property corresponds to the Unsaturated Atom property in ISIS, with the addition of an Unspecified value.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Node may or may not be unsaturated |
| 1 | MustBeAbsent | Node must not have any unsaturated bonds |
| 2 | MustBePresent | Node must have at least one unsaturated bond |

**If this property is absent:**

Node may or may not be unsaturated.

---

[CDX Documentation index](/web/20190912231552/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)