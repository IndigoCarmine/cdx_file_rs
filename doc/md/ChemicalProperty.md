CDX Format Specification: Chemical Property Object
## Chemical Property Object

| CDXML Name: | chemicalproperty |
| --- | --- |
| CDX Constant Name: | kCDXObj_ChemicalProperty |
| CDX Constant Value: | 0x8026 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 9.0 |

**Description:**  

This object ties together a collection of objects and a single 'display' object. Some physical property, such as a chemical name, might be derived from the collection, and then displayed. A Chemical Property object links the collection with the display of the property. A program (such as ChemDraw) that implements support for the Chemical Property object might then automatically update the display object based on changes to the collection.

**Subobjects:**  
*(none)*

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0008 | [kCDXProp_Name](properties/Name.md) | Name | [CDXString](DataType/CDXString.md) |
|  | Required for objecttags.Name of an object. |  |  |
| 0x0B82 | [kCDXProp_BasisObjects](properties/BasisObjects.md) | BasisObjects | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Required for geometries and constraints.An ordered list of objects used to define this object. |  |  |
| 0x0BB0 | [kCDXProp_ChemicalPropertyType](properties/ChemicalPropertyType.md) | ChemicalPropertyType | [UINT32](DataType/CDXNumeric.md) |
|  | The type of property (name, formula, molecular weight, etc.).
This is an enumerated property. |  |  |
| 0x0BB1 | [kCDXProp_ChemicalPropertyDisplayID](properties/ChemicalPropertyDisplayID.md) | ChemicalPropertyDisplayID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | The ID of a graphical object used to display the property value. |  |  |
| 0x0BB2 | [kCDXProp_ChemicalPropertyIsActive](properties/ChemicalPropertyIsActive.md) | ChemicalPropertyIsActive | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Whether the property should be recalculated in response to changes in the basis objects. |  |  |
| 0x0D06 | [kCDXProp_Positioning](properties/Positioning.md) | PositioningType | [INT8](DataType/CDXNumeric.md) |
|  | How the object should be positioned with respect to its containing object.
This is an enumerated property. |  |  |
| 0x0D07 | [kCDXProp_PositioningAngle](properties/PositioningAngle.md) | PositioningAngle | [INT32](DataType/CDXNumeric.md) |
|  | Angular positioning, in degrees * 65536. |  |  |
| 0x0D08 | [kCDXProp_PositioningOffset](properties/PositioningOffset.md) | PositioningOffset | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Offset positioning. |  |  |

---

[CDX Documentation index](index.md)