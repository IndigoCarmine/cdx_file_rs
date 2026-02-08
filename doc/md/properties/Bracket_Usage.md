## Bracket_Usage Property

| CDXML Name: | BracketUsage |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bracket_Usage |
| CDX Constant Value: | 0x0A24 |
| Data Size: | [INT8](/web/20190608235139/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Graphic](/web/20190608235139/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md),[kCDXObj_BracketedGroup](/web/20190608235139/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | No |

**Description:**

The syntactical chemical meaning of the bracket (SRU, mer, mon, xlink, etc.).

In a general sense, brackets (and by extension, parentheses and braces) enclose structures or structural fragments that are to be repeated some number of times. The CDX file format offers several properties of brackets that allow users to specify the context of those repeating units, as well as their orientation.

There are two main representations for polymers, *structure-based* and *source-based*. There is little benefit of one form over the other, and both have been widely used.

Structure-based representations enclose only a portion of a structure within brackets. One or more bonds will cross each of the brackets that enclose the structure. With such a representation, those crossing bonds "match up," making the polymer structure more explicit. Structure-based representations are created in ChemDraw by enclosing a portion of a structure in brackets with a Bracket usage of kCDXBracketUsage_SRU. The subscript "n" is the notation recommended by IUPAC for structure-based representations of unknown size, but that text can be specified via the [kCDXProp_Bracket_SRULabel](Bracket_SRULabel.md) property.

*Poly(vinyl chloride), structure-based representation*

The Bracket Usage types kCDXBracketUsage_Crosslink, kCDXBracketUsage_Graft, and kCDXBracketUsage_Modification can also be used to represent specific types of repeating units within a structure-based representation, but they are rarely encountered and their use is discouraged. For more information about these types, please consult the references at the bottom.

In a source-based representation, an entire discrete chemical substance is enclosed within the brackets. The mode of polymerization might be obvious from context (for example, the breaking of a double bond to form a diradical), or it might be unknown. Source-based representations are particularly useful when describing processes where the starting materials are known, but the exact structure of the reaction product is not. Source-based representations are most commonly created in ChemDraw by enclosing a structure in brackets with a Bracket Usage of kCDXBracketUsage_Monomer.

*Poly(vinyl chloride), source-based representation*

Brackets with a Bracket Usage of kCDXBracketUsage_Mer are also used for source based representations, but their usage is limited to components of a copolymer where those individual components are known not to repeat with themselves (that is, they must alternate with the other component(s).

Copolymers, as above, represent substances with more than one repeating unit. For most purposes, the Bracket Usage kCDXBracketUsage_Copolymer will be sufficient. The Bracket Usage types kCDXBracketUsage_CopolymerAlternating, kCDXBracketUsage_CopolymerBlock, and kCDXBracketUsage_CopolymerRandom can also be used to represent specific types of copolymers within a source-based representation, but they are rarely encountered and their use is discouraged. For more information about these types, please consult the references at the bottom.

The Bracket Usage of kCDXBracketUsage_MixtureUnordered is not used for polymers, but to represent a collection of substances that might all be present, but not necessarily in known amounts. It could be used, for example, to indicate the products of a reaction. Brackets with a Bracket Usage of kCDXBracketUsage_Component are used to indicate individual elements within a mixture.

The Bracket Usage of kCDXBracketUsage_MixtureOrdered is primarily used to describe manufacturing processes, where the ordering of components is a critical part of the process (and where a different ordering might produce a different final product!). As with Unordered Mixtures, individual elements of an Ordered Mixture should be surrounded by brackets with a Bracket Usage of kCDXBracketUsage_Component. Unlike with Unordered Mixtures, however, components of Ordered Mixtures must represent their ordering via the [kCDXProp_Bracket_ComponentOrder](Bracket_ComponentOrder.md) property.

Finally, the Bracket Usage of kCDXBracketUsage_MultipleGroup is used to indicate that the enclosed items are repeated a specific known number of times. Multiple Group brackets may enclose entire structures, or may enclose a portion of the structures only. The numeric repeat count can be specified via the [kCDXProp_Bracket_RepeatCount](Bracket_RepeatCount.md) property.

**Some references for chemical meanings of polymers and polymer representations:**

"Graphic Representations (Chemical Formulae) of Macromolecules (Recommendations 1994)" *Pure Appl. Chem.,***66**, 2469-2482 (1994).

"Source-Based Nomenclature for Copolymers (Recommendations 1985)" *Pure Appl. Chem.,***57**, 1427-1440 (1985). (also available at http://www.iupac.org/publications/books/pbook/PurpleBook-C7.pdf)

"Basic Definitions of Terms Relating to Polymers (1974)" *Pure Appl. Chem.,***40, **479-491 (1974) (also available at http://www.iupac.org/reports/1996/6812jenkins/index.md).

See also Guide for the authors of papers and reports in polymer science and technology (from IUPAC)

Although this property is written to both [Graphic](/web/20190608235139/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Graphic.md) objects and to [Bracketed Groups](/web/20190608235139/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/BracketedGroup.md), as of ChemDraw 7.0 it is read only from Graphic objects.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Unspecified bracket usage |
| 1 | Unused1 | (unused) |
| 2 | Unused2 | (unused) |
| 3 | SRU | Brackets enclose a Structural Repeating Unit in a structure-based representation |
| 4 | Monomer | Brackets enclose a source-based monomeric unit |
| 5 | Mer | Brackets enclose a source-based monomeric unit that is known not to self-polymerize |
| 6 | Copolymer | Brackets enclose one of several repeating units that co-polymerize |
| 7 | CopolymerAlternating | Brackets enclose one of several repeating units that co-polymerize in an alternating fashion |
| 8 | CopolymerRandom | Brackets enclose one of several repeating units that co-polymerize in a random fashion |
| 9 | CopolymerBlock | Brackets enclose one of several repeating units that co-polymerize in a block fashion |
| 10 | Crosslink | Brackets enclose a cross-linking repeating unit in a source-based representation |
| 11 | Graft | Brackets enclose a graft repeating unit in a source-based representation |
| 12 | Modification | Brackets enclose a modified repeating unit in a source-based representation |
| 13 | Component | Brackets enclose an individual component of an ordered or unordered mixture |
| 14 | MixtureUnordered | Brackets enclose a collection of substances that comprise an unordered mixture |
| 15 | MixtureOrdered | Brackets enclose a collection of substances that comprise an ordered mixture (also called a formulation) |
| 16 | MultipleGroup | Brackets enclose a structure or fragment that is repeated some number of times |
| 17 | Generic | Brackets enclose a generic polymer |
| 18 | Anypolymer | Brackets enclose any polymer |

**If this property is absent:**

The bracket usage is treated as Unspecified.

---

[CDX Documentation index](/web/20190608235139/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)
