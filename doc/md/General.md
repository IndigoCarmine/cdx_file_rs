CDX Format Specification: General
## Introduction

#### What is CDX?

CDX is the native file format of ChemDraw, and is guaranteed to save 
anything drawn in ChemDraw without loss of data. At the same time, 
however, its architecture was carefully designed to make it a flexible 
and general-purpose chemical format. It is intended to be a practical 
choice for use by non-graphical programs, i.e. ones dealing in 
connection tables rather than desktop publishing. Because of its ability
 to incorporate custom information, and because it is in the public 
domain, CDX has been adopted by the U.S. Patent Office as its standard 
chemical format. We are working to make the format useful to third 
parties and would appreciate feedback.

#### What is CDXML?

CDXML is an XML encoding of CDX -- a variant of CDX that complies with the [XML](https://www.xml.org/)
 specification. It differs from CDX only in the details of its 
formatting, and it doesn't even differ by that much. Everything that 
can be stored in a CDX file can also be stored in a CDXML file. 
Everything that can be stored in a CDXML file can also be stored in a 
CDX file. This is a very important point: a document can be converted 
from binary CDX to text-based CDXML and back again with absolutely no 
loss of information.

#### What are the CDX/CDXML File Formats?

The CDX File Format is a tagged file format, meaning that it consists
 of a series of objects, each of which is preceded by a tag that 
identifies what the object represents (for example, a bond). Tagged file
 formats in general are very flexible. Readers of a tagged file can 
efficiently skip over parts they aren't interested in or do not 
recognize, and in fact they are expected to do so. Among other things, 
this flexibility means that a tagged file format can be expanded without
 invalidating any existing files. We fully expect that ChemDraw 4.0 
will be able to read files created by ChemDraw 40.0. The older version 
surely won't understand any new features added in the meantime, but the 
basic contents should be intact.

This flexibility also means that the CDX/CDXML formats may be 
extended by companies other than CambridgeSoft. As a practical matter, 
we would request that anyone wishing to extend these formats let us 
know, it isn't strictly *necessary* that they do so. Contacting 
us simply lets us document any changes publically; the files will be 
readable by ChemDraw whether or not we are notified.

In the simplest view, a CDX file consists of a document header followed by a stream of *tagged items* followed by the *end* of the Document. Each tagged item is self-contained:

DocumentHeader, TaggedItem, TaggedItem, TaggedItem, ...., DocumentEnd

There are three kinds of tagged item: an *object header*, an *object end*, and a *property*. An *object*
 consists of an object header and all tagged items until a matching 
object end is reached. Every object header has a corresponding object 
end. For example,

DocumentHeader, ObjectHeader, ObjectEnd, ObjectHeader, Property, ObjectEnd, ..., DocumentEnd

Objects can contain other objects and properties. The enclosing 
object is called a "container." A typical CDX file consists of layers of
 nested objects. A simple example might look like this:

---

---

---

| Document Header, | ObjectHeader, | Property, | ObjectHeader, | Property, | ObjectEnd, | ObjectEnd, | DocumentEnd, |
| --- | --- | --- | --- | --- | --- | --- | --- |

The nesting can be arbitrarily deep with no limit to the number of 
objects and properties at any level. Although the order of objects or 
properties at any level does not matter, the nesting structure is very 
significant. Using *Obj* to represent an Objectheader, and */Obj* to represent an ObjectEnd, a general CDX-format file looks like this:

DocumentHeader, Obj(Prop, Prop, Obj(Obj (Prop, Obj(...) )Prop)),
 Obj(...), ..., End

This nesting can be difficult to see in a raw binary file. We have 
provided a simple program specifically designed to make it easy to view 
the nesting. CDXHexDumper.exe
 is a very crude program that reads a CDX file and dumps it in hex to 
the file c:\temp\dump.txt. The file is interpreted into objects and 
properties and shows the complete structure of the CDX file. The program
 does not provide any semantic interpretation; it simply reformats the 
data to make it easier to see the different objects and properties. It 
is much simpler to see the nesting in a text-based CDXML file, and there
 are many XML-reading programs readily available that can make the 
nesting even more apparent if you like.

#### How does ChemDraw store a document?

ChemDraw stores a document as a set of nested objects and properties.
 Objects are things such as atoms, bonds, fragments, arrows, and text. 
Properties are things like position, color, arrow type, and bond order. 
Each object has zero or more properties and may contain zero or more 
other nested objects -- themselves each containing zero or more 
properties and nested objects, and so on.

For example, a molecular fragment might contain an atom, a bond, 
text, and another fragment containing atoms, bonds, and various 
properties. The following diagram omits the various properties of the 
atoms and bonds, showing only objects for simplicity. "Node" refers to 
an attachment point, usually an atom.

## More info...

- [The CDX binary file format](IntroCDX.md)
- [The CDXML text-based file format](IntroCDXML.md)
- [Simple example](IntroExampleSimple.md)
- [Less-simple example](IntroExampleComplex.md)

---

CDX Documentation index