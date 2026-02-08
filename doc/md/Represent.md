## Represents Property Object

| CDXML Name: | represent |
| --- | --- |
| CDX Constant Name: | kCDXProp_RepresentsProperty |
| CDX Constant Value: | 0x000e |
| Contained by objects: | [kCDXObj_Graphic](Graphic.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**

A Represents Property object is used to indicate that its containing object has chemical meaning that is also represented in another object. This is useful, for example, when a circle-minus [Graphic](Graphic.md) object is located physically near a [Node](Node.md) object. A program saving the file might indicate the negative charge directly as a property of the node. But the circle-minus charge object still must be saved to preserve its absolute positioning. The Represents Property object is saved as a subject of the graphic, indicating (in this case) that the circle-minus graphical object *represents* the charge *property* of the node. Seeing this property, a program reading the file should not attempt to reinterpret the circle-minus charge object and add a second negative charge to the already-negative node.

Both properties of this object are required, but it has no required objects.

**Subobjects:**
*(none)*

**Properties:**

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [object](properties/object.md) | [CDXObjectID](DataType/CDXObjectID.md) |
|  | Required for RepresentsProperty.The object that has a property that is represented graphically by the containing object. |  |  |
| n/a | n/a | [attribute](properties/attribute.md) | [INT16](DataType/CDXNumeric.md) |
|  | Required for RepresentsProperty.The property of another object that is represented graphically by the containing object. |  |  |

---

[CDX Documentation index](index.md)