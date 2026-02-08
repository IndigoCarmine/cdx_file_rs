## BasisObjects Property

| CDXML Name: | BasisObjects |
| --- | --- |
| CDX Constant Name: | kCDXProp_BasisObjects |
| CDX Constant Value: | 0x0B82 |
| Data Size: | [CDXObjectIDArray](/web/20160912000102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_Geometry](/web/20160912000102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md),[kCDXObj_Constraint](/web/20160912000102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md),[kCDXObj_ChemicalProperty](/web/20160912000102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ChemicalProperty.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | Required for geometries and constraints |

**Description:**

An ordered list of objects used to define this object.

The ordering of the objects may or may not actually be relevant. For example, if the objects consist of three points that define a best-fit plane, the ordering does not matter.

---

[CDX Documentation index](/web/20160912000102/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)