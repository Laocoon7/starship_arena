use bevy::log::{Level, LogPlugin};

pub fn my_log_plugin() -> LogPlugin {
    let log_level = get_log_level();
    LogPlugin {
        level: log_level,
        filter: get_log_filters(log_level).to_string(),
    }
}

pub const fn get_log_level() -> Level {
    if cfg!(feature = "trace") {
        Level::TRACE
    } else if cfg!(feature = "debug") {
        Level::DEBUG
    } else if cfg!(feature = "dev") {
        Level::INFO
    } else {
        Level::ERROR
    }
}

pub const fn get_log_filters(log_level: Level) -> &'static str {
    match log_level {
        Level::INFO => {
            "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
             bevy_render::render_resource::pipeline_cache=warn,big_brain=debug,sequence=debug,naga=warn"
        },
        Level::TRACE | Level::DEBUG => {
            "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
             bevy_render::render_resource::pipeline_cache=warn,bevy_app=debug,big_brain=debug,sequence=debug,\
             naga=warn"
        },
        _ => "",
    }
}
