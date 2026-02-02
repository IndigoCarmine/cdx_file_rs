CDX Format Specification: ReactionStep_Atom_Map Property
## ReactionStep_Atom_Map Property

| CDXML Name: | ReactionStepAtomMap |
| --- | --- |
| CDX Constant Name: | kCDXProp_ReactionStep_Atom_Map |
| CDX Constant Value: | 0x0C00 |
| Data Size: | [CDXObjectIDArray](/web/20160912012637/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_ReactionStep](/web/20160912012637/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ReactionStep.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**  

Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to to a product atom.

This property is redundant with the two properties [kCDXProp_ReactionStep_Atom_Map_Manual](ReactionStep_Atom_Map_Manual.md) and [kCDXProp_ReactionStep_Atom_Map_Auto](ReactionStep_Atom_Map_Auto.md). A given mapping may be listed in this property and/or in one of the others. Programs that care about the presence of a mapping but not the history of its creation (automatically or manually) should examine this field; programs that care about the automatic/manual history of a mapping should consult the others.

**If this property is absent:**  

This Reaction Step has no atom-atom mapping.

---

[CDX Documentation index](/web/20160912012637/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)