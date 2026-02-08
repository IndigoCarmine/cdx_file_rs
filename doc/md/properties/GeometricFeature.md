## GeometricFeature Property

| CDXML Name: | GeometricFeature |
| --- | --- |
| CDX Constant Name: | kCDXProp_GeometricFeature |
| CDX Constant Value: | 0x0B80 |
| Data Size: | [INT8](/web/20160911223252/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Geometry](/web/20160911223252/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**

The type of the geometrical feature (point, line, plane, etc.).

For the purpose of Geometry objects based on other objects:

- A **point** is defined as a [Node](/web/20160911223252/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) or a Geometry object with a Geometric Feature property equal to `kCDXGeometricFeature_PointFromPointPointDistance`, `kCDXGeometricFeature_PointFromPointPointPercentage`, `kCDXGeometricFeature_PointFromPointNormalDistance`, or `kCDXGeometricFeature_CentroidFromPoints`.
- A **line** is defined as a Geometry object with a Geometric Feature property equal to `kCDXGeometricFeature_LineFromPoints` or `kCDXGeometricFeature_NormalFromPointPlane`.
- A **plane** is defined as a Geometry object with a Geometric Feature property equal to `kCDXGeometricFeature_PlaneFromPoints` or `kCDXGeometricFeature_PlaneFromPointLine`.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Undefined | Geometric feature type is undefined |
| 1 | PointFromPointPointDistance | A point defined by two points and a distance. [kCDXProp_BasisObjects](BasisObjects.md) should be present and contain an ordered list of two other points. This new point will be positioned at a given [distance](RelationValue.md) (may be negative) from the first of those points in the direction of the second of those points. |
| 2 | PointFromPointPointPercentage | A point defined by two points and a percentage. [kCDXProp_BasisObjects](BasisObjects.md) should be present and contain an ordered list of two other points. This new point will be positioned at a given [percentage](RelationValue.md) of the distance (may be negative) from the first of those points in the direction of the second of those points. |
| 3 | PointFromPointNormalDistance | A point defined by a point, a normal line, and a distance. [kCDXProp_BasisObjects](BasisObjects.md) should be present and contain an unordered list consisting of a point and a normal. This new point will be positioned at a given [distance](RelationValue.md) (may be negative) from the point in the direction of the normal. |
| 4 | LineFromPoints | A best fit line defined by two or more points. [kCDXProp_BasisObjects](BasisObjects.md) should be present and contain an unordered list consisting of at least two points. If more than two points are used to define this line, a [maximum RMS deviation](RelationValue.md) may also be specified. |
| 5 | PlaneFromPoints | A best fit plane defined by three or more points. [kCDXProp_BasisObjects](BasisObjects.md) should be present and contain an unordered list consisting of at least three points. If more than three points are used to define this plane, a [maximum RMS deviation](RelationValue.md) may also be specified. |
| 6 | PlaneFromPointLine | A plane defined by a point and a line. [kCDXProp_BasisObjects](BasisObjects.md) should be present and contain an unordered list consisting of a point and a line. |
| 7 | CentroidFromPoints | A centroid defined by points. [kCDXProp_BasisObjects](BasisObjects.md) should be present and contain an unordered list consisting of at least one point. |
| 8 | NormalFromPointPlane | A normal line defined by a point and a plane. [kCDXProp_BasisObjects](BasisObjects.md) should be present and contain an unordered list consisting of a point and a plane. |

**If this property is absent:**

The type will be treated as Undefined.

---

[CDX Documentation index](/web/20160911223252/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)