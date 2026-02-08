## Atom_EnhancedStereoGroupNum Property

| CDXML Name: | EnhancedStereoGroupNum |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_EnhancedStereoGroupNum |
| CDX Constant Value: | 0x0447 |
| Data Size: | [UINT16](/web/20190327001551/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327001551/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 10.0 |
| Required? | No |

**Description:**

The group number associated with Or and And enhanced stereochemistry types.

This property is meaningful only for nodes with a [kCDXProp_Atom_EnhancedStereoType](Atom_EnhancedStereoType.md) of type Or or And. It is required for such nodes and must contain a non-negative integer value.

**If this property is absent:**

The node will be assumed to be in the first And or Or group ("or1" or "&1&qout;").

---

[CDX Documentation index](/web/20190327001551/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)