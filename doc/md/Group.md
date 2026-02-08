## Group Object

| CDXML Name: | group |
| --- | --- |
| CDX Constant Name: | kCDXObj_Group |
| CDX Constant Value: | 0x8002 |
| Contained by objects: | [kCDXObj_Page](Page.md), [kCDXObj_Group](Group.md), [kCDXObj_NamedAlternativeGroup](NamedAltGroup.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**

A Group is a logical collection of objects. ChemDraw creates Group objects via the Group command in the Object menu. Groups are especially useful to associate otherwise-disjoint fragments, for example, an organic anion and the counterion that accompanies it.

ChemDraw 6.0.x and earlier do not support nested groups. Files containing nested groups (groups that contain other groups) will be flattened so that only the innermost groups are preserved. No objects will ever be lost in this case, just the hierarchical arrangement of the grouping.

A Group has no required objects or properties, but a Group without any objects is pretty useless.

**Subobjects:**

| Value | Name | CDXML Name |
| --- | --- | --- |
| 0x8002 | [kCDXObj_Group](Group.md) | group |
|  | A logical collection of objects. |  |
| 0x8003 | [kCDXObj_Fragment](Fragment.md) | fragment |
|  | A collection of nodes and their connectivity (bonds). |  |
| 0x8006 | [kCDXObj_Text](Text.md) | t |
|  | An arbitrary block of (possibly styled) text. |  |
| 0x8007 | [kCDXObj_Graphic](Graphic.md) | graphic |
|  | A (generally non-chemical) graphic object such as a line, arc, circle, or rectangle. |  |
| 0x8008 | [kCDXObj_Curve](Curve.md) | curve |
|  | A Bézier curve. |  |
| 0x800A | [kCDXObj_NamedAlternativeGroup](NamedAltGroup.md) | altgroup |
|  | A container object holding fragments that represent alternative substituents for a query. |  |
| 0x800E | [kCDXObj_ReactionStep](ReactionStep.md) | step |
|  | A description of one step in a reaction. It contains the constituents, conditions, and products. |  |
| 0x8010 | [kCDXObj_Spectrum](Spectrum.md) | spectrum |
|  | An NMR, MS, IR or other sort of spectral plot. |  |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x1100 | [kCDXProp_Group_Integral](properties/Group_Integral.md) | Integral | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The group is considered to be integral (non-subdivisible) if non-zero. |  |  |

---

[CDX Documentation index](index.md)