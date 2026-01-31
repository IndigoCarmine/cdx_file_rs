# 未知のタグ収集結果

## 見つかった未知のタグ

| タグ | 16進数 | ファイル | 備考 |
|------|---------|----------|------|
| 32811 | 0x802B | Reaction.cdx, Analysis.cdx, ReactionAnalysis.cdx | 最初のエラーで停止 |

## 詳細

### Tag 32811 (0x802B)
- **状態**: 未実装
- **影響範囲**: 複雑な分子構造（反応図、分析図など）の読み込み
- **対応ファイル**:
  - sample_cdx/Reaction.cdx
  - sample_cdx/Analysis.cdx
  - sample_cdx/ReactionAnalysis.cdx
  - sample_cdx/ReactionAnalysis.out.cdx (外出力)

## 注記

現在のパーサーは最初に遭遇したエラーで停止するため、各ファイルに複数の未知のタグが含まれている可能性があります。より詳細なタグを知るには、パーサーのエラーハンドリングを改善する必要があります。

## 対応状況

✅ **動作確認済み（タグ対応完了）**:
- benzene.cdx
- benzene.out.cdx
- benzene.out.out.cdx
- yellow_colored.cdx

❌ **未対応（未知のタグあり）**:
- Reaction.cdx (Tag 0x802B)
- Reaction.out.cdx (Tag 0x802B)
- Reaction.out.out.cdx (Tag 0x802B)
- Analysis.cdx (Tag 0x802B)
- Analysis.out.cdx (Tag 0x802B)
- Analysis.out.out.cdx (Tag 0x802B)
- ReactionAnalysis.cdx (Tag 0x802B)
- ReactionAnalysis.out.cdx (Tag 0x802B)
- ReactionAnalysis.out.out.cdx (Tag 0x802B)
