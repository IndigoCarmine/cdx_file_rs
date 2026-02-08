## Bracket_RepeatCount Property

| CDXML Name: | RepeatCount |
| --- | --- |
| CDX Constant Name: | kCDXProp_Bracket_RepeatCount |
| CDX Constant Value: | 0x0A28 |
| Data Size: | FLOAT64 |
| Property of objects: | kCDXObj_BracketedGroup |
| First written/read in: | ChemDraw 7.0 / (not read) |
| Required? | No |

**Description:**

The number of times a multiple-group BracketedGroup is repeated.

The Repeat Count should be a positive number. Non-positive and non-numeric Repeat Counts have no meaning.

Although fractional Repeat Counts are allowed, it is recommended that integral values be used wherever possible for maximum compatibility.

This property is relevant only for objects with a [kCDXProp_Bracket_Usage](Bracket_Usage.md) of kCDXBracketUsage_MultipleGroup.

**If this property is absent:**

The bracketed objects are arbitrarily assumed to repeat twice. Since this guess is strictly arbitrary, it is strongly recommended that this property not be omitted from relevant objects.

---

CDX Documentation index