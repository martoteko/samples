use std::{thread::sleep, time::Duration};

// lib.rsでビルドしたFFIコードを参照
use wmx3_rerun::*;

use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ret = unsafe { open_wmx() };
    println!("open_wmx = {}", ret);

    // rerunビューワをスポーン。
    let rec = rerun::RecordingStreamBuilder::new("wmx3-rerun").spawn()?;

    // あらかじめ波形を登録しておく。
    rec.log_static(
        "axis0/cmdpos0",
        &rerun::SeriesLines::new()
            .with_colors([[255, 0, 0]])
            .with_names(["cmdpos0"])
            .with_widths([2.0]),
    )?;

    let ret = unsafe { start_memlog(0) };
    println!("start_memlog = {}", ret);

    // 10秒間メモリログで指令位置を取得してプロットに適用する。横軸はサイクルカウンタとする。
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        // 一括で1000サイクル分とってくる。実際に取得したサイズはcountで得る。
        const N: usize = 1000;
        let mut pos: [f64; N] = [0.0; N];
        let mut cycle_counter: [i64; N] = [0; N];
        let mut count: usize = 0;
        let ret = unsafe { get_memlog(pos.as_mut_ptr(), cycle_counter.as_mut_ptr(), &mut count) };
        println!("get_memlog = {} count {}", ret, count);

        for i in 0..count {
            rec.set_time_sequence("cycle", cycle_counter[i]);
            rec.log("axis0/cmdpos0", &rerun::Scalars::single(pos[i]))?;
        }

        // 直近1000サイクル取得するので、周期が1msなら1000ms分になる。
        // 500msならsleepしても漏れなくとれる。
        sleep(Duration::from_millis(500));
    }

    let ret = unsafe { stop_memlog() };
    println!("stop_memlog = {}", ret);

    let ret = unsafe { close_wmx() };
    println!("close_wmx = {}", ret);

    Ok(())
}
