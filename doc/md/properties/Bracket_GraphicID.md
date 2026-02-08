## Bracket_GraphicID Property

| CDXML Name: | GraphicID |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bracket_GraphicID |
| CDX Constant Value: | 0x0A2B |
| Data Size: | [CDXObjectID](/web/20190819212253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXObjectID.md) |
| Property of objects: | [kCDXObj_BracketAttachment](/web/20190819212253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketAttachment.md) |
| First written/read in: | ChemDraw 7.0 / (not read) |
| Required? | No |

**Description:**

The ID of a graphical object (bracket, brace, or parenthesis) associated with a Bracket Attachment.

**If this property is absent:**

The Bracket Attachment is not represented by an existing graphical object. It would be the responsibility of the program reading the file to create a new graphical object (bracket, etc) and position it appropriately relative to the other objects in the file.

---

[CDX Documentation index](/web/20190819212253/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)