use std::io::{self};
use csv;

type MyResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> MyResult {
    let conds = get_cond();
    let input = io::stdin();
    let input = io::BufReader::new(input.lock());
    let mut rdr = csv::Reader::from_reader(input);
    let out = io::stdout();
    let mut wtr = csv::Writer::from_writer(out.lock());
    let header = rdr.headers()?.clone();
    let mut update_or_stay: Vec<(bool, String)> = Vec::new();
    for column in header.iter() {
        let mut is_update_target = false;
        let mut update_dst = "".to_string();
        for (fld, val) in conds.iter() {
            if fld == column {
                is_update_target = true;
                update_dst = val.clone();
            }
        }
        update_or_stay.push((is_update_target, update_dst));
    }
    wtr.write_record(&header)?;
    for result in rdr.records() {
        let record_org = result?;
        let mut record: Vec<String> = Vec::new();
        for (i, (is_update_target, update_dst)) in update_or_stay.iter().enumerate() {
            if *is_update_target {
                record.push(update_dst.clone());
            } else {
                record.push(record_org[i].to_string());
            }
        }
        wtr.write_record(&record)?;
    }
    Ok(())
}

fn get_cond() -> Vec<(String, String)> {
    let mut conds: Vec<(String, String)> = Vec::new();
    let args = std::env::args();
    if args.len() < 2 {
        return conds;
    }
    let mut is_cmd_name = true;
    for arg in args {
        if is_cmd_name {
            is_cmd_name = false;
            continue;
        }
        let s: Vec<&str> = arg.splitn(2, ',').collect();
        if s.len() == 1 {
            conds.push((s[0].to_string(), "".to_string()));
        } else {
            conds.push((s[0].to_string(), s[1].to_string()));
        }
    }
    conds
}
