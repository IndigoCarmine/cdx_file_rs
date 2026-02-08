## Atom_AbnormalValence Property

| CDXML Name: | AbnormalValence |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_AbnormalValence |
| CDX Constant Value: | 0x0429 |
| Data Size: | CDXBooleanImplied |
| Property of objects: | kCDXObj_Node |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

Signifies that an abnormal valence for an atom is permitted.

This attribute indicates that valence checks should be bypassed for this atom, and that the valence should be computed from the bonds actually drawn. The definition of what valence is normal may vary from application to application, so CDX files must contain explicit information about what was drawn or computed, vs. what was inferred.

**If this property is absent:**

Abnormal valence is not permitted. An atom drawn with other-than-normal valence should be considered to be in error.

---

CDX Documentation index