CDX Format Specification: ReactionStep_Atom_Map_Auto Property
## ReactionStep_Atom_Map_Auto Property

| CDXML Name: | ReactionStepAtomMapAuto |
| --- | --- |
| CDX Constant Name: | kCDXProp_ReactionStep_Atom_Map_Auto |
| CDX Constant Value: | 0x0C08 |
| Data Size: | [CDXObjectIDArray](/web/20160912170350/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_ReactionStep](/web/20160912170350/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ReactionStep.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**  

Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to to a product atom.

The combination of this property and [kCDXProp_ReactionStep_Atom_Map_Manual](ReactionStep_Atom_Map_Manual.md) is redundant with the property and [kCDXProp_ReactionStep_Atom_Map](ReactionStep_Atom_Map.md). A given mapping listed in this property may also be listed in kCDXProp_ReactionStep_Atom_Map. Programs should choose which fields to read based on which information they are interested in.

**If this property is absent:**  

This Reaction Step has no atom-atom mapping.

---

[CDX Documentation index](/web/20160912170350/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)