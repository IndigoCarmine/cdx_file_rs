CDX Format Specification: TLC Spot Object
## TLC Spot Object

| CDXML Name: | tlcspot |
| --- | --- |
| CDX Constant Name: | kCDXObj_TLCSpot |
| CDX Constant Value: | 0x8025 |
| Contained by objects: | [kCDXObj_TLCLane](TLCLane.md) |
| First written/read in: | ChemDraw 8.0 |

**Description:**  

TLC Lane objects technically have no required properties or objects, but non-zero values for [kCDXProp_Height](properties/Height.md) and [kCDXProp_Width](properties/Width.md) would probably be helpful.

The [kCDXProp_Height](properties/Height.md), [kCDXProp_Width](properties/Width.md), and [kCDXProp_TLC_Tail](properties/TLC_Tail.md) distances refer to the unrotated reference frame. If the spot's parent [TLC Plate](TLCPlate.md) is rotated 90°, the Height value will actuall refer to a horizontal distance, and so on.

Spots are displayed as modified ellipsoids. If Height == Width and Tail == 0, then the spot is circular. With Height != Width and Tail == 0, the spot is elliptical. If the Tail value is non-zero, the top half of the spot is drawn as a semiellipse with the height = Height and width = Width, and the bottom half of the spot is draw as a second semiellipse with height = (Height + 2 * Tail) and width = Width.

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
| 0x0011 | [kCDXProp_Visible](properties/Visible.md) | Visible | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The object is visible if non-zero. |  |  |
| 0x0812 | [kCDXProp_Width](properties/Width.md) | Width | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The width of an object in CDX coordinate units, possibly in a rotated or skewed frame. |  |  |
| 0x0813 | [kCDXProp_Height](properties/Height.md) | Height | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The height of an object in CDX coordinate units, possibly in a rotated or skewed frame. |  |  |
| 0x0A08 | [kCDXProp_Curve_Type](properties/Curve_Type.md) | CurveType | [INT16](DataType/CDXNumeric.md) |
|  | The type of curve object.
This is a bit-encoded property. |  |  |
| 0x0AB0 | [kCDXProp_TLC_Rf](properties/TLC_Rf.md) | Rf | [FLOAT64](DataType/CDXNumeric.md) |
|  | The Retention Factor (Rf) of an individual spot. |  |  |
| 0x0AB1 | [kCDXProp_TLC_Tail](properties/TLC_Tail.md) | Tail | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The length of the "tail" of an individual spot. |  |  |
| 0x0AB2 | [kCDXProp_TLC_ShowRf](properties/TLC_ShowRf.md) | ShowRf | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the spot's Retention Fraction (Rf) value if non-zero.. |  |  |

---

[CDX Documentation index](index.md)