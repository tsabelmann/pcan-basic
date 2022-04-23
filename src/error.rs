use crate::pcan;

///
#[derive(Debug, PartialEq)]
pub enum PcanError {
    ///
    XmtFull,
    ///
    Overrun,
    ///
    BusLight,
    ///
    BusHeavy,
    ///
    BusPassive,
    ///
    BusOff,
    ///
    AnyBusErr,
    ///
    QrcvEmpty,
    ///
    QOverrun,
    ///
    QxmtFull,
    ///
    RegTest,
    ///
    NoDriver,
    ///
    HwInUse,
    ///
    NetInUse,
    ///
    IllHw,
    ///
    IllNet,
    ///
    IllClient,
    ///
    Resource,
    ///
    IllParamType,
    ///
    IllParamVal,
    ///
    Unknown,
    ///
    IllData,
    ///
    IllMode,
    ///
    Caution,
    ///
    Initialize,
    ///
    IllOperation,
}

///
#[derive(Debug, PartialEq)]
pub enum PcanOkError {
    ///
    Ok,
    ///
    Err(PcanError),
}

impl From<PcanError> for u32 {
    fn from(value: PcanError) -> u32 {
        match value {
            PcanError::XmtFull => pcan::PCAN_ERROR_XMTFULL,
            PcanError::Overrun => pcan::PCAN_ERROR_OVERRUN,
            PcanError::BusLight => pcan::PCAN_ERROR_BUSLIGHT,
            PcanError::BusHeavy => pcan::PCAN_ERROR_BUSHEAVY,
            PcanError::BusPassive => pcan::PCAN_ERROR_BUSPASSIVE,
            PcanError::BusOff => pcan::PCAN_ERROR_BUSOFF,
            PcanError::AnyBusErr => {
                let mut value = pcan::PCAN_ERROR_BUSWARNING;
                value |= pcan::PCAN_ERROR_BUSLIGHT;
                value |= pcan::PCAN_ERROR_BUSHEAVY;
                value |= pcan::PCAN_ERROR_BUSOFF;
                value |= pcan::PCAN_ERROR_BUSPASSIVE;
                value
            }
            PcanError::QrcvEmpty => pcan::PCAN_ERROR_QRCVEMPTY,
            PcanError::QOverrun => pcan::PCAN_ERROR_QOVERRUN,
            PcanError::QxmtFull => pcan::PCAN_ERROR_QXMTFULL,
            PcanError::RegTest => pcan::PCAN_ERROR_REGTEST,
            PcanError::NoDriver => pcan::PCAN_ERROR_NODRIVER,
            PcanError::HwInUse => pcan::PCAN_ERROR_HWINUSE,
            PcanError::NetInUse => pcan::PCAN_ERROR_NETINUSE,
            PcanError::IllHw => pcan::PCAN_ERROR_ILLHW,
            PcanError::IllNet => pcan::PCAN_ERROR_ILLNET,
            PcanError::IllClient => pcan::PCAN_ERROR_ILLCLIENT,
            PcanError::Resource => pcan::PCAN_ERROR_RESOURCE,
            PcanError::IllParamType => pcan::PCAN_ERROR_ILLPARAMTYPE,
            PcanError::IllParamVal => pcan::PCAN_ERROR_ILLPARAMVAL,
            PcanError::Unknown => pcan::PCAN_ERROR_UNKNOWN,
            PcanError::IllData => pcan::PCAN_ERROR_ILLDATA,
            PcanError::IllMode => pcan::PCAN_ERROR_ILLMODE,
            PcanError::Caution => pcan::PCAN_ERROR_CAUTION,
            PcanError::Initialize => pcan::PCAN_ERROR_INITIALIZE,
            PcanError::IllOperation => pcan::PCAN_ERROR_ILLOPERATION,
        }
    }
}

impl From<PcanOkError> for u32 {
    fn from(value: PcanOkError) -> u32 {
        match value {
            PcanOkError::Ok => pcan::PCAN_ERROR_OK,
            PcanOkError::Err(error) => u32::from(error),
        }
    }
}

impl TryFrom<u32> for PcanError {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            pcan::PCAN_ERROR_XMTFULL => Ok(PcanError::XmtFull),
            pcan::PCAN_ERROR_OVERRUN => Ok(PcanError::Overrun),
            pcan::PCAN_ERROR_BUSLIGHT => Ok(PcanError::BusLight),
            pcan::PCAN_ERROR_BUSHEAVY => Ok(PcanError::BusHeavy),
            pcan::PCAN_ERROR_BUSPASSIVE => Ok(PcanError::BusPassive),
            pcan::PCAN_ERROR_BUSOFF => Ok(PcanError::BusOff),
            pcan::PCAN_ERROR_ANYBUSERR => Ok(PcanError::AnyBusErr),
            pcan::PCAN_ERROR_QRCVEMPTY => Ok(PcanError::QrcvEmpty),
            pcan::PCAN_ERROR_QOVERRUN => Ok(PcanError::QOverrun),
            pcan::PCAN_ERROR_QXMTFULL => Ok(PcanError::QxmtFull),
            pcan::PCAN_ERROR_REGTEST => Ok(PcanError::RegTest),
            pcan::PCAN_ERROR_NODRIVER => Ok(PcanError::NoDriver),
            pcan::PCAN_ERROR_HWINUSE => Ok(PcanError::HwInUse),
            pcan::PCAN_ERROR_NETINUSE => Ok(PcanError::NetInUse),
            pcan::PCAN_ERROR_ILLHW => Ok(PcanError::IllHw),
            pcan::PCAN_ERROR_ILLNET => Ok(PcanError::IllNet),
            pcan::PCAN_ERROR_ILLCLIENT => Ok(PcanError::IllClient),
            pcan::PCAN_ERROR_RESOURCE => Ok(PcanError::Resource),
            pcan::PCAN_ERROR_ILLPARAMTYPE => Ok(PcanError::IllParamType),
            pcan::PCAN_ERROR_ILLPARAMVAL => Ok(PcanError::IllParamVal),
            pcan::PCAN_ERROR_UNKNOWN => Ok(PcanError::Unknown),
            pcan::PCAN_ERROR_ILLDATA => Ok(PcanError::IllData),
            pcan::PCAN_ERROR_ILLMODE => Ok(PcanError::IllMode),
            pcan::PCAN_ERROR_CAUTION => Ok(PcanError::Caution),
            pcan::PCAN_ERROR_INITIALIZE => Ok(PcanError::Initialize),
            pcan::PCAN_ERROR_ILLOPERATION => Ok(PcanError::IllOperation),
            _ => Err(()),
        }
    }
}

impl TryFrom<u32> for PcanOkError {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            pcan::PCAN_ERROR_OK => Ok(PcanOkError::Ok),
            _ => {
                let err = PcanError::try_from(value)?;
                Ok(PcanOkError::Err(err))
            }
        }
    }
}
