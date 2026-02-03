CDX Format Specification: Predefined Attributes
## Predefined Attributes

| (click column header to sort) |  |  |  |
| --- | --- | --- | --- |
| [Value](TableOfProperties.md) | [Name](TableOfPropertiesByName.md) | [CDXML Name](TableOfPropertiesByCDXMLName.md) | [Type](TableOfPropertiesByType.md) |
|  | Description |  |  |
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
| 0x000A | [kCDXProp_ZOrder](properties/ZOrder.md) | Z | [INT16](DataType/CDXNumeric.md) |
|  | Back-to-front ordering index in 2D drawing. |  |  |
| 0x000B | [kCDXProp_RegistryNumber](properties/RegistryNumber.md) | RegistryNumber | [CDXString](DataType/CDXString.md) |
|  | A registry or catalog number of a molecule object. |  |  |
| 0x000C | [kCDXProp_RegistryAuthority](properties/RegistryAuthority.md) | RegistryAuthority | [CDXString](DataType/CDXString.md) |
|  | A string that specifies the authority which issued a registry or catalog number. Some examples of registry authorities are CAS, Beilstein, Aldrich, and Merck. |  |  |
| 0x000E | [kCDXProp_RepresentsProperty](properties/RepresentsProperty.md) | RepresentsProperty | [CDXRepresentsProperty](DataType/CDXRepresentsProperty.md) |
|  | Indicates that this object represents some property in some other object. |  |  |
| 0x000F | [kCDXProp_IgnoreWarnings](properties/IgnoreWarnings.md) | IgnoreWarnings | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether chemical warnings should be suppressed on this object. |  |  |
| 0x0010 | [kCDXProp_ChemicalWarning](properties/ChemicalWarning.md) | Warning | [CDXString](DataType/CDXString.md) |
|  | A warning concerning possible chemical problems with this object. |  |  |
| 0x0011 | [kCDXProp_Visible](properties/Visible.md) | Visible | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The object is visible if non-zero. |  |  |
| 0x0012 | kCDXProp_SupersededBy | SupersededBy | [CDXObjectID](DataType/CDXObjectID.md) |
|  | The ID of the object that should be read instead of this one. |  |  |
| 0x0100 | [kCDXProp_FontTable](properties/FontTable.md) | fonttable | [CDXFontTable](DataType/CDXFontTable.md) |
|  | Required if fonts are used.A list of fonts used in the document. |  |  |
| 0x0200 | [kCDXProp_2DPosition](properties/2DPosition.md) | p | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The 2D location (in the order of vertical and horizontal locations) of an object. |  |  |
| 0x0201 | [kCDXProp_3DPosition](properties/3DPosition.md) | xyz | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location (in the order of X-, Y-, and Z-locations in right-handed coordinate system) of an object in CDX coordinate units. The precise meaning of this attribute varies depending on the type of object. |  |  |
| 0x0202 | [kCDXProp_2DExtent](properties/2DExtent.md) | extent | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Required for templategrids.The width and height of an object in CDX coordinate units. The precise meaning of this attribute varies depending on the type of object. |  |  |
| 0x0203 | [kCDXProp_3DExtent](properties/3DExtent.md) | extent3D | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The width, height, and depth of an object in CDX coordinate units (right-handed coordinate system). The precise meaning of this attribute varies depending on the type of object. |  |  |
| 0x0204 | [kCDXProp_BoundingBox](properties/BoundingBox.md) | BoundingBox | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | Required for pictures and spectra. Required for graphics and text until 6.0.The smallest rectangle that encloses the graphical representation of the object. |  |  |
| 0x0205 | [kCDXProp_RotationAngle](properties/RotationAngle.md) | RotationAngle | [INT32](DataType/CDXNumeric.md) |
|  | The angular orientation of an object in degrees * 65536. |  |  |
| 0x0207 | [kCDXProp_3DHead](properties/3DHead.md) | Head3D | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location (in the order of X-, Y-, and Z-locations in right-handed coordinate system) of the head of an object in CDX coordinate units. |  |  |
| 0x0208 | [kCDXProp_3DTail](properties/3DTail.md) | Tail3D | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location (in the order of X-, Y-, and Z-locations in right-handed coordinate system) of the tail of an object in CDX coordinate units. |  |  |
| 0x0209 | [kCDXProp_TopLeft](properties/TopLeft.md) | TopLeft | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The location of the top-left corner of a quadrilateral object, possibly in a rotated or skewed frame. |  |  |
| 0x020A | [kCDXProp_TopRight](properties/TopRight.md) | TopRight | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The location of the top-right corner of a quadrilateral object, possibly in a rotated or skewed frame. |  |  |
| 0x020B | [kCDXProp_BottomRight](properties/BottomRight.md) | BottomRight | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The location of the bottom-right corner of a quadrilateral object, possibly in a rotated or skewed frame. |  |  |
| 0x020C | [kCDXProp_BottomLeft](properties/BottomLeft.md) | BottomLeft | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The location of the bottom-left corner of a quadrilateral object, possibly in a rotated or skewed frame. |  |  |
| 0x020D | kCDXProp_3DCenter | Center3D | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location of the logical center of an object. |  |  |
| 0x020E | kCDXProp_3DMajorAxisEnd | Center3D | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location of the end of the major axis of an object in CDX coordinate units. |  |  |
| 0x020F | kCDXProp_3DMinorAxisEnd | Center3D | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location of the end of the minor axis of an object in CDX coordinate units. |  |  |
| 0x0300 | [kCDXProp_ColorTable](properties/ColorTable.md) | colortable | [CDXColorTable](DataType/CDXColorTable.md) |
|  | The color palette used throughout the document. |  |  |
| 0x0301 | [kCDXProp_ForegroundColor](properties/ForegroundColor.md) | color | [UINT16](DataType/CDXNumeric.md) |
|  | The foreground color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0302 | [kCDXProp_BackgroundColor](properties/BackgroundColor.md) | bgcolor | [INT16](DataType/CDXNumeric.md) |
|  | The background color of an object represented as the two-based index into the object's color table. |  |  |
| 0x0400 | [kCDXProp_Node_Type](properties/Node_Type.md) | NodeType | [INT16](DataType/CDXNumeric.md) |
|  | The type of a node object.
This is an enumerated property. |  |  |
| 0x0401 | [kCDXProp_Node_LabelDisplay](properties/Node_LabelDisplay.md) | LabelDisplay | [INT8](DataType/CDXNumeric.md) |
|  | The characteristics of node label display.
This is an enumerated property. |  |  |
| 0x0402 | [kCDXProp_Node_Element](properties/Node_Element.md) | Element | [INT16](DataType/CDXNumeric.md) |
|  | The atomic number of the atom representing this node. |  |  |
| 0x0403 | [kCDXProp_Atom_ElementList](properties/Atom_ElementList.md) | ElementList | [CDXElementList](DataType/CDXElementList.md) |
|  | A list of atomic numbers. |  |  |
| 0x0404 | [kCDXProp_Atom_Formula](properties/Atom_Formula.md) | Formula | [CDXFormula](DataType/CDXFormula.md) |
|  | The composition of a node representing a fragment whose composition is known, but whose connectivity is not. For example, C4H9represents a mixture of the 4 butyl isomers. |  |  |
| 0x0420 | [kCDXProp_Atom_Isotope](properties/Atom_Isotope.md) | Isotope | [INT16](DataType/CDXNumeric.md) |
|  | The absolute isotopic mass of an atom (2 for deuterium, 14 for carbon-14). |  |  |
| 0x0421 | [kCDXProp_Atom_Charge](properties/Atom_Charge.md) | Charge | [INT8](DataType/CDXNumeric.md) |
|  | The atomic charge of an atom. |  |  |
| 0x0422 | [kCDXProp_Atom_Radical](properties/Atom_Radical.md) | Radical | [UINT8](DataType/CDXNumeric.md) |
|  | The atomic radical attribute of an atom.
This is an enumerated property. |  |  |
| 0x0423 | [kCDXProp_Atom_RestrictFreeSites](properties/Atom_RestrictFreeSites.md) | FreeSites | [UINT8](DataType/CDXNumeric.md) |
|  | Indicates that up to the specified number of additional substituents are permitted on this atom. |  |  |
| 0x0424 | [kCDXProp_Atom_RestrictImplicitHydrogens](properties/Atom_RestrictImplicitHydrogens.md) | ImplicitHydrogens | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies that implicit hydrogens are not allowed on this atom. |  |  |
| 0x0425 | [kCDXProp_Atom_RestrictRingBondCount](properties/Atom_RestrictRingBondCount.md) | RingBondCount | [INT8](DataType/CDXNumeric.md) |
|  | The number of ring bonds attached to an atom.
This is an enumerated property. |  |  |
| 0x0426 | [kCDXProp_Atom_RestrictUnsaturatedBonds](properties/Atom_RestrictUnsaturatedBonds.md) | UnsaturatedBonds | [INT8](DataType/CDXNumeric.md) |
|  | Indicates whether unsaturation should be present or absent.
This is an enumerated property. |  |  |
| 0x0427 | [kCDXProp_Atom_RestrictRxnChange](properties/Atom_RestrictRxnChange.md) | RxnChange | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | If present, signifies that the reaction change of an atom must be as specified. |  |  |
| 0x0428 | [kCDXProp_Atom_RestrictRxnStereo](properties/Atom_RestrictRxnStereo.md) | RxnStereo | [INT8](DataType/CDXNumeric.md) |
|  | The change of stereochemistry of an atom during a reaction.
This is an enumerated property. |  |  |
| 0x0429 | [kCDXProp_Atom_AbnormalValence](properties/Atom_AbnormalValence.md) | AbnormalValence | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies that an abnormal valence for an atom is permitted. |  |  |
| 0x042B | [kCDXProp_Atom_NumHydrogens](properties/Atom_NumHydrogens.md) | NumHydrogens | [UINT16](DataType/CDXNumeric.md) |
|  | The number of (explicit) hydrogens in a labeled atom consisting of one heavy atom and (optionally) the symbol H (e.g., CH3). |  |  |
| 0x042E | [kCDXProp_Atom_HDot](properties/Atom_HDot.md) | HDot | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies the presence of an implicit hydrogen with stereochemistry specified equivalent to an explicit H atom with a wedged bond. |  |  |
| 0x042F | [kCDXProp_Atom_HDash](properties/Atom_HDash.md) | HDash | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies the presence of an implicit hydrogen with stereochemistry specified equivalent to an explicit H atom with a hashed bond. |  |  |
| 0x0430 | [kCDXProp_Atom_Geometry](properties/Atom_Geometry.md) | Geometry | [INT8](DataType/CDXNumeric.md) |
|  | The geometry of the bonds about this atom.
This is an enumerated property. |  |  |
| 0x0431 | [kCDXProp_Atom_BondOrdering](properties/Atom_BondOrdering.md) | BondOrdering | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordering of the bonds to this node, used for stereocenters, fragments, and named alternative groups with more than one attachment. |  |  |
| 0x0432 | [kCDXProp_Node_Attachments](properties/Node_Attachments.md) | Attachments | [CDXObjectIDArrayWithCounts](DataType/CDXObjectID.md) |
|  | Required for multi- and variable attached nodes.For multicenter attachment nodes or variable attachment nodes, a list of IDs of the nodes which are multiply or variably attached to this node. |  |  |
| 0x0433 | [kCDXProp_Atom_GenericNickname](properties/Atom_GenericNickname.md) | GenericNickname | [CDXString](DataType/CDXString.md) |
|  | The name of the generic nickname. |  |  |
| 0x0434 | [kCDXProp_Atom_AltGroupID](properties/Atom_AltGroupID.md) | AltGroupID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | The ID of the alternative group object that describes this node. |  |  |
| 0x0435 | [kCDXProp_Atom_RestrictSubstituentsUpTo](properties/Atom_RestrictSubstituentsUpTo.md) | SubstituentsUpTo | [UINT8](DataType/CDXNumeric.md) |
|  | Indicates that substitution is restricted to no more than the specified value. |  |  |
| 0x0436 | [kCDXProp_Atom_RestrictSubstituentsExactly](properties/Atom_RestrictSubstituentsExactly.md) | SubstituentsExactly | [UINT8](DataType/CDXNumeric.md) |
|  | Indicates that exactly the specified number of substituents must be present. |  |  |
| 0x0437 | [kCDXProp_Atom_CIPStereochemistry](properties/Atom_CIPStereochemistry.md) | AS | [INT8](DataType/CDXNumeric.md) |
|  | The node's absolute stereochemistry according to the Cahn-Ingold-Prelog system.
This is an enumerated property. |  |  |
| 0x0438 | [kCDXProp_Atom_Translation](properties/Atom_Translation.md) | Translation | [INT8](DataType/CDXNumeric.md) |
|  | Provides for restrictions on whether a given node may match other more- or less-general nodes.
This is an enumerated property. |  |  |
| 0x0439 | [kCDXProp_Atom_AtomNumber](properties/Atom_AtomNumber.md) | AtomNumber | [CDXString](DataType/CDXString.md) |
|  | Atom number, as text. |  |  |
| 0x043A | [kCDXProp_Atom_ShowQuery](properties/Atom_ShowQuery.md) | ShowAtomQuery | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the query indicator if non-zero. |  |  |
| 0x043B | [kCDXProp_Atom_ShowStereo](properties/Atom_ShowStereo.md) | ShowAtomStereo | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the stereochemistry indicator if non-zero. |  |  |
| 0x043C | [kCDXProp_Atom_ShowAtomNumber](properties/Atom_ShowAtomNumber.md) | ShowAtomNumber | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the atom number if non-zero. |  |  |
| 0x043D | [kCDXProp_Atom_LinkCountLow](properties/Atom_LinkCountLow.md) | LinkCountLow | [INT16](DataType/CDXNumeric.md) |
|  | Low end of repeat count for link nodes. |  |  |
| 0x043E | [kCDXProp_Atom_LinkCountHigh](properties/Atom_LinkCountHigh.md) | LinkCountHigh | [INT16](DataType/CDXNumeric.md) |
|  | High end of repeat count for link nodes. |  |  |
| 0x043F | [kCDXProp_Atom_IsotopicAbundance](properties/Atom_IsotopicAbundance.md) | IsotopicAbundance | [INT8](DataType/CDXNumeric.md) |
|  | Isotopic abundance of this atom's isotope.
This is an enumerated property. |  |  |
| 0x0440 | [kCDXProp_Atom_ExternalConnectionType](properties/Atom_ExternalConnectionType.md) | ExternalConnectionType | [INT8](DataType/CDXNumeric.md) |
|  | Type of external connection, for atoms of type kCDXNodeType_ExternalConnectionPoint.
This is an enumerated property. |  |  |
| 0x0441 | [kCDXProp_Atom_GenericList](properties/Atom_GenericList.md) | GenericList | [CDXGenericList](DataType/CDXGenericList.md) |
|  | A list of generic nicknames. |  |  |
| 0x0442 | kCDXProp_Atom_ShowTerminalCarbonLabels | ShowTerminalCarbonLabels | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether terminal carbons (carbons with zero or one bond) should display a text label with the element symbol and appropriate hydrogens. |  |  |
| 0x0443 | kCDXProp_Atom_ShowNonTerminalCarbonLabels | ShowNonTerminalCarbonLabels | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether non-terminal carbons (carbons with more than one bond) should display a text label with the element symbol and appropriate hydrogens. |  |  |
| 0x0444 | kCDXProp_Atom_HideImplicitHydrogens | HideImplicitHydrogens | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether implicit hydrogens should be displayed on otherwise-atomic atom labels (NH2 versus N). |  |  |
| 0x0445 | [kCDXProp_Atom_ShowEnhancedStereo](properties/Atom_ShowEnhancedStereo.md) | ShowAtomEnhancedStereo | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the enhanced stereochemistry indicator if non-zero. |  |  |
| 0x0446 | [kCDXProp_Atom_EnhancedStereoType](properties/Atom_EnhancedStereoType.md) | EnhancedStereoType | [UINT8](DataType/CDXNumeric.md) |
|  | The type of enhanced stereochemistry present on this atom.
This is an enumerated property. |  |  |
| 0x0447 | [kCDXProp_Atom_EnhancedStereoGroupNum](properties/Atom_EnhancedStereoGroupNum.md) | EnhancedStereoGroupNum | [UINT16](DataType/CDXNumeric.md) |
|  | The group number associated with Or and And enhanced stereochemistry types. |  |  |
| 0x0500 | [kCDXProp_Mole_Racemic](properties/Mole_Racemic.md) | Racemic | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Indicates that the molecule is a racemic mixture. |  |  |
| 0x0501 | [kCDXProp_Mole_Absolute](properties/Mole_Absolute.md) | Absolute | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Indicates that the molecule has known absolute configuration. |  |  |
| 0x0502 | [kCDXProp_Mole_Relative](properties/Mole_Relative.md) | Relative | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Indicates that the molecule has known relative stereochemistry, but unknown absolute configuration. |  |  |
| 0x0503 | [kCDXProp_Mole_Formula](properties/Mole_Formula.md) | Formula | [CDXFormula](DataType/CDXFormula.md) |
|  | The molecular formula representation of a molecule object. |  |  |
| 0x0504 | [kCDXProp_Mole_Weight](properties/Mole_Weight.md) | Weight | [FLOAT64](DataType/CDXNumeric.md) |
|  | The average molecular weight of a molecule object. |  |  |
| 0x0505 | [kCDXProp_Frag_ConnectionOrder](properties/Frag_ConnectionOrder.md) | ConnectionOrder | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of attachment points within a fragment. |  |  |
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
| 0x0700 | [kCDXProp_Text](properties/Text.md) | (not used) | [CDXString](DataType/CDXString.md) |
|  | Required for text objects.The text of a text object. |  |  |
| 0x0701 | [kCDXProp_Justification](properties/Justification.md) | Justification | [INT8](DataType/CDXNumeric.md) |
|  | The horizontal justification of a text object.
This is an enumerated property. |  |  |
| 0x0702 | [kCDXProp_LineHeight](properties/LineHeight.md) | LineHeight | [UINT16](DataType/CDXNumeric.md) |
|  | The line height of a text object. |  |  |
| 0x0703 | [kCDXProp_WordWrapWidth](properties/WordWrapWidth.md) | WordWrapWidth | [INT16](DataType/CDXNumeric.md) |
|  | The word-wrap width of a text object. |  |  |
| 0x0704 | [kCDXProp_LineStarts](properties/LineStarts.md) | LineStarts | [INT16ListWithCounts](DataType/INT16ListWithCounts.md) |
|  | The number of lines of a text object followed by that many values indicating the zero-based text position of each line start. |  |  |
| 0x0705 | [kCDXProp_LabelAlignment](properties/LabelAlignment.md) | LabelAlignment | [INT8](DataType/CDXNumeric.md) |
|  | The alignment of the text with respect to the node position.
This is an enumerated property. |  |  |
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
| 0x0822 | [kCDXProp_BondSpacingAbs](properties/BondSpacingAbs.md) | BondSpacingAbs | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The absolute distance between segments of a multiple bond. |  |  |
| 0x0823 | [kCDXProp_LabelJustification](properties/LabelJustification.md) | LabelJustification | [INT8](DataType/CDXNumeric.md) |
|  | The default justification for atom labels.
This is an enumerated property. |  |  |
| 0x0824 | [kCDXProp_FixInplaceExtent](properties/FixInplaceExtent.md) | FixInPlaceExtent | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Defines a size for OLE In-Place editing. |  |  |
| 0x0825 | [kCDXProp_Side](properties/Side.md) | Side | [UINT16](DataType/CDXNumeric.md) |
|  | Required.A specific side of an object (rectangle).
This is an enumerated property. |  |  |
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
| 0x0A00 | [kCDXProp_Graphic_Type](properties/Graphic_Type.md) | GraphicType | [INT16](DataType/CDXNumeric.md) |
|  | The type of graphical object.
This is an enumerated property. |  |  |
| 0x0A01 | [kCDXProp_Line_Type](properties/Line_Type.md) | LineType | [INT16](DataType/CDXNumeric.md) |
|  | The type of a line object.
This is an enumerated property. |  |  |
| 0x0A02 | [kCDXProp_Arrow_Type](properties/Arrow_Type.md) | ArrowType | [INT16](DataType/CDXNumeric.md) |
|  | The type of arrow object, which represents line, arrow, arc, rectangle, or orbital.
This is an enumerated property. |  |  |
| 0x0A03 | [kCDXProp_Rectangle_Type](properties/Rectangle_Type.md) | RectangleType | [INT16](DataType/CDXNumeric.md) |
|  | The type of a rectangle object.
This is an enumerated property. |  |  |
| 0x0A04 | [kCDXProp_Oval_Type](properties/Oval_Type.md) | OvalType | [INT16](DataType/CDXNumeric.md) |
|  | The type of an arrow object that represents a circle or ellipse.
This is an enumerated property. |  |  |
| 0x0A05 | [kCDXProp_Orbital_Type](properties/Orbital_Type.md) | OrbitalType | [INT16](DataType/CDXNumeric.md) |
|  | The type of orbital object.
This is an enumerated property. |  |  |
| 0x0A06 | [kCDXProp_Bracket_Type](properties/Bracket_Type.md) | BracketType | [INT16](DataType/CDXNumeric.md) |
|  | The type of symbol object.
This is an enumerated property. |  |  |
| 0x0A07 | [kCDXProp_Symbol_Type](properties/Symbol_Type.md) | SymbolType | [INT16](DataType/CDXNumeric.md) |
|  | The type of symbol object.
This is an enumerated property. |  |  |
| 0x0A08 | [kCDXProp_Curve_Type](properties/Curve_Type.md) | CurveType | [INT16](DataType/CDXNumeric.md) |
|  | The type of curve object.
This is a bit-encoded property. |  |  |
| 0x0A20 | kCDXProp_Arrowhead_Size | HeadSize | [INT16](DataType/CDXNumeric.md) |
|  | The size of the arrow's head. |  |  |
| 0x0A21 | [kCDXProp_Arc_AngularSize](properties/Arc_AngularSize.md) | AngularSize | [INT16](DataType/CDXNumeric.md) |
|  | The size of an arc (in degrees * 10, so 90 degrees = 900). |  |  |
| 0x0A22 | [kCDXProp_Bracket_LipSize](properties/Bracket_LipSize.md) | LipSize | [INT16](DataType/CDXNumeric.md) |
|  | The size of a bracket. |  |  |
| 0x0A23 | [kCDXProp_Curve_Points](properties/Curve_Points.md) | CurvePoints | [CDXCurvePoints](DataType/CDXCurvePoints.md) |
|  | Required for curves.The Bézier curve's control point locations. |  |  |
| 0x0A24 | [kCDXProp_Bracket_Usage](properties/Bracket_Usage.md) | BracketUsage | [INT8](DataType/CDXNumeric.md) |
|  | The syntactical chemical meaning of the bracket (SRU, mer, mon, xlink, etc).
This is an enumerated property. |  |  |
| 0x0A25 | [kCDXProp_Polymer_RepeatPattern](properties/Polymer_RepeatPattern.md) | PolymerRepeatPattern | [INT8](DataType/CDXNumeric.md) |
|  | The head-to-tail connectivity of objects contained within the bracket.
This is an enumerated property. |  |  |
| 0x0A26 | [kCDXProp_Polymer_FlipType](properties/Polymer_FlipType.md) | PolymerFlipType | [INT8](DataType/CDXNumeric.md) |
|  | The flip state of objects contained within the bracket.
This is an enumerated property. |  |  |
| 0x0A27 | [kCDXProp_BracketedObjects](properties/BracketedObjects.md) | BracketedObjectIDs | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | The set of objects contained in a BracketedGroup. |  |  |
| 0x0A28 | [kCDXProp_Bracket_RepeatCount](properties/Bracket_RepeatCount.md) | RepeatCount | [FLOAT64](DataType/CDXNumeric.md) |
|  | The number of times a multiple-group BracketedGroup is repeated. |  |  |
| 0x0A29 | [kCDXProp_Bracket_ComponentOrder](properties/Bracket_ComponentOrder.md) | ComponentOrder | [INT16](DataType/CDXNumeric.md) |
|  | The component order associated with a BracketedGroup. |  |  |
| 0x0A2A | [kCDXProp_Bracket_SRULabel](properties/Bracket_SRULabel.md) | SRULabel | [CDXString](DataType/CDXString.md) |
|  | The label associated with a BracketedGroup that represents an SRU. |  |  |
| 0x0A2B | [kCDXProp_Bracket_GraphicID](properties/Bracket_GraphicID.md) | GraphicID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | The ID of a graphical object (bracket, brace, or parenthesis) associated with a Bracket Attachment. |  |  |
| 0x0A2C | [kCDXProp_Bracket_BondID](properties/Bracket_BondID.md) | BondID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | Required.The ID of a bond that crosses a Bracket Attachment. |  |  |
| 0x0A2D | [kCDXProp_Bracket_InnerAtomID](properties/Bracket_InnerAtomID.md) | InnerAtomID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | Required.The ID of the node located within the Bracketed Group and attached to a bond that crosses a Bracket Attachment. |  |  |
| 0x0A2E | [kCDXProp_Curve_Points3D](properties/Curve_Points3D.md) | CurvePoints3D | [CDXCurvePoints3D](DataType/CDXCurvePoints3D.md) |
|  | The Bézier curve's control point locations in 3D space. |  |  |
| 0x0A2F | kCDXProp_Arrowhead_Type | ArrowHeadType | [INT16](DataType/CDXNumeric.md) |
|  | The type of arrowhead.
This is an enumerated property. |  |  |
| 0x0A30 | kCDXProp_Arrowhead_CenterSize | HeadCenterSize | [UINT16](DataType/CDXNumeric.md) |
|  | The size of the arrow's head from the tip to the back of the head. |  |  |
| 0x0A31 | kCDXProp_Arrowhead_Width | HeadWidth | [UINT16](DataType/CDXNumeric.md) |
|  | The half-width of the arrow's head. |  |  |
| 0x0A32 | kCDXProp_ShadowSize | ShadowSize | [UINT16](DataType/CDXNumeric.md) |
|  | The size of the object's shadow. |  |  |
| 0x0A33 | kCDXProp_Arrow_ShaftSpacing | ArrowShaftSpacing | [UINT16](DataType/CDXNumeric.md) |
|  | The width of the space between a multiple-component arrow shaft, as in an equilibrium arrow. |  |  |
| 0x0A34 | kCDXProp_Arrow_EquilibriumRatio | ArrowEquilibriumRatio | [UINT16](DataType/CDXNumeric.md) |
|  | The ratio of the length of the left component of an equilibrium arrow (viewed from the end to the start) to the right component. |  |  |
| 0x0A35 | kCDXProp_Arrow_ArrowHead_Head | ArrowHeadHead | [INT16](DataType/CDXNumeric.md) |
|  | The type of arrowhead at the head of the arrow.
This is an enumerated property. |  |  |
| 0x0A36 | kCDXProp_Arrow_ArrowHead_Tail | ArrowHeadTail | [INT16](DataType/CDXNumeric.md) |
|  | The type of arrowhead at the tail of the arrow.
This is an enumerated property. |  |  |
| 0x0A37 | kCDXProp_Fill_Type | FillType | [INT16](DataType/CDXNumeric.md) |
|  | The type of the fill, for objects that can be filled.
This is an enumerated property. |  |  |
| 0x0A38 | kCDXProp_Curve_Spacing | CurveSpacing | [UINT16](DataType/CDXNumeric.md) |
|  | The width of the space between a a Doubled curve. |  |  |
| 0x0A38 | kCDXProp_Closed | Closed | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Signifies whether object is closed. |  |  |
| 0x0A3A | kCDXProp_Arrow_Dipole | Dipole | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Signifies whether the arrow is a dipole arrow. |  |  |
| 0x0A3B | kCDXProp_Arrow_NoGo | NoGo | [INT8](DataType/CDXNumeric.md) |
|  | Signifies whether arrow is a no-go arrow, and the type of no-go (crossed-through or hashed-out) if so.
This is an enumerated property. |  |  |
| 0x0A3C | kCDXProp_CornerRadius | CornerRadius | [INT16](DataType/CDXNumeric.md) |
|  | The radius of the rounded corner of a rounded rectangle. |  |  |
| 0x0A3D | kCDXProp_Frame_Type | FrameType | [INT16](DataType/CDXNumeric.md) |
|  | The type of frame on an object.
This is an enumerated property. |  |  |
| 0x0A60 | [kCDXProp_Picture_Edition](properties/Picture_Edition.md) | Edition | [Unformatted](DataType/Unformatted.md) |
|  | The section information (SectionHandle) of the Macintosh Publish & Subscribe edition embedded in the CDX picture object. |  |  |
| 0x0A61 | [kCDXProp_Picture_EditionAlias](properties/Picture_EditionAlias.md) | EditionAlias | [Unformatted](DataType/Unformatted.md) |
|  | The alias information of the Macintosh Publish & Subscribe edition embedded in the CDX picture object. |  |  |
| 0x0A62 | [kCDXProp_MacPICT](properties/MacPICT.md) | MacPICT | [Unformatted](DataType/Unformatted.md) |
|  | A Macintosh PICT data object. |  |  |
| 0x0A63 | [kCDXProp_WindowsMetafile](properties/WindowsMetafile.md) | WindowsMetafile | [Unformatted](DataType/Unformatted.md) |
|  | A Microsoft Windows Metafile object. |  |  |
| 0x0A64 | [kCDXProp_OLEObject](properties/OLEObject.md) | OLEObject | [Unformatted](DataType/Unformatted.md) |
|  | An OLE object. |  |  |
| 0x0A65 | [kCDXProp_EnhancedMetafile](properties/EnhancedMetafile.md) | EnhancedMetafile | [Unformatted](DataType/Unformatted.md) |
|  | A Microsoft Windows Enhanced Metafile object. |  |  |
| 0x0A6E | [kCDXProp_GIF](properties/GIF.md) | GIF | [Unformatted](DataType/Unformatted.md) |
|  | A binary GIF data object. |  |  |
| 0x0A6F | [kCDXProp_TIFF](properties/TIFF.md) | TIFF | [Unformatted](DataType/Unformatted.md) |
|  | A binary TIFF data object. |  |  |
| 0x0A70 | [kCDXProp_PNG](properties/PNG.md) | PNG | [Unformatted](DataType/Unformatted.md) |
|  | A binary PNG data object. |  |  |
| 0x0A71 | [kCDXProp_JPEG](properties/JPEG.md) | JPEG | [Unformatted](DataType/Unformatted.md) |
|  | A binary JPEG data object. |  |  |
| 0x0A72 | [kCDXProp_BMP](properties/BMP.md) | BMP | [Unformatted](DataType/Unformatted.md) |
|  | A binary BMP data object. |  |  |
| 0x0A80 | [kCDXProp_Spectrum_XSpacing](properties/Spectrum_XSpacing.md) | XSpacing | [FLOAT64](DataType/CDXNumeric.md) |
|  | Required for spectra.The spacing in logical units (ppm, Hz, wavenumbers) between points along the X-axis of an evenly-spaced grid. |  |  |
| 0x0A81 | [kCDXProp_Spectrum_XLow](properties/Spectrum_XLow.md) | XLow | [FLOAT64](DataType/CDXNumeric.md) |
|  | Required for spectra.The first data point for the X-axis of an evenly-spaced grid. |  |  |
| 0x0A82 | [kCDXProp_Spectrum_XType](properties/Spectrum_XType.md) | XType | [INT16](DataType/CDXNumeric.md) |
|  | The type of units the X-axis represents.
This is an enumerated property. |  |  |
| 0x0A83 | [kCDXProp_Spectrum_YType](properties/Spectrum_YType.md) | YType | [INT16](DataType/CDXNumeric.md) |
|  | The type of units the Y-axis represents.
This is an enumerated property. |  |  |
| 0x0A84 | [kCDXProp_Spectrum_XAxisLabel](properties/Spectrum_XAxisLabel.md) | XAxisLabel | [CDXString](DataType/CDXString.md) |
|  | A label for the X-axis. |  |  |
| 0x0A85 | [kCDXProp_Spectrum_YAxisLabel](properties/Spectrum_YAxisLabel.md) | YAxisLabel | [CDXString](DataType/CDXString.md) |
|  | A label for the Y-axis. |  |  |
| 0x0A86 | [kCDXProp_Spectrum_DataPoint](properties/Spectrum_DataPoint.md) | (not used) | [FLOAT64](DataType/CDXNumeric.md) |
|  | Required for spectra.The Y-axis values for the spectrum. It is an array of double values corresponding to X-axis values. |  |  |
| 0x0A87 | [kCDXProp_Spectrum_Class](properties/Spectrum_Class.md) | Class | [INT16](DataType/CDXNumeric.md) |
|  | The type of spectrum represented.
This is an enumerated property. |  |  |
| 0x0A88 | [kCDXProp_Spectrum_YLow](properties/Spectrum_YLow.md) | YLow | [FLOAT64](DataType/CDXNumeric.md) |
|  | Y value to be used to offset data when storing XML. |  |  |
| 0x0A89 | [kCDXProp_Spectrum_YScale](properties/Spectrum_YScale.md) | YScale | [FLOAT64](DataType/CDXNumeric.md) |
|  | Y scaling used to scale data when storing XML. |  |  |
| 0x0AA0 | [kCDXProp_TLC_OriginFraction](properties/TLC_OriginFraction.md) | OriginFraction | [FLOAT64](DataType/CDXNumeric.md) |
|  | The distance of the origin line from the bottom of a TLC Plate, as a fraction of the total height of the plate. |  |  |
| 0x0AA1 | [kCDXProp_TLC_SolventFrontFraction](properties/TLC_SolventFrontFraction.md) | SolventFrontFraction | [FLOAT64](DataType/CDXNumeric.md) |
|  | The distance of the solvent front from the top of a TLC Plate, as a fraction of the total height of the plate. |  |  |
| 0x0AA2 | [kCDXProp_TLC_ShowOrigin](properties/TLC_ShowOrigin.md) | ShowOrigin | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the origin line near the base of the TLC Plate if non-zero. |  |  |
| 0x0AA3 | [kCDXProp_TLC_ShowSolventFront](properties/TLC_ShowSolventFront.md) | ShowSolventFront | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the solvent front line near the top of the TLC Plate if non-zero. |  |  |
| 0x0AA4 | [kCDXProp_TLC_ShowBorders](properties/TLC_ShowBorders.md) | ShowBorders | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show borders around the edges of the TLC Plate if non-zero. |  |  |
| 0x0AA5 | [kCDXProp_TLC_ShowSideTicks](properties/TLC_ShowSideTicks.md) | ShowSideTicks | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show tickmarks up the side of the TLC Plate if non-zero. |  |  |
| 0x0AB0 | [kCDXProp_TLC_Rf](properties/TLC_Rf.md) | Rf | [FLOAT64](DataType/CDXNumeric.md) |
|  | The Retention Factor (Rf) of an individual spot. |  |  |
| 0x0AB1 | [kCDXProp_TLC_Tail](properties/TLC_Tail.md) | Tail | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The length of the "tail" of an individual spot. |  |  |
| 0x0AB2 | [kCDXProp_TLC_ShowRf](properties/TLC_ShowRf.md) | ShowRf | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the spot's Retention Fraction (Rf) value if non-zero.. |  |  |
| 0x0B00 | [kCDXProp_NamedAlternativeGroup_TextFrame](properties/NamedAlternativeGroup_TextFrame.md) | TextFrame | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The bounding box of upper portion of the Named Alternative Group, containing the name of the group. |  |  |
| 0x0B01 | [kCDXProp_NamedAlternativeGroup_GroupFrame](properties/NamedAlternativeGroup_GroupFrame.md) | GroupFrame | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The bounding box of the lower portion of the Named Alternative Group, containing the definition of the group. |  |  |
| 0x0B02 | [kCDXProp_NamedAlternativeGroup_Valence](properties/NamedAlternativeGroup_Valence.md) | Valence | [INT16](DataType/CDXNumeric.md) |
|  | The number of attachment points in each alternative in a named alternative group. |  |  |
| 0x0B80 | [kCDXProp_GeometricFeature](properties/GeometricFeature.md) | GeometricFeature | [INT8](DataType/CDXNumeric.md) |
|  | The type of the geometrical feature (point, line, plane, etc.).
This is an enumerated property. |  |  |
| 0x0B81 | [kCDXProp_RelationValue](properties/RelationValue.md) | RelationValue | [FLOAT64](DataType/CDXNumeric.md) |
|  | The numeric relationship (if any) among the basis objects used to define this object. |  |  |
| 0x0B82 | [kCDXProp_BasisObjects](properties/BasisObjects.md) | BasisObjects | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Required for geometries and constraints.An ordered list of objects used to define this object. |  |  |
| 0x0B83 | [kCDXProp_ConstraintType](properties/ConstraintType.md) | ConstraintType | [INT8](DataType/CDXNumeric.md) |
|  | The constraint type (distance, angle, or exclusion sphere).
This is an enumerated property. |  |  |
| 0x0B84 | [kCDXProp_ConstraintMin](properties/ConstraintMin.md) | ConstraintMin | [FLOAT64](DataType/CDXNumeric.md) |
|  | The minimum value of the constraint. |  |  |
| 0x0B85 | [kCDXProp_ConstraintMax](properties/ConstraintMax.md) | ConstraintMax | [FLOAT64](DataType/CDXNumeric.md) |
|  | The maximum value of the constraint. |  |  |
| 0x0B86 | [kCDXProp_IgnoreUnconnectedAtoms](properties/IgnoreUnconnectedAtoms.md) | IgnoreUnconnectedAtoms | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether unconnected atoms should be ignored within the exclusion sphere. |  |  |
| 0x0B87 | [kCDXProp_DihedralIsChiral](properties/DihedralIsChiral.md) | DihedralIsChiral | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | Signifies whether a dihedral is signed or unsigned. |  |  |
| 0x0B88 | [kCDXProp_PointIsDirected](properties/PointIsDirected.md) | PointIsDirected | [CDXBooleanImplied](DataType/CDXBoolean.md) |
|  | For a point based on a normal, signifies whether it is in a specific direction relative to the reference point. |  |  |
| 0x0BB0 | [kCDXProp_ChemicalPropertyType](properties/ChemicalPropertyType.md) | ChemicalPropertyType | [UINT32](DataType/CDXNumeric.md) |
|  | The type of property (name, formula, molecular weight, etc.).
This is an enumerated property. |  |  |
| 0x0BB1 | [kCDXProp_ChemicalPropertyDisplayID](properties/ChemicalPropertyDisplayID.md) | ChemicalPropertyDisplayID | [CDXObjectID](DataType/CDXObjectID.md) |
|  | The ID of a graphical object used to display the property value. |  |  |
| 0x0BB2 | [kCDXProp_ChemicalPropertyIsActive](properties/ChemicalPropertyIsActive.md) | ChemicalPropertyIsActive | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Whether the property should be recalculated in response to changes in the basis objects. |  |  |
| 0x0C00 | [kCDXProp_ReactionStep_Atom_Map](properties/ReactionStep_Atom_Map.md) | ReactionStepAtomMap | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to to a product atom. |  |  |
| 0x0C01 | [kCDXProp_ReactionStep_Reactants](properties/ReactionStep_Reactants.md) | ReactionStepReactants | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An order list of reactants present in the Reaction Step. |  |  |
| 0x0C02 | [kCDXProp_ReactionStep_Products](properties/ReactionStep_Products.md) | ReactionStepProducts | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An order list of products present in the Reaction Step. |  |  |
| 0x0C03 | [kCDXProp_ReactionStep_Plusses](properties/ReactionStep_Plusses.md) | ReactionStepPlusses | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of pluses used to separate components of the Reaction Step. |  |  |
| 0x0C04 | [kCDXProp_ReactionStep_Arrows](properties/ReactionStep_Arrows.md) | ReactionStepArrows | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An ordered list of arrows used to separate components of the Reaction Step. |  |  |
| 0x0C05 | [kCDXProp_ReactionStep_ObjectsAboveArrow](properties/ReactionStep_ObjectsAboveArrow.md) | ReactionStepObjectsAboveArrow | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An order list of objects above the arrow in the Reaction Step. |  |  |
| 0x0C06 | [kCDXProp_ReactionStep_ObjectsBelowArrow](properties/ReactionStep_ObjectsBelowArrow.md) | ReactionStepObjectsBelowArrow | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An order list of objects below the arrow in the Reaction Step. |  |  |
| 0x0C07 | [kCDXProp_ReactionStep_Atom_Map_Manual](properties/ReactionStep_Atom_Map_Manual.md) | ReactionStepAtomMapManual | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to to a product atom. |  |  |
| 0x0C08 | [kCDXProp_ReactionStep_Atom_Map_Auto](properties/ReactionStep_Atom_Map_Auto.md) | ReactionStepAtomMapAuto | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | Represents pairs of mapped atom IDs; each pair is a reactant atom mapped to to a product atom. |  |  |
| 0x0D00 | [kCDXProp_ObjectTag_Type](properties/ObjectTag_Type.md) | TagType | [INT16](DataType/CDXNumeric.md) |
|  | The tag's data type.
This is an enumerated property. |  |  |
| 0x0D03 | [kCDXProp_ObjectTag_Tracking](properties/ObjectTag_Tracking.md) | Tracking | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The tag will participate in tracking if non-zero. |  |  |
| 0x0D04 | [kCDXProp_ObjectTag_Persistent](properties/ObjectTag_Persistent.md) | Persistent | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The tag will be resaved to a CDX file if non-zero. |  |  |
| 0x0D05 | [kCDXProp_ObjectTag_Value](properties/ObjectTag_Value.md) | Value | varies |
|  | The value is a INT32, FLOAT64 or unformatted string depending on the value of ObjectTag_Type. |  |  |
| 0x0D06 | [kCDXProp_Positioning](properties/Positioning.md) | PositioningType | [INT8](DataType/CDXNumeric.md) |
|  | How the object should be positioned with respect to its containing object.
This is an enumerated property. |  |  |
| 0x0D07 | [kCDXProp_PositioningAngle](properties/PositioningAngle.md) | PositioningAngle | [INT32](DataType/CDXNumeric.md) |
|  | Angular positioning, in degrees * 65536. |  |  |
| 0x0D08 | [kCDXProp_PositioningOffset](properties/PositioningOffset.md) | PositioningOffset | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | Offset positioning. |  |  |
| 0x0E00 | [kCDXProp_Sequence_Identifier](properties/Sequence_Identifier.md) | SequenceIdentifier | [CDXString](DataType/CDXString.md) |
|  | Required for sequences.A unique (but otherwise random) identifier for a given Sequence object. |  |  |
| 0x0F00 | [kCDXProp_CrossReference_Container](properties/CrossReference_Container.md) | CrossReferenceContainer | [CDXString](DataType/CDXString.md) |
|  | An external object containing (as an embedded object) the document containing the Sequence object being referenced. |  |  |
| 0x0F01 | [kCDXProp_CrossReference_Document](properties/CrossReference_Document.md) | CrossReferenceDocument | [CDXString](DataType/CDXString.md) |
|  | An external document containing the Sequence object being referenced. |  |  |
| 0x0F02 | [kCDXProp_CrossReference_Identifier](properties/CrossReference_Identifier.md) | CrossReferenceIdentifier | [CDXString](DataType/CDXString.md) |
|  | Required for cross-references..A unique (but otherwise random) identifier for a given Cross-Reference object. |  |  |
| 0x0F03 | [kCDXProp_CrossReference_Sequence](properties/CrossReference_Sequence.md) | CrossReferenceSequence | [CDXString](DataType/CDXString.md) |
|  | Required for cross-references..A value matching the SequenceIdentifier of the Sequence object to be referenced. |  |  |
| 0x1000 | [kCDXProp_Template_PaneHeight](properties/Template_PaneHeight.md) | PaneHeight | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | Required for templategrids.The height of the viewing window of a template grid. |  |  |
| 0x1001 | [kCDXProp_Template_NumRows](properties/Template_NumRows.md) | NumRows | [INT16](DataType/CDXNumeric.md) |
|  | Required for templategrids.The number of rows of the CDX TemplateGrid object. |  |  |
| 0x1002 | [kCDXProp_Template_NumColumns](properties/Template_NumColumns.md) | NumColumns | [INT16](DataType/CDXNumeric.md) |
|  | Required for templategrids.The number of columns of the CDX TemplateGrid object. |  |  |
| 0x1100 | [kCDXProp_Group_Integral](properties/Group_Integral.md) | Integral | [CDXBoolean](DataType/CDXBoolean.md) |
|  | The group is considered to be integral (non-subdivisible) if non-zero. |  |  |
| 0x1FF0 | [kCDXProp_SplitterPositions](properties/SplitterPositions.md) | SplitterPositions | [CDXObjectIDArray](DataType/CDXObjectID.md) |
|  | An array of vertical positions that subdivide a page into regions. |  |  |
| 0x1FF1 | [kCDXProp_PageDefinition](properties/PageDefinition.md) | PageDefinition | [INT8](DataType/CDXNumeric.md) |
|  | A description of the type of formatting used by the page, or by the splitter.
This is an enumerated property. |  |  |
| 0x206 | [kCDXProp_BoundsInParent](properties/BoundsInParent.md) | BoundsInParent | [CDXRectangle](DataType/CDXCoordinates.md) |
|  | The rectangle containing a page in the coordinate space of the containing page. |  |  |

---

[CDX Documentation index](index.md)