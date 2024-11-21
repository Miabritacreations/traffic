#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

/// Represents a traffic report submitted by users.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TrafficReport {
    id: u64,
    reporter_id: u64,
    description: String,
    location: String, // Geo-coordinates or area name
    timestamp: u64,
    severity: u8, // 1 to 5 scale
    resolved: bool,
}

/// Represents a user's profile, tracking their activity and rewards.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UserProfile {
    id: u64,
    username: String,
    points: u64,
    contributions: u64, // Number of reports submitted
    route_tokens: u64,  // Reward tokens earned
}

/// Error handling enum for CRUD operations.
#[derive(candid::CandidType, Serialize, Deserialize)]
enum AppError {
    NotFound(String),
    AlreadyExists(String),
    OperationFailed(String),
}

/// Implements the traits for `TrafficReport` to be stored in stable memory.
impl Storable for TrafficReport {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TrafficReport {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for UserProfile {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for UserProfile {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static REPORT_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USER_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );

    static REPORT_STORAGE: RefCell<StableBTreeMap<u64, TrafficReport, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static USER_STORAGE: RefCell<StableBTreeMap<u64, UserProfile, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

/// CREATE: Adds a new traffic report.
#[ic_cdk::update]
fn add_traffic_report(
    description: String,
    location: String,
    severity: u8,
) -> Result<TrafficReport, AppError> {
    let id = REPORT_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .map_err(|_| AppError::OperationFailed("Failed to generate report ID".into()))?;

    let reporter_id = 1; // Example authenticated user ID. Replace with actual auth.

    let report = TrafficReport {
        id,
        reporter_id,
        description,
        location,
        timestamp: time(),
        severity,
        resolved: false,
    };

    REPORT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, report.clone());
    });

    update_user_profile(reporter_id, 10)?; // Reward points for reporting.
    Ok(report)
}

/// READ: Retrieves a traffic report by ID.
#[ic_cdk::query]
fn get_traffic_report(id: u64) -> Result<TrafficReport, AppError> {
    REPORT_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .ok_or_else(|| AppError::NotFound(format!("Report with ID {} not found", id)))
    })
}

/// UPDATE: Updates a traffic report by ID.
#[ic_cdk::update]
fn update_traffic_report(
    id: u64,
    description: Option<String>,
    location: Option<String>,
    severity: Option<u8>,
) -> Result<TrafficReport, AppError> {
    REPORT_STORAGE.with(|storage| {
        let mut report = storage
            .borrow_mut()
            .get(&id)
            .ok_or_else(|| AppError::NotFound(format!("Report with ID {} not found", id)))?;

        if let Some(desc) = description {
            report.description = desc;
        }
        if let Some(loc) = location {
            report.location = loc;
        }
        if let Some(sev) = severity {
            report.severity = sev;
        }
        storage.borrow_mut().insert(id, report.clone());
        Ok(report)
    })
}

/// DELETE: Deletes a traffic report by ID.
#[ic_cdk::update]
fn delete_traffic_report(id: u64) -> Result<TrafficReport, AppError> {
    REPORT_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .remove(&id)
            .ok_or_else(|| AppError::NotFound(format!("Report with ID {} not found", id)))
    })
}

/// CREATE: Adds or updates a user profile.
#[ic_cdk::update]
fn update_user_profile(user_id: u64, points: u64) -> Result<UserProfile, AppError> {
    USER_STORAGE.with(|storage| {
        let mut user = storage
            .borrow_mut()
            .get(&user_id)
            .unwrap_or(UserProfile {
                id: user_id,
                username: format!("User{}", user_id),
                ..Default::default()
            });

        user.points += points;
        user.contributions += 1;
        storage.borrow_mut().insert(user_id, user.clone());
        Ok(user)
    })
}

/// READ: Retrieves a user profile by ID.
#[ic_cdk::query]
fn get_user_profile(user_id: u64) -> Result<UserProfile, AppError> {
    USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&user_id)
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))
    })
}

/// DELETE: Deletes a user profile by ID.
#[ic_cdk::update]
fn delete_user_profile(user_id: u64) -> Result<UserProfile, AppError> {
    USER_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .remove(&user_id)
            .ok_or_else(|| AppError::NotFound(format!("User with ID {} not found", user_id)))
    })
}

// Export candid for front-end integration.
ic_cdk::export_candid!();
