CDX Format Specification: Reaction Scheme Object
## Reaction Scheme Object

| CDXML Name: | scheme |
| --- | --- |
| CDX Constant Name: | kCDXObj_ReactionScheme |
| CDX Constant Value: | 0x800D |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | (not written/read) |

**Description:**  

A Reaction Scheme describes a single- or multi-step reaction. It contains one or more
Reaction Steps.

A single-step reaction will, unsurprisingly, contain only a single Reaction Step. A multi-step reaction will contain more than one Reaction Step, one for each step in the reaction. Although the various Reaction Step objects will often appear in the CDX file in the same logical order that they appear in the reaction, this is not guaranteed and should not be assumed. In many cases, it is impossible to specify a single unique order to the Reaction Steps. For example, a convergent synthesis will have more than one 'first' step, while a divergent reaction will have more than one 'last' step. A circular reaction may have no 'first' or 'last' steps at all in a logical sense, yet all of the steps will be present in the cdx file in some unspecified order.

There are no required properties, but a Reaction Scheme must contain at least one [Reaction Step](ReactionStep.md).

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x800E | [kCDXObj_ReactionStep](ReactionStep.md) | step |  |
|  | A description of one step in a reaction. It contains the constituents, conditions, and products. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |

---

[CDX Documentation index](index.md)