## Group_Integral Property

| CDXML Name: | Integral |
| --- | --- |
| CDX Constant Name: | kCDXProp_Group_Integral |
| CDX Constant Value: | 0x1100 |
| Data Size: | [CDXBoolean](/web/20170424055106/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Group](/web/20170424055106/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Group.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

The group is considered to be integral (non-subdivisible) if non-zero.

An Integral [Group](/web/20170424055106/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Group.md) is always treated as a single, non-subdivisible object for the purposes of interactions with the user. Selection of any submember of the group will cause all group members to become selected.

**If this property is absent:**

The group is not considered to be integral.

---

[CDX Documentation index](/web/20170424055106/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)