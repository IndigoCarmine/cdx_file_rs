CDX Format Specification: Bracketed Group Object
## Bracketed Group Object

| CDXML Name: | bracketedgroup |
| --- | --- |
| CDX Constant Name: | kCDXObj_BracketedGroup |
| CDX Constant Value: | 0x8017 |
| Contained by objects: | [kCDXObj_Page](Page.md),[kCDXObj_BracketedGroup](BracketedGroup.md) |
| First written/read in: | ChemDraw 7.0 (written only) |

**Description:**  

Bracketed groups generally represent a collection of objects that is repeated some number of times, such as with polymers, mixtures, or other repeating units. Each contained object is listed by its ID in the [kCDXProp_BracketedObjects](properties/BracketedObjects.md) property, and external connections are indicated by [kCDXObj_BracketAttachment](BracketAttachment.md) subobjects.

Bracketed groups may be nested, indicating that one set of brackets fully contains another set.

Each Bracketed Group may contain zero or more [Bracket Attachments](BracketAttachment.md), indicating which bonds cross from the inside to the outside of the bracket. Although the cdx specification places no limits on the number of Bracket Attachments associated with each Bracketed Group, the presence of more than four Bracket Attachments can be ambiguous or unclear, and so it is recommended that four Bracket Attachments per Bracketed Group be considered a practical maximum.

As of ChemDraw 7.0, Bracketed Groups are written to files as they are saved, but they (and all contained properties and subobjects) are ignored when reading.

Although there are no required properties or objects, a Bracketed Group without any [Bracketed Objects](properties/BracketedObjects.md) is pretty useless.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8017 | [kCDXObj_BracketedGroup](BracketedGroup.md) | bracketedgroup |  |
|  | A collection of objects surrounded by brackets (or braces or parentheses). |  |  |
| 0x8018 | [kCDXObj_BracketAttachment](BracketAttachment.md) | bracketattachment |  |
|  | A linkage that connects a Bracketed Group to some object outside that group. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0A24 | [kCDXProp_Bracket_Usage](properties/Bracket_Usage.md) | BracketUsage | [INT8](DataType/CDXNumeric.md) |
|  | The syntactical chemical meaning of the bracket (SRU, mer, mon, xlink, etc).
This is an enumerated property. |  |  |
| 0x0A25 | [kCDXProp_Polymer_RepeatPattern](properties/Polymer_RepeatPattern.md) | PolymerRepeatPattern | [INT8](DataType/CDXNumeric.md) |
|  | The head-to-tail connectivity of objects contained within the bracket.
This is an enumerated property. |  |  |
| 0x0A26 | [kCDXProp_Polymer_FlipType](properties/Polymer_FlipType.md) | PolymerFlipType | [INT8](DataType/CDXNumeric.md) |
|  | The flip state of objects contained within the bracket.
This is an enumerated property. |  |  |
| 0x0A27 | [kCDXProp_BracketedObjects](properties/BracketedObjects.md) | BracketedObjectIDs | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | The set of objects contained in a BracketedGroup. |  |  |
| 0x0A28 | [kCDXProp_Bracket_RepeatCount](properties/Bracket_RepeatCount.md) | RepeatCount | [FLOAT64](DataType/CDXNumeric.md) |
|  | The number of times a multiple-group BracketedGroup is repeated. |  |  |
| 0x0A29 | [kCDXProp_Bracket_ComponentOrder](properties/Bracket_ComponentOrder.md) | ComponentOrder | [INT16](DataType/CDXNumeric.md) |
|  | The component order associated with a BracketedGroup. |  |  |
| 0x0A2A | [kCDXProp_Bracket_SRULabel](properties/Bracket_SRULabel.md) | SRULabel | [CDXString](DataType/CDXString.md) |
|  | The label associated with a BracketedGroup that represents an SRU. |  |  |

---

[CDX Documentation index](index.md)