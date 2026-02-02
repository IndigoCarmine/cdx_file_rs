CDX Format Specification: LineWidth Property
## LineWidth Property

| CDXML Name: | LineWidth |
| --- | --- |
| CDX Constant Name: | kCDXProp_LineWidth |
| CDX Constant Value: | 0x0807 |
| Data Size: | [CDXCoordinate](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Node](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Bond](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Graphic](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Table](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_Spectrum](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_Border](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Border.md),[kCDXObj_Geometry](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md),[kCDXObj_Constraint](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md),[kCDXObj_TLCPlate](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md),[kCDXObj_Arrow](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**  

The default line width.

Generally, this property is used to provide a default value for plain lines. ChemDraw uses it to determine the following:

- Line width for most bond types
- Line width for most arrow types
- Line width for most curve types
- Line width for the frame of NamedAlternativeGroups
- Size of the HDot indicator (diameter of dot is 5 * lineWid)
- Size of the HDash indicator
- Size of MultiCenter node indicators
- Thickness of the borders of Attachment Rank Indicator diamonds
- Thickness of the border of Atom-Atom map indicators
- Line width for chemical warning indicators

**If this property is absent:**  

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20190326220447/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)