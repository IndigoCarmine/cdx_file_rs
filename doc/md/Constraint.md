CDX Format Specification: Constraint Object
## Constraint Object

| CDXML Name: | constraint |
| --- | --- |
| CDX Constant Name: | kCDXObj_Constraint |
| CDX Constant Value: | 0x8022 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 8.0 |

**Description:**  

A distance or angle constraint between one or more objects. The type of the relationship is specified by the [kCDXProp_ConstraintType](properties/ConstraintType.md) property, and the objects that specify the constraint are listed within the [kCDXProp_BasisObjects](properties/BasisObjects.md) property.

If present, an [Object Tag](ObjectTag.md) with the name "distance" will contain a graphic representation of the acceptable values for a Constraint with a [kCDXProp_ConstraintType](properties/ConstraintType.md) equal to kCDXConstraintType_Distance or kCDXConstraintType_ExclusionSphere. Similarly, an [Object Tag](ObjectTag.md) with the name "angle", if present will contain a graphic representation of the acceptable values for a Constraint with a [kCDXProp_ConstraintType](properties/ConstraintType.md) equal to kCDXConstraintType_Angle.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0008 | [kCDXProp_Name](properties/Name.md) | Name | [CDXString](DataType/CDXString.md) |
|  | Required for objecttags.Name of an object. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0805 | [kCDXProp_BondLength](properties/BondLength.md) | BondLength | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default bond length. |  |  |
| 0x0807 | [kCDXProp_LineWidth](properties/LineWidth.md) | LineWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default line width. |  |  |
| 0x0809 | [kCDXProp_HashSpacing](properties/HashSpacing.md) | HashSpacing | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default spacing between hashed lines used in wedged hashed bonds. |  |  |
| 0x081A | [kCDXProp_LabelStyleFont](properties/LabelStyleFont.md) | LabelFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font family for atom labels. |  |  |
| 0x081C | [kCDXProp_LabelStyleSize](properties/LabelStyleSize.md) | LabelSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for atom labels. |  |  |
| 0x081E | [kCDXProp_LabelStyleFace](properties/LabelStyleFace.md) | LabelFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for atom labels. |  |  |
| 0x0820 | [kCDXProp_LabelStyleColor](properties/LabelStyleColor.md) | LabelColor | [INT16](DataType/CDXNumeric.md) |
|  | The default color for atom labels |  |  |
| 0x0B82 | [kCDXProp_BasisObjects](properties/BasisObjects.md) | BasisObjects | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Required for geometries and constraints.An ordered list of objects used to define this object. |  |  |
| 0x0B83 | [kCDXProp_ConstraintType](properties/ConstraintType.md) | ConstraintType | [INT8](DataType/CDXNumeric.md) |
|  | The constraint type (distance, angle, or exclusion sphere).
This is an enumerated property. |  |  |
| 0x0B84 | [kCDXProp_ConstraintMin](properties/ConstraintMin.md) | ConstraintMin | [FLOAT64](DataType/CDXNumeric.md) |
|  | The minimum value of the constraint. |  |  |
| 0x0B85 | [kCDXProp_ConstraintMax](properties/ConstraintMax.md) | ConstraintMax | [FLOAT64](DataType/CDXNumeric.md) |
|  | The maximum value of the constraint. |  |  |
| 0x0B86 | [kCDXProp_IgnoreUnconnectedAtoms](properties/IgnoreUnconnectedAtoms.md) | IgnoreUnconnectedAtoms | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether unconnected atoms should be ignored within the exclusion sphere. |  |  |
| 0x0B87 | [kCDXProp_DihedralIsChiral](properties/DihedralIsChiral.md) | DihedralIsChiral | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether a dihedral is signed or unsigned. |  |  |

---

[CDX Documentation index](index.md)