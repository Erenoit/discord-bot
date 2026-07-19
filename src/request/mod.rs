#[cfg(feature = "database")]
pub mod cookie_jar;
pub mod reddit_structs;
#[cfg(feature = "spotify")]
pub mod sp_structs;
pub mod yt_structs;

use std::{
    mem::MaybeUninit,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::{Duration, SystemTime},
};

use reqwest::Client;

#[cfg(feature = "database")]
use crate::request::cookie_jar::COOKIE_JAR;

const BUFFER_SIZE: usize = 2;
const UPDATE_DURATION: Duration = Duration::from_hours(24 * 7);
const FALLBACK_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:149.0) Gecko/20100101 Firefox/149.0";

static IS_UPDATING: AtomicBool = AtomicBool::new(false);
static IDX: AtomicUsize = AtomicUsize::new(0);
static mut LAST_UPDATES: [SystemTime; BUFFER_SIZE] = [SystemTime::UNIX_EPOCH; BUFFER_SIZE];
static mut USER_AGENTS: [String; BUFFER_SIZE] = [const { String::new() }; BUFFER_SIZE];
static mut REQWEST_CLIENTS: [MaybeUninit<Client>; BUFFER_SIZE] =
    [const { MaybeUninit::uninit() }; BUFFER_SIZE];

pub fn get_user_agent() -> &'static String {
    check_and_update();
    unsafe { &USER_AGENTS[IDX.load(Ordering::Acquire)] }
}

pub fn get_reqwest_client() -> Client {
    check_and_update();
    #[expect(
        clippy::multiple_unsafe_ops_per_block,
        reason = "Its clear so no need to divide"
    )]
    unsafe { REQWEST_CLIENTS[IDX.load(Ordering::Acquire)].assume_init_ref() }.clone()
}

fn check_and_update() {
    if let Ok(d) = unsafe { LAST_UPDATES[IDX.load(Ordering::Acquire)].elapsed() }
        && d < UPDATE_DURATION
    {
        return;
    }

    if IS_UPDATING
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    let next_idx = (IDX.load(Ordering::Acquire) + 1) % BUFFER_SIZE;

    let response_task = async {
        let r = Client::new()
            .get("https://raw.githubusercontent.com/jnrbsn/user-agents/main/user-agents.json")
            .send()
            .await?;
        r.text().await
    };

    let response = match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(response_task)),
        Err(_) =>
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create fallback runtime for reqwest")
                .block_on(response_task),
    }
    .unwrap_or(String::new());

    let new_agent = sonic_rs::from_str::<Vec<&str>>(&response)
        .ok()
        .and_then(|v| {
            v.into_iter()
                .rfind(|s| s.contains("Gecko/") && s.contains("Windows NT"))
        })
        .unwrap_or(FALLBACK_USER_AGENT);

    unsafe {
        let s = &mut USER_AGENTS[next_idx];
        s.clear();
        s.push_str(new_agent);
    }

    let client_builder = Client::builder()
        .user_agent(new_agent)
        .use_rustls_tls()
        .https_only(true);

    #[cfg(feature = "database")]
    let client_builder = client_builder.cookie_provider(Arc::clone(&COOKIE_JAR));

    unsafe {
        REQWEST_CLIENTS[next_idx].write(
            client_builder
                .build()
                .expect("TLS backend cannot be initialized"),
        );
    }

    unsafe {
        LAST_UPDATES[next_idx] = SystemTime::now();
    }

    IDX.store(next_idx, Ordering::Release);
    IS_UPDATING.store(false, Ordering::Release);
}
