## Sequence_Identifier Property

| CDXML Name: | SequenceIdentifier |
| --- | --- |
| CDX Constant Name: | kCDXProp_Sequence_Identifier |
| CDX Constant Value: | 0x0E00 |
| Data Size: | [CDXString](/web/20160912170420/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXString.md) |
| Property of objects: | [kCDXObj_Sequence](/web/20160912170420/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Sequence.md) |
| First written/read in: | ChemDraw 7.0 |
| Required? | Required for sequences |

**Description:**

A unique (but otherwise random) identifier for a given Sequence object.

Unlike the object's [ID](id.md), this identifier is guaranteed to be both persistent and unique, even across multiple documents. It is a GUID, a standard Globally-Unique IDentifier broadly used under Microsoft Windows. The standard Windows program guidgen.exe may be used to generate GUIDs external to ChemDraw.

---

[CDX Documentation index](/web/20160912170420/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)