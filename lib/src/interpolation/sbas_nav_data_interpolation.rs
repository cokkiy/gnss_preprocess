use hifitime::Epoch;
use lagrangian_interpolation::lagrange_interpolate;

use crate::nav_data::SBASNavData;

use super::Interpolation;

impl Interpolation for Vec<(&Epoch, &SBASNavData)> {
    type Output = SBASNavData;

    fn interpolate(&self, epoch: &Epoch) -> Self::Output {
        SBASNavData {
            clock_bias: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.clock_bias))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            clock_drift: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.clock_drift))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            // time of message
            tom: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.tom))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            x: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.x))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            vel_x: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.vel_x))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            accel_x: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.accel_x))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            health: 0.0,
            y: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.y))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            vel_y: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.vel_y))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            accel_y: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.accel_y))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            ura: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.ura))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            z: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.z))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            vel_z: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.vel_z))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            accel_z: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.accel_z))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            // issue of data navigation
            iodn: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.iodn))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
        }
    }
}
