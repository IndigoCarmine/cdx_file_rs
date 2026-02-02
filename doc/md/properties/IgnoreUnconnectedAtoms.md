CDX Format Specification: IgnoreUnconnectedAtoms Property
## IgnoreUnconnectedAtoms Property

| CDXML Name: | IgnoreUnconnectedAtoms |
| --- | --- |
| CDX Constant Name: | kCDXProp_IgnoreUnconnectedAtoms |
| CDX Constant Value: | 0x0B86 |
| Data Size: | [CDXBooleanImplied](/web/20160912002257/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Constraint](/web/20160912002257/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md) |
| First written/read in: | (not written/read) |
| Required? | No |

**Description:**  

Signifies whether unconnected atoms should be ignored within the exclusion sphere.

This property is meaningful only for constraints with a [kCDXProp_ConstraintType](ConstraintType.md) of type ExclusionSphere.

**If this property is absent:**  

Unconnected atoms will not be ignored.

---

[CDX Documentation index](/web/20160912002257/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)