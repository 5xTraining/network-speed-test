use reqwest;
use std::time::{Instant, Duration};
use std::io::Write;
use tokio::task;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let url = "http://http.speed.hinet.net/test_200m.zip";
    // println!("開始測試下載速度...");
    let loading_dots_task = task::spawn(async {
        let mut count = 0;
        loop {
            sleep(Duration::from_secs(1)).await;
            count +=1;

            if count <= 3 {
                print!(".");
            } else {
                count = 0;
                print!("\r開始測試下載速度 ");
            }

            std::io::stdout().flush().unwrap();
        }
    });

    let start_time = Instant::now();

    match reqwest::get(url).await {
        Ok(response) => {
            if !response.status().is_success() {
                println!("測試下載發生錯誤: {:?}", response.status());
                return;
            }

            match response.bytes().await {
                Ok(bytes) => {
                    let duration = start_time.elapsed();
                    let speed_in_mbps =
                        (bytes.len() * 8) as f64 / duration.as_secs_f64() / 1_000_000.0;
                    
                    loading_dots_task.abort();
                    print!("\r{: <30}", "");
                    print!("\r");

                    println!("網路下載速度: {:.2} Mbps", speed_in_mbps);
                    println!("下載完成，耗時: {:.2} 秒", duration.as_secs_f64());
                }
                Err(e) => {
                    println!("讀取下載資料時發生錯誤: {:?}", e);
                }
            }
        }

        Err(e) => {
            println!("下載時發生錯誤: {:?}", e);
        }
    }
}
