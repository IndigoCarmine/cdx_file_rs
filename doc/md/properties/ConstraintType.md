CDX Format Specification: ConstraintType Property
## ConstraintType Property

| CDXML Name: | ConstraintType |
| --- | --- |
| CDX Constant Name: | kCDXProp_ConstraintType |
| CDX Constant Value: | 0x0B83 |
| Data Size: | [INT8](/web/20160912060600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Constraint](/web/20160912060600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**  

The constraint type (distance, angle, or exclusion sphere).

For the purpose of Constraint objects based on other objects:

- A **point** is defined as a [Node](/web/20160912060600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) or a [Geometry](/web/20160912060600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md) object with a Geometric Feature property equal to kCDXGeometricFeature_PointFromPointPointDistance, kCDXGeometricFeature_PointFromPointPointPercentage, kCDXGeometricFeature_PointFromPointNormalDistance, or kCDXGeometricFeature_CentroidFromPoints
- A **line** is defined as a [Geometry](/web/20160912060600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md) object with a Geometric Feature property equal to kCDXGeometricFeature_LineFromPoints or kCDXGeometricFeature_NormalFromPointPlane
- A **plane** is defined as a [Geometry](/web/20160912060600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md) object with a Geometric Feature property equal to kCDXGeometricFeature_PlaneFromPoints or kCDXGeometricFeature_PlaneFromPointLine

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Undefined | The type of constraint is undefined. |
| 1 | Distance | The[kCDXProp_BasisObjects](BasisObjects.md)property should be present and contain an unordered list of two other objects: two points, a point and a line, or a point and a plane. The[kCDXProp_ConstraintMin](ConstraintMin.md)and[kCDXProp_ConstraintMax](ConstraintMax.md)properties will contain the minimum and maximum acceptable distances that define the constraint range (both values in Angstroms). |
| 2 | Angle | The[kCDXProp_BasisObjects](BasisObjects.md)property should be present and contain one of four set of objects: anorderedlist of three points (constraining the A-B-C angle), anorderedlist of four points (constraining the A-B-C-D dihedral angle), an unordered pair of lines (constraining the angle between the lines), or an unordered pair of planes (constraining the dihedral angle between the planes). The[kCDXProp_ConstraintMin](ConstraintMin.md)and[kCDXProp_ConstraintMax](ConstraintMax.md)properties contain the minimum and maximum acceptable angles that define the constraint range (both values in degrees). |
| 3 | ExclusionSphere | The[kCDXProp_BasisObjects](BasisObjects.md)property should be present and contain a list of at least one point. The Exclusion Sphere will be centered around the first point in the list. If additional points are present, those points are assumed to be allowed within the radious of the sphere. The[kCDXProp_ConstraintMin](ConstraintMin.md)and[kCDXProp_ConstraintMax](ConstraintMax.md)properties contain the minimum and maximum acceptable radii that define the exclusion sphere (both values in Angstroms). Under most normal cases, the kCDXProp_ConstraintMin for an exclusion sphere will be zero. A non-zero value would indicate a shell (hollow sphere of some thickness) rather than the usual solid sphere. |

**If this property is absent:**  

The type will be treated as Undefined.

---

[CDX Documentation index](/web/20160912060600/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)