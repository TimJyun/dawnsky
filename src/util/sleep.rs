pub async fn sleep(millis: u32) {
    #[cfg(feature = "web")]
    {
        gloo_timers::future::TimeoutFuture::new(millis).await;
    }

    // #[cfg(feature = "server")]
    // {
    //     tokio::time::sleep(tokio::time::Duration::from_millis(millis as u64)).await;
    // }
}
