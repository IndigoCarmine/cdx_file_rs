CDX Format Specification: 2DPosition Property
## 2DPosition Property

| CDXML Name: | p |
| --- | --- |
| CDX Constant Name: | kCDXProp_2DPosition |
| CDX Constant Value: | 0x0200 |
| Data Size: | [CDXPoint2D](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Node](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md),[kCDXObj_Text](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md),[kCDXObj_Splitter](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Splitter.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 6.0 |

**Description:**  

The 2D location (in the order of vertical and horizontal locations) of an object.

The precise meaning of this attribute varies depending on the type of object. For instance, it is the center of the characters representing the element for Element nodes. On the other hand it is the left-most text baseline position for left-aligned caption objects.

**If this property is absent:**  

For [Text](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Text.md) objects, ChemDraw will try to calculate a position based on the [kCDXProp_BoundingBox](BoundingBox.md), if present. Otherwise, if the text object is an atom label, ChemDraw will try to calculate a position based on the associated [Node](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) object. Otherwise, if the text is a subobject of a [Named Alternative Group](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/NamedAltGroup.md), its location will be calculated based on the position of that object. Failing all of those, the text will be placed at (0, 0), which likely will not be desired.

For [Node](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Node.md) objects, the absence of this property indicates that the creation program might have known the chemical properties of the file (how the atoms and bonds are connected, for example), but was not able to select a specific display style. In such a case, ChemDraw will automatically run its Clean Up Structure command on the relevent node(s) to generate reasonable position(s). If some nodes do have this property and some do not, only the ones without it will be repositioned, and their new positions will take into account the positions of the other nodes. This would be a very reasonable approach, for example, if a program that normally handled only 3D data wanted to create a 2D CDX/CDXML file: it could simply omit all of the Node positions, and let ChemDraw come up with something reasonable.

If positions are omitted from asymmetric nodes, or from nodes connected to asymmetric bonds, the appropriate [kCDXProp_Atom_Geometry](Atom_Geometry.md), [kCDXProp_Atom_BondOrdering](Atom_BondOrdering.md), and [kCDXProp_Bond_BondOrdering](Bond_BondOrdering.md) properties should be present.

---

[CDX Documentation index](/web/20190326232847/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)