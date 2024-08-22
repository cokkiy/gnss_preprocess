use gnss_preprocess::ObsFileProvider;

fn main() {
    let obs_files_provider = ObsFileProvider::new("/mnt/d/GNSS_Data/Data/Obs/");
    for file in obs_files_provider.iter() {}
}
