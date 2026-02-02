CDX Format Specification: TLC_ShowSideTicks Property
## TLC_ShowSideTicks Property

| CDXML Name: | ShowSideTicks |
| --- | --- |
| CDX Constant Name: | kCDXProp_TLC_ShowSideTicks |
| CDX Constant Value: | 0x0AA5 |
| Data Size: | [CDXBoolean](/web/20160912170329/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: |  |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**  

Show tickmarks up the side of the TLC Plate if non-zero.

If the distance between the origin line and the solvent front is at least 100 points, ChemDraw will display tickmarks every 10% of that distance (8 tick marks, at 10%, 20%, 30%, etc). Otherwise, it will show three tick marks at 25%, 50%, and 75%.

**If this property is absent:**  

Side ticks will not be shown.

---

[CDX Documentation index](/web/20160912170329/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)