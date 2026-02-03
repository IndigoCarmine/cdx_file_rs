CDX Format Specification: Node Object
## Node Object

| CDXML Name: | n |
| --- | --- |
| CDX Constant Name: | kCDXObj_Node |
| CDX Constant Value: | 0x8004 |
| Contained by objects: | [kCDXObj_Fragment](Fragment.md) |
| First written/read in: | ChemDraw 4.0 |

**Description:**  

A Node object is the basic building block of CDX chemical objects. At its most basic level, a Node is "a thing at the end of a bond." Most commonly, a Node will represent a single atom, but it may have other chemical meanings according to the value of the [kCDXProp_Node_Type](properties/Node_Type.md) property.

When looking at a Node, there are really two places to find most types of information. Consider a node with the atom label (a contained [Text](Text.md) object) CH2. The Text object contains the three characters 'C', 'H', and '2'. You could take the Text object itself and -- with no other information -- figure out that this node represents a carbon atom and two attached hydrogens. For such a simple case, that sort of analysis is easy, but things can get very complicated very quickly.

For that reason, ChemDraw will always *write* redundant information. It is always safe to ignore the label entirely when interpreting ChemDraw-generated files. Instead, you can find out that this node is a carbon by looking at the [kCDXProp_Node_Element](properties/Node_Element.md) property, and you can find the number of attached hydrogens by looking at the [kCDXProp_Atom_NumHydrogens](properties/Atom_NumHydrogens.md) property.

All Nodes must be contained in [Fragment](Fragment.md) objects; they cannot be stored directly in [Page](Page.md) or [Fragment](Fragment.md) objects. This redundancy extends across all node types.

When *reading* files, ChemDraw does two things. If a contained Text object is present, ChemDraw uses that object as is as the Node's label, and ignores the chemical properties entirely. If a Text object is not present, ChemDraw then looks to the chemical properties and creates an atom label on the fly, if appropriate. Accordingly, programs writing their own CDX files are welcome to write either a Text object or a collection of chemical properties, whichever is most convenient. Of course, they can write out both, too, just like ChemDraw does -- but they do not have to do so.

Most Node objects have no required properties or objects, but nodes representing multicenter attachments or variable attachment points require the [kCDXProp_Node_Attachments](properties/Node_Attachments.md) property.

**Subobjects:**  

| Value | Name | CDXML Name |  |
| --- | --- | --- | --- |
| 0x8003 | [kCDXObj_Fragment](Fragment.md) | fragment |  |
|  | A collection of nodes and their connectivity (bonds). |  |  |
| 0x8006 | [kCDXObj_Text](Text.md) | t |  |
|  | An arbitrary block of (possibly styled) text. |  |  |
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
| 0x0200 | [kCDXProp_2DPosition](properties/2DPosition.md) | p | [CDXPoint2D](DataType/CDXCoordinates.md) |
|  | The 2D location (in the order of vertical and horizontal locations) of an object. |  |  |
| 0x0201 | [kCDXProp_3DPosition](properties/3DPosition.md) | xyz | [CDXPoint3D](DataType/CDXCoordinates.md) |
|  | The 3D location (in the order of X-, Y-, and Z-locations in right-handed coordinate system) of an object in CDX coordinate units. The precise meaning of this attribute varies depending on the type of object. |  |  |
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
| 0x0445 | [kCDXProp_Atom_ShowEnhancedStereo](properties/Atom_ShowEnhancedStereo.md) | ShowAtomEnhancedStereo | [CDXBoolean](DataType/CDXBoolean.md) |
|  | Show the enhanced stereochemistry indicator if non-zero. |  |  |
| 0x0446 | [kCDXProp_Atom_EnhancedStereoType](properties/Atom_EnhancedStereoType.md) | EnhancedStereoType | [UINT8](DataType/CDXNumeric.md) |
|  | The type of enhanced stereochemistry present on this atom.
This is an enumerated property. |  |  |
| 0x0447 | [kCDXProp_Atom_EnhancedStereoGroupNum](properties/Atom_EnhancedStereoGroupNum.md) | EnhancedStereoGroupNum | [UINT16](DataType/CDXNumeric.md) |
|  | The group number associated with Or and And enhanced stereochemistry types. |  |  |
| 0x0807 | [kCDXProp_LineWidth](properties/LineWidth.md) | LineWidth | [CDXCoordinate](DataType/CDXCoordinates.md) |
|  | The default line width. |  |  |
| 0x080A | [kCDXProp_LabelStyle](properties/LabelStyle.md) | (not used) | [CDXFontStyle](DataType/CDXString.md) |
|  | The default style for atom labels.. |  |  |
| 0x081A | [kCDXProp_LabelStyleFont](properties/LabelStyleFont.md) | LabelFont | [INT16](DataType/CDXNumeric.md) |
|  | The default font family for atom labels. |  |  |
| 0x081C | [kCDXProp_LabelStyleSize](properties/LabelStyleSize.md) | LabelSize | [INT16](DataType/CDXNumeric.md) |
|  | The default font size for atom labels. |  |  |
| 0x081E | [kCDXProp_LabelStyleFace](properties/LabelStyleFace.md) | LabelFace | [INT16](DataType/CDXNumeric.md) |
|  | The default font style for atom labels. |  |  |

---

[CDX Documentation index](index.md)