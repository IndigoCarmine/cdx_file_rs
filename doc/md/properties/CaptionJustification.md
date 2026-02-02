CDX Format Specification: CaptionJustification Property
## CaptionJustification Property

| CDXML Name: | CaptionJustification |
| --- | --- |
| CDX Constant Name: | kCDXProp_CaptionJustification |
| CDX Constant Value: | 0x080C |
| Data Size: | [INT8](/web/20190121162155/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Document](/web/20190121162155/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Text](/web/20190121162155/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

The horizontal justification of a caption (non-atomlabel text object)

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| -1 | Right | Right justified text |
| 0 | Left | Left justified text |
| 1 | Center | Centered text |
| 2 | Full | Fully justified text |
| 3 | Above | Text is stacked above |
| 4 | Below | Text is stacked below |
| 5 | Auto | Text is automatically positioned |
| 6 | Best | Text is positioned at creation-type in the alignment that best avoids any associated objects, but that alignment is fixed from that point onward |

**If this property is absent:**  

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190121162155/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet. If no information is found in the Style Sheet either, the text is treated as left-justified.

---

[CDX Documentation index](/web/20190121162155/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)