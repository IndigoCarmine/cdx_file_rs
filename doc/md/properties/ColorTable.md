CDX Format Specification: ColorTable Property
## ColorTable Property

| CDXML Name: | colortable |
| --- | --- |
| CDX Constant Name: | kCDXProp_ColorTable |
| CDX Constant Value: | 0x0300 |
| Data Size: | [CDXColorTable](/web/20190326231030/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXColorTable.md) |
| Property of objects: | [kCDXObj_Document](/web/20190326231030/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**  

The color palette used throughout the document.

Color indexes 0 and 1 always correspond to black and white and are not saved in the color table. The first and second RGB values (color indexes 2 and 3) are the default background and foreground colors.

This property is used only in CDX files. In CDXML files, a [Color Table](/web/20190326231030/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ColorTable.md) object is saved instead.

**If this property is absent:**  

If no color table is present in the object or any of it's ancestors, only the standard color indexes 0 through 3 may be used. In that case, the background color (2) is assumed to be white, and the foreground color
(3) is assumed to be black.

---

[CDX Documentation index](/web/20190326231030/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)