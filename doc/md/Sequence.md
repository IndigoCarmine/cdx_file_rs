## Sequence Object

| CDXML Name: | sequence |
| --- | --- |
| CDX Constant Name: | kCDXObj_Sequence |
| CDX Constant Value: | 0x8013 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 7.0 |

**Description:**

A Sequence object is one member of an ordered series; the contents of its Text object may change as other objects are added to or removed from the series. It may function as a bookmark so that other objects ([Cross-References](CrossReference.md)) may link to it, even if the other objects are in different documents altogether.

There are no required properties or objects.

**Subobjects:**
*(none)*

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| 0x0E00 | [kCDXProp_Sequence_Identifier](properties/Sequence_Identifier.md) | SequenceIdentifier | [CDXString](DataType/CDXString.md) |
|  | Required for sequences. A unique (but otherwise random) identifier for a given Sequence object. |  |  |

---

[CDX Documentation index](index.md)