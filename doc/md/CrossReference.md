## Cross-Reference Object

| CDXML Name: | crossreference |
| --- | --- |
| CDX Constant Name: | kCDXObj_CrossReference |
| CDX Constant Value: | 0x8014 |
| Contained by objects: | [kCDXObj_Page](Page.md) |
| First written/read in: | ChemDraw 7.0 |

**Description:**  

A Cross-Reference object represents a link to some [Sequence](Sequence.md) object (possibly not even in the same file!); the contents of its Text object may change as the other Sequence object is updated.

There are no required properties or objects.

**Subobjects:**  
*(none)*

**Properties:**  

| Value | Name | CDXML Name | Type |
| --- | --- | --- | --- |
| 0x0F00 | [kCDXProp_CrossReference_Container](properties/CrossReference_Container.md) | CrossReferenceContainer | [CDXString](DataType/CDXString.md) |
| An external object containing (as an embedded object) the document containing the Sequence object being referenced. |  |  |
| 0x0F01 | [kCDXProp_CrossReference_Document](properties/CrossReference_Document.md) | CrossReferenceDocument | [CDXString](DataType/CDXString.md) |
| An external document containing the Sequence object being referenced. |  |  |
| 0x0F02 | [kCDXProp_CrossReference_Identifier](properties/CrossReference_Identifier.md) | CrossReferenceIdentifier | [CDXString](DataType/CDXString.md) |
| Required for cross-references. A unique (but otherwise random) identifier for a given Cross-Reference object. |  |  |
| 0x0F03 | [kCDXProp_CrossReference_Sequence](properties/CrossReference_Sequence.md) | CrossReferenceSequence | [CDXString](DataType/CDXString.md) |
| Required for cross-references. A value matching the SequenceIdentifier of the Sequence object to be referenced. |  |  |

---

[CDX Documentation index](index.md)