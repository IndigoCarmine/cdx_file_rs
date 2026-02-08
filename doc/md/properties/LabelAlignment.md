## LabelAlignment Property

| CDXML Name: | LabelAlignment |
| --- | --- |
| CDX Constant Name: | kCDXProp_LabelAlignment |
| CDX Constant Value: | 0x0705 |
| Data Size: | [INT8](/web/20190121154129/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Text](/web/20190121154129/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

The alignment of the text with respect to the node position.

ChemDraw reads and writes this property only for multi-attached atom labels. This property stores a value that indicates what the actual label alignment for a non-multi-attached atom label would have to be to match the desired alignment of this label. Consider an atom label with Automatic alignment, positioned on the right side of a molecule. If this label becomes multi-attached, the alignment must "freeze" at its current (left-justified value), so that if the structure is modified (rotated, flipped, etc), the label alignment doesn't change. On the other hand, the original Automatic alignment must still be preserved, in case all but the first bond are deleted in the future, at which point the label should revert to being Automatic.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Auto | Label is aligned automatically |
| 1 | Left | Label is left-aligned |
| 2 | Center | Label is centered |
| 3 | Right | Label is right-aligned |
| 4 | Above | Label is stacked above |
| 5 | Below | Label is stacked below |
| 6 | Best | Label is created in the best alignment for the given bonds |

**If this property is absent:**

The text is assumed to be automatically aligned.

---

[CDX Documentation index](/web/20190121154129/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)