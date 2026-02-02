CDX Format Specification: Name Property
## Name Property

| CDXML Name: | Name |
| --- | --- |
| CDX Constant Name: | kCDXProp_Name |
| CDX Constant Value: | 0x0008 |
| Data Size: | [CDXString](/web/20190912224955/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Document](/web/20190912224955/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_ObjectTag](/web/20190912224955/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md),[kCDXObj_Geometry](/web/20190912224955/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md),[kCDXObj_Constraint](/web/20190912224955/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md),[kCDXObj_ChemicalProperty](/web/20190912224955/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ChemicalProperty.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Required for objecttags |

**Description:**  

Name of an object.

Although this value is normally stored as a CDXString, ChemDraw will ignore any style information, if present.

When used as a property of an [Object Tag](/web/20190912224955/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/ObjectTag.md), this property is stored as unformatted data (that is, there are no styles).

---

[CDX Documentation index](/web/20190912224955/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)