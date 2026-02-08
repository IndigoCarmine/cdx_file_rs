## Bracket Attachment Object

| CDXML Name: | bracketattachment |
| --- | --- |
| CDX Constant Name: | kCDXObj_BracketAttachment |
| CDX Constant Value: | 0x8018 |
| Contained by objects: | [kCDXObj_BracketedGroup](BracketedGroup.md) |
| First written/read in: | ChemDraw 7.0 (written only) |

**Description:**

An individual bracket (or brace or parenthesis) that, together with others, defines a [Bracketed Group](BracketedGroup.md).

Each Bracket Attachment may contain zero or more [Crossing Bonds](CrossingBond.md), indicating which bonds cross from the inside to the outside of the bracket. Although the cdx specification places no limits on the number of Crossing Bonds associated with each Bracket Attachment, the presence of more than two Crossing Bonds can be ambiguous or unclear, and so it is recommended that two Crossing Bonds per Bracket Attachment be considered a practical maximum.

There are no required properties or objects.

**Subobjects:**

| Value | Name | CDXML Name |
| --- | --- | --- |
| 0x8019 | [kCDXObj_CrossingBond](CrossingBond.md) | crossingbond |
|  | A Bond that connects a Bracketed Group to a Node outside that group. |  |

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0A2B | [kCDXProp_Bracket_GraphicID](properties/Bracket_GraphicID.md) | GraphicID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | The ID of a graphical object (bracket, brace, or parenthesis) associated with a Bracket Attachment. |  |  |

---

[CDX Documentation index](index.md)