use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::scm::SCMEngine;

pub struct ModelWatcher {
    _watcher: RecommendedWatcher,
    path_to_node: HashMap<PathBuf, String>,
}

impl ModelWatcher {
    pub fn new(
        engine: Arc<SCMEngine>,
        node_file_mappings: HashMap<String, String>,
    ) -> Result<Self, String> {
        let (tx, rx) = channel::<notify::Result<Event>>();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())
            .map_err(|e| format!("Failed to initialize watcher: {}", e))?;

        let mut path_to_node = HashMap::new();

        for (node, file_path) in node_file_mappings {
            let path = Path::new(&file_path);
            if let Ok(canonical_path) = path.canonicalize() {
                watcher
                    .watch(&canonical_path, RecursiveMode::NonRecursive)
                    .map_err(|e| format!("Failed to watch path {:?}: {}", canonical_path, e))?;

                path_to_node.insert(canonical_path, node);
            } else {
                return Err(format!("File path does not exist on disk: {}", file_path));
            }
        }

        let path_map_clone = path_to_node.clone();

        thread::spawn(move || {
            Self::event_loop(rx, engine, path_map_clone);
        });

        Ok(ModelWatcher {
            _watcher: watcher,
            path_to_node,
        })
    }

    fn event_loop(
        rx: Receiver<notify::Result<Event>>,
        engine: Arc<SCMEngine>,
        path_to_node: HashMap<PathBuf, String>,
    ) {
        let mut last_reloaded: HashMap<PathBuf, Instant> = HashMap::new();
        let debounce_duration = Duration::from_millis(500);

        while let Ok(event_result) = rx.recv() {
            if let Ok(event) = event_result {
                if matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Access(notify::event::AccessKind::Close(_))
                ) {
                    for path in event.paths {
                        if let Ok(canonical_path) = path.canonicalize() {
                            if let Some(node_name) = path_to_node.get(&canonical_path) {
                                let now = Instant::now();
                                if let Some(last_time) = last_reloaded.get(&canonical_path) {
                                    if now.duration_since(*last_time) < debounce_duration {
                                        continue;
                                    }
                                }

                                last_reloaded.insert(canonical_path.clone(), now);

                                if let Some(str_path) = canonical_path.to_str() {
                                    let _ = engine.reload_node_model(node_name, str_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
