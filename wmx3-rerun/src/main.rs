use std::time::Duration;

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

    // 10秒間できるだけ指令位置を取得してプロットに適用する。
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        let pos = unsafe { get_pos(0) };

        rec.set_time("time", std::time::SystemTime::now());
        rec.log("axis0/cmdpos0", &rerun::Scalars::single(pos))?;
    }

    let ret = unsafe { close_wmx() };
    println!("close_wmx = {}", ret);

    Ok(())
}
