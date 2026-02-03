CDX Format Specification: CaptionStyle Property
## CaptionStyle Property

| CDXML Name: | temp_CaptionStyle |
| --- | --- |
| CDX Constant Name: | kCDXProp_CaptionStyle |
| CDX Constant Value: | 0x080B |
| Data Size: | [CDXFontStyle](/web/20190327000448/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Document](/web/20190327000448/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Text](/web/20190327000448/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Graphic](/web/20190327000448/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Arrow](/web/20190327000448/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**  

The default style for non-atomlabel text objects..

This property is used only in CDX files. In CDXML files, the family, face, and size of a font are written as separate properties of the [s](/web/20190327000448/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Style.md) object.

In addition to the standard values, the family, face, or size may contain the value -1. Such a value indicates that the default value for that property should be used (this is useful only if at least one of the other two values is *not* -1). Default values are determined as if this property were absent entirely.

**If this property is absent:**  

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190327000448/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20190327000448/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)