CDX Format Specification: InterpretChemically Property
## InterpretChemically Property

| CDXML Name: | InterpretChemically |
| --- | --- |
| CDX Constant Name: | kCDXProp_InterpretChemically |
| CDX Constant Value: | 0x0708 |
| Data Size: | [CDXBooleanImplied](/web/20190121162651/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXBoolean.md) |
| Property of objects: | [kCDXObj_Document](/web/20190121162651/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Text](/web/20190121162651/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) |
| First written/read in: | ChemDraw 8.0 |
| Required? | No |

**Description:**  

Signifies whether to the text label should be interpreted chemically (if possible).

This attribute indicates whether the given caption or atom label should be interpreted chemically if at all possible. This allows for a distinction between a "C" that represents elemental carbon and a "C" that indicates the third item in a list starting with "A" and "B".

Note the "if at all possible" qualification. Wholly uninterpretable text like "wqertsrtrst" or "Fred" or "Boiling Point" will never be treated as chemically meaningful, regardless of the value of this property.

**If this property is absent:**  

The text will be treated chemically if at all possible.

---

[CDX Documentation index](/web/20190121162651/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)