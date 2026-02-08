## TIFF Property

| CDXML Name: | TIFF |
| --- | --- |
| CDX Constant Name: | kCDXProp_TIFF |
| CDX Constant Value: | 0x0A6F |
| Data Size: | [Unformatted](/web/20160912170142/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/Unformatted.md) |
| Property of objects: | [kCDXObj_EmbeddedObject](/web/20160912170142/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md) |
| First written/read in: | ChemDraw 9.0 |
| Required? | No |

**Description:**

A binary TIFF data object.

**If this property is absent:**

The various types of [Embedded Objects](/web/20160912170142/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md) are independent. There should be at least one of the [kCDXProp_Picture_Edition](Picture_Edition.md), [kCDXProp_MacPICT](MacPICT.md), [kCDXProp_OLEObject](OLEObject.md), [kCDXProp_EnhancedMetafile](EnhancedMetafile.md), [kCDXProp_WindowsMetafile](WindowsMetafile.md), [kCDXProp_GIF](GIF.md), [kCDXProp_TIFF](TIFF.md), [kCDXProp_PNG](PNG.md), [kCDXProp_JPEG](JPEG.md), and [kCDXProp_BMP](BMP.md) properties present. If none are present (or the only one present is unreadable on a given platform or version), the object displays as an empty box.

---

[CDX Documentation index](/web/20160912170142/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)