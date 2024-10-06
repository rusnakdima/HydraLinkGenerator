use std::{fs::{self, File}, io::Write, path::Path};

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SteamGG {
  name: String,
  downloads: Vec<Download>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Download {
  title: String,
  uploadDate: String,
  fileSize: String,
  uris: Vec<String>,
}

#[tokio::main]
async fn main() {
  let response = reqwest::get("https://steamgg.net/a-z-games/")
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

  let mut data = SteamGG {
    name: String::from("SteamGG"),
    downloads: Vec::new(),
  };

  let document = Html::parse_document(&response);

  let items_inner = Selector::parse(".items-inner").unwrap();

  let items = document.select(&items_inner).next().unwrap();

  let ul_elem = Selector::parse(".az-columns").unwrap();
  for (i, ul) in items.select(&ul_elem).enumerate() {
    data.downloads = Vec::new();
    let cur: usize = 19;
    if i < cur || i > cur {
      continue;
    }

    let mut f: File;
    if Path::new(&format!("./steamgg{}.json", i)).exists() {
      let _ = fs::remove_file(format!("./steamgg{}.json", i));
    }
    f = File::create(format!("./steamgg{}.json", i)).unwrap();

    println!("\n\n\ni: {i}");

    let li_elem = Selector::parse("li").unwrap();
    for (j, li) in ul.select(&li_elem).enumerate() {
      println!("j: {j}");

      let mut data_download: Download = Download {
        title: "".to_string(),
        uploadDate: format!("{}", chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00")),
        fileSize: "".to_string(),
        uris: Vec::<String>::new(),
      };

      let a_elem = Selector::parse("a").unwrap();
      let a = li.select(&a_elem).next().unwrap();

      let url_link = a.value().attr("href").unwrap();
      let res_page = reqwest::get(url_link)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

      let document_page = Html::parse_document(&res_page);

      let blog_post_elem = Selector::parse(".blog-post").unwrap();
      let blog_post = document_page.select(&blog_post_elem).next().unwrap();

      let blog_content_title_elem = Selector::parse(".blog-content-title").unwrap();
      let blog_content_title = blog_post.select(&blog_content_title_elem).next().unwrap();
      let h2_elem = Selector::parse("h2").unwrap();
      let h2 = blog_content_title.select(&h2_elem).next().unwrap();
      let title = h2.inner_html();
      data_download.title = title;

      let blog_content_elem = Selector::parse(".blog-content").unwrap();
      let blog_content = blog_post.select(&blog_content_elem).next().unwrap();

      let btn_elem = Selector::parse(".vc_btn3").unwrap();
      for btn in blog_content.select(&btn_elem) {
        let href = btn.value().attr("href").unwrap_or("");
        data_download.uris.push(href.to_string());
      }


      data.downloads.push(data_download);
    }
    let _ = f.write_all(serde_json::to_string(&data).unwrap().as_bytes()).unwrap();
  }
}