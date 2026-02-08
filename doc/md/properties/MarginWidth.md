## MarginWidth Property

| CDXML Name: | MarginWidth |
| --- | --- |
| CDX Constant Name: | kCDXProp_MarginWidth |
| CDX Constant Value: | 0x0808 |
| Data Size: | [CDXCoordinate](/web/20160912060913/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20160912060913/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Bond](/web/20160912060913/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Table](/web/20160912060913/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_TLCPlate](/web/20160912060913/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**

The default amount of space surrounding atom labels.

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20160912060913/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20160912060913/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)