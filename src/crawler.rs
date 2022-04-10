use crate::spiders::Spider;
use futures::stream::StreamExt;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::{mpsc, Barrier},
    time::sleep,
};

use crate::nodes::ServerNode;

pub struct Crawler {
    delay: Duration,
    crawler_count: usize,
    processor_count: usize,
}

impl Crawler {
    pub fn new(delay: Duration, crawler_count: usize, processor_count: usize) -> Self {
        Crawler {
            delay,
            crawler_count,
            processor_count,
        }
    }

    pub async fn run(&self, spider: Arc<dyn Spider>) -> Vec<ServerNode> {
        let mut visited_urls = HashSet::<String>::new();
        let crawler_count = self.crawler_count;
        let crawler_queue_size = crawler_count * 400;
        let processor_count = self.processor_count;
        let processor_queue_size = processor_count * 10;
        let active_count = Arc::new(AtomicUsize::new(0));

        let (urls_to_visit_tx, urls_to_visit_rx) = mpsc::channel(crawler_queue_size);
        let (items_tx, items_rx) = mpsc::channel::<ServerNode>(processor_queue_size);
        let (new_urls_tx, mut new_urls_rx) = mpsc::channel(crawler_queue_size);
        let barrier = Arc::new(Barrier::new(3));

        for url in spider.start_urls() {
            visited_urls.insert(url.clone());
            let _ = urls_to_visit_tx.send(url).await;
        }

        let barrier_copy = barrier.clone();
        let handle = tokio::spawn(async move {
            let nodes: Vec<ServerNode> = tokio_stream::wrappers::ReceiverStream::new(items_rx)
                .collect()
                .await;
            barrier_copy.wait().await;
            nodes
        });
        self.launch_scrapers(
            crawler_count,
            spider.clone(),
            urls_to_visit_rx,
            new_urls_tx.clone(),
            items_tx,
            active_count.clone(),
            self.delay,
            barrier.clone(),
        );

        loop {
            if let Ok((visited_url, new_urls)) = new_urls_rx.try_recv() {
                visited_urls.insert(visited_url);

                for url in new_urls {
                    if !visited_urls.contains(&url) {
                        visited_urls.insert(url.clone());
                        log::debug!("queueing: {}", url);
                        let _ = urls_to_visit_tx.send(url).await;
                    }
                }
            }

            if new_urls_tx.capacity() == crawler_queue_size
                && urls_to_visit_tx.capacity() == crawler_queue_size
                && active_count.load(Ordering::SeqCst) == 0
            {
                break;
            }

            sleep(Duration::from_millis(5)).await;
        }

        log::info!("crawler: control loop exited");

        drop(urls_to_visit_tx);

        barrier.wait().await;

        handle.await.unwrap_or_else(|_| vec![])
    }

    #[allow(clippy::too_many_arguments)]
    fn launch_scrapers(
        &self,
        concurrency: usize,
        spider: Arc<dyn Spider>,
        urls_to_visit: mpsc::Receiver<String>,
        new_urls: mpsc::Sender<(String, Vec<String>)>,
        items_tx: mpsc::Sender<ServerNode>,
        active_spiders: Arc<AtomicUsize>,
        delay: Duration,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(urls_to_visit)
                .for_each_concurrent(concurrency, |queued_url| {
                    let queued_url = queued_url.clone();
                    async {
                        active_spiders.fetch_add(1, Ordering::SeqCst);
                        let mut urls = Vec::new();
                        let res = spider
                            .scrape(queued_url.clone())
                            .await
                            .map_err(|err| {
                                log::error!("{}", err);
                                err
                            })
                            .ok();

                        if let Some((items, new_urls)) = res {
                            for item in items {
                                let _ = items_tx.send(item).await;
                            }
                            urls = new_urls;
                        }

                        let _ = new_urls.send((queued_url, urls)).await;
                        sleep(delay).await;
                        active_spiders.fetch_sub(1, Ordering::SeqCst);
                    }
                })
                .await;

            drop(items_tx);
            barrier.wait().await;
        });
    }
}
