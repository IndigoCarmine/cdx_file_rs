CDX Format Specification: Document Object
## Document Object

| CDXML Name: | CDXML |
| --- | --- |
| CDX Constant Name: | kCDXObj_Document |
| CDX Constant Value: | 0x8000 |
| Contained by objects: |  |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

A Document is the top-level CDX object. It contains all CDX propertys and objects. It is necessary (by definition) for any valid CDX or CDXML file.

A Document must contain at least one [Page](Page.md) object, but it has no required properties.

In versions of ChemDraw prior to 6.0, the [bounding box](properties/BoundingBox.md) property was read and written as a member of the Document object, but its use is discouraged unless you need to maintain maximum compatibility with those older versions. Starting with ChemDraw 6.0, the data that used to be stored in the Document's bounding box property is now stored in the bounding box property of the relevent [Page](Page.md) object.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8001 | [kCDXObj_Page](Page.md) | page |  |
|  | A drawing space that can contain other objects. |  |  |
| 0x800B | [kCDXObj_TemplateGrid](TemplateGrid.md) | templategrid |  |
|  | A TemplateGrid indicates how multiple CDX page objects should be arranged in a Template document. |  |  |
| 0x0300 | [kCDXProp_ColorTable](ColorTable.md) | colortable |  |
|  | The color palette used throughout the document. Color indexes 0 and 1 always correspond to black and white and are not saved in the color table. The first and second RGB values (color indexes 2 and 3) are the default background and foreground colors, and other colors are numbered sequentially. |  |  |
| 0x0100 | [kCDXProp_FontTable](FontTable.md) | fonttable |  |
|  | A list of fonts used in the document. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| 0x0001 | [kCDXProp_CreationUserName](properties/CreationUserName.md) | CreationUserName | [CDXString](DataType/CDXString.md) |
|  | The name of the creator (program user's name) of the document. |  |  |
| 0x0002 | [kCDXProp_CreationDate](properties/CreationDate.md) | CreationDate | [CDXDate](DataType/DateAndTime.md) |
|  | The time of object creation. |  |  |
| 0x0003 | [kCDXProp_CreationProgram](properties/CreationProgram.md) | CreationProgram | [CDXString](DataType/CDXString.md) |
|  | The name of the program, including version and platform, that created the associated CDX object. ChemDraw 4.0 uses "ChemDraw 4.0" as the value of CreationProgram. |  |  |
| 0x0004 | [kCDXProp_ModificationUserName](properties/ModificationUserName.md) | ModificationUserName | [CDXString](DataType/CDXString.md) |
|  | The name of the last modifier (program user's name) of the document. |  |  |
| 0x0005 | [kCDXProp_ModificationDate](properties/ModificationDate.md) | ModificationDate | [CDXDate](DataType/DateAndTime.md) |
|  | Time of the last modification. |  |  |
| 0x0006 | [kCDXProp_ModificationProgram](properties/ModificationProgram.md) | ModificationProgram | [CDXString](DataType/CDXString.md) |
|  | The name of the program, including version and platform, of the last program to perform a modification. ChemDraw 4.0 uses "ChemDraw 4.0" as the value of CreationProgram. |  |  |
| 0x0008 | [kCDXProp_Name](properties/Name.md) | Name | [CDXString](DataType/CDXString.md) |
|  | Required for objecttags.Name of an object. |  |  |
| 0x0009 | [kCDXProp_Comment](properties/Comment.md) | Comment | [CDXString](DataType/CDXString.md) |
|  | An arbitrary string intended to be meaningful to a user. |  |  |
| 0x0100 | [kCDXProp_FontTable](properties/FontTable.md) | fonttable | [CDXFontTable](DataType/CDXFontTable.md) |
|  | Required if fonts are used.A list of fonts used in the document. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0300 | [kCDXProp_ColorTable](properties/ColorTable.md) | colortable | [CDXColorTable](DataType/CDXColorTable.md) |
|  | The color palette used throughout the document. |  |  |
| 0x043A | [kCDXProp_Atom_ShowQuery](properties/Atom_ShowQuery.md) | ShowAtomQuery | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the query indicator if non-zero. |  |  |
| 0x043B | [kCDXProp_Atom_ShowStereo](properties/Atom_ShowStereo.md) | ShowAtomStereo | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the stereochemistry indicator if non-zero. |  |  |
| 0x043C | [kCDXProp_Atom_ShowAtomNumber](properties/Atom_ShowAtomNumber.md) | ShowAtomNumber | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the atom number if non-zero. |  |  |
| 0x060C | [kCDXProp_Bond_ShowQuery](properties/Bond_ShowQuery.md) | ShowBondQuery | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the query indicator if non-zero. |  |  |
| 0x060D | [kCDXProp_Bond_ShowStereo](properties/Bond_ShowStereo.md) | ShowBondStereo | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the stereochemistry indicator if non-zero. |  |  |
| 0x060F | [kCDXProp_Bond_ShowRxn](properties/Bond_ShowRxn.md) | ShowBondRxn | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the reaction-change indicator if non-zero. |  |  |
| 0x0706 | [kCDXProp_LabelLineHeight](properties/LabelLineHeight.md) | LabelLineHeight | [INT16](DataType/CDXNumeric.md) |
|  | Text line height for atom labels |  |  |
| 0x0707 | [kCDXProp_CaptionLineHeight](properties/CaptionLineHeight.md) | CaptionLineHeight | [INT16](DataType/CDXNumeric.md) |
|  | Text line height for non-atomlabel text objects |  |  |
| 0x0708 | [kCDXProp_InterpretChemically](properties/InterpretChemically.md) | InterpretChemically | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether to the text label should be interpreted chemically (if possible). |  |  |
| 0x0800 | [kCDXProp_MacPrintInfo](properties/MacPrintInfo.md) | MacPrintInfo | [Unformatted](DataType/Unformatted.md) |
|  | The 120 byte Macintosh TPrint data associated with the CDX document object. Refer to Macintosh Toolbox manual for detailed description. |  |  |
| 0x0801 | [kCDXProp_WinPrintInfo](properties/WinPrintInfo.md) | WinPrintInfo | [Unformatted](DataType/Unformatted.md) |
|  | The Windows DEVMODE structure associated with the CDX document object. |  |  |
| 0x0802 | [kCDXProp_PrintMargins](properties/PrintMargins.md) | PrintMargins | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The outer margins of the Document. |  |  |
| 0x0803 | [kCDXProp_ChainAngle](properties/ChainAngle.md) | ChainAngle | [INT32](DataType/CDXNumeric.md) |
|  | The default chain angle setting in degrees * 65536. |  |  |
| 0x0804 | [kCDXProp_BondSpacing](properties/BondSpacing.md) | BondSpacing | [INT16](DataType/CDXNumeric.md) |
|  | The spacing between segments of a multiple bond, measured relative to bond length. |  |  |
| 0x0805 | [kCDXProp_BondLength](properties/BondLength.md) | BondLength | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default bond length. |  |  |
| 0x0806 | [kCDXProp_BoldWidth](properties/BoldWidth.md) | BoldWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default bold bond width. |  |  |
| 0x0807 | [kCDXProp_LineWidth](properties/LineWidth.md) | LineWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default line width. |  |  |
| 0x0808 | [kCDXProp_MarginWidth](properties/MarginWidth.md) | MarginWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default amount of space surrounding atom labels. |  |  |
| 0x0809 | [kCDXProp_HashSpacing](properties/HashSpacing.md) | HashSpacing | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default spacing between hashed lines used in wedged hashed bonds. |  |  |
| 0x080A | [kCDXProp_LabelStyle](properties/LabelStyle.md) | (not used) | [CDXFontStyle](DataType/CDXString.md) |
|  | The default style for atom labels.. |  |  |
| 0x080B | [kCDXProp_CaptionStyle](properties/CaptionStyle.md) | (not used) | [CDXFontStyle](DataType/CDXString.md) |
|  | The default style for non-atomlabel text objects.. |  |  |
| 0x080C | [kCDXProp_CaptionJustification](properties/CaptionJustification.md) | CaptionJustification | [INT8](DataType/CDXNumeric.md) |
|  | The horizontal justification of a caption (non-atomlabel text object)
This is an enumerated property. |  |  |
| 0x080D | [kCDXProp_FractionalWidths](properties/FractionalWidths.md) | FractionalWidths | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether to use fractional width information when drawing text. |  |  |
| 0x080E | [kCDXProp_Magnification](properties/Magnification.md) | Magnification | [INT16](DataType/CDXNumeric.md) |
|  | The view magnification factor |  |  |
| 0x081A | [kCDXProp_LabelStyleFont](properties/LabelStyleFont.md) | LabelFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font family for atom labels. |  |  |
| 0x081B | [kCDXProp_CaptionStyleFont](properties/CaptionStyleFont.md) | CaptionFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for captions (non-atom-label text objects). |  |  |
| 0x081C | [kCDXProp_LabelStyleSize](properties/LabelStyleSize.md) | LabelSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for atom labels. |  |  |
| 0x081D | [kCDXProp_CaptionStyleSize](properties/CaptionStyleSize.md) | CaptionSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for captions (non-atom-label text objects). |  |  |
| 0x081E | [kCDXProp_LabelStyleFace](properties/LabelStyleFace.md) | LabelFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for atom labels. |  |  |
| 0x081F | [kCDXProp_CaptionStyleFace](properties/CaptionStyleFace.md) | CaptionFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font face for captions (non-atom-label text objects). |  |  |
| 0x0820 | [kCDXProp_LabelStyleColor](properties/LabelStyleColor.md) | LabelColor | [INT16](DataType/CDXNumeric.md) |
|  | The default color for atom labels |  |  |
| 0x0821 | [kCDXProp_CaptionStyleColor](properties/CaptionStyleColor.md) | CaptionColor | [INT16](DataType/CDXNumeric.md) |
|  | The default color for captions (non-atom-label text objects). |  |  |
| 0x0823 | [kCDXProp_LabelJustification](properties/LabelJustification.md) | LabelJustification | [INT8](DataType/CDXNumeric.md) |
|  | The default justification for atom labels.
This is an enumerated property. |  |  |
| 0x0824 | [kCDXProp_FixInplaceExtent](properties/FixInplaceExtent.md) | FixInPlaceExtent | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Defines a size for OLE In-Place editing. |  |  |
| 0x0826 | [kCDXProp_FixInplaceGap](properties/FixInplaceGap.md) | FixInPlaceGap | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Defines a padding for OLE In-Place editing. |  |  |
| 0x0827 | [kCDXProp_CartridgeData](properties/CartridgeData.md) | CartridgeData | [Unformatted](DataType/Unformatted.md) |
|  | Transient data used by the CambridgeSoft Oracle Cartridge. |  |  |
| 0x0900 | [kCDXProp_Window_IsZoomed](properties/Window_IsZoomed.md) | WindowIsZoomed | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether the main viewing window is zoomed (maximized). |  |  |
| 0x0901 | [kCDXProp_Window_Position](properties/Window_Position.md) | WindowPosition | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The top-left position of the main viewing window. |  |  |
| 0x0902 | [kCDXProp_Window_Size](properties/Window_Size.md) | WindowSize | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Height and width of the document window. |  |  |

---

[CDX Documentation index](index.md)