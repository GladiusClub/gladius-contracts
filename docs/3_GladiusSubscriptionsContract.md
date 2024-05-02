## The Gladius Subscription and Token Distribution Smart Contract

- Sport Clubs can open sport subscriptions at a fixed price in EURC.
Parents can subscribe a Student and pay the price `p+k` for a monthly subscription.
- The Contract splits p directly to Sport Clubs and `k` to the Gladius Coin Token Emitter Contract. In exchange, the contract receives Gladius Coins.
- The contract holds Gladius Coins on behalf of Sport Clubs.
- The contract maintains a list of Sport Clubs, Parents, Students, and Subscriptions with their expiration date.
- Sport Clubs can distribute these Gladius Coins only to Students who have been subscribed.
- Sport Clubs can also distribute these Gladius Coins to some NFT contract so physically redeemable NFTs have economic value.

Interface:
```rust

/// Initializes the contract with administrator, token, and Gladius coin emitter addresses.
/// 
/// # Arguments
///
/// * `e` - The environment.
/// * `admin` - The address of the administrator.
/// * `token` - The address of the token.
/// * `gladius_coin_emitter` - The address of the Gladius coin emitter.
fn initialize(
    e: Env,
    admin: Address,
    token: Address,
    gladius_coin_emitter: Address) -> Result<(), GladiusSubscriptionsError>;

// Admin Functions

// TODO: Write change admin function

/// Sets the status of whether an address is a sport club or not.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `addr` - The address to set the status for.
/// * `is` - The boolean value indicating whether the address is a sport club.
fn set_is_sport_club(e: Env, addr: Address, is: bool) -> Result<(), GladiusSubscriptionsError>;

/// Sets the status of whether an address is a parent or not.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `addr` - The address to set the status for.
/// * `is` - The boolean value indicating whether the address is a parent.
fn set_is_parent(e: Env, addr: Address, is: bool) -> Result<(), GladiusSubscriptionsError>;

/// Sets the status of whether an address is a student or not.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `addr` - The address to set the status for.
/// * `is` - The boolean value indicating whether the address is a student.
fn set_is_student(e: Env, addr: Address, is: bool) -> Result<(), GladiusSubscriptionsError>;

// Sport Clubs Functions

/// Creates a new course and returns its index.
/// 
/// # Arguments
///
/// * `e` - The environment.
/// * `sport_club` - The address of the sport club creating the course.
/// * `price` - The price of the course.
/// * `incentive` - The incentive for the course.
/// * `title` - The title of the course.
///
/// # Returns
///
/// The index of the newly created course.
fn create_course(
    e: Env, 
    sport_club: Address, 
    price: i128, 
    incentive: i128,
    title: String) -> Result<u32, GladiusSubscriptionsError>;

/// Distributes Gladius Coins to students enrolled in the specified course.
/// 
/// # Arguments
///
/// * `e` - The environment.
/// * `course_index` - The index of the course.
/// * `student` - The address of the student to receive Gladius Coins.
/// * `amount` - The amount of Gladius Coins to distribute.
fn distribute_gladius_coins(
    e: Env,
    course_index: u32,
    student: Address,
    amount: i128) -> Result<(), GladiusSubscriptionsError>;

// Parents Functions

/// Subscribes a student to a course and handles payment and token transfer.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `parent` - The address of the parent.
/// * `student` - The address of the student.
/// * `course_index` - The index of the course to subscribe to.
fn subscribe_course(
    e:Env, 
    parent: Address, 
    student: Address, 
    course_index: u32) -> Result<(), GladiusSubscriptionsError>;

fn is_sport_club(e:Env, addr: Address) -> bool;
fn is_parent(e:Env, addr: Address) -> bool;
fn is_student(e:Env, addr: Address) -> bool;

fn get_admin(e:Env) -> Result<Address, GladiusSubscriptionsError>;
fn get_token(e:Env) -> Result<Address, GladiusSubscriptionsError>;
fn get_gladius_coin_emitter(e:Env) -> Result<Address, GladiusSubscriptionsError>;
fn get_course(e: Env, course_index: u32) -> Result<Course, GladiusSubscriptionsError>;
fn get_total_courses(e: Env) -> u32;
```
