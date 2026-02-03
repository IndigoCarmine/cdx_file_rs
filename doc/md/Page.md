CDX Format Specification: Page Object
## Page Object

| CDXML Name: | page |
| --- | --- |
| CDX Constant Name: | kCDXObj_Page |
| CDX Constant Value: | 0x8001 |
| Contained by objects: | [kCDXObj_Document](Document.md),[kCDXObj_Table](Table.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

A Page is used to divide objects into separate drawing spaces. If there is only one drawing space, all the graphical items should be placed in one page object (this is how ChemDraw stores 'multi-page' documents). In most documents, a Page object will correspond to a physical piece of paper when printed.

A Page has no required objects or properties.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8002 | [kCDXObj_Group](Group.md) | group |  |
|  | A logical collection of objects. |  |  |
| 0x8003 | [kCDXObj_Fragment](Fragment.md) | fragment |  |
|  | A collection of nodes and their connectivity (bonds). |  |  |
| 0x8006 | [kCDXObj_Text](Text.md) | t |  |
|  | An arbitrary block of (possibly styled) text. |  |  |
| 0x8007 | [kCDXObj_Graphic](Graphic.md) | graphic |  |
|  | A (generally non-chemical) graphic object such as a line, arc, circle, or rectangle. |  |  |
| 0x8017 | [kCDXObj_BracketedGroup](BracketedGroup.md) | bracketedgroup |  |
|  | A collection of objects surrounded by brackets (or braces or parentheses). |  |  |
| 0x8008 | [kCDXObj_Curve](Curve.md) | curve |  |
|  | A Bézier curve. |  |  |
| 0x8009 | [kCDXObj_EmbeddedObject](EmbeddedObject.md) | embeddedobject |  |
|  | A Macintosh PICT, a Windows Metafile, or a Windows OLE Object. |  |  |
| 0x8016 | [kCDXObj_Table](Table.md) | table |  |
|  | A grid-like arrangement of drawing spaces. |  |  |
| 0x800A | [kCDXObj_NamedAlternativeGroup](NamedAltGroup.md) | altgroup |  |
|  | A container object holding fragments that represent alternative substituents for a query. |  |  |
| 0x800D | [kCDXObj_ReactionScheme](ReactionScheme.md) | scheme |  |
|  | A description of a single- or multi-step reaction, containing one or more Reaction Steps. |  |  |
| 0x800E | [kCDXObj_ReactionStep](ReactionStep.md) | step |  |
|  | A description of one step in a reaction. It contains the constituents, conditions, and products. |  |  |
| 0x8010 | [kCDXObj_Spectrum](Spectrum.md) | spectrum |  |
|  | An NMR, MS, IR or other sort of spectral plot. |  |  |
| 0x8013 | [kCDXObj_Sequence](Sequence.md) | sequence |  |
|  | One member of an ordered series; the contents of its Text object may change as other objects are added to or removed from the series. |  |  |
| 0x8014 | [kCDXObj_CrossReference](CrossReference.md) | crossreference |  |
|  | A link to some Sequence object. |  |  |
| 0x8020 | [kCDXObj_Border](Border.md) | border |  |
|  | A collection of information describing one edge of an object. |  |  |
| 0x8021 | [kCDXObj_Geometry](Geometry.md) | geometry |  |
|  | A geometrical relationship between one or more objects. |  |  |
| 0x8022 | [kCDXObj_Constraint](Constraint.md) | constraint |  |
|  | A distance or angle constraint between one or more objects. |  |  |
| 0x8023 | [kCDXObj_TLCPlate](TLCPlate.md) | tlcplate |  |
|  | A rectangular object representing a Thin Layer Chromatography (TLC) plate. |  |  |
| 0x8015 | [kCDXObj_Splitter](Splitter.md) | splitter |  |
|  | An object that divides the page into horizontal bands |  |  |
| 0x8026 | [kCDXObj_ChemicalProperty](ChemicalProperty.md) | chemicalproperty |  |
|  | A (physical) chemical property associated with a collection of objects, usually atoms and bonds. |  |  |
| 0x8027 | [kCDXObj_Arrow](Arrow.md) | arrow |  |
|  | A line or arc, optionally with arrowheads on one or both ends |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x080F | [kCDXProp_WidthPages](properties/WidthPages.md) | WidthPages | [INT16](DataType/CDXNumeric.md) |
|  | The width of the document in pages. |  |  |
| 0x0810 | [kCDXProp_HeightPages](properties/HeightPages.md) | HeightPages | [INT16](DataType/CDXNumeric.md) |
|  | The height of the document in pages. |  |  |
| 0x0811 | [kCDXProp_DrawingSpaceType](properties/DrawingSpaceType.md) | DrawingSpace | [INT8](DataType/CDXNumeric.md) |
|  | The type of drawing space used for this document.
This is an enumerated property. |  |  |
| 0x0812 | [kCDXProp_Width](properties/Width.md) | Width | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The width of an object in CDX coordinate units, possibly in a rotated or skewed frame. |  |  |
| 0x0813 | [kCDXProp_Height](properties/Height.md) | Height | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The height of an object in CDX coordinate units, possibly in a rotated or skewed frame. |  |  |
| 0x0814 | [kCDXProp_PageOverlap](properties/PageOverlap.md) | PageOverlap | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The amount of overlap of pages when a poster is tiled. |  |  |
| 0x0815 | [kCDXProp_Header](properties/Header.md) | Header | [CDXString](DataType/CDXString.md) |
|  | The text of the header. |  |  |
| 0x0816 | [kCDXProp_HeaderPosition](properties/HeaderPosition.md) | HeaderPosition | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The vertical offset of the header baseline from the top of the page. |  |  |
| 0x0817 | [kCDXProp_Footer](properties/Footer.md) | Footer | [CDXString](DataType/CDXString.md) |
|  | The text of the footer. |  |  |
| 0x0818 | [kCDXProp_FooterPosition](properties/FooterPosition.md) | FooterPosition | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The vertical offset of the footer baseline from the bottom of the page. |  |  |
| 0x0819 | [kCDXProp_PrintTrimMarks](properties/PrintTrimMarks.md) | PrintTrimMarks | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | If present, trim marks are to printed in the margins. |  |  |
| 0x1FF0 | [kCDXProp_SplitterPositions](properties/SplitterPositions.md) | SplitterPositions | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An array of vertical positions that subdivide a page into regions. |  |  |
| 0x1FF1 | [kCDXProp_PageDefinition](properties/PageDefinition.md) | PageDefinition | [INT8](DataType/CDXNumeric.md) |
|  | A description of the type of formatting used by the page, or by the splitter.
This is an enumerated property. |  |  |
| 0x206 | [kCDXProp_BoundsInParent](properties/BoundsInParent.md) | BoundsInParent | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The rectangle containing a page in the coordinate space of the containing page. |  |  |

---

[CDX Documentation index](index.md)