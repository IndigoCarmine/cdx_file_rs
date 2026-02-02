CDX Format Specification: Bond Object
## Bond Object

| CDXML Name: | b |
| --- | --- |
| CDX Constant Name: | kCDXObj_Bond |
| CDX Constant Value: | 0x8005 |
| Contained by objects: | [kCDXObj_Fragment](Fragment.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

A Bond object defines a connection between Node objects. It corresponds directly to the notion of a chemical bond.

All Bonds must be contained in [Fragment](Fragment.md) objects; they cannot be stored directly in [Page](Page.md) or [Fragment](Fragment.md) objects.

Bond objects require Begin and End properties, but do not have any required objects.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8011 | [kCDXObj_ObjectTag](ObjectTag.md) | objecttag |  |
|  | Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| n/a | n/a | [id](properties/id.md) | [UINT16](DataType/CDXNumeric.md) |
|  | A unique identifier for an object, used when other objects refer to it. |  |  |
| 0x000A | [kCDXProp_ZOrder](properties/ZOrder.md) | Z | [INT16](DataType/CDXNumeric.md) |
|  | Back-to-front ordering index in 2D drawing. |  |  |
| 0x000F | [kCDXProp_IgnoreWarnings](properties/IgnoreWarnings.md) | IgnoreWarnings | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether chemical warnings should be suppressed on this object. |  |  |
| 0x0010 | [kCDXProp_ChemicalWarning](properties/ChemicalWarning.md) | Warning | [CDXString](DataType/CDXString.md) |
|  | A warning concerning possible chemical problems with this object. |  |  |
| 0x0011 | [kCDXProp_Visible](properties/Visible.md) | Visible | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The object is visible if non-zero. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0600 | [kCDXProp_Bond_Order](properties/Bond_Order.md) | Order | [INT16](DataType/CDXNumeric.md) |
|  | The order of a bond object.
This is a bit-encoded property. |  |  |
| 0x0601 | [kCDXProp_Bond_Display](properties/Bond_Display.md) | Display | [INT16](DataType/CDXNumeric.md) |
|  | The display type of a bond object.
This is an enumerated property. |  |  |
| 0x0602 | [kCDXProp_Bond_Display2](properties/Bond_Display2.md) | Display2 | [INT16](DataType/CDXNumeric.md) |
|  | The display type for the second line of a double bond.
This is an enumerated property. |  |  |
| 0x0603 | [kCDXProp_Bond_DoublePosition](properties/Bond_DoublePosition.md) | DoublePosition | [INT16](DataType/CDXNumeric.md) |
|  | The position of the second line of a double bond.
This is an enumerated property. |  |  |
| 0x0604 | [kCDXProp_Bond_Begin](properties/Bond_Begin.md) | B | [CDXObjectID](DataType/CDXObjectID.md) |
|  | Required for bonds.The ID of the CDX node object at the first end of a bond. |  |  |
| 0x0605 | [kCDXProp_Bond_End](properties/Bond_End.md) | E | [CDXObjectID](DataType/CDXObjectID.md) |
|  | Required for bonds.The ID of the CDX node object at the second end of a bond. |  |  |
| 0x0606 | [kCDXProp_Bond_RestrictTopology](properties/Bond_RestrictTopology.md) | Topology | [INT8](DataType/CDXNumeric.md) |
|  | Indicates the desired topology of a bond in a query.
This is an enumerated property. |  |  |
| 0x0607 | [kCDXProp_Bond_RestrictRxnParticipation](properties/Bond_RestrictRxnParticipation.md) | RxnParticipation | [INT8](DataType/CDXNumeric.md) |
|  | Specifies that a bond is affected by a reaction.
This is an enumerated property. |  |  |
| 0x0608 | [kCDXProp_Bond_BeginAttach](properties/Bond_BeginAttach.md) | BeginAttach | [UINT8](DataType/CDXNumeric.md) |
|  | Indicates where within the Bond_Begin node a bond is attached. |  |  |
| 0x0609 | [kCDXProp_Bond_EndAttach](properties/Bond_EndAttach.md) | EndAttach | [UINT8](DataType/CDXNumeric.md) |
|  | Indicates where within the Bond_End node a bond is attached. |  |  |
| 0x060A | [kCDXProp_Bond_CIPStereochemistry](properties/Bond_CIPStereochemistry.md) | BS | [INT8](DataType/CDXNumeric.md) |
|  | The bond's absolute stereochemistry according to the Cahn-Ingold-Prelog system.
This is an enumerated property. |  |  |
| 0x060B | [kCDXProp_Bond_BondOrdering](properties/Bond_BondOrdering.md) | BondCircularOrdering | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Ordered list of attached bond IDs. |  |  |
| 0x060C | [kCDXProp_Bond_ShowQuery](properties/Bond_ShowQuery.md) | ShowBondQuery | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the query indicator if non-zero. |  |  |
| 0x060D | [kCDXProp_Bond_ShowStereo](properties/Bond_ShowStereo.md) | ShowBondStereo | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the stereochemistry indicator if non-zero. |  |  |
| 0x060E | [kCDXProp_Bond_CrossingBonds](properties/Bond_CrossingBonds.md) | CrossingBonds | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | The set of bonds that cross a given bond. |  |  |
| 0x060F | [kCDXProp_Bond_ShowRxn](properties/Bond_ShowRxn.md) | ShowBondRxn | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the reaction-change indicator if non-zero. |  |  |
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
| 0x081A | [kCDXProp_LabelStyleFont](properties/LabelStyleFont.md) | LabelFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font family for atom labels. |  |  |
| 0x081C | [kCDXProp_LabelStyleSize](properties/LabelStyleSize.md) | LabelSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for atom labels. |  |  |
| 0x081E | [kCDXProp_LabelStyleFace](properties/LabelStyleFace.md) | LabelFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for atom labels. |  |  |
| 0x0822 | [kCDXProp_BondSpacingAbs](properties/BondSpacingAbs.md) | BondSpacingAbs | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The absolute distance between segments of a multiple bond. |  |  |

---

[CDX Documentation index](index.md)