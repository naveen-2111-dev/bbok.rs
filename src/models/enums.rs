use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[DieselType = "BusType"]
pub enum BusTypeEnum {
    #[db_rename = "AC"]
    Ac,
    #[db_rename = "NON_AC"]
    NonAc,
    #[db_rename = "SLEEPER"]
    Sleeper,
    #[db_rename = "SEMI_SLEEPER"]
    SemiSleeper,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[DieselType = "BookingStatus"]
pub enum BookingStatusEnum {
    PENDING,
    CONFIRMED,
    CANCELLED,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[DieselType = "License"]
pub enum LicenseEnum {
    HEAVY,
    LIGHT,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[DieselType = "PaymentStatus"]
pub enum PaymentStatusEnum {
    PENDING,
    SUCCESS,
    FAILED,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[DieselType = "PaymentType"]
pub enum PaymentTypeEnum {
    UPI,
    CARD,
    BANK,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[DieselType = "RefundStatus"]
pub enum RefundStatusEnum {
    PENDING,
    SUCCESS,
    FAILED,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[DieselType = "TripStatus"]
pub enum TripStatusEnum {
    SCHEDULED,
    CONFIRMED,
    CANCELLED,
}
