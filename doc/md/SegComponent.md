## SGComponent Object

| CDXML Name:            | sgcomponent                                  |
| ---------------------- | -------------------------------------------- |
| CDX Constant Name:     | kCDXObj_SGComponent                          |
| CDX Constant Value:    | 0x8023                                       |
| Contained by objects:  | SegData |
| First written/read in: | ChemDraw (version unknown)                   |

**Description:**

An SGComponent object represents a single component within a structural group (SG).
It is used to describe the role and presentation characteristics of an individual component participating in a superatom or grouped structure, such as whether the component functions as a reactant or a header element.

There are no explicitly required properties.


**Properties:**

| Value | Name                                                                                 | CDXML Name          | Type    |
| ----- | ------------------------------------------------------------------------------------ | ------------------- | ------- |
| 2066  | Width                                                                                | Width               | Integer |
|       | Specifies the display width associated with the SG component.                        |                     |         |
| 4611  | ComponentIsReactant                                                                  | ComponentIsReactant | Boolean |
|       | Indicates whether the component functions as a reactant within the structural group. |                     |         |
| 4612  | ComponentIsHeader                                                                    | ComponentIsHeader   | Boolean |
|       | Indicates whether the component serves as a header element for the structural group. |                     |         |

---

[CDX Documentation index](index.md)

---

必要であれば、型定義（Integer / Boolean）の CDX データ型表現や、SG（Structural Group）全体のオブジェクト階層との関係も追記できる。
