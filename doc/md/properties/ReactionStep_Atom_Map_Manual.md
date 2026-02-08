## ReactionStep_Atom_Map_Manual Property

| CDXML Name: | ReactionStepAtomMapManual |
| --- | --- |
| CDX Constant Name: | kCDXProp_ReactionStep_Atom_Map_Manual |
| CDX Constant Value: | 0x0C07 |
| Data Size: | [CDXObjectIDArray](/web/20160912170303/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_ReactionStep](/web/20160912170303/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ReactionStep.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**

Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to a product atom.

The combination of this property and [kCDXProp_ReactionStep_Atom_Map_Auto](ReactionStep_Atom_Map_Auto.md) is redundant with the property and [kCDXProp_ReactionStep_Atom_Map](ReactionStep_Atom_Map.md). A given mapping listed in this property may also be listed in kCDXProp_ReactionStep_Atom_Map. Programs should choose which fields to read based on which information they are interested in.

**If this property is absent:**

This Reaction Step has no atom-atom mapping.

---

[CDX Documentation index](/web/20160912170303/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)