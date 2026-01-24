
---

## 1. 基本仕様（必須）

* **バイトオーダー**: すべて *little-endian*
* **全体構造**

  ```
  [Fixed Header]
  [Tagged Items（Object / Property のツリー）]
  [00 00（EOF）]
  ```

---

## 2. ファイルヘッダ（固定長・必須）

| オフセット | サイズ      | 内容                   |
| ----- | -------- | -------------------- |
| 0x00  | 8 bytes  | マジック文字列 `"VjCD0100"` |
| 0x08  | 4 bytes  | 予約領域（04 03 02 01）    |
| 0x0C  | 16 bytes | 予約領域（すべて 0x00）       |

→ ヘッダ直後から **Document Object** が開始

---

## 3. Tagged Item の判別ロジック（最重要）

最初の **2 bytes（Tag Identifier）** を読む。

* **bit15 = 0** → Property
* **bit15 = 1** → Object
* **bit14**

  * 0: predefined
  * 1: user-defined

---

## 4. Property の構造

```
[TagID: 2 bytes]
[Length: 2 bytes]
[Data: Length bytes]
```

### 詳細

* **TagID**: MSB(bit15)=0
* **Length**

  * 通常: データ長（bytes）
  * `0x0000`: データなし（存在自体が意味を持つ）
  * `0xFFFF`: 拡張長
    → 直後に **4-byte Length (uint32)** が続く
* **Data**

  * 型は Tag に依存（INT16, FLOAT, angle 等）

---

## 5. Object の構造

```
[TagID: 2 bytes]
[ObjectID: 4 bytes]
[Contents: Properties / Objects ...]
[00 00]  ← EndObject
```

### 詳細

* **TagID**: MSB(bit15)=1
* **ObjectID**

  * uint32
  * 0 = IDなし（参照されない場合のみ許可）
* **Contents**

  * Property と Object が任意順でネスト
* **EndObject**

  * 必ず `00 00`

---

## 6. 終端条件

* **Object終端**: `00 00`
* **File終端**: 最終的に `00 00`

※ パーサは
「Property/Object を読み続け、`00 00` が来たら現在の Object を終了」
という **再帰 or スタック構造** が必須。
