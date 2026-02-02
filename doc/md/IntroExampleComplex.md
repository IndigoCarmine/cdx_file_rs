CDX Format Specification: A complex example
## A complex example

Given the following graphical drawing for dimethylamine:

Let us look at the CDX and CDXML files as produced by ChemDraw. Unlike the [previous example](IntroExampleSimple.md), all of the extraneous information has been left in these versions.

These files can be represented schematically like this:

Objects are circled and the properties they contain radiate from 
them. Object nesting is represented by hierarchy. The one and only 
fragment (ID 8) object holds the entire molecule. Nodes 4 and 6 are the 
two carbon atoms; Node 5 is the nitrogen. Note that Node 5 contains an 
additional two properties plus a nested Text object. Nodes 4 and 6 don't
 need to specify Element because carbon is the default element. Text 
object 7 represents the NH text in the drawing.

### The CDX version

[Original binary version](dimethylamine.cdx)

56 6A 43 44 30 31 30 30 04 03 02 01 00 00 00 00   

00 00 00 00 00 00 00 00 00 00 00 00 03 00 OE 00   

00 00 43 68 65 6D 44 72 61 77 20 37 2E 30 08 00   

0A 00 00 00 74 65 73 74 2E 63 64 78 00 03 32 00   

08 00 FF FF FF FF FF FF 00 00 00 00 00 00 FF FF   

00 00 00 00 FF FF FF FF 00 00 00 00 FF FF 00 00   

00 00 FF FF FF FF 00 00 00 00 FF FF FF FF 00 00   

FF FF 01 09 08 00 00 40 EC 00 00 C0 EA 00 02 09   

08 00 00 40 FD 01 00 00 25 02 02 08 10 00 00 00   

24 00 00 00 24 00 00 00 24 00 00 00 24 00 3A 04   

01 00 01 3B 04 01 00 00 3C 04 01 00 00 0C 06 01   

00 01 0D 06 01 00 00 06 07 02 00 00 00 07 07 02   

00 01 00 03 08 04 00 00 00 78 00 05 08 04 00 00   

00 1E 00 06 08 04 00 00 00 04 00 07 08 04 00 00   

00 01 00 08 08 04 00 00 00 02 00 09 08 04 00 33   

B3 02 00 0C 08 01 00 00 0D 08 00 00 23 08 01 00   

05 04 08 02 00 78 00 0A 08 08 00 03 00 60 00 C8   

00 03 00 0B 08 08 00 04 00 00 00 F0 00 03 00 00   

08 78 00 00 03 00 00 02 58 02 58 00 00 00 00 18   

F5 13 19 FF 9B FF 94 19 63 13 80 03 67 05 28 03   

FC 00 02 00 00 02 58 02 58 00 00 00 00 18 F5 13   

19 00 01 00 64 00 64 00 00 00 01 00 01 01 01 00   

00 00 01 27 0F 00 01 00 01 00 00 00 00 00 00 00   

00 00 00 00 00 00 02 00 19 01 90 00 00 00 00 00   

60 00 00 00 00 00 00 00 00 00 01 00 00 00 00 00   

00 00 00 00 00 00 00 00 00 00 00 04 02 10 00 00   

C0 96 00 C2 D7 74 00 2B 0C AF 00 3D 28 B4 00 00   

01 24 00 00 00 02 00 03 00 E4 04 05 00 41 72 69   

61 6C 04 00 E4 04 0F 00 54 69 6D 65 73 20 4E 65   

77 20 52 6F 6D 61 6E 01 80 01 00 00 00 04 02 10   

00 00 C0 96 00 C2 D7 74 00 2B 0C AF 00 3D 28 B4   

00 0F 08 02 00 01 00 10 08 02 00 01 00 16 08 04   

00 00 00 24 00 18 08 04 00 00 00 24 00 19 08 00   

00 03 80 08 00 00 00 04 02 10 00 00 40 98 00 C2   

57 76 00 2B 8C AD 00 3D A8 B2 00 04 80 05 00 00   

00 0A 00 02 00 01 00 00 02 08 00 00 40 A4 00 00   

80 94 00 06 80 11 00 00 00 0A 00 02 00 06 00 00   

07 0E 00 01 00 00 00 03 00 60 00 C8 00 03 00 4E   

48 04 07 06 00 02 00 02 00 03 00 05 07 01 00 04   

00 02 08 00 EE CE 9F 00 00 B0 91 00 04 02 10 00   

00 40 98 00 00 40 92 00 00 00 A8 00 00 80 97 00   

23 08 01 00 00 00 00 37 04 01 00 01 02 04 02 00   

07 00 2B 04 02 00 01 00 00 00 04 80 06 00 00 00   

0A 00 02 00 02 00 00 02 08 00 BA 03 AC 00 4F 7A   

B1 00 37 04 01 00 01 00 00 05 80 02 00 00 00 0A   

00 02 00 03 00 04 06 04 00 05 00 00 00 05 06 04   

00 06 00 00 00 0A 06 01 00 01 00 00 04 80 04 00   

00 00 0A 00 02 00 04 00 00 02 08 00 BA 03 AC 00   

B0 85 77 00 37 04 01 00 01 00 00 05 80 03 00 00   

00 0A 00 02 00 05 00 04 06 04 00 05 00 00 00 05   

06 04 00 04 00 00 00 0A 06 01 00 01 00 00 00 00   

00 00 00 00   

### The CDXML version

<?xml version="1.0" encoding="UTF-8" ?><!DOCTYPE CDXML SYSTEM "http://www.camsoft.com/xml/cdxml.dtd" >  

<CDXML  

 CreationProgram="ChemDraw 7.0d182"  

 Name="test.cdxml"  

 BoundingBox="116.84 150.75 180.16 175.05"  

 WindowPosition="234 236"  

 WindowSize="549 509"  

 FractionalWidths="yes"  

 ShowAtomQuery="yes"  

 ShowBondQuery="yes"  

 LabelFont="3"  

 LabelSize="10"  

 LabelFace="96"  

 CaptionFont="4"  

 CaptionSize="12"  

 HashSpacing="2.7"  

 MarginWidth="2"  

 LineWidth="1"  

 BoldWidth="4"  

 BondLength="30"  

 BondSpacing="12"  

 ChainAngle="120"  

 PrintMargins="36 36 36 36"  

 
MacPrintInfo="00030000025802580000000018F51319FF9BFF94196313800367052803FC00020000025802580000000018F51319000100640064000000010001010100000001270F000100010000000000000000000000000002001901900000000000600000000000000000000100000000000000000000000000000000"  

><colortable>  

<color r="1" g="1" b="1"/>  

<color r="0" g="0" b="0"/>  

<color r="1" g="0" b="0"/>  

<color r="1" g="1" b="0"/>  

<color r="0" g="1" b="0"/>  

<color r="0" g="1" b="1"/>  

<color r="0" g="0" b="1"/>  

<color r="1" g="0" b="1"/>  

</colortable><fonttable>  

<font id="3" charset="iso-8859-1" name="Arial"/>  

<font id="4" charset="iso-8859-1" name="Times New Roman"/>  

</fonttable><page  

 id="1"  

 BoundingBox="116.84 150.75 180.16 175.05"  

 HeaderPosition="36"  

 FooterPosition="36"  

 PrintTrimMarks="yes"  

 HeightPages="1"  

 WidthPages="1"  

><fragment  

 id="8"  

 BoundingBox="118.34 152.25 178.66 173.55"  

><n  

 id="5"  

 p="148.5 164.25"  

 Z="1"  

 Element="7"  

 NumHydrogens="1"  

 AS="N"  

><t  

 id="17"  

 p="145.69 159.81"  

 BoundingBox="146.25 152.25 151.5 168"  

 Z="6"  

 LabelAlignment="Above"  

 LineStarts="2 3"  

><s font="3" size="10" face="96">NH</s></t></n><n  

 id="6"  

 p="177.48 172.01"  

 Z="2"  

 AS="N"  

/><n  

 id="4"  

 p="119.52 172.01"  

 Z="4"  

 AS="N"  

/><b  

 id="2"  

 Z="3"  

 B="5"  

 E="6"  

 BS="N"  

/><b  

 id="3"  

 Z="5"  

 B="5"  

 E="4"  

 BS="N"  

/></fragment></page></CDXML>

### Comparison

Here is a side-by-side listing of the two file formats to highlight 
how similar they are. The CDX file is byte-for-byte identical to the 
original listing above.

The CDXML file has been rearranged to match the order of the CDX 
file. Since objects and fragments can be listed in any order, this 
reordering actually is almost a valid CDXML file in itself. The only 
major differences to note are the Color Table and the Font Table, both 
of which are properties in CDX but distinct objects in CDXML. Although 
the ordering of objects and properties is not important, an object 
cannot appear in the middle of a list of properties in CDXML files.

| 56 6A 43 44 30 31 30 30 04 03 02 01 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 | <?xml version="1.0" encoding="UTF-8" ?> <!DOCTYPE CDXML SYSTEM "http://www.camsoft.com/ xml/cdxml.dtd" > <CDXML |  |
| --- | --- | --- |
| 03 00 OE 00 00 00 43 68 65 6D 44 72 61 77 20 37 2E 30 | CreationProgram="ChemDraw 7.0" |  |
| 08 00 0A 00 00 00 74 65 73 74 2E 63 64 78 | Name="test.cdxml" |  |
| 00 03 32 00 08 00 FF FF FF FF FF FF 00 00 00 00 00 00 FF
FF 00 00 00 00 FF FF FF FF 00 00 00 00 FF FF 00 00 00 00 FF FF FF FF 00 00 00 00
FF FF FF FF 00 00 FF FF | <colortable> <color r="1" g="1" b="1"/> <color r="0" g="0" b="0"/> <color r="1" g="0" b="0"/> <color r="1" g="1" b="0"/> <color r="0" g="1" b="0"/> <color r="0" g="1" b="1"/> <color r="0" g="0" b="1"/> <color r="1" g="0" b="1"/> </colortable> |  |
| 01 09 08 00 00 40 EC 00 00 C0 EA 00 | WindowPosition="234 236" |  |
| 02 09 08 00 00 40 FD 01 00 00 25 02 | WindowSize="549 509" |  |
| 02 08 10 00 00 00 24 00 00 00 24 00 00 00 24 00 00 00 24
00 | PrintMargins="36 36 36 36" |  |
| 3A 04 01 00 01 | ShowAtomQuery="yes" |  |
| 3B 04 01 00 00 | ShowAtomStereo="no" | omitted from CDXML because this is the default |
| 3C 04 01 00 00 | ShowAtomNumber="no" | omitted from CDXML because this is the default |
| 0C 06 01 00 01 | ShowBondQuery="yes" |  |
| 0D 06 01 00 00 | ShowBondStereo="no" | omitted from CDXML because this is the default |
| 06 07 02 00 00 00 | LabelLineHeight="0" | omitted from CDXML because this is the default |
| 07 07 02 00 01 00 | CaptionLineHeight="1" | omitted from CDXML because this is the default |
| 03 08 04 00 00 00 78 00 | ChainAngle="120" |  |
| 05 08 04 00 00 00 1E 00 | BondLength="30" |  |
| 06 08 04 00 00 00 04 00 | BoldWidth="4" |  |
| 07 08 04 00 00 00 01 00 | LineWidth="1" |  |
| 08 08 04 00 00 00 02 00 | MarginWidth="2" |  |
| 09 08 04 00 33 B3 02 00 | HashSpacing="2.7" |  |
| 0C 08 01 00 00 | CaptionJustification="Left" | omitted from CDXML because this is the default |
| 0D 08 00 00 | FractionalWidths="yes" |  |
| 23 08 01 00 05 | LabelJustification="Auto" | omitted from CDXML because this is the default |
| 04 08 02 00 78 00 | BondSpacing="12" |  |
| 0A 08 08 00 03 00 60 00 C8 00 03 00 | LabelFont="3" LabelSize="10" LabelFace="96" |  |
| 0B 08 08 00 04 00 00 00 F0 00 03 00 | CaptionFont="4" CaptionSize="12" CaptionFace="0" | CaptionFace omitted from CDXML because it is the default |
| 00 08 78 00 00 03 00 00 02 58 02 58 00 00 00 00 18 F5 13
19 FF 9B FF 94 19 63 13 80 03 67 05 28 03 FC 00 02 00 00 02 58 02 58 00 00 00 00
18 F5 13 19 00 01 00 64 00 64 00 00 00 01 00 01 01 01 00 00 00 01 27 0F 00 01 00
01 00 00 00 00 00 00 00 00 00 00 00 00 00 02 00 19 01 90 00 00 00 00 00 60 00 00 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | MacPrintInfo=" 0003000002580258 0000000018F51319 FF9BFF9419631380 0367052803FC0002 0000025802580000 000018F513190001 0064006400000001 0001010100000001 270F000100010000 0000000000000000 0000000200190190 0000000000600000 0000000000000001 0000000000000000 0000000000000000" |  |
| 04 02 10 00 00 C0 96 00 C2 D7 74 00 2B 0C AF 00 3D 28 B4
00 | BoundingBox= "116.84 150.75 180.16 175.05" | Omitted from CDXML; written to CDX only for
backwards-compatibility |
| 00 01 24 00 00 00 02 00 03 00 E4 04 05 00 41 72 69 61 6C 04
00 E4 04 0F 00 54 69 6D 65 73 20 4E 65 77 20 52 6F 6D 61 6E | <fonttable> <font id="3" charset="iso-8859-1" name="Arial"/> <font id="4" charset="iso-8859-1" name="Times New Roman"/> </fonttable> |  |
| 01 80 | <page | Start of Page object |
| 01 00 00 00 | id="1" |  |
| 04 02 10 00 00 C0 96 00 C2 D7 74 00 2B 0C AF 00 3D 28 B4
00 | BoundingBox= "116.84 150.75 180.16 175.05" |  |
| 0F 08 02 00 01 00 | WidthPages="1" |  |
| 10 08 02 00 01 00 | HeightPages="1" |  |
| 16 08 04 00 00 00 24 00 | HeaderPosition="36" |  |
| 18 08 04 00 00 00 24 00 | FooterPosition="36" |  |
| 19 08 00 00 | PrintTrimMarks="yes" |  |
| 03 80 | <fragment | Start of Fragment object |
| 08 00 00 00 | id="8" |  |
| 04 02 10 00 00 40 98 00 C2 57 76 00 2B 8C AD 00 3D A8 B2
00 | BoundingBox= "118.34 152.25 178.66 173.55" |  |
| 04 80 | <n | Start of Node object |
| 05 00 00 00 | id="5" |  |
| 0A 00 02 00 01 00 | Z="1" |  |
| 00 02 08 00 00 40 A4 00 00 80 94 00 | p="148.5 164.25" |  |
| 06 80 | <t | Start of Text object |
| 11 00 00 00 | id="17" |  |
| 0A 00 02 00 06 00 | Z="6" |  |
| 00 07 0E 00 01 00 00 00 03 00 60 00 C8 00 03 00 4E 48 | <s font="3" size="10" face="96">NH</s> |  |
| 04 07 06 00 02 00 02 00 03 00 | LineStarts="2 3" |  |
| 05 07 01 00 04 | LabelAlignment="Above" |  |
| 00 02 08 00 EE CE 9F 00 00 B0 91 00 | p="145.69 159.81" |  |
| 04 02 10 00 00 40 98 00 00 40 92 00 00 00 A8 00 00 80 97
00 | BoundingBox= "146.25 152.25 151.5 168" |  |
| 23 08 01 00 00 | LabelJustification="Auto" | omitted from CDXML because this is the default |
| 00 00 | > | End of Text object |
| 37 04 01 00 01 | AS="N" |  |
| 02 04 02 00 07 00 | Element="7" |  |
| 2B 04 02 00 01 00 | NumHydrogens="1" |  |
| 00 00 | > | End of Node object |
| 04 80 | <n | Start of Node object |
| 06 00 00 00 | id="6" |  |
| 0A 00 02 00 02 00 | Z="2" |  |
| 00 02 08 00 BA 03 AC 00 4F 7A B1 00 | p="177.48 172.01" |  |
| 37 04 01 00 01 | AS="N" |  |
| 00 00 | > | End of Node object |
| 05 80 | <b | Start of Bond object |
| 02 00 00 00 | id="2" |  |
| 0A 00 02 00 03 00 | Z="3" |  |
| 04 06 04 00 05 00 00 00 | B="5" |  |
| 05 06 04 00 06 00 00 00 | E="6" |  |
| 0A 06 01 00 01 | BS="N" |  |
| 00 00 | > | End of Bond object |
| 04 80 | <n | Start of Node object |
| 04 00 00 00 | id="4" |  |
| 0A 00 02 00 04 00 | Z="4" |  |
| 00 02 08 00 BA 03 AC 00 B0 85 77 00 | p="119.52 172.01" |  |
| 37 04 01 00 01 | AS="N" |  |
| 00 00 | > | End of Node object |
| 05 80 | <b | Start of Bond object |
| 03 00 00 00 | id="3" |  |
| 0A 00 02 00 05 00 | Z="5" |  |
| 04 06 04 00 05 00 00 00 | B="5" |  |
| 05 06 04 00 04 00 00 00 | E="4" |  |
| 0A 06 01 00 01 | BS="N" |  |
| 00 00 | > | End of Bond object |
| 00 00 | > | End of Fragment object |
| 00 00 | > | End of Page object |
| 00 00 | > | End of Document |

[Return to Introduction](General.md)

---

[CDX Documentation index](index.md)