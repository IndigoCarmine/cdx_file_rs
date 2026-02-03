CDX Format Specification: MacPrintInfo Property
## MacPrintInfo Property

| CDXML Name: | MacPrintInfo |
| --- | --- |
| CDX Constant Name: | kCDXProp_MacPrintInfo |
| CDX Constant Value: | 0x0800 |
| Data Size: | [Unformatted](/web/20160912060925/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/Unformatted.md) |
| Property of objects: | [kCDXObj_Document](/web/20160912060925/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The 120 byte Macintosh TPrint data associated with the CDX document object. Refer to Macintosh Toolbox manual for detailed description.

This property is an excellent example of what *not* to do when designing a cross-platform file format. Because of the extreme nastiness involved in dealing with this data blob, it is our intent that it will become deprecated in some future version of ChemDraw. Until then, however...

This object contains a classic Macintosh TPrint record. It may be treated, nominally, as:

INT16 macPrint[60];

Most bytes within this structure are ignored by ChemDraw, and should be written as zeroes if this structure is created on a non-Macintosh platform. Important bytes are shown as follows:

| macPrint[2] | Vertical resolution, in dpi |
| --- | --- |
| macPrint[3] | Horizontal resolution, in dpi |
| macPrint[4] macPrint[5] macPrint[6] macPrint[7] | Coordinates of the top, left, bottom, and right edges, respectively, of the imageable page area, in the scale of the vertical and horizontal values above. |
| macPrint[8] macPrint[9] macPrint[10] macPrint[11] | Coordinates of the top, left, bottom, and right edges, respectively, of the physical paper area, in the scale of the vertical and horizontal values above. |
| macPrint[12] | Should be 0xFFFF for portrait-orientation pages, and 0xFFFD for landscape-orientation pages. |
| macPrint[13] | Height of the page, in the scale of the vertical and horizontal values above. |
| macPrint[14] | Width of the page, in the scale of the vertical and horizontal values above. |
| macPrint[24] | Horizontal scale, in percent. |
| macPrint[25] | Vertical scale, in percent |

**If this property is absent:**  

ChemDraw will use values based on the last-selected Style Sheet, or from the currently-selected printer if no Style Sheet is available. In a last resort, a portrait-orientation, letter-size page will be used.

---

[CDX Documentation index](/web/20160912060925/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)