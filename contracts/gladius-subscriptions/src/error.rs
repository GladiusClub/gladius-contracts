use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GladiusSubscriptionsError {
    NotInitialized = 201,
    AlreadyInitialized = 202,
    NegativesNotSupported = 203,
    SportClubNotFound = 224,
    ParentNotFound = 225,
    StudentNotFound = 208,
    CourseDoesNotContainsStudent = 226,
    InsufficientFunds = 209,
    StudentAlreadyEnrolled = 207,
}


