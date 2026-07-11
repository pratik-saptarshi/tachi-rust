use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tachi_desktop::app::DesktopApp;
use tachi_desktop::dispatch_desktop_command_with_progress;
use tachi_shell::progress::{CancellationToken, ProgressEvent, ProgressReporter};

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
struct RecordingReporter(Arc<Mutex<Vec<ProgressEvent>>>);

impl ProgressReporter for RecordingReporter {
    fn emit(&mut self, event: ProgressEvent) {
        self.0.lock().expect("reporter mutex").push(event);
    }
}

fn fixture_root(prefix: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "{prefix}-{}-{}",
        std::process::id(),
        TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(root.join("scripts")).expect("create fixture root");
    root
}

fn write_executable(path: &Path, content: &str) {
    fs::write(path, content).expect("write executable fixture");
    let mut permissions = fs::metadata(path)
        .expect("read fixture metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("make fixture executable");
}

#[test]
fn desktop_host_composes_preview_save_and_policy_failure() {
    let root = fixture_root("tachi-desktop-command-e2e");
    let target_dir = root.join("examples/agentic-app/sample-report");
    let template_dir = root.join("templates/tachi/security-report");
    fs::create_dir_all(&target_dir).expect("create report target");
    fs::create_dir_all(&template_dir).expect("create report templates");
    fs::write(
        target_dir.join("threats.md"),
        "# Agentic AI Application\n\n## 7. Recommended Actions\n",
    )
    .expect("write analysis input");

    let mut preview_app = DesktopApp::new(root.clone());
    let preview = preview_app.state_mut().run_command(
        "report-data",
        &[
            "--target-dir",
            target_dir.to_string_lossy().as_ref(),
            "--template-dir",
            template_dir.to_string_lossy().as_ref(),
        ],
    );
    assert_eq!(preview.status, 0, "preview failed: {}", preview.stderr);
    assert!(preview.stdout.contains("#let project-name ="));
    assert_eq!(preview_app.state().last_command().unwrap().status, 0);

    let output_path = root.join("generated/report-data.typ");
    let mut save_app = DesktopApp::new(root.clone());
    let saved = save_app.state_mut().run_command(
        "report-data",
        &[
            "--target-dir",
            target_dir.to_string_lossy().as_ref(),
            "--template-dir",
            template_dir.to_string_lossy().as_ref(),
            "--output",
            output_path.to_string_lossy().as_ref(),
        ],
    );
    assert_eq!(saved.status, 0, "save failed: {}", saved.stderr);
    assert_eq!(
        fs::read(&output_path).expect("read saved artifact"),
        preview.stdout.as_bytes()
    );
    assert!(save_app.render_text().contains("Command: report-data"));

    let escaped_path = root.join("../desktop-command-escape.typ");
    let escaped = save_app.state_mut().run_command(
        "report-data",
        &[
            "--target-dir",
            target_dir.to_string_lossy().as_ref(),
            "--template-dir",
            template_dir.to_string_lossy().as_ref(),
            "--output",
            escaped_path.to_string_lossy().as_ref(),
        ],
    );
    assert_ne!(escaped.status, 0);
    assert!(!escaped_path.exists());
    assert!(save_app.render_text().contains("Latest command:"));
}

#[test]
fn desktop_host_cancellation_returns_typed_status_and_event() {
    let root = fixture_root("tachi-desktop-cancellation-e2e");
    write_executable(
        &root.join("scripts/install.sh"),
        "#!/usr/bin/env bash\ntrap 'exit 130' TERM\nprintf 'begin\\n'\nsleep 5\nprintf 'done\\n'\n",
    );

    let token = CancellationToken::new();
    let worker_token = token.clone();
    let events = Arc::new(Mutex::new(Vec::new()));
    let worker_events = events.clone();
    let worker_root = root.clone();
    let handle = std::thread::spawn(move || {
        let mut reporter = RecordingReporter(worker_events);
        dispatch_desktop_command_with_progress(
            "install",
            &worker_root,
            &["--root", worker_root.to_string_lossy().as_ref()],
            &worker_token,
            &mut reporter,
        )
    });

    std::thread::sleep(Duration::from_millis(100));
    token.cancel();
    let output = handle.join().expect("join cancelled desktop command");

    assert_eq!(output.status, 130);
    assert!(events
        .lock()
        .expect("read progress events")
        .iter()
        .any(|event| { event.command == "install" && event.message == "cancelled" }));
}
