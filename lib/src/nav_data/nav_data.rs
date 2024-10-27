use hifitime::Epoch;

use super::{
    BeiDouNavData, GPSNavData, GalileoNavData, GlonassNavData, IRNSSNavData, QZSSNavData,
    SBASNavData,
};

/// 导航电文数据
pub enum NavData {
    /// GPS 导航电文数据
    GPSNavData((Epoch, GPSNavData)),
    /// Glonass 导航电文数据
    GlonassNavData((Epoch, GlonassNavData)),
    /// Galileo 导航电文数据
    GalileoNavData((Epoch, GalileoNavData)),
    /// BeiDou 导航电文数据
    BeiDouNavData((Epoch, BeiDouNavData)),
    /// IRNSS 导航电文数据
    IRNSSNavData((Epoch, IRNSSNavData)),
    /// QZSS 导航电文数据
    QZSSNavData((Epoch, QZSSNavData)),
    /// SBAS 导航电文数据
    SBASNavData((Epoch, SBASNavData)),
}

impl NavData {
    const MAX_FIELDS_NUMBER: usize = 19;

    /// 从 GPS 导航电文数据创建导航电文数据
    pub fn from_gps_nav_data(epoch: Epoch, nav_data: GPSNavData) -> Self {
        NavData::GPSNavData((epoch, nav_data))
    }
    /// 从 Glonass 导航电文数据创建导航电文数据
    pub fn from_glonass_nav_data(epoch: Epoch, nav_data: GlonassNavData) -> Self {
        NavData::GlonassNavData((epoch, nav_data))
    }
    /// 从 Galileo 导航电文数据创建导航电文数据
    pub fn from_galileo_nav_data(epoch: Epoch, nav_data: GalileoNavData) -> Self {
        NavData::GalileoNavData((epoch, nav_data))
    }
    /// 从 BeiDou 导航电文数据创建导航电文数据
    pub fn from_beidou_nav_data(epoch: Epoch, nav_data: BeiDouNavData) -> Self {
        NavData::BeiDouNavData((epoch, nav_data))
    }
    /// 从 IRNSS 导航电文数据创建导航电文数据
    pub fn from_irnss_nav_data(epoch: Epoch, nav_data: IRNSSNavData) -> Self {
        NavData::IRNSSNavData((epoch, nav_data))
    }
    /// 从 QZSS 导航电文数据创建导航电文数据
    pub fn from_qzss_nav_data(epoch: Epoch, nav_data: QZSSNavData) -> Self {
        NavData::QZSSNavData((epoch, nav_data))
    }
    /// 从 SBAS 导航电文数据创建导航电文数据
    pub fn from_sbas_nav_data(epoch: Epoch, nav_data: SBASNavData) -> Self {
        NavData::SBASNavData((epoch, nav_data))
    }

    /// Checks if the NavData is GPSNavData
    pub fn is_gps_nav_data(&self) -> bool {
        matches!(self, NavData::GPSNavData(_))
    }

    /// Checks if the NavData is GlonassNavData
    pub fn is_glonass_nav_data(&self) -> bool {
        matches!(self, NavData::GlonassNavData(_))
    }

    /// Checks if the NavData is GalileoNavData
    pub fn is_galileo_nav_data(&self) -> bool {
        matches!(self, NavData::GalileoNavData(_))
    }

    /// Checks if the NavData is BeiDouNavData
    pub fn is_beidou_nav_data(&self) -> bool {
        matches!(self, NavData::BeiDouNavData(_))
    }

    /// Checks if the NavData is IRNSSNavData
    pub fn is_irnss_nav_data(&self) -> bool {
        matches!(self, NavData::IRNSSNavData(_))
    }

    /// Checks if the NavData is QZSSNavData
    pub fn is_qzss_nav_data(&self) -> bool {
        matches!(self, NavData::QZSSNavData(_))
    }

    /// Checks if the NavData is SBASNavData
    pub fn is_sbas_nav_data(&self) -> bool {
        matches!(self, NavData::SBASNavData(_))
    }
}

impl From<NavData> for Vec<f64> {
    fn from(value: NavData) -> Self {
        let mut vec: Vec<f64> = match value {
            NavData::GPSNavData((_, nav_data)) => (&nav_data).into(),
            NavData::GlonassNavData((_, nav_data)) => (&nav_data).into(),
            NavData::GalileoNavData((_, nav_data)) => (&nav_data).into(),
            NavData::BeiDouNavData((_, nav_data)) => (&nav_data).into(),
            NavData::IRNSSNavData((_, nav_data)) => (&nav_data).into(),
            NavData::QZSSNavData((_, nav_data)) => (&nav_data).into(),
            NavData::SBASNavData((_, nav_data)) => (&nav_data).into(),
        };
        vec.resize(NavData::MAX_FIELDS_NUMBER, 0.0);
        vec
    }
}

impl<'a> From<&'a NavData> for Option<(&'a Epoch, &'a GPSNavData)> {
    fn from(value: &'a NavData) -> Self {
        match value {
            NavData::GPSNavData((epoch, nav_data)) => Some((epoch, nav_data)),
            _ => None,
        }
    }
}

impl<'a> From<&'a NavData> for Option<(&'a Epoch, &'a BeiDouNavData)> {
    fn from(value: &'a NavData) -> Self {
        match value {
            NavData::BeiDouNavData((epoch, nav_data)) => Some((epoch, nav_data)),
            _ => None,
        }
    }
}

impl<'a> From<&'a NavData> for Option<(&'a Epoch, &'a GalileoNavData)> {
    fn from(value: &'a NavData) -> Self {
        match value {
            NavData::GalileoNavData((epoch, nav_data)) => Some((epoch, nav_data)),
            _ => None,
        }
    }
}

impl<'a> From<&'a NavData> for Option<(&'a Epoch, &'a GlonassNavData)> {
    fn from(value: &'a NavData) -> Self {
        match value {
            NavData::GlonassNavData((epoch, nav_data)) => Some((epoch, nav_data)),
            _ => None,
        }
    }
}

impl<'a> From<&'a NavData> for Option<(&'a Epoch, &'a QZSSNavData)> {
    fn from(value: &'a NavData) -> Self {
        match value {
            NavData::QZSSNavData((epoch, nav_data)) => Some((epoch, nav_data)),
            _ => None,
        }
    }
}

impl<'a> From<&'a NavData> for Option<(&'a Epoch, &'a IRNSSNavData)> {
    fn from(value: &'a NavData) -> Self {
        match value {
            NavData::IRNSSNavData((epoch, nav_data)) => Some((epoch, nav_data)),
            _ => None,
        }
    }
}

impl<'a> From<&'a NavData> for Option<(&'a Epoch, &'a SBASNavData)> {
    fn from(value: &'a NavData) -> Self {
        match value {
            NavData::SBASNavData((epoch, nav_data)) => Some((epoch, nav_data)),
            _ => None,
        }
    }
}
