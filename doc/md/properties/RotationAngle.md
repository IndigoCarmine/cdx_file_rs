CDX Format Specification: RotationAngle Property
## RotationAngle Property

| CDXML Name: | RotationAngle |
| --- | --- |
| CDX Constant Name: | kCDXProp_RotationAngle |
| CDX Constant Value: | 0x0205 |
| Data Size: | [INT32](/web/20190121160337/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Text](/web/20190121160337/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_EmbeddedObject](/web/20190121160337/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

The angular orientation of an object in degrees * 65536.

Rotation angle has been available for [Text](/web/20190121160337/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) objects since version 4.0, but was added for [Embedded Objects](/web/20190121160337/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md) only in version 9.0. All Embedded Objects are treated as unrotated in earlier versions.

**If this property is absent:**  

The object is not rotated (has an angular orientation of zero degrees).

---

[CDX Documentation index](/web/20190121160337/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)