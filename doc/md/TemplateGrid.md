CDX Format Specification: Template Grid Object
## Template Grid Object

| CDXML Name: | templategrid |
| --- | --- |
| CDX Constant Name: | kCDXObj_TemplateGrid |
| CDX Constant Value: | 0x800B |
| Contained by objects: | [kCDXObj_Document](Document.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

A TemplateGrid is a thumbnail view of multiple CDX page objects in a grid. If a CDX document object has a TemplateGrid object, then the document has the template grid view associated with it when viewed in ChemDraw. The document must also contain exactly Template_NumRows * Template_NumColumns number of page objects, although some pages might be empty.

Since this object is used only for template documents, it will be encountered only rarely.

All properties within a TemplateGrid object are required. If any property is missing, the file must be considered invalid and should not be displayed.

In versions of ChemDraw prior to 8.0, TemplateGrid objects with IDs other than zero were ignored. Since there is nothing to be gained from a non-zero ID in this case (there will never be more than one TemplateGrid in a document, and no other object will ever reference it), we recommend that an ID of zero always be used.

**Subobjects:**  
*(none)*

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| 0x0202 | [kCDXProp_2DExtent](properties/2DExtent.md) | extent | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Required for templategrids.The width and height of an object in CDX coordinate units. The precise meaning of this attribute varies depending on the type of object. |  |  |
| 0x1000 | [kCDXProp_Template_PaneHeight](properties/Template_PaneHeight.md) | PaneHeight | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | Required for templategrids.The height of the viewing window of a template grid. |  |  |
| 0x1001 | [kCDXProp_Template_NumRows](properties/Template_NumRows.md) | NumRows | [INT16](DataType/CDXNumeric.md) |
|  | Required for templategrids.The number of rows of the CDX TemplateGrid object. |  |  |
| 0x1002 | [kCDXProp_Template_NumColumns](properties/Template_NumColumns.md) | NumColumns | [INT16](DataType/CDXNumeric.md) |
|  | Required for templategrids.The number of columns of the CDX TemplateGrid object. |  |  |

---

[CDX Documentation index](index.md)