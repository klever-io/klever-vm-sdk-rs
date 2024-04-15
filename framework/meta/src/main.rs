// #[cfg(all(target_env="musl", target_pointer_width = "64"))]
// #[global_allocator]
// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    klever_sc_meta::cli_main_standalone().await;
}
