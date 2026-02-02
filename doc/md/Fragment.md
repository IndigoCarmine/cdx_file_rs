CDX Format Specification: Fragment Object
## Fragment Object

| CDXML Name: | fragment |
| --- | --- |
| CDX Constant Name: | kCDXObj_Fragment |
| CDX Constant Value: | 0x8003 |
| Contained by objects: | [kCDXObj_Page](Page.md),[kCDXObj_Group](Group.md),[kCDXObj_Node](Node.md),[kCDXObj_NamedAlternativeGroup](NamedAltGroup.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

Fragment object is a collection of nodes and their connectivity (bonds). Generally, all nodes within a fragment will be connected, but this is not strictly guaranteed. For example, a cyclopentadienyl anion might be represented as a single fragment containing five nodes, five bonds, a curve (an ellipse, to represent delocalization), and a graphic (a circled minus charge, to represent the delocalized charge).

Unlike with [Group](Group.md) objects, Fragment objects are guaranteed to be chemically meaningful.

A Fragment has no required objects or properties, but a Fragment without any objects is pretty useless.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8004 | [kCDXObj_Node](Node.md) | n |  |
|  | The basic building block of chemical objects, usually referring to a single atom |  |  |
| 0x8005 | [kCDXObj_Bond](Bond.md) | b |  |
|  | A connection between two Node objects. |  |  |
| 0x8007 | [kCDXObj_Graphic](Graphic.md) | graphic |  |
|  | A (generally non-chemical) graphic object such as a line, arc, circle, or rectangle. |  |  |
| 0x8008 | [kCDXObj_Curve](Curve.md) | curve |  |
|  | A Bézier curve. |  |  |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0500 | [kCDXProp_Mole_Racemic](properties/Mole_Racemic.md) | Racemic | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Indicates that the molecule is a racemic mixture. |  |  |
| 0x0501 | [kCDXProp_Mole_Absolute](properties/Mole_Absolute.md) | Absolute | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Indicates that the molecule has known absolute configuration. |  |  |
| 0x0502 | [kCDXProp_Mole_Relative](properties/Mole_Relative.md) | Relative | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Indicates that the molecule has known relative stereochemistry, but unknown absolute configuration. |  |  |
| 0x0503 | [kCDXProp_Mole_Formula](properties/Mole_Formula.md) | Formula | [CDXFormula](DataType/CDXFormula.md) |
|  | The molecular formula representation of a molecule object. |  |  |
| 0x0504 | [kCDXProp_Mole_Weight](properties/Mole_Weight.md) | Weight | [FLOAT64](DataType/CDXNumeric.md) |
|  | The average molecular weight of a molecule object. |  |  |
| 0x0505 | [kCDXProp_Frag_ConnectionOrder](properties/Frag_ConnectionOrder.md) | ConnectionOrder | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of attachment points within a fragment. |  |  |

---

[CDX Documentation index](index.md)