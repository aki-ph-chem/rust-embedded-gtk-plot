# learn plotters

## Contents

ディレクトリ中の`<src_name>.rs`を実行するには

```bash
$ cargo run --bin <src_name>
```

とする。

画像ファイルを生成する場合では`target/figure` \*.pngファイルが生成される。

### windowに埋め込む(realtimeのみ違う) 

1. real\_time.rs: リアルタイムにカウントダウンを描画 
<img src="figure/real_time.png" width="200" height="150">

2. plot\_1.rs: 色々な関数をplotできるようにした 
3. many\_plot.rs: 多くのplotを重ね書きする
4. scatter.rs: 散布図

### basics

基本的な機能を試す

1. line.rs: 直線だけを描画する
<img src="figure/line.png" width="200" height="150">

2. mesh.rs: meshだけを描画する
<img src="figure/mesh.png" width="200" height="150">

3. axes.rs: meshと軸だけ描画する
<img src="figure/axes.png" width="200" height="150">

4. title.rs: mesh + 軸 + タイトル 
<img src="figure/title.png" width="200" height="150">

5. line\_series.rs: 簡単な曲線のplot 
<img src="figure/line_series.png" width="200" height="150">

6. legend.rs: 曲線のplotに凡例を付ける
<img src="figure/legend.png" width="200" height="150">

7. scatter.rs: 散布図
<img src="figure/scatter.png" width="200" height="150">

8. area\_chart.rs: エリアチャート
<img src="figure/area_chart.png" width="200" height="150">

9. histogram.rs: ヒストグラム
<img src="figure/histogram.png" width="200" height="150">

### 3d

3d plotの機能を試す

1. axes\_3d.rs: 三次元の軸のみ描画
<img src="figure/axes_3d.png" width="200" height="150">

2. line\_3d.rs: 三次元座標中に曲線を描画
<img src="figure/line_3d.png" width="200" height="150">

3. surface\_3d.rs: 三次元座標中に曲面を描画
<img src="figure/surface_3d.png" width="200" height="150">

4. line\_3d\_perspective: 三次元座標の視点の調整
<img src="figure/line_3d_perspective.png" width="200" height="150">
