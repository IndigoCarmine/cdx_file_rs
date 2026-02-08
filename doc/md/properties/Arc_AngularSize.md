## Arc_AngularSize Property

| CDXML Name: | AngularSize |
| --- | --- |
| CDX Constant Name: | kCDXProp_Arc_AngularSize |
| CDX Constant Value: | 0x0A21 |
| Data Size: | [INT16](/web/20190326235725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190326235725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Arrow](/web/20190326235725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The size of an arc (in degrees * 10, so 90 degrees = 900).

ChemDraw does not support arcs of arbitrary angle size, so this value is quantized in files produced by ChemDraw. When reading files, ChemDraw rounds the value in the file to the nearest supported angle (90, 120, 180, or 270 degrees).

**In CDX files**, this value is stored in degrees * 10, so 90 degrees = 900.

**In CDXML files**, this value is stored "raw", so 90 degrees = 90.

**If this property is absent:**

The value is treated as zero, which is then rounded up to 90 degrees by ChemDraw.

---

[CDX Documentation index](/web/20190326235725/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)