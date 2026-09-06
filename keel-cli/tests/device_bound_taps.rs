//! D0201 OPTION B / dcAttestationBindsToDevice: a console tap is bound to the DEVICE that made it. On a
//! live `keel serve` over a scaffold: a tap with no device is refused with nothing written; a paired
//! device's correct HMAC records; a wrong HMAC is refused with nothing written; a wrong pairing code
//! enrols nothing. Plain HTTP over a TcpStream so the test has no client dependency.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

fn keel_bin() -> PathBuf {
    let mut p = std::env::current_exe().expect("test binary path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(if cfg!(windows) { "keel.exe" } else { "keel" })
}

fn run(dir: &Path, args: &[&str]) -> bool {
    Command::new(keel_bin()).args(args).current_dir(dir).env("KEEL_ACTOR", "ai").output().is_ok_and(|o| o.status.success())
}

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git").arg("-C").arg(root).args(args).output().expect("git runs");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

/// One HTTP/1.1 request; returns (status, body).
fn http(port: u16, method: &str, path: &str, body: &str) -> (u16, String) {
    let mut s = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    s.set_read_timeout(Some(Duration::from_secs(20))).expect("timeout");
    let req = format!("{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
    s.write_all(req.as_bytes()).expect("write");
    let mut bytes = Vec::new();
    if let Err(e) = s.read_to_end(&mut bytes) {
        if bytes.is_empty() {
            panic!("no response from serve for {method} {path}: {e}");
        }
    }
    let raw = String::from_utf8_lossy(&bytes).to_string();
    let status: u16 = raw.split_whitespace().nth(1).and_then(|c| c.parse().ok()).unwrap_or(0);
    let body = raw.split_once("\r\n\r\n").map_or_else(|| format!("RAW<{}>", raw.chars().take(300).collect::<String>()), |(_, b)| b.to_string());
    (status, body)
}

/// HMAC-SHA256 through the library, exactly as the browser computes it.
fn sign(key: &[u8], canonical: &str) -> String {
    keel_cli::device::hex(&keel_cli::device::hmac_sha256(key, canonical.as_bytes()))
}

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

/// Start `keel serve` on `port`, return the child and the pairing code it printed to stderr.
fn serve(root: &Path, port: u16) -> (Server, String) {
    let mut child = Command::new(keel_bin())
        .args(["serve", ".", "--port", &port.to_string()])
        .current_dir(root)
        .env("KEEL_ACTOR", "ai")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn serve");
    let stderr = child.stderr.take().expect("stderr");
    let mut reader = BufReader::new(stderr);
    let mut code = String::new();
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(30) {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        if let Some(rest) = line.split("PAIRING CODE ").nth(1) {
            code = rest.split_whitespace().next().unwrap_or("").to_string();
            break;
        }
    }
    assert_eq!(code.len(), 6, "serve prints a six-digit pairing code to its terminal");
    // keep draining stderr: dropping the pipe would make the server's next eprintln fail and kill it
    std::thread::spawn(move || {
        let mut sink = String::new();
        while reader.read_line(&mut sink).unwrap_or(0) > 0 {
            sink.clear();
        }
    });
    // wait for the port
    let start = Instant::now();
    while TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(start.elapsed() < Duration::from_secs(30), "serve did not bind");
        std::thread::sleep(Duration::from_millis(200));
    }
    (Server(child), code)
}

#[test]
fn a_tap_records_only_when_signed_by_a_paired_device() {
    let base = if cfg!(windows) { PathBuf::from("C:\\kt") } else { std::env::temp_dir() };
    let root = base.join(format!("dev{}", std::process::id() % 10_000));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("mkdir");
    assert!(run(&root, &["init", "."]), "scaffold");
    // a Person to judge, and a proposed Decision to accept
    std::fs::write(root.join(".tracking/actors.sysml"), "package ProjectActors {\n    private import EngineElement::*;\n    part hum : Person { :>> name = \"Hum\"; :>> email = \"h@x\"; }\n    part ai : Actor { :>> name = \"AI\"; :>> kind = ActorKind::ai; }\n}\n").expect("actors");
    std::fs::write(root.join(".engine/decisions/0001-probe.sysml"), "package Decision0001 {\n    private import EngineElement::*;\n    part d0001 : Decision {\n        :>> id = \"00000000-0000-4000-8000-000000000001\";\n        :>> title = \"probe\";\n        :>> createdAt = \"2026-09-05\";\n        :>> createdBy = \"ai\";\n        :>> status = DecisionStatus::proposed;\n        :>> context = \"c\";\n        :>> decision = \"d\";\n        :>> rationale = \"r\";\n        :>> consequences = \"q\";\n    }\n}\n").expect("decision");
    git(&root, &["init", "-q", "."]);
    git(&root, &["add", "-A"]);
    git(&root, &["-c", "user.email=p@x", "-c", "user.name=p", "-c", "commit.gpgsign=false", "commit", "-q", "-m", "seed"]);

    let port = 7900 + (std::process::id() % 80) as u16;
    let (_server, code) = serve(&root, port);
    let dec = || std::fs::read_to_string(root.join(".engine/decisions/0001-probe.sysml")).expect("read");
    let accept = |extra: &str| format!("{{\"decision\":\"d0001\",\"file\":\".engine/decisions/0001-probe.sysml\",\"note\":\"yes, exactly this\",\"judged_at\":\"2026-09-05\",\"judged_by\":\"hum\"{extra}}}");

    // 1. no device: refused, nothing written
    let (st, body) = http(port, "POST", "/api/decision/accept", &accept(""));
    assert_eq!(st, 401, "{body}");
    assert!(body.contains("names no device"), "{body}");
    assert!(!dec().contains("AcceptR1"), "nothing written");

    // 2. a wrong pairing code enrols nothing
    let key = b"0123456789abcdef0123456789abcdef";
    let key_hex = keel_cli::device::hex(key);
    let (st, body) = http(port, "POST", "/api/device/enroll", &format!("{{\"code\":\"000000\",\"device_id\":\"browser-test01\",\"key\":\"{key_hex}\",\"label\":\"test\"}}"));
    assert!(st == 401 || (code == "000000" && st == 200), "wrong code refused: {st} {body}");

    // 3. pair with the printed code
    let (st, body) = http(port, "POST", "/api/device/enroll", &format!("{{\"code\":\"{code}\",\"device_id\":\"browser-test01\",\"key\":\"{key_hex}\",\"label\":\"test\"}}"));
    assert_eq!(st, 200, "{body}");
    assert!(root.join(".keel/devices.toml").is_file());

    // 4. wrong signature: refused, nothing written
    let bad = sign(b"another-key", &keel_cli::device::canonical("accept", "d0001", "2026-09-05", "hum", "yes, exactly this"));
    let (st, body) = http(port, "POST", "/api/decision/accept", &accept(&format!(",\"device_id\":\"browser-test01\",\"hmac\":\"{bad}\"")));
    assert_eq!(st, 401, "{body}");
    assert!(body.contains("does not match"), "{body}");
    assert!(!dec().contains("AcceptR1"), "nothing written on a wrong signature");

    // 5. the paired device's signature: recorded, and the record names the device
    let good = sign(key, &keel_cli::device::canonical("accept", "d0001", "2026-09-05", "hum", "yes, exactly this"));
    let (st, body) = http(port, "POST", "/api/decision/accept", &accept(&format!(",\"device_id\":\"browser-test01\",\"hmac\":\"{good}\"")));
    assert_eq!(st, 200, "{body}");
    let text = dec();
    assert!(text.contains("AcceptR1") && text.contains("[device browser-test HMAC-verified]"), "recorded and device-tagged:\n{text}");
    let _ = std::fs::remove_dir_all(&root);
}
