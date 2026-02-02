CDX Format Specification: Predefined Objects
## Predefined Objects

| Object | Value | Name | CDXML Name |
| --- | --- | --- | --- |
| [Document](Document.md) | 0x8000 | kCDXObj_Document | CDXML |
| The top-level CDX object. It contains all CDX attributes and objects. |  |  |  |
| [Page](Page.md) | 0x8001 | kCDXObj_Page | page |
| A drawing space that can contain other objects. |  |  |  |
| [Group](Group.md) | 0x8002 | kCDXObj_Group | group |
| A logical collection of objects. |  |  |  |
| [Fragment](Fragment.md) | 0x8003 | kCDXObj_Fragment | fragment |
| A collection of nodes and their connectivity (bonds). |  |  |  |
| [Node](Node.md) | 0x8004 | kCDXObj_Node | n |
| The basic building block of chemical objects, usually referring to a single atom |  |  |  |
| [Bond](Bond.md) | 0x8005 | kCDXObj_Bond | b |
| A connection between two Node objects. |  |  |  |
| [Text](Text.md) | 0x8006 | kCDXObj_Text | t |
| An arbitrary block of (possibly styled) text. |  |  |  |
| [Graphic](Graphic.md) | 0x8007 | kCDXObj_Graphic | graphic |
| A (generally non-chemical) graphic object such as a line, arc, circle, or rectangle. |  |  |  |
| [Bracketed Group](BracketedGroup.md) | 0x8017 | kCDXObj_BracketedGroup | bracketedgroup |
| A collection of objects surrounded by brackets (or braces or parentheses). |  |  |  |
| [Bracket Attachment](BracketAttachment.md) | 0x8018 | kCDXObj_BracketAttachment | bracketattachment |
| A linkage that connects a Bracketed Group to some object outside that group. |  |  |  |
| [Crossing Bond](CrossingBond.md) | 0x8019 | kCDXObj_CrossingBond | crossingbond |
| A Bond that connects a Bracketed Group to a Node outside that group. |  |  |  |
| [Curve](Curve.md) | 0x8008 | kCDXObj_Curve | curve |
| A B zier curve. |  |  |  |
| [Embedded Object](EmbeddedObject.md) | 0x8009 | kCDXObj_EmbeddedObject | embeddedobject |
| A Macintosh PICT, a Windows Metafile, or a Windows OLE Object. |  |  |  |
| [Table](Table.md) | 0x8016 | kCDXObj_Table | table |
| A grid-like arrangement of drawing spaces. |  |  |  |
| [Named Alternative Group](NamedAltGroup.md) | 0x800A | kCDXObj_NamedAlternativeGroup | altgroup |
| A container object holding fragments that represent alternative substituents for a query. |  |  |  |
| [Template Grid](TemplateGrid.md) | 0x800B | kCDXObj_TemplateGrid | templategrid |
| A TemplateGrid indicates how multiple CDX page objects should be arranged in a Template document. |  |  |  |
| [Registry Number](RegistryNumber.md) | 0x800C | kCDXObj_RegistryNumber | regnum |
| A registry or catalog number, along with the name of the authority which issued the number. |  |  |  |
| [Reaction Scheme](ReactionScheme.md) | 0x800D | kCDXObj_ReactionScheme | scheme |
| A description of a single- or multi-step reaction, containing one or more Reaction Steps. |  |  |  |
| [Reaction Step](ReactionStep.md) | 0x800E | kCDXObj_ReactionStep | step |
| A description of one step in a reaction. It contains the constituents, conditions, and products. |  |  |  |
| [Spectrum](Spectrum.md) | 0x8010 | kCDXObj_Spectrum | spectrum |
| An NMR, MS, IR or other sort of spectral plot. |  |  |  |
| [Object Tag](ObjectTag.md) | 0x8011 | kCDXObj_ObjectTag | objecttag |
| Arbitrarily named property, one or more of which can be attached to any ChemDraw object. |  |  |  |
| [Sequence](Sequence.md) | 0x8013 | kCDXObj_Sequence | sequence |
| One member of an ordered series; the contents of its 
Text object may change as other objects are added to or removed from the
 series. |  |  |  |
| [Cross-Reference](CrossReference.md) | 0x8014 | kCDXObj_CrossReference | crossreference |
| A link to some Sequence object. |  |  |  |
| [Border](Border.md) | 0x8020 | kCDXObj_Border | border |
| A collection of information describing one edge of an object. |  |  |  |
| [Geometry](Geometry.md) | 0x8021 | kCDXObj_Geometry | geometry |
| A geometrical relationship between one or more objects. |  |  |  |
| [Constraint](Constraint.md) | 0x8022 | kCDXObj_Constraint | constraint |
| A distance or angle constraint between one or more objects. |  |  |  |
| [TLC Plate](TLCPlate.md) | 0x8023 | kCDXObj_TLCPlate | tlcplate |
| A rectangular object representing a Thin Layer Chromatography (TLC) plate. |  |  |  |
| [TLC Lane](TLCLane.md) | 0x8024 | kCDXObj_TLCLane | tlclane |
| A logical object representing a series of spots arranged vertically on a TLC plate. |  |  |  |
| [TLC Spot](TLCSpot.md) | 0x8025 | kCDXObj_TLCSpot | tlcspot |
| A single spot on a TLC plate. |  |  |  |
| [Splitter](Splitter.md) | 0x8015 | kCDXObj_Splitter | splitter |
| An object that divides the page into horizontal bands |  |  |  |
| [Chemical Property](ChemicalProperty.md) | 0x8026 | kCDXObj_ChemicalProperty | chemicalproperty |
| A (physical) chemical property associated with a collection of objects, usually atoms and bonds. |  |  |  |
| [Color Table](ColorTable.md) | 0x0300 | kCDXProp_ColorTable | colortable |
| The color palette used throughout the document. Color 
indexes 0 and 1 always correspond to black and white and are not saved 
in the color table. The first and second RGB values (color indexes 2 and
 3) are the default background and foreground colors, and other colors 
are numbered sequentially. |  |  |  |
| [Color](Color.md) | n/a | n/a | color |
| An RGB color. |  |  |  |
| [Font Table](FontTable.md) | 0x0100 | kCDXProp_FontTable | fonttable |
| A list of fonts used in the document. |  |  |  |
| [Font](Font.md) | n/a | n/a | font |
| A logical font definition. |  |  |  |
| [Style](Style.md) | n/a | n/a | s |
| A string of text in exactly one style. |  |  |  |
| [Represents Property](Represent.md) | 0x000e | kCDXProp_RepresentsProperty | represent |
| An object used to indicate that its containing object has chemical meaning that is also represented in another object. |  |  |  |
| Arrow | 0x8027 | kCDXObj_Arrow | arrow |
| A line or arc, optionally with arrowheads on one or both ends |  |  |  |

---

[CDX Documentation index](index.md)