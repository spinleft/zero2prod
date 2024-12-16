/*
 * @Author: spinleft spinleftgit@gmail.com
 * @Date: 2024-12-15 20:25:05
 * @LastEditors: spinleft spinleftgit@gmail.com
 * @LastEditTime: 2024-12-17 00:51:53
 * @FilePath: \zero2prod\tests\api\health_check.rs
 * @Description:
 *
 * Copyright (c) 2024 by ${git_name_email}, All Rights Reserved.
 */
use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
