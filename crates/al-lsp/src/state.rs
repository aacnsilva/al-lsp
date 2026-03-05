use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use dashmap::DashMap;
use lsp_types::Url;

use al_syntax::ast::AlSymbolKind;
use al_syntax::document::DocumentState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedObjectEntry {
    pub uri: Url,
    pub object_start_byte: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UriIndexedObjectEntry {
    kind_lower: String,
    name_lower: String,
    object_start_byte: usize,
}

/// Global server state holding all open documents.
pub struct WorldState {
    pub documents: DashMap<Url, DocumentState>,
    /// Workspace-wide object index:
    /// `(object_kind_lower, object_name_lower) -> object locations`.
    pub object_index: DashMap<(String, String), Vec<IndexedObjectEntry>>,
    /// Reverse index of object names by kind for name completion.
    pub object_names_by_kind: DashMap<String, Vec<String>>,
    /// Reverse index of indexed objects per URI so reindexing can update only touched keys.
    uri_object_index: DashMap<Url, Vec<UriIndexedObjectEntry>>,
    /// Serializes background workspace indexing so rescans do not overlap.
    pub workspace_index_running: AtomicBool,
    /// Marks that another background index pass should run after the current one finishes.
    pub workspace_index_pending: AtomicBool,
    /// Workspace root directories (resolved from InitializeParams).
    pub workspace_roots: std::sync::Mutex<Vec<PathBuf>>,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
            documents: DashMap::new(),
            object_index: DashMap::new(),
            object_names_by_kind: DashMap::new(),
            uri_object_index: DashMap::new(),
            workspace_index_running: AtomicBool::new(false),
            workspace_index_pending: AtomicBool::new(false),
            workspace_roots: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Insert or replace a document and refresh object index entries for its URI.
    pub fn upsert_document(&self, uri: Url, doc: DocumentState) {
        self.documents.insert(uri.clone(), doc);
        self.reindex_document(&uri);
    }

    /// Remove a document and all object index entries pointing to it.
    pub fn remove_document(&self, uri: &Url) {
        self.documents.remove(uri);
        self.remove_uri_from_object_index(uri);
    }

    /// Rebuild object index entries for a loaded document.
    pub fn reindex_document(&self, uri: &Url) {
        self.remove_uri_from_object_index(uri);
        let Some(doc) = self.documents.get(uri) else {
            return;
        };

        let mut uri_entries = Vec::new();
        for object in &doc.symbol_table.symbols {
            let AlSymbolKind::Object(kind) = object.kind else {
                continue;
            };
            let kind_lower = kind.label().to_ascii_lowercase();
            let name_lower = object.name.to_ascii_lowercase();
            let key = (kind_lower.clone(), name_lower.clone());
            self.object_index
                .entry(key)
                .or_default()
                .push(IndexedObjectEntry {
                    uri: uri.clone(),
                    object_start_byte: object.start_byte,
                });
            {
                let mut names = self.object_names_by_kind.entry(kind_lower.clone()).or_default();
                if !names.iter().any(|name| name == &name_lower) {
                    names.push(name_lower.clone());
                }
            }
            uri_entries.push(UriIndexedObjectEntry {
                kind_lower,
                name_lower,
                object_start_byte: object.start_byte,
            });
        }
        self.uri_object_index.insert(uri.clone(), uri_entries);
    }

    fn remove_uri_from_object_index(&self, uri: &Url) {
        let Some((_, indexed_objects)) = self.uri_object_index.remove(uri) else {
            return;
        };

        for indexed_object in indexed_objects {
            let key = (
                indexed_object.kind_lower.clone(),
                indexed_object.name_lower.clone(),
            );
            let mut remove_key = false;
            if let Some(mut entries) = self.object_index.get_mut(&key) {
                entries.retain(|entry| {
                    entry.uri != *uri || entry.object_start_byte != indexed_object.object_start_byte
                });
                remove_key = entries.is_empty();
            }

            if remove_key {
                self.object_index.remove(&key);
                let mut remove_kind = false;
                if let Some(mut names) = self.object_names_by_kind.get_mut(&indexed_object.kind_lower)
                {
                    names.retain(|name| name != &indexed_object.name_lower);
                    remove_kind = names.is_empty();
                }
                if remove_kind {
                    self.object_names_by_kind.remove(&indexed_object.kind_lower);
                }
            }
        }
    }

    pub fn visit_object_names_for_kind<F>(&self, object_kind: &str, mut visit: F) -> bool
    where
        F: FnMut(&str) -> bool,
    {
        let object_kind_lower = object_kind.to_ascii_lowercase();
        let Some(names) = self.object_names_by_kind.get(&object_kind_lower) else {
            return false;
        };
        for name in names.iter() {
            if visit(name) {
                return true;
            }
        }
        false
    }

    /// Test helper that loads both workspace source files and `.alpackages`.
    #[cfg(test)]
    pub fn load_workspace_files(&self) -> usize {
        self.load_workspace_source_files() + self.load_workspace_alpackages()
    }

    /// Scan workspace roots for source `.al` files only.
    pub fn load_workspace_source_files(&self) -> usize {
        let roots = self.workspace_roots.lock().unwrap().clone();
        if roots.is_empty() {
            tracing::warn!("no workspace roots configured — cannot scan for .al files");
            return 0;
        }
        let mut count = 0;
        for root in &roots {
            tracing::info!("scanning workspace root: {}", root.display());
            count += self.scan_directory(root);
        }
        count
    }

    /// Load `.al` files from `.app` archives (ZIP) in `.alpackages/` directories.
    pub fn load_workspace_alpackages(&self) -> usize {
        let roots = self.workspace_roots.lock().unwrap().clone();
        if roots.is_empty() {
            return 0;
        }
        self.load_alpackages(&roots)
    }

    /// Load `.al` files from `.app` archives (ZIP) in `.alpackages/` directories.
    fn load_alpackages(&self, roots: &[PathBuf]) -> usize {
        let mut count = 0;
        for root in roots {
            let packages_dir = root.join(".alpackages");
            if !packages_dir.is_dir() {
                continue;
            }
            tracing::info!("scanning .alpackages in {}", root.display());
            let entries = match std::fs::read_dir(&packages_dir) {
                Ok(e) => e,
                Err(e) => {
                    tracing::warn!("failed to read .alpackages: {}", e);
                    continue;
                }
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case("app"))
                    != Some(true)
                {
                    continue;
                }
                count += self.load_app_file(&path);
            }
        }
        if count > 0 {
            tracing::info!("loaded {} files from .alpackages", count);
        }
        count
    }

    /// Load `.al` files from a single `.app` archive (ZIP format).
    fn load_app_file(&self, path: &Path) -> usize {
        let file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => {
                tracing::warn!("failed to open {}: {}", path.display(), e);
                return 0;
            }
        };
        let mut archive = match zip::ZipArchive::new(file) {
            Ok(a) => a,
            Err(e) => {
                tracing::warn!("failed to read ZIP {}: {}", path.display(), e);
                return 0;
            }
        };

        let app_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let mut count = 0;
        for i in 0..archive.len() {
            let mut entry = match archive.by_index(i) {
                Ok(e) => e,
                Err(_) => continue,
            };
            let entry_name = match entry.enclosed_name() {
                Some(n) => n.to_owned(),
                None => continue,
            };
            if !entry_name
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("al"))
                .unwrap_or(false)
            {
                continue;
            }

            let mut source = String::new();
            if entry.read_to_string(&mut source).is_err() {
                continue;
            }

            let uri_str = format!("alpackage://{}/{}", app_name, entry_name.display());
            let uri = match Url::parse(&uri_str) {
                Ok(u) => u,
                Err(_) => continue,
            };

            if self.documents.contains_key(&uri) {
                continue;
            }

            if let Some(doc) = DocumentState::new(&source) {
                self.upsert_document(uri, doc);
                count += 1;
            }
        }
        count
    }

    fn scan_directory(&self, dir: &Path) -> usize {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!("failed to read directory {}: {}", dir.display(), e);
                return 0;
            }
        };

        let mut count = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip hidden dirs and common non-source dirs
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.') || name == "node_modules" || name == "target" {
                        continue;
                    }
                }
                count += self.scan_directory(&path);
            } else if path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("al"))
                == Some(true)
                && self.load_file(&path)
            {
                count += 1;
            }
        }
        count
    }

    /// Load a single file by path. Returns true if it was loaded (or already loaded).
    pub fn load_file(&self, path: &Path) -> bool {
        let uri = match Url::from_file_path(path) {
            Ok(u) => u,
            Err(_) => {
                tracing::warn!("failed to convert path to URI: {}", path.display());
                return false;
            }
        };

        self.load_file_uri(&uri, path)
    }

    /// Load a single file by URI. Returns true if loaded.
    pub fn load_file_uri(&self, uri: &Url, path: &Path) -> bool {
        // Skip if already loaded
        if self.documents.contains_key(uri) {
            return true;
        }

        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("failed to read {}: {}", path.display(), e);
                return false;
            }
        };

        match DocumentState::new(&source) {
            Some(doc) => {
                self.upsert_document(uri.clone(), doc);
                true
            }
            None => {
                tracing::warn!("failed to parse {}", path.display());
                false
            }
        }
    }

    /// Reload a file from disk (e.g. after an external change notification).
    pub fn reload_file_from_disk(&self, uri: &Url) {
        let path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return,
        };

        if !path.exists() {
            // File was deleted
            self.remove_document(uri);
            return;
        }

        let source = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => return,
        };

        if let Some(doc) = DocumentState::new(&source) {
            self.upsert_document(uri.clone(), doc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_workspace_files() {
        let dir = tempfile::tempdir().unwrap();

        // Create a .al file in root
        let f1 = dir.path().join("hello.al");
        fs::write(
            &f1,
            r#"codeunit 50100 Hello
{
    procedure Greet()
    begin
    end;
}"#,
        )
        .unwrap();

        // Create a subdirectory with another .al file
        let sub = dir.path().join("src");
        fs::create_dir(&sub).unwrap();
        let f2 = sub.join("world.al");
        fs::write(
            &f2,
            r#"codeunit 50101 World
{
    procedure DoWork()
    begin
    end;
}"#,
        )
        .unwrap();

        // Create a non-.al file (should be ignored)
        fs::write(dir.path().join("readme.md"), "hello").unwrap();

        // Create a hidden directory (should be skipped)
        let hidden = dir.path().join(".hidden");
        fs::create_dir(&hidden).unwrap();
        fs::write(hidden.join("ignored.al"), "codeunit 50102 X {}").unwrap();

        let state = WorldState::new();
        *state.workspace_roots.lock().unwrap() = vec![dir.path().to_path_buf()];

        let count = state.load_workspace_files();
        assert_eq!(count, 2, "should load exactly 2 .al files");
        assert_eq!(state.documents.len(), 2);
    }

    #[test]
    fn test_load_workspace_no_roots() {
        let state = WorldState::new();
        let count = state.load_workspace_files();
        assert_eq!(count, 0);
        assert_eq!(state.documents.len(), 0);
    }
}
