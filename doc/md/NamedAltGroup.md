CDX Format Specification: Named Alternative Group Object
## Named Alternative Group Object

| CDXML Name: | altgroup |
| --- | --- |
| CDX Constant Name: | kCDXObj_NamedAlternativeGroup |
| CDX Constant Value: | 0x800A |
| Contained by objects: | [kCDXObj_Page](Page.md),[kCDXObj_Group](Group.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

A NamedAlternativeGroup is a container object holding fragments that represent alternative substituents for a query. It should contain two or more fragments and/or groups containing fragments. The optional property [kCDXProp_NamedAlternativeGroup_Valence](properties/NamedAlternativeGroup_Valence.md) (1 if not present) describes the number of external connection points. Each fragment or group must contain exactly that number of external connection point nodes. Generally, there will be a [Node](Node.md) in another fragment not contained in this NamedAlternativeGroup which has Node_Type equal to kCDXNodeType_NamedAlternativeGroup, and a kCDXProp_Node_AltGroupID property that refers to this NamedAlternativeGroup object.
  

The number of bonds connected to a named alternative group node should match the valence (number of external connection points in each fragment) of the alternative group. If a named alternative group node has more than one bond connected to it, the named alternative group node should contain a kCDXProp_BondOrdering property.

A NamedAlternativeGroup is also commonly known as an R-Group or a G-Group.

A NamedAlternativeGroup must contain a [Text](Text.md) object, and a NamedAlternativeGroup without any contained fragments is pretty useless. It has no required properties.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8002 | [kCDXObj_Group](Group.md) | group |  |
|  | A logical collection of objects. |  |  |
| 0x8003 | [kCDXObj_Fragment](Fragment.md) | fragment |  |
|  | A collection of nodes and their connectivity (bonds). |  |  |
| 0x8006 | [kCDXObj_Text](Text.md) | t |  |
|  | An arbitrary block of (possibly styled) text. |  |  |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x000A | [kCDXProp_ZOrder](properties/ZOrder.md) | Z | [INT16](DataType/CDXNumeric.md) |
|  | Back-to-front ordering index in 2D drawing. |  |  |
| 0x000F | [kCDXProp_IgnoreWarnings](properties/IgnoreWarnings.md) | IgnoreWarnings | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether chemical warnings should be suppressed on this object. |  |  |
| 0x0010 | [kCDXProp_ChemicalWarning](properties/ChemicalWarning.md) | Warning | [CDXString](DataType/CDXString.md) |
|  | A warning concerning possible chemical problems with this object. |  |  |
| 0x0011 | [kCDXProp_Visible](properties/Visible.md) | Visible | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The object is visible if non-zero. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0B00 | [kCDXProp_NamedAlternativeGroup_TextFrame](properties/NamedAlternativeGroup_TextFrame.md) | TextFrame | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The bounding box of upper portion of the Named Alternative Group, containing the name of the group. |  |  |
| 0x0B01 | [kCDXProp_NamedAlternativeGroup_GroupFrame](properties/NamedAlternativeGroup_GroupFrame.md) | GroupFrame | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The bounding box of the lower portion of the Named Alternative Group, containing the definition of the group. |  |  |
| 0x0B02 | [kCDXProp_NamedAlternativeGroup_Valence](properties/NamedAlternativeGroup_Valence.md) | Valence | [INT16](DataType/CDXNumeric.md) |
|  | The number of attachment points in each alternative in a named alternative group. |  |  |

---

[CDX Documentation index](index.md)