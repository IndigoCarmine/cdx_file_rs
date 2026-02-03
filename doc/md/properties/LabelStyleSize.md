CDX Format Specification: LabelStyleSize Property
## LabelStyleSize Property

| CDXML Name: | LabelSize |
| --- | --- |
| CDX Constant Name: | kCDXProp_LabelStyleSize |
| CDX Constant Value: | 0x081C |
| Data Size: | [INT16](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Document](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Node](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Bond](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Text](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Table](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_Spectrum](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_Geometry](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md),[kCDXObj_Constraint](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md),[kCDXObj_TLCPlate](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**  

The default font size for atom labels.

Generally, this property is used to provide a default size for chemically-significant text. ChemDraw uses it to determine the following:

- Default font size for Atom labels
- Default font size for NamedAlternativeGroups
- Default font size for Attachment Rank Indicator diamonds (subscripted size)
- Default font size for Atom and bond properties (including stereochemistry indicators) (subscripted size)
- Default font size for Atom-Atom map indicators (subscripted size)
- Default size of Drawing Symbols (radicals, charges, etc)

**If this property is absent:**  

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20190326232657/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)