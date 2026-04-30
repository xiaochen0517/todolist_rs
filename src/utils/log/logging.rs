use std::io;
use tracing_appender::rolling;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logging(is_production: bool) {
    if is_production {
        // 生产环境：按日期轮转文件日志
        let file_appender = rolling::daily("./logs", "app.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::registry()
            .with(EnvFilter::new("info"))
            .with(
                fmt::layer()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_line_number(true)
                    .pretty(), // 或使用 .json() 用于 JSON 格式
            )
            .init();

        println!("✓ 生产日志已配置，输出到 ./logs 目录（按日期轮转）");
    } else {
        // 开发环境：输出到控制台
        tracing_subscriber::registry()
            .with(EnvFilter::new("debug"))
            .with(
                fmt::layer()
                    .with_writer(io::stdout)
                    .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string()))
                    .with_level(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_target(false)
                    .with_file(false)
                    .with_line_number(false)
                    .with_span_events(fmt::format::FmtSpan::NONE)
                    .compact(),
            )
            .init();

        println!("✓ 开发日志已配置，输出到控制台");
    }
}
