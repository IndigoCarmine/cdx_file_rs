## Crossing Bond Object

| CDXML Name: | crossingbond |
| --- | --- |
| CDX Constant Name: | kCDXObj_CrossingBond |
| CDX Constant Value: | 0x8019 |
| Contained by objects: | [kCDXObj_BracketAttachment](BracketAttachment.md) |
| First written/read in: | ChemDraw 7.0 (written only) |

**Description:**

A Bond that connects a Bracketed Group to a Node outside that group.

This object necessarily must have [kCDXProp_Bracket_BondID](properties/Bracket_BondID.md) and [kCDXProp_Bracket_InnerAtomID](properties/Bracket_InnerAtomID.md) properties. There are no required subobjects.

**Subobjects:**
*(none)*

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0A2C | [kCDXProp_Bracket_BondID](properties/Bracket_BondID.md) | BondID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | Required. The ID of a bond that crosses a Bracket Attachment. |  |  |
| 0x0A2D | [kCDXProp_Bracket_InnerAtomID](properties/Bracket_InnerAtomID.md) | InnerAtomID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | Required. The ID of the node located within the Bracketed Group and attached to a bond that crosses a Bracket Attachment. |  |  |

---

[CDX Documentation index](index.md)