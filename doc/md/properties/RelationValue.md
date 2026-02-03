CDX Format Specification: RelationValue Property
## RelationValue Property

| CDXML Name: | RelationValue |
| --- | --- |
| CDX Constant Name: | kCDXProp_RelationValue |
| CDX Constant Value: | 0x0B81 |
| Data Size: | [FLOAT64](/web/20160912060941/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Geometry](/web/20160912060941/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**  

The numeric relationship (if any) among the basis objects used to define this object.

For [Geometry](/web/20160912060941/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md) objects with a [Geometric Feature](GeometricFeature.md) type of kCDXGeometricFeature_PointFromPointPointDistance or kCDXGeometricFeature_PointFromPointNormalDistance, this value represents the distance.

For those with a Geometric Feature type of kCDXGeometricFeature_PointFromPointPointPercentage, this value represents the percentage.

For those with a Geometric Feature type of kCDXGeometricFeature_LineFromPoints or kCDXGeometricFeature_PlaneFromPoints, this value represents the maximum RMS deviation for the line or plane.

**If this property is absent:**  

The value will be treated as zero.

---

[CDX Documentation index](/web/20160912060941/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)