CDX Format Specification: Picture_Edition Property
## Picture_Edition Property

| CDXML Name: | Edition |
| --- | --- |
| CDX Constant Name: | kCDXProp_Picture_Edition |
| CDX Constant Value: | 0x0A60 |
| Data Size: | [Unformatted](/web/20160912060751/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/Unformatted.md) |
| Property of objects: | [kCDXObj_EmbeddedObject](/web/20160912060751/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The section information (SectionHandle) of the Macintosh Publish & Subscribe edition embedded in the CDX picture object.

It exists only on the Macintosh platform. Apple Computer, Inc. has remove support of Publish and Subscribe from recent versions of MacOS. This property will only be written or read in ChemDraw 6.0 and earlier.

**If this property is absent:**  

The various types of [Embedded Objects](/web/20160912060751/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md) are independent. There should be at least one of the [kCDXProp_Picture_Edition](Picture_Edition.md), [kCDXProp_MacPICT](MacPICT.md), [kCDXProp_OLEObject](OLEObject.md), [kCDXProp_EnhancedMetafile](EnhancedMetafile.md), [kCDXProp_WindowsMetafile](WindowsMetafile.md), [kCDXProp_GIF](GIF.md), [kCDXProp_TIFF](TIFF.md), [kCDXProp_PNG](PNG.md), [kCDXProp_JPEG](JPEG.md), and [kCDXProp_BMP](BMP.md) properties present. If none are present (or the only one present is unreadable on a given platform or version), the object displays as an empty box.

---

[CDX Documentation index](/web/20160912060751/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)