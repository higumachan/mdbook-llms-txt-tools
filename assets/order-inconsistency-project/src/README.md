# Order Inconsistency Project

このプロジェクトは、mdbookのチャプターの順序がSUMMARY.mdの順序に従うのか、それともディレクトリの辞書順に従うのかを検証するためのものです。

## プロジェクト構成

このプロジェクトでは、以下のようにファイル名と`SUMMARY.md`での順序を意図的に異なるものにしています：

1. `z_chapter.md` (Chapter A) - 辞書順では最後
2. `a_chapter.md` (Chapter B) - 辞書順では最初
3. `m_chapter.md` (Chapter C) - 辞書順では2番目

このような構成にすることで、mdbookがどちらの順序を優先するかを確認することができます。 