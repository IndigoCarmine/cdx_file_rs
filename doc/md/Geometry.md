## Object Tag Object

| CDXML Name: | objecttag |
| --- | --- |
| CDX Constant Name: | kCDXObj_ObjectTag |
| CDX Constant Value: | 0x8011 |
| Contained by objects: | [kCDXObj_Group](Group.md),[kCDXObj_Fragment](Fragment.md),[kCDXObj_Node](Node.md),[kCDXObj_Bond](Bond.md),[kCDXObj_Text](Text.md),[kCDXObj_Graphic](Graphic.md),[kCDXObj_Curve](Curve.md),[kCDXObj_EmbeddedObject](EmbeddedObject.md),[kCDXObj_Table](Table.md),[kCDXObj_NamedAlternativeGroup](NamedAltGroup.md),[kCDXObj_Spectrum](Spectrum.md),[kCDXObj_Geometry](Geometry.md),[kCDXObj_Constraint](Constraint.md),[kCDXObj_TLCPlate](TLCPlate.md),[kCDXObj_TLCLane](TLCLane.md),[kCDXObj_TLCSpot](TLCSpot.md) |
| First written/read in: | ChemDraw 6.0 |

**Description:**

Arbitrarily named property, one or more of which can be attached to any ChemDraw object. This object can be used to associate arbitrary data with an object.

An Object Tag may contain a [Text](Text.md) object. If present, the Text object will be displayed in the document according to the state of its [kCDXProp_Visible](properties/Visible.md) property. Object Tags of this type might be used as a visual display for some other property, such as a Node's or Bond's stereochemistry indicator.

Object tag names are critical to their management, since the name tells ChemDraw how to interpret the tag. All tags assigned internally by ChemDraw begin with the character sequence "/CS/CD/". Tags created for user purposes should begin with some other character sequence. Any ChemDraw object can have any number of tags attached to it, including multiple tags with the same name.

Spectrum-structure assignment is done by assigning an object tag with the name "/CS/CD/assign" to one or more atoms and a spectrum. The value of the tag is a string consisting of one or more ranges of x-axis values. The x-axis ranges define one or more bands or lines in the spectrum. The atoms containing a particular assignment are assigned to the bands specified. For example, in an IR spectrum, two atoms and the spectrum might all contain an object tag with the value "905.7-944.35,1123-1200" and a object tag dictionary index pointing to an object tag dictionary entry with the name "/CS/CD/assign" and the tag type kCDXObjectTagType_String. This will create an assignment between those two atoms and the bands lying between 905.7 and 944.35 cm-1, and 1123 and 1200 cm-1.

Although ChemDraw recognizes the types "unknown", "query" (for query indicators), "stereo" (for stereochemistry indicators), and "number" (for atom numbers), any sequence of bytes is a valid name. For unknown and user-defined types that contain a Text object, ChemDraw will faithfully display the tag, but no significance will be ascribed to the tag's text.

Object Tags require a Name, but have no required objects.

**Subobjects:**

| Value | Name | CDXML Name |
| --- | --- | --- |
| 0x8006 | [kCDXObj_Text](Text.md) | t |
|  | An arbitrary block of (possibly styled) text. |  |

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0008 | [kCDXProp_Name](properties/Name.md) | Name | [CDXString](DataType/CDXString.md) |
|  | Required for objecttags. Name of an object. |  |  |
| 0x0D00 | [kCDXProp_ObjectTag_Type](properties/ObjectTag_Type.md) | TagType | [INT16](DataType/CDXNumeric.md) |
|  | The tag's data type. This is an enumerated property. |  |  |
| 0x0D03 | [kCDXProp_ObjectTag_Tracking](properties/ObjectTag_Tracking.md) | Tracking | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The tag will participate in tracking if non-zero. |  |  |
| 0x0D04 | [kCDXProp_ObjectTag_Persistent](properties/ObjectTag_Persistent.md) | Persistent | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The tag will be resaved to a CDX file if non-zero. |  |  |
| 0x0D05 | [kCDXProp_ObjectTag_Value](properties/ObjectTag_Value.md) | Value | varies |
|  | The value is a INT32, FLOAT64 or unformatted string depending on the value of ObjectTag_Type. |  |  |
| 0x0D06 | [kCDXProp_Positioning](properties/Positioning.md) | PositioningType | [INT8](DataType/CDXNumeric.md) |
|  | How the object should be positioned with respect to its containing object. This is an enumerated property. |  |  |
| 0x0D07 | [kCDXProp_PositioningAngle](properties/PositioningAngle.md) | PositioningAngle | [INT32](DataType/CDXNumeric.md) |
|  | Angular positioning, in degrees * 65536. |  |  |
| 0x0D08 | [kCDXProp_PositioningOffset](properties/PositioningOffset.md) | PositioningOffset | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Offset positioning. |  |  |

---

[CDX Documentation index](index.md)
