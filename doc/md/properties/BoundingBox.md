## BoundingBox Property

| CDXML Name: | BoundingBox |
| --- | --- |
| CDX Constant Name: | kCDXProp_BoundingBox |
| CDX Constant Value: | 0x0204 |
| Data Size: | [CDXRectangle](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Page](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Page.md),[kCDXObj_Group](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Group.md),[kCDXObj_Fragment](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Fragment.md),[kCDXObj_Text](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Graphic](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_Curve](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Curve.md),[kCDXObj_EmbeddedObject](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/EmbeddedObject.md),[kCDXObj_Table](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Table.md),[kCDXObj_NamedAlternativeGroup](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/NamedAltGroup.md),[kCDXObj_Spectrum](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Spectrum.md),[kCDXObj_TLCPlate](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/TLCPlate.md),[kCDXObj_Arrow](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Arrow.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Required for pictures and spectra. Required for graphics and text until 6.0 |

**Description:**

The smallest rectangle that encloses the graphical representation of the object.

When used in conjunction with [Graphic](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) objects, this properly does not, in fact, represent a bounding box but rather represents a pair of points whose (*x*, *y*) values consist of the (left, top) and (right, bottom) members of the rectangle. Those two points are then used to further define the Graphic in different ways depending on the [kCDXProp_Graphic_Type](Graphic_Type.md) of the graphic. This is, obviously, an unfortunate complication to the CDX format. It will likely be resolved in a future version of ChemDraw.

**If this property is absent:**

If possible, a reasonable bounding box will be guessed. For example, if a bounding box is omitted for an atom label (in ChemDraw 6.0 and later), the label will be positioned based on the location of its atom. Some objects (such as atoms) are defined fully by their [2D Position](2DPosition.md), and no bounding box is necessary. Note that this property is required for pictures and spectra, since no guess can be made about the size of those objects.

---

[CDX Documentation index](/web/20160912060508/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)