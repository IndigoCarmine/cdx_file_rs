CDX Format Specification: Bond_CrossingBonds Property
## Bond_CrossingBonds Property

| CDXML Name: | CrossingBonds |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_CrossingBonds |
| CDX Constant Value: | 0x060E |
| Data Size: | [CDXObjectIDArray](/web/20160912060826/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160912060826/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 8.0 / (not read) |
| Required? | No |

**Description:**  

The set of bonds that cross a given bond.

Crossing bonds may be either above or below the given bond. The relative ordering may be determined by comparing the [kCDXProp_ZOrder](ZOrder.md) of the two bonds.

**If this property is absent:**  

Crossing bonds should be determined from the coordinates of the objects in the file.

---

[CDX Documentation index](/web/20160912060826/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)