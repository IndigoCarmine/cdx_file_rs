## BondLength Property

| CDXML Name: | BondLength |
| --- | --- |
| CDX Constant Name: | kCDXProp_BondLength |
| CDX Constant Value: | 0x0805 |
| Data Size: | [CDXCoordinate](/web/20180315141948/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXCoordinates.md) |
| Property of objects: | [kCDXObj_Document](/web/20180315141948/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md),[kCDXObj_Bond](/web/20180315141948/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md),[kCDXObj_Geometry](/web/20180315141948/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Geometry.md),[kCDXObj_Constraint](/web/20180315141948/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Constraint.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | Until ChemDraw 4.5 |

**Description:**

The default bond length.

Generally, this property is used to provide a default value for bold lines. ChemDraw uses it to determine the following:

- Default bond length when drawing bonds, chains, and rings by hand
- Default size of orbitals and electrons
- Scale factor when pasting and placing templates
- Space between NamedAlternativeGroups without bounding boxes (horizontal spacing is this value)
- Space between NamedAlternativeGroups components without bounding boxes (vertical spacing is half of this value)
- Size of frames for NamedAlternativeGroups without bounding boxes (frames are outset from components by half of this value)
- Spacing between reaction components without bounding boxes (horizontal spacing is this value)
- Charge objects must be within a distance equal to half of this value if they are to be recognized as being applied to some other object

**If this property is absent:**

The value from the same property of a containing object will be used. That is, if this property is omitted from an object, but present for the [Document](/web/20180315141948/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Document.md), the Document's value will be used. If no value is found at all, ChemDraw will use the value from its last-used Style Sheet.

---

[CDX Documentation index](/web/20180315141948/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)