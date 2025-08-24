// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "booking_status"))]
    pub struct BookingStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "bus_type"))]
    pub struct BusType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "license"))]
    pub struct License;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_status"))]
    pub struct PaymentStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_type"))]
    pub struct PaymentType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "refund_status"))]
    pub struct RefundStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "trip_status"))]
    pub struct TripStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BookingStatus;

    booking (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        trip_id -> Nullable<Int4>,
        seat_id -> Nullable<Int4>,
        booking_time -> Timestamp,
        status -> BookingStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BusType;

    bus (id) {
        id -> Int4,
        #[max_length = 9]
        bus_number -> Varchar,
        bus_type -> BusType,
        capacity -> Int4,
        #[max_length = 20]
        org_name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        organization_id -> Nullable<Int4>,
    }
}

diesel::table! {
    busroutes (id) {
        id -> Int4,
        #[max_length = 10]
        route_number -> Varchar,
        #[max_length = 100]
        start_location -> Varchar,
        #[max_length = 100]
        end_location -> Varchar,
        distance -> Int4,
        travel_time -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RefundStatus;

    cancellations (id) {
        id -> Int4,
        booking_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        reason -> Nullable<Text>,
        refund_amount -> Nullable<Numeric>,
        refund_status -> Nullable<RefundStatus>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::License;

    driver (id) {
        id -> Int4,
        #[max_length = 20]
        driver_name -> Varchar,
        driver_age -> Int4,
        #[max_length = 12]
        driver_phone -> Varchar,
        #[max_length = 15]
        driver_license_number -> Varchar,
        license_type -> License,
    }
}

diesel::table! {
    organization_ratings (id) {
        id -> Int4,
        organization_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        rating -> Nullable<Int4>,
        review -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        contact_email -> Nullable<Varchar>,
        #[max_length = 20]
        contact_phone -> Nullable<Varchar>,
        address -> Nullable<Text>,
        #[max_length = 50]
        gst_number -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PaymentType;
    use super::sql_types::PaymentStatus;

    payments (id) {
        id -> Int4,
        booking_id -> Int4,
        amount -> Numeric,
        payment_method -> PaymentType,
        status -> Nullable<PaymentStatus>,
        #[max_length = 100]
        transaction_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ratings (id) {
        id -> Int4,
        booking_id -> Int4,
        user_id -> Int4,
        bus_id -> Int4,
        driver_id -> Nullable<Int4>,
        operator_id -> Nullable<Int4>,
        rating -> Int4,
        review -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    seats (id) {
        id -> Int4,
        trip_id -> Nullable<Int4>,
        bus_id -> Nullable<Int4>,
        seat_number -> Int4,
        is_booked -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TripStatus;

    trips (id) {
        id -> Int4,
        bus_id -> Nullable<Int4>,
        route_id -> Nullable<Int4>,
        arrival_time -> Timestamp,
        departure_time -> Timestamp,
        #[max_length = 20]
        starting_location -> Varchar,
        price -> Numeric,
        status -> TripStatus,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 20]
        email -> Varchar,
        #[max_length = 12]
        phone -> Varchar,
        #[max_length = 8]
        password -> Varchar,
    }
}

diesel::joinable!(booking -> seats (seat_id));
diesel::joinable!(booking -> trips (trip_id));
diesel::joinable!(booking -> users (user_id));
diesel::joinable!(bus -> organizations (organization_id));
diesel::joinable!(cancellations -> booking (booking_id));
diesel::joinable!(cancellations -> users (user_id));
diesel::joinable!(organization_ratings -> organizations (organization_id));
diesel::joinable!(organization_ratings -> users (user_id));
diesel::joinable!(payments -> booking (booking_id));
diesel::joinable!(ratings -> booking (booking_id));
diesel::joinable!(ratings -> bus (bus_id));
diesel::joinable!(ratings -> driver (driver_id));
diesel::joinable!(ratings -> organizations (operator_id));
diesel::joinable!(ratings -> users (user_id));
diesel::joinable!(seats -> bus (bus_id));
diesel::joinable!(seats -> trips (trip_id));
diesel::joinable!(trips -> bus (bus_id));
diesel::joinable!(trips -> busroutes (route_id));

diesel::allow_tables_to_appear_in_same_query!(
    booking,
    bus,
    busroutes,
    cancellations,
    driver,
    organization_ratings,
    organizations,
    payments,
    ratings,
    seats,
    trips,
    users,
);
