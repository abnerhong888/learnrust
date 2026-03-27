// add to toml
// [dependencies]
// tokio = { version = "1", features = ["full"] }

use std::time::Duration;

// 1. Define an async function. It returns a 'Future' that will eventually yield a String.
async fn fetch_user_data(user_id: i32) -> String{
    println!("-> [DB] START: Fetching data for user: {}", user_id);
    
    // Simulate a database delay (non-blocking)
    // We must .await this delay, or NOTHING happens.
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // The delay finished. This code is now rescheduled and resumes.
    println!("<- [DB] DONE: User {} data retrieved.", user_id);
    format!("UserName_for_{}", user_id)
}

// 2. Define a second async function.
async fn fetch_account_status(user_id: i32) -> &'static str {
    println!("-> [API] START: Checking account status for: {}", user_id);
    
    // Simulate an API delay (non-blocking)
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    println!("<- [API] DONE: Account status for {} retrieved.", user_id);
    "Active"
}

pub async fn run(){
    println!("===== mod_09_async::run() =====");

    let target_user = 42;
    
    // tokio runs both concurrently on the same thread
    let (user_info, account_status) = tokio::join!(
        fetch_user_data(target_user), 
        fetch_account_status(target_user)
        );


    // println!("Main Thread: Program starting. Requesting info for user {}.", target_user);

    // println!("\n*** Starting First Request ***");
    
    // // 4. SEQUENTIAL EXECUTION: We call the first function and immediately .await it.
    // // main() is now PAUSED until fetch_user_data finishes.
    // let user_info = fetch_user_data(target_user).await;
    
    // println!("\n*** Starting Second Request ***");

    // // 5. This code only runs *after* fetch_user_data is fully complete.
    // // main() is PAUSED again.
    // let account_status = fetch_account_status(target_user).await;

    // 6. Both Futures are complete. main() resumes for the last time.
    println!("\nMain Thread: All data collected.");
    println!("User: {} | Status: {}", user_info, account_status);

    println!();
}