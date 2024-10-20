use crate::{obs_files_tree::ObsFilesTree, stations_manager::StationsManager};
use rand::seq::SliceRandom;
use rand::thread_rng;
#[allow(dead_code)]
pub struct GNSSDataProvider<'a> {
    base_path: String,
    stations_manager: StationsManager,
    all_stations: Vec<String>,
    training_stations: &'a [String],
    testing_stations: &'a [String],
}

#[allow(dead_code)]
impl<'a> GNSSDataProvider<'a> {
    pub fn new(base_path: &str) -> Self {
        let obs_files_tree = ObsFilesTree::create_obs_tree(base_path);
        let stations_manager = StationsManager::new(&obs_files_tree);
        Self {
            base_path: base_path.to_string(),
            stations_manager,
            all_stations: vec![],
            training_stations: &[],
            testing_stations: &[],
        }
    }

    pub fn split_by_name(&'a mut self, percent: u8) {
        self.all_stations = self.stations_manager.get_all_stations();
        let mut rng = thread_rng();
        self.all_stations.shuffle(&mut rng);
        let split_index =
            (self.all_stations.len() as f64 * percent as f64 / 100.0).round() as usize;
        let (training_stations, testing_stations) = self.all_stations.split_at(split_index);
        self.training_stations = training_stations;
        self.testing_stations = testing_stations;
    }

    // pub fn train_iter(&self) -> impl Iterator<Item = Vec<f64>> + '_ {
    //     if self.training_stations.is_empty() {
    //         panic!("Training stations are not set. Call split_by_name() first.");
    //     }
    //     let station = self.training_stations.choose(&mut thread_rng()).unwrap();
    //     let epoch_providers = self
    //         .stations_manager
    //         .get_station_epoch_provider(&self.base_path, station);
    //     epoch_providers.iter().map(|epoch_provider| {
    //         let epoch_data = epoch_provider.next_epoch().next().unwrap();
    //         let epoch = epoch_data.get_epoch();
    //         let station = epoch_data.get_station();
    //         let data = epoch_data.get_data();
    //         let mut result = vec![epoch.get_day_number() as f64];
    //         result.extend_from_slice(&station.get_coordinates());
    //         for sv_data in data {
    //             result.push(sv_data.get_sv());
    //             result.extend_from_slice(&sv_data.get_data());
    //         }
    //         result
    //     })
    // }
}
