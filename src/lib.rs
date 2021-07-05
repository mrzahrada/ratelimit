#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::{raw, Context, NextArg, RedisResult};
use std::os::raw::c_void;

pub struct ApiKey {}

static API_KEY: RedisType = RedisType::new(
    "rl_api_key",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        free: Some(free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,

        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,
    },
);

unsafe extern "C" fn free(value: *mut c_void) {
    Box::from_raw(value as *mut ApiKey);
}

fn check_api_key(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let fingerprint = args.next_string()?;
    // TODO key has to by valid base64 sha1 hash -> size, characters

    let key = ctx.open_key_writable(&fingerprint);

    let key = key.get_value::<ApiKey>(&API_KEY)?;

    Ok("".into())
}

redis_module! {
    name: "alloc",
    version: 1,
    data_types: [
        API_KEY,
    ],
    commands: [
        ["rl.apikey.check", check_api_key, "write", 1, 1, 1],
    ],
}
