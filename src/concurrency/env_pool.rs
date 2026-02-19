// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/22/2026.

use crate::util::env::Env;
use std::sync::LazyLock;
use std::sync::{Condvar, Mutex, Once};

static INIT: Once = Once::new();
static LOCK_CV_PAIR: LazyLock<(Mutex<bool>, Condvar)> = LazyLock::new(|| (Mutex::new(true), Condvar::new()));

static POOL: LazyLock<Mutex<Vec<Env>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub struct EnvPoolGuard {
    pub env: Env,
}

impl EnvPoolGuard {
    pub fn new() -> Self {
        Self { env: EnvPool::get() }
    }
}

impl Default for EnvPoolGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EnvPoolGuard {
    fn drop(&mut self) {
        EnvPool::put(self.env);
    }
}

pub struct EnvPool;
impl EnvPool {
    /// By default, the EnvPool is statically initialized to use test environments.  If a different set
    /// of environments is desired, this function may be called prior to the first use of the get/put methods.
    /// Note that change_envs is not thread safe and cannot be called once the pool is in use.
    pub fn change_envs(envs: impl Iterator<Item=Env>) {
        let mut pool = POOL.lock().unwrap();
        pool.clear();
        pool.extend(envs);
    }

    pub fn get() -> Env {
        EnvPool::init();

        let (lock, cvar) = &*LOCK_CV_PAIR;
        let mut is_env_available = lock.lock().unwrap();

        while !*is_env_available {
            is_env_available = cvar.wait(is_env_available).unwrap();
        }

        let mut pool = POOL.lock().unwrap();
        let env = pool.pop().unwrap();
        if pool.is_empty() {
            *is_env_available = false;
        } else {
            cvar.notify_one();
        }
        env
    }

    pub fn init() {
        // Initialize the pool
        INIT.call_once(|| {
            let mut pool = POOL.lock().unwrap();
            // Check if the pool is empty before proceeding.  We don't want to override a previous call to
            // change_envs.
            if pool.is_empty() {
                for env in Env::test_envs() {
                    pool.push(env);
                }
            }
        });
    }

    pub fn put(env: Env) {
        let (lock, cvar) = &*LOCK_CV_PAIR;
        let mut is_env_available = lock.lock().unwrap();
        let mut pool = POOL.lock().unwrap();
        pool.push(env);
        *is_env_available = true;
        cvar.notify_one();
    }
}
