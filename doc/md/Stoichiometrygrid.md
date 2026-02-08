## Stoichiometry Grid Object

| CDXML Name:            | stoichiometrygrid         |
| ---------------------- | ------------------------- |
| CDX Constant Name:     | kCDXObj_StoichiometryGrid |
| CDX Constant Value:    | 0x8022                    |

### 説明

Stoichiometry Grid Object は、反応における当量計算を行うことのできるTableである。

本オブジェクトは、位置情報を含むプロパティと、未知の生データを含むプロパティを持つ。


### プロパティ

| Value  | Name                                                     | CDXML Name | Type       |
| ------ | -------------------------------------------------------- | ---------- | ---------- |
| 0x0200 | （仮）Position                                              | p          | CDXPoint2D |
|        | オブジェクトの 2 次元位置を表す。縦方向座標および横方向座標の順で格納される。                 |            |            |
| 0x000A | [kCDXProp_ZOrder](properties/ZOrder.md) | Z | [INT16](DataType/CDXNumeric.md) |
|  | Back-to-front ordering index in 2D drawing. |  |  |
