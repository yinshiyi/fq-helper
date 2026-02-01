use anyhow::Result;
use kseq::parse_path;
use medians::Median;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .expect("Usage: fastq_counter <input.fastq[.gz]>");

    let mut records = parse_path(path).unwrap();
    let mut read_count: u64 = 0;
    let mut base_count: u64 = 0;
    let mut gc_count: u64 = 0;
    let mut lengths: Vec<usize> = Vec::new();

    while let Some(record) = records.iter_record().unwrap() {
        read_count += 1;
        let seq = record.seq();
        base_count += seq.len() as u64;
        lengths.push(seq.len());

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

    let mean_len = if read_count > 0 {
        base_count / read_count
    } else {
        0
    };

    let median_len = lengths
        .as_mut_slice()
        .uqmedian(|&x| x as u64)
        .unwrap();

    println!("Total reads : {}", read_count);
    println!("Total bases : {}", base_count);
    println!("Mean length : {}", mean_len);
    println!("Median lens : {}", median_len);
    println!("GC%         : {:.2}%", gc_percent);
    Ok(())
}
