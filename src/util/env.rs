// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/15/2026.

use crate::util::consts;
use std::fmt;
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

//static PROD_ENV: usize = Env::Prod as usize;

static FIRST_TEST_ENV: usize = Env::Test1 as usize;
static LAST_TEST_ENV: usize = Env::Test5 as usize;

#[derive(Copy, Clone, Debug, EnumIter, PartialEq)]
pub enum Env {
    Prod,
    Test1,
    Test2,
    Test3,
    Test4,
    Test5,
}

impl Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Env::Prod => "Master",
                Env::Test1 => "Test1",
                Env::Test2 => "Test2",
                Env::Test3 => "Test3",
                Env::Test4 => "Test4",
                Env::Test5 => "Test5",
            }
        )
    }
}

impl Env {
    pub fn relative_path(&self) -> &'static str {
        match self {
            Env::Prod => consts::PROD_RELATIVE_PATH,
            Env::Test1 => consts::TEST1_RELATIVE_PATH,
            Env::Test2 => consts::TEST2_RELATIVE_PATH,
            Env::Test3 => consts::TEST3_RELATIVE_PATH,
            Env::Test4 => consts::TEST4_RELATIVE_PATH,
            Env::Test5 => consts::TEST5_RELATIVE_PATH,
        }
    }

    pub fn is_test_env(env: Env) -> bool {
        env as usize >= FIRST_TEST_ENV && env as usize <= LAST_TEST_ENV
    }

    pub fn test_envs() -> impl Iterator<Item=Env> {
        Env::iter().skip(FIRST_TEST_ENV).take(LAST_TEST_ENV - FIRST_TEST_ENV + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_test_env_works() {
        for env in Env::test_envs() {
            assert!(Env::is_test_env(env));
        }
    }

    #[test]
    fn test_test_envs_works() {
        let mut count = 0;
        let mut expected_env = Env::Test1 as usize;
        for env in Env::test_envs() {
            assert!(env as usize >= FIRST_TEST_ENV);
            assert!(env as usize <= LAST_TEST_ENV);
            assert_eq!(expected_env, env as usize);
            count += 1;
            expected_env += 1
        }
        assert_eq!(count, LAST_TEST_ENV - FIRST_TEST_ENV + 1);
    }
}
