use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

use crate::agents::ClaudeAdapter;
use crate::pty::adapters::{EngineAdapter, PortablePtyAdapter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineAdapterQuery {
  /// Returns all registered adapters (no filtering).
  All,
  /// Returns adapters where `detect()` is true.
  Detected,
  /// Returns adapters where `detect()` is true and `health()` returns Ok.
  Healthy,
}

/// Thread-safe registry for engine adapters.
///
/// - "Typed": values are `Arc<dyn EngineAdapter>` rather than loosely typed maps.
/// - "Dynamic registration": adapters can be registered/unregistered at runtime.
/// - "Queries": supports filtering by detected/healthy, and querying metadata.
#[derive(Default)]
pub struct EngineAdapterRegistry {
  adapters: RwLock<HashMap<String, Arc<dyn EngineAdapter>>>,
}

impl EngineAdapterRegistry {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_builtins() -> Self {
    let registry = Self::new();
    // Built-in adapters are registered eagerly so the app has at least one adapter.
    let _ = registry.register(Arc::new(PortablePtyAdapter));
    // CLI ajan adaptörü (Claude Code) — kurulu değilse detect() false döner.
    let _ = registry.register(Arc::new(ClaudeAdapter));
    registry
  }

  pub fn register(&self, adapter: Arc<dyn EngineAdapter>) -> Result<(), String> {
    let id = adapter.id().trim().to_string();
    if id.is_empty() {
      return Err("engine adapter id cannot be empty".to_string());
    }

    let meta = adapter.metadata();
    if meta.engine_type.trim().is_empty() {
      return Err("engine adapter must have a non-empty engine_type in metadata".to_string());
    }

    let mut adapters = self
      .adapters
      .write()
      .map_err(|_| "engine adapter registry lock poisoned".to_string())?;

    if adapters.contains_key(&id) {
      return Err(format!("engine adapter already registered: {id}"));
    }
    adapters.insert(id, adapter);
    Ok(())
  }

  pub fn unregister(&self, id: &str) -> Result<Option<Arc<dyn EngineAdapter>>, String> {
    let mut adapters = self
      .adapters
      .write()
      .map_err(|_| "engine adapter registry lock poisoned".to_string())?;
    Ok(adapters.remove(id))
  }

  pub fn get(&self, id: &str) -> Result<Option<Arc<dyn EngineAdapter>>, String> {
    let adapters = self
      .adapters
      .read()
      .map_err(|_| "engine adapter registry lock poisoned".to_string())?;
    Ok(adapters.get(id).cloned())
  }

  pub fn find_by_engine_type(&self, engine_type: &str) -> Result<Vec<Arc<dyn EngineAdapter>>, String> {
    let adapters = self
      .adapters
      .read()
      .map_err(|_| "engine adapter registry lock poisoned".to_string())?;

    let mut matches = Vec::new();
    for adapter in adapters.values() {
      if adapter.metadata().engine_type == engine_type {
        matches.push(adapter.clone());
      }
    }
    Ok(matches)
  }

  pub fn find_by_version(&self, engine_type: &str, version: &str) -> Result<Vec<Arc<dyn EngineAdapter>>, String> {
    let adapters = self
      .adapters
      .read()
      .map_err(|_| "engine adapter registry lock poisoned".to_string())?;

    let mut matches = Vec::new();
    for adapter in adapters.values() {
      let meta = adapter.metadata();
      if meta.engine_type == engine_type && meta.version.as_deref() == Some(version) {
        matches.push(adapter.clone());
      }
    }
    Ok(matches)
  }

  pub fn query_metadata<F>(&self, filter: F) -> Result<Vec<Arc<dyn EngineAdapter>>, String>
  where
    F: Fn(&crate::pty::adapters::EngineMetadata) -> bool,
  {
    let adapters = self
      .adapters
      .read()
      .map_err(|_| "engine adapter registry lock poisoned".to_string())?;

    let mut matches = Vec::new();
    for adapter in adapters.values() {
      if filter(&adapter.metadata()) {
        matches.push(adapter.clone());
      }
    }
    Ok(matches)
  }

  pub fn list_ids(&self) -> Result<Vec<String>, String> {
    self.query_ids(EngineAdapterQuery::All)
  }

  pub fn query_ids(&self, query: EngineAdapterQuery) -> Result<Vec<String>, String> {
    let adapters = self
      .adapters
      .read()
      .map_err(|_| "engine adapter registry lock poisoned".to_string())?;

    let mut ids = Vec::new();
    for (id, adapter) in adapters.iter() {
      let include = match query {
        EngineAdapterQuery::All => true,
        EngineAdapterQuery::Detected => adapter.detect(),
        EngineAdapterQuery::Healthy => adapter.detect() && adapter.health().is_ok(),
      };
      if include {
        ids.push(id.clone());
      }
    }

    // Deterministic ordering helps with tests and avoids UI churn.
    ids.sort();
    Ok(ids)
  }

  /// Picks a default adapter to use for spawn operations.
  ///
  /// Selection order:
  /// 1) first adapter in lexicographic id order that is healthy
  /// 2) first adapter in lexicographic id order that is detected
  pub fn select_default(&self) -> Result<Option<Arc<dyn EngineAdapter>>, String> {
    let adapters = self
      .adapters
      .read()
      .map_err(|_| "engine adapter registry lock poisoned".to_string())?;

    let mut keys: Vec<_> = adapters.keys().cloned().collect();
    keys.sort();

    for id in &keys {
      let adapter = adapters.get(id).expect("key must exist");
      if adapter.detect() && adapter.health().is_ok() {
        return Ok(Some(adapter.clone()));
      }
    }

    for id in &keys {
      let adapter = adapters.get(id).expect("key must exist");
      if adapter.detect() {
        return Ok(Some(adapter.clone()));
      }
    }

    Ok(None)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::pty::adapters::{EngineMetadata, SpawnedPty};
  use portable_pty::CommandBuilder;

  #[derive(Clone)]
  struct MockAdapter {
    id: String,
    detected: bool,
    healthy: bool,
    engine_type: String,
    version: Option<String>,
  }

  impl EngineAdapter for MockAdapter {
    fn id(&self) -> &str {
      &self.id
    }

    fn metadata(&self) -> EngineMetadata {
      EngineMetadata {
        engine_type: self.engine_type.clone(),
        version: self.version.clone(),
        capabilities: vec![],
      }
    }

    fn detect(&self) -> bool {
      self.detected
    }

    fn health(&self) -> Result<(), String> {
      if self.healthy {
        Ok(())
      } else {
        Err("unhealthy".to_string())
      }
    }

    fn spawn(&self, _cmd: CommandBuilder, _cols: u16, _rows: u16) -> Result<SpawnedPty, String> {
      Err("not implemented for unit test".to_string())
    }

    fn stop(&self, _child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String> {
      Ok(())
    }
  }

  #[test]
  fn register_validation() {
    let reg = EngineAdapterRegistry::new();

    // Empty ID
    let err = reg.register(Arc::new(MockAdapter {
      id: "".to_string(),
      detected: true,
      healthy: true,
      engine_type: "test".to_string(),
      version: None,
    })).unwrap_err();
    assert_eq!(err, "engine adapter id cannot be empty");

    // Empty engine_type
    let err = reg.register(Arc::new(MockAdapter {
      id: "valid".to_string(),
      detected: true,
      healthy: true,
      engine_type: "   ".to_string(),
      version: None,
    })).unwrap_err();
    assert_eq!(err, "engine adapter must have a non-empty engine_type in metadata");
  }

  #[test]
  fn register_and_get_roundtrip() {
    let reg = EngineAdapterRegistry::new();
    reg
      .register(Arc::new(MockAdapter {
        id: "mock-a".to_string(),
        detected: true,
        healthy: true,
        engine_type: "test".to_string(),
        version: None,
      }))
      .unwrap();

    let a = reg.get("mock-a").unwrap().unwrap();
    assert_eq!(a.id(), "mock-a");
  }

  #[test]
  fn query_ids_filters_and_is_deterministic() {
    let reg = EngineAdapterRegistry::new();
    reg
      .register(Arc::new(MockAdapter {
        id: "b".to_string(),
        detected: true,
        healthy: true,
        engine_type: "test".to_string(),
        version: None,
      }))
      .unwrap();
    reg
      .register(Arc::new(MockAdapter {
        id: "a".to_string(),
        detected: false,
        healthy: true,
        engine_type: "test".to_string(),
        version: None,
      }))
      .unwrap();
    reg
      .register(Arc::new(MockAdapter {
        id: "c".to_string(),
        detected: true,
        healthy: false,
        engine_type: "test".to_string(),
        version: None,
      }))
      .unwrap();

    assert_eq!(reg.query_ids(EngineAdapterQuery::All).unwrap(), vec!["a", "b", "c"]);
    assert_eq!(reg.query_ids(EngineAdapterQuery::Detected).unwrap(), vec!["b", "c"]);
    assert_eq!(reg.query_ids(EngineAdapterQuery::Healthy).unwrap(), vec!["b"]);
  }

  #[test]
  fn query_metadata() {
    let reg = EngineAdapterRegistry::new();
    reg
      .register(Arc::new(MockAdapter {
        id: "a".to_string(),
        detected: true,
        healthy: true,
        engine_type: "docker".to_string(),
        version: Some("20.10".to_string()),
      }))
      .unwrap();
    reg
      .register(Arc::new(MockAdapter {
        id: "b".to_string(),
        detected: true,
        healthy: true,
        engine_type: "pty".to_string(),
        version: Some("1.0".to_string()),
      }))
      .unwrap();
    reg
      .register(Arc::new(MockAdapter {
        id: "c".to_string(),
        detected: true,
        healthy: true,
        engine_type: "docker".to_string(),
        version: Some("24.0".to_string()),
      }))
      .unwrap();

    let docker_adapters = reg.find_by_engine_type("docker").unwrap();
    assert_eq!(docker_adapters.len(), 2);

    let docker_v24 = reg.find_by_version("docker", "24.0").unwrap();
    assert_eq!(docker_v24.len(), 1);
    assert_eq!(docker_v24[0].id(), "c");

    let custom_query = reg.query_metadata(|meta| meta.engine_type == "pty").unwrap();
    assert_eq!(custom_query.len(), 1);
    assert_eq!(custom_query[0].id(), "b");
  }

  #[test]
  fn select_default_prefers_healthy_then_detected() {
    let reg = EngineAdapterRegistry::new();
    reg
      .register(Arc::new(MockAdapter {
        id: "a".to_string(),
        detected: true,
        healthy: false,
        engine_type: "test".to_string(),
        version: None,
      }))
      .unwrap();
    reg
      .register(Arc::new(MockAdapter {
        id: "b".to_string(),
        detected: true,
        healthy: true,
        engine_type: "test".to_string(),
        version: None,
      }))
      .unwrap();

    let selected = reg.select_default().unwrap().unwrap();
    assert_eq!(selected.id(), "b");
  }

  #[test]
  fn builtins_register_portable_pty() {
    let reg = EngineAdapterRegistry::with_builtins();
    let ids = reg.list_ids().unwrap();
    assert!(ids.contains(&"portable-pty-native".to_string()));
    assert!(ids.contains(&"claude-code".to_string()));
  }

  #[test]
  fn unregister_removes_adapter() {
    let reg = EngineAdapterRegistry::new();
    reg
      .register(Arc::new(MockAdapter {
        id: "mock-a".to_string(),
        detected: true,
        healthy: true,
        engine_type: "test".to_string(),
        version: None,
      }))
      .unwrap();

    assert!(reg.get("mock-a").unwrap().is_some());
    let removed = reg.unregister("mock-a").unwrap();
    assert!(removed.is_some());
    assert!(reg.get("mock-a").unwrap().is_none());
  }
}
