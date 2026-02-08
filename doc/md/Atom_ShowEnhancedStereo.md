## Atom_ShowEnhancedStereo Property

| CDXML Name: | ShowAtomEnhancedStereo |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_ShowEnhancedStereo |
| CDX Constant Value: | 0x0445 |
| Data Size: | CDXBoolean |
| Property of objects: | kCDXObj_Node |
| First written/read in: | ChemDraw 10.0 |
| Required? | No |

**Description:**

Show the enhanced stereochemistry indicator if non-zero.

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the Document, the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet. If no information is found in the Style Sheet either, this value is treated as true (enhanced stereochemistry indicators are shown).

---

CDX Documentation index