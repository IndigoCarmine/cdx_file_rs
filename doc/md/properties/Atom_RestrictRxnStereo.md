CDX Format Specification: Atom_RestrictRxnStereo Property
## Atom_RestrictRxnStereo Property

| CDXML Name: | RxnStereo |
| --- | --- |
| CDX Constant Name: | kCDXProp_Atom_RestrictRxnStereo |
| CDX Constant Value: | 0x0428 |
| Data Size: | [INT8](/web/20190327224309/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Node](/web/20190327224309/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The change of stereochemistry of an atom during a reaction.

The value stored in this property corresponds exactly to the Inversion/Retention flag in ISIS.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Unspecified change in stereochemistry |
| 1 | Inversion | Absolute stereochemistry does not change during the reaction |
| 2 | Retention | Absolute stereochemistry is retained during the reaction |

**If this property is absent:**  

The change of stereochemistry is unspecified.

---

[CDX Documentation index](/web/20190327224309/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)