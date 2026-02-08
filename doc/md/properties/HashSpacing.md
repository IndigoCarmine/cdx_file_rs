## HashSpacing Property

| CDXML Name: | HashSpacing |
| --- | --- |
| CDX Constant Name: | kCDXProp_HashSpacing |
| CDX Constant Value: | 0x0809 |
| Data Size: | [CDXCoordinate](/web/20160912015812/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20160912015812/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Bond](/web/20160912015812/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Constraint](/web/20160912015812/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**

The default spacing between hashed lines used in wedged hashed bonds.

Generally, this property is used to provide a default value for spacing between hash-like and dash-like objects. ChemDraw uses it to determine the following:

- Spacing between hashes in hashed bonds
- Spacing between dashes in dashed bonds
- Spacing between dashes in dashed curves

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20160912015812/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20160912015812/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)