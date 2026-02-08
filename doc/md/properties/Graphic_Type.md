## Graphic_Type Property

| CDXML Name: | GraphicType |
| --- | --- |
| CDX Constant Name: | kCDXProp_Graphic_Type |
| CDX Constant Value: | 0x0A00 |
| Data Size: | [INT16](/web/20190326221703/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190326221703/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The type of graphical object.

In CDX files produced by ChemDraw 8.0, this property was mistakenly written as a 1-byte INT8 value. When reading CDX files, if the size of this property is found to be 1 byte instead of the expected 2 bytes, the actual value should be interpreted as actualType = (INT16)savedType. CDX format interpreters that follow the [best practices for reading integer values from CDX files](/web/20190326221703/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) will handle this error automatically. ChemDraw returned to writing INT16 values starting with version 8.0.6.

[Graphic](/web/20190326221703/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) objects are the only objects whose [kCDXProp_BoundingBox](BoundingBox.md) property has a special meaning, representing a pair of points rather than a rectangle. The meaning of those two points in the context of each graphic type is shown below.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Undefined | Undefined |
| 1 | Line | Line ([kCDXProp_Line_Type](Line_Type.md) should be present. [kCDXProp_Arrow_Type](Arrow_Type.md) may also be present.) The two points stored within the [kCDXProp_BoundingBox](BoundingBox.md) represent the start and end of the line. |
| 2 | Arc | Arc ([kCDXProp_Line_Type](Line_Type.md) should be present. [kCDXProp_Arc_AngularSize](Arc_AngularSize.md) should be present. [kCDXProp_Arrow_Type](Arrow_Type.md) may also be present.) The two points stored within the [kCDXProp_BoundingBox](BoundingBox.md) represent the center and end of the arc. |
| 3 | Rectangle | Rectangle ([kCDXProp_Rectangle_Type](Rectangle_Type.md) should be present) The two points stored within the [kCDXProp_BoundingBox](BoundingBox.md) represent two opposing corners of the rectangle. |
| 4 | Oval | Oval ([kCDXProp_Oval_Type](Oval_Type.md) should be present) The two points stored within the [kCDXProp_BoundingBox](BoundingBox.md) represent the center and semimajor end of the oval. |
| 5 | Orbital | Orbital ([kCDXProp_Orbital_Type](Orbital_Type.md) should be present) The two points stored within the [kCDXProp_BoundingBox](BoundingBox.md) represent the center and end of the orbital. |
| 6 | Bracket | Bracket ([kCDXProp_Bracket_Type](Bracket_Type.md) should be present) The two points stored within the [kCDXProp_BoundingBox](BoundingBox.md) represent the two ends of the bracket. |
| 7 | Symbol | Symbol ([kCDXProp_Symbol_Type](Symbol_Type.md) should be present) The two points stored within the [kCDXProp_BoundingBox](BoundingBox.md) represent the center of the symbol and a second point indicating the symbol's size. |

**If this property is absent:**

The graphic is treated as having Undefined type. This is not recommended.

---

[CDX Documentation index](/web/20190326221703/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)