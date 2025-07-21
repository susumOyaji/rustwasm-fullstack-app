問題点:

   * dt タグには class*=\"_FxPriceBoard__term\" のようなクラスがありません。
   * dd タグにも class*=\"_FxPriceBoard__description\" のようなクラスがありません。
   * 価格の span タグには class*=\"_FxPriceBoard__price\" のようなクラスがありません。代わりに
     class="number__3BGK" が使われています。

  したがって、現在のセレクタでは bid_value を取得できていません。

  修正案:

  parse_financial_data 関数内の USDJPY=X
  の場合のセレクタを、提供されたHTMLに合わせて修正する必要があります。

  bid_value (買気配) と current_value (売気配) の両方を取得できるように修正します。

    1 // 修正後の parse_financial_data 関数の一部
    2 if url.contains("USDJPY=X") {
    3     // 買気配 (Bid) の取得
    4     let bid_dt_selector = Selector::parse("div.price__1a9E dl dt").unwrap();
    5     let bid_dd_selector = Selector::parse("div.price__1a9E dl dd span.number__3BGK"
      ).unwrap();
    6 
    7     if let Some(dt_element) = document.select(&bid_dt_selector).next() {
    8         if dt_element.text().collect::<String>().trim() == "買気配（Bid）" {
    9             if let Some(dd_element) = document.select(&bid_dd_selector).next() {
   10                 data.bid_value = Some(dd_element.text().collect::<String>());
   11             }
   12         }
   13     }
   14 
   15     // 売気配 (Ask) を current_value として取得
   16     // 2番目の price__1a9E div をターゲットにする
   17     let ask_dt_selector = Selector::parse("div.price__1a9E:nth-of-type(2) dl dt").
      unwrap();
   18     let ask_dd_selector = Selector::parse("div.price__1a9E:nth-of-type(2) dl dd 
      span.number__3BGK").unwrap();
   19 
   20     if let Some(dt_element) = document.select(&ask_dt_selector).next() {
   21         if dt_element.text().collect::<String>().trim() == "売気配（Ask）" {
   22             if let Some(dd_element) = document.select(&ask_dd_selector).next() {
   23                 data.current_value = Some(dd_element.text().collect::<String>());
   24             }
   25         }
   26     }
   27 
   28     // update_time は既存の time セレクタで取得できるはず
   29     let time_selector = Selector::parse("time").unwrap();
   30     if let Some(element) = document.select(&time_selector).next() {
   31         data.update_time = Some(element.text().collect::<String>());
   32     }
   33 
   34 } else {
   35     // 既存の株価取得ロジック
   36     // ...
   37 }

  この修正をRustのコードに適用する必要があります。私は直接Rustのコードを編集することはできませ
  んので、この修正案をRustのプロジェクトに適用してください。

  補足:

   * USDJPY=X のページには previous_day_change や change_rate
     に相当する要素が見当たらないため、これらは引き続き None になるのが正しい動作です。
   * current_value に売気配（Ask）の値を設定するようにしました。これは、フロントエンドで株価の
     current_value と同じフィールドを使用するためです。必要に応じて、ask_value
     のような新しいフィールドを FinancialData に追加することも検討できます。

  この修正を適用後、再度 USDJPY=FX のデータ取得を試してみてください。

