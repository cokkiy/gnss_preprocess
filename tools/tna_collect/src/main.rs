use std::{collections::HashMap, path::PathBuf};

use gnss_preprocess::ObsFileProvider;
use rinex::{header::Header, reader::BufferedReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let obs_path = std::env::args()
        .nth(1)
        .expect("Please provide the observation path as an argument");
    let obs_files_provider = ObsFileProvider::new(&obs_path);
    let total_count = obs_files_provider.get_total_count();
    let mut count = 0_usize;
    let mut constellation_codes: HashMap<_, Vec<_>> = HashMap::new();
    for (_, _, file) in obs_files_provider.iter() {
        let path = PathBuf::from(&obs_path).join(file);
        //let obs_file = Rinex::from_file(path.to_str().ok_or("Invalid UTF-8 path")?)?;
        let fullpath = path.to_string_lossy().to_string();

        // create buffered reader
        let mut reader = BufferedReader::new(&fullpath)?;
        // Parse header fields
        let header = Header::new(&mut reader)?;
        if let Some(obs) = header.obs {
            for (c, v) in obs.codes.iter() {
                let codes = constellation_codes
                    .entry(c.clone())
                    .or_insert_with(Vec::new);
                for code in v.iter() {
                    match code {
                        rinex::prelude::Observable::Phase(_)
                        | rinex::prelude::Observable::Doppler(_)
                        | rinex::prelude::Observable::SSI(_)
                        | rinex::prelude::Observable::PseudoRange(_)
                        | rinex::prelude::Observable::ChannelNumber(_) => {
                            let code_string = code.to_string();
                            if !codes.contains(&code_string) {
                                //println!("{}: {} added", c, code_string);
                                codes.push(code_string);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        count += 1;
        println!(
            "{}/{} {} processed. ",
            count,
            total_count,
            path.to_str().unwrap()
        );
    }
    // write to file
    let mut writer = csv::Writer::from_path("constellation_codes.csv")?;
    writer.write_record(&["Constellation", "Codes"])?;
    for (c, v) in constellation_codes.iter() {
        writer.write_record(&[&format!("{:?}", c), &v.join(",")])?;
    }
    writer.flush()?;

    println!("Done.");
    Ok(())
}

mod tests {

    use rinex::prelude::Constellation;

    #[test]
    fn constellation_display_test() {
        let collections = vec![
            Constellation::GPS,
            Constellation::Glonass,
            Constellation::BeiDou,
            Constellation::QZSS,
            Constellation::Galileo,
            Constellation::IRNSS,
            Constellation::WAAS,
            Constellation::EGNOS,
            Constellation::MSAS,
            Constellation::GAGAN,
            Constellation::BDSBAS,
            Constellation::KASS,
            Constellation::SDCM,
            Constellation::ASBAS,
            Constellation::SPAN,
            Constellation::SBAS,
            Constellation::AusNZ,
            Constellation::GBAS,
            Constellation::NSAS,
            Constellation::ASAL,
            Constellation::Mixed,
        ];

        for c in collections {
            println!("{}", c);
        }
    }
}
