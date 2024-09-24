use gnss_preprocess::GNSSDataProvider;

fn main() {
    let mut gnssdata_provider = GNSSDataProvider::new("/mnt/d/GNSS_Data/Data", Some(100));
    let iter = gnssdata_provider.train_iter();
    for (i, data) in iter.enumerate() {
        println!("{:?}", data);
        if i == 10 {
            break;
        }
    }
}
