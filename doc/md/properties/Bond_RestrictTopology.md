CDX Format Specification: Bond_RestrictTopology Property
## Bond_RestrictTopology Property

| CDXML Name: | Topology |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_RestrictTopology |
| CDX Constant Value: | 0x0606 |
| Data Size: | [INT8](/web/20160912060917/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160912060917/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**  

Indicates the desired topology of a bond in a query.

Unspecified differs from RingOrChain in that the latter indicates a specific setting on the part of the user. Different databases may treat Unspecified bonds differently. Some will allow an acyclic bond in a query to match a cyclic bond in the target if the acyclic bond has an Unspecified topology; others will require that the bond be marked as RingOrChain explicitly.

Note that it is possible to assign impossible values, for example by saying that one of the bonds in benzene must be a Chain bond. Impossibilities of that sort are not forbidden by this specification, but it is assumed that such a structure would match nothing else if posed as a structure query in some database.

The value stored in this property corresponds to the Bond Topology flag in ISIS, with the addition of an explicit RingOrChain value.

This is an enumerated property. Acceptible values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Ring/chain status of the bond is unspecified |
| 1 | Ring | Bond must be in a ring |
| 2 | Chain | Bond must not be in a ring |
| 3 | RingOrChain | Bond may be in either a ring or a chain |

**If this property is absent:**  

The topology is treated as Unspecified.

---

[CDX Documentation index](/web/20160912060917/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)