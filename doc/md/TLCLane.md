CDX Format Specification: TLC Lane Object
## TLC Lane Object

| CDXML Name: | tlclane |
| --- | --- |
| CDX Constant Name: | kCDXObj_TLCLane |
| CDX Constant Value: | 0x8024 |
| Contained by objects: | [kCDXObj_TLCPlate](TLCPlate.md) |
| First written/read in: | ChemDraw 8.0 |

**Description:**  

TLC Lane objects technically have no required properties or objects, but should probably contain at least one [Spot](TLCSpot.md) if they are to be useful.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |
| 0x8025 | [kCDXObj_TLCSpot](TLCSpot.md) | tlcspot |  |
|  | A single spot on a TLC plate. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0011 | [kCDXProp_Visible](properties/Visible.md) | Visible | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The object is visible if non-zero. |  |  |

---

[CDX Documentation index](index.md)