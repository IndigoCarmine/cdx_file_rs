## ReactionStep_ObjectsAboveArrow Property

| CDXML Name: | ReactionStepObjectsAboveArrow |
| --- | --- |
| CDX Constant Name: | kCDXProp_ReactionStep_ObjectsAboveArrow |
| CDX Constant Value: | 0x0C05 |
| Data Size: | [CDXObjectIDArray](/web/20160912005313/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_ReactionStep](/web/20160912005313/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ReactionStep.md) |
| First written/read in: | ChemDraw 6.0 / (not read) |
| Required? | No |

**Description:**

An ordered list of objects above the arrow in the Reaction Step.

The use of the word "above" in this property name is intended to be reminiscent of a horizontal arrow. If the arrow in question is more vertical than horizontal, then "above" will actually mean "to the left of."

Logically, objects listed by this property will often be treated the same as those listed by [kCDXProp_ReactionStep_ObjectsAboveArrow](ReactionStep_ObjectsAboveArrow.md), however they are split out in case a reading program wants to treat them differently.

**If this property is absent:**

This property is not read by ChemDraw. It is written as a courtesy only. There is no consequence to omitting it.

---

[CDX Documentation index](/web/20160912005313/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)