## Reaction Step Object

| CDXML Name: | step |
| --- | --- |
| CDX Constant Name: | kCDXObj_ReactionStep |
| CDX Constant Value: | 0x800E |
| Contained by objects: | [kCDXObj_Page](Page.md), [kCDXObj_Group](Group.md), [kCDXObj_ReactionScheme](ReactionScheme.md) |
| First written/read in: | ChemDraw 4.1 |

**Description:**

A Reaction Step describes one step in a reaction.

Technically, this object has no required objects or properties, but it is pretty useless without any reactants or products.

**Subobjects:**
*(none)*

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0C00 | [kCDXProp_ReactionStep_Atom_Map](properties/ReactionStep_Atom_Map.md) | ReactionStepAtomMap | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to a product atom. |  |  |
| 0x0C01 | [kCDXProp_ReactionStep_Reactants](properties/ReactionStep_Reactants.md) | ReactionStepReactants | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of reactants present in the Reaction Step. |  |  |
| 0x0C02 | [kCDXProp_ReactionStep_Products](properties/ReactionStep_Products.md) | ReactionStepProducts | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of products present in the Reaction Step. |  |  |
| 0x0C03 | [kCDXProp_ReactionStep_Plusses](properties/ReactionStep_Plusses.md) | ReactionStepPlusses | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of plusses used to separate components of the Reaction Step. |  |  |
| 0x0C04 | [kCDXProp_ReactionStep_Arrows](properties/ReactionStep_Arrows.md) | ReactionStepArrows | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of arrows used to separate components of the Reaction Step. |  |  |
| 0x0C05 | [kCDXProp_ReactionStep_ObjectsAboveArrow](properties/ReactionStep_ObjectsAboveArrow.md) | ReactionStepObjectsAboveArrow | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of objects above the arrow in the Reaction Step. |  |  |
| 0x0C06 | [kCDXProp_ReactionStep_ObjectsBelowArrow](properties/ReactionStep_ObjectsBelowArrow.md) | ReactionStepObjectsBelowArrow | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of objects below the arrow in the Reaction Step. |  |  |
| 0x0C07 | [kCDXProp_ReactionStep_Atom_Map_Manual](properties/ReactionStep_Atom_Map_Manual.md) | ReactionStepAtomMapManual | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to a product atom. |  |  |
| 0x0C08 | [kCDXProp_ReactionStep_Atom_Map_Auto](properties/ReactionStep_Atom_Map_Auto.md) | ReactionStepAtomMapAuto | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to a product atom. |  |  |

---

[CDX Documentation index](index.md)