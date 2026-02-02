CDX Format Specification: Embedded Object Object
## Embedded Object Object

| CDXML Name: | embeddedobject |
| --- | --- |
| CDX Constant Name: | kCDXObj_EmbeddedObject |
| CDX Constant Value: | 0x8009 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

The EmbeddedObject object is, as its name implies, an externally-generated object embedded in the file. It may be a Macintosh PICT, a Windows Metafile, or a Windows OLE Object.

The bounding box property is required, but there are no required objects.

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
| 0x000A | [kCDXProp_ZOrder](properties/ZOrder.md) | Z | [INT16](DataType/CDXNumeric.md) |
|  | Back-to-front ordering index in 2D drawing. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | Required for pictures and spectra. Required for graphics and text until 6.0.The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0205 | [kCDXProp_RotationAngle](properties/RotationAngle.md) | RotationAngle | [INT32](DataType/CDXNumeric.md) |
|  | The angular orientation of an object in degrees * 65536. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0A60 | [kCDXProp_Picture_Edition](properties/Picture_Edition.md) | Edition | [Unformatted](DataType/Unformatted.md) |
|  | The section information (SectionHandle) of the Macintosh Publish & Subscribe edition embedded in the CDX picture object. |  |  |
| 0x0A61 | [kCDXProp_Picture_EditionAlias](properties/Picture_EditionAlias.md) | EditionAlias | [Unformatted](DataType/Unformatted.md) |
|  | The alias information of the Macintosh Publish & Subscribe edition embedded in the CDX picture object. |  |  |
| 0x0A62 | [kCDXProp_MacPICT](properties/MacPICT.md) | MacPICT | [Unformatted](DataType/Unformatted.md) |
|  | A Macintosh PICT data object. |  |  |
| 0x0A63 | [kCDXProp_WindowsMetafile](properties/WindowsMetafile.md) | WindowsMetafile | [Unformatted](DataType/Unformatted.md) |
|  | A Microsoft Windows Metafile object. |  |  |
| 0x0A64 | [kCDXProp_OLEObject](properties/OLEObject.md) | OLEObject | [Unformatted](DataType/Unformatted.md) |
|  | An OLE object. |  |  |
| 0x0A65 | [kCDXProp_EnhancedMetafile](properties/EnhancedMetafile.md) | EnhancedMetafile | [Unformatted](DataType/Unformatted.md) |
|  | A Microsoft Windows Enhanced Metafile object. |  |  |
| 0x0A6E | [kCDXProp_GIF](properties/GIF.md) | GIF | [Unformatted](DataType/Unformatted.md) |
|  | A binary GIF data object. |  |  |
| 0x0A6F | [kCDXProp_TIFF](properties/TIFF.md) | TIFF | [Unformatted](DataType/Unformatted.md) |
|  | A binary TIFF data object. |  |  |
| 0x0A70 | [kCDXProp_PNG](properties/PNG.md) | PNG | [Unformatted](DataType/Unformatted.md) |
|  | A binary PNG data object. |  |  |
| 0x0A71 | [kCDXProp_JPEG](properties/JPEG.md) | JPEG | [Unformatted](DataType/Unformatted.md) |
|  | A binary JPEG data object. |  |  |
| 0x0A72 | [kCDXProp_BMP](properties/BMP.md) | BMP | [Unformatted](DataType/Unformatted.md) |
|  | A binary BMP data object. |  |  |

---

[CDX Documentation index](index.md)