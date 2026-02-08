## LabelJustification Property

| CDXML Name: | LabelJustification |
| --- | --- |
| CDX Constant Name: | kCDXProp_LabelJustification |
| CDX Constant Value: | 0x0823 |
| Data Size: | [INT8](/web/20190121153938/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Document](/web/20190121153938/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Text](/web/20190121153938/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

The default justification for atom labels.

In an Automatically-justified atom label, the atom is assumed to be associated with the first element in the label, reading from left to right. The rest of the label is positioned so that it best stays out of the way of any attached bonds, while trying to keep that first element at the atom's position. For atoms attached on the right end of a bond, this results in an atom label whose display is equivalent to Left, such as "CH3" or "OMe". For atoms attached on the left end of a bond, the atom label will be swapped, producing text such as "H3C" or "MeO". Accordingly, if you have an atom label that you know should always be displayed from left-to-right, you should definitely specify some justification other than Automatic.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| -1 | Right | Text is right justified |
| 0 | Left | Text is left justified |
| 1 | Center | Text is centered |
| 2 | Full | Text is fully justified |
| 3 | Above | Text is stacked above |
| 4 | Below | Text is stacked below |
| 5 | Auto | Text is automatically positioned |
| 6 | Best | Text is positioned at creation-type in the alignment that best avoids any associated objects, but that alignment is fixed from that point onward |

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190121153938/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet. If no information is found in the Style Sheet either, the text is automatically justified.

---

[CDX Documentation index](/web/20190121153938/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)