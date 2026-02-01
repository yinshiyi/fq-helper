use anyhow::Result;
use kseq::parse_path;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: fastq_counter <input.fastq[.gz]>");

    let mut records = parse_path(path).unwrap();
    let mut read_count: u64 = 0;
    let mut base_count: u64 = 0;
    let mut gc_count: u64 = 0;

    while let Some(record) = records.iter_record().unwrap() {
        read_count += 1;
        let seq = record.seq();
        base_count += seq.len() as u64;

        gc_count += seq
            .bytes()
            .filter(|b| matches!(b, b'G' | b'g' | b'C' | b'c'))
            .count() as u64;
    }

    let gc_percent = if base_count > 0 {
        (gc_count as f64 / base_count as f64) * 100.0
    } else {
        0.0
    };

    println!("Total reads : {}", read_count);
    println!("Total bases : {}", base_count);
    println!("GC%         : {:.2}%", gc_percent);
    Ok(())
}
