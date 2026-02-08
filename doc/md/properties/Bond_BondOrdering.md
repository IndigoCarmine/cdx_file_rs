## Bond_BondOrdering Property

| CDXML Name: | BondCircularOrdering |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_BondOrdering |
| CDX Constant Value: | 0x060B |
| Data Size: | [CDXObjectIDArray](/web/20160913174115/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160913174115/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 6.0 |
| Required? | No |

**Description:**

Ordered list of attached bond IDs.

This property is important for preserving stereochemistry in files that do not otherwise have coordinates. An array member value of zero means "no bond at this location", so an array like 5 0 7 0 indicates a *trans* bond.

This property is used only for bonds of order 2; accordingly, it may be considered either as a clockwise or counterclockwise ordering without affecting the meaning.

**If this property is absent:**

The ordering of bonds is not considered significant.

Note that this property is redundant with the positioning of [Nodes](/web/20160913174115/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md), and may be safely omitted from any file with fully-defined node [positions](2DPosition.md).

---

[CDX Documentation index](/web/20160913174115/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)