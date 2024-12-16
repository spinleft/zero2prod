/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2024-08-19 19:51:56
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2024-12-15 21:36:30
 * @FilePath: \zero2prod\src\main.rs
 * @Description:
 *
 * Copyright (c) 2024 by ${git_name_email}, All Rights Reserved.
 */
use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
