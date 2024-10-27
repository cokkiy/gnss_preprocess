use hifitime::Epoch;
use lagrangian_interpolation::lagrange_interpolate;

use crate::nav_data::BeiDouNavData;

use super::Interpolation;

impl Interpolation for Vec<(&Epoch, &BeiDouNavData)> {
    type Output = BeiDouNavData;

    fn interpolate(&self, epoch: &Epoch) -> Self::Output {
        BeiDouNavData {
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
            // age of data
            aode: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.aode))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            crs: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.crs))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            delta_n: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.delta_n))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            m0: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.m0))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            cuc: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.cuc))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            e: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.e))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            cus: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.cus))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            sqrt_a: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.sqrt_a))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            toe: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.toe))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            cic: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.cic))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            omega_0: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.omega_0))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            cis: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.cis))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            i0: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.i0))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            crc: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.crc))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            omega: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.omega))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            omega_dot: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.omega_dot))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
            i_dot: lagrange_interpolate(
                &self
                    .iter()
                    .map(|(x, y)| (x.to_tai_seconds(), y.i_dot))
                    .collect::<Vec<_>>(),
                epoch.to_tai_seconds(),
            ),
        }
    }
}
