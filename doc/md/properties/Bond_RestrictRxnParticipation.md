## Bond_RestrictRxnParticipation Property

| CDXML Name: | RxnParticipation |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bond_RestrictRxnParticipation |
| CDX Constant Value: | 0x0607 |
| Data Size: | [INT8](/web/20160913174342/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/DataType/CDXNumeric.md) |
| Property of objects: | [kCDXObj_Bond](/web/20160913174342/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/Bond.md) |
| First written/read in: | ChemDraw 4.0 |
| Required? | No |

**Description:**

Specifies that a bond is affected by a reaction.

The value stored in this property corresponds to the Reacting Center Status property.

This is an enumerated property. Acceptable values are shown in the following list:

| Value | CDXML Name | Description |
| --- | --- | --- |
| 0 | Unspecified | Bond involvement in reacting center is not specified |
| 1 | ReactionCenter | Bond is part of reacting center but not made/broken nor order changed |
| 2 | MakeOrBreak | Bond is made or broken in reaction |
| 3 | ChangeType | Bond's order changes in reaction |
| 4 | MakeAndChange | Bond is made or broken, or its order changes in the reaction |
| 5 | NotReactionCenter | Bond is not part of reacting center |
| 6 | NoChange | Bond does not change in course of reaction, but it is part of the reacting center |
| 7 | Unmapped | The structure was partially mapped, but the reaction involvement of this bond was not determined |

**If this property is absent:**

The reaction participation is treated as Unspecified.

---

[CDX Documentation index](/web/20160913174342/http://www.cambridgesoft.com/services/documentation/sdk/chemdraw/cdx/index.md)