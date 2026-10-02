//! Bounded browser loose-file preparation for the immutable base encyclopedia.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::future::{poll_fn, Future};
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;

use rebellion_data::encyclopedia::{
    parse_catalog, parse_manifest, EncyclopediaCatalog, EncyclopediaManifest,
    CATALOG_JSON_BYTES_LIMIT, MANIFEST_JSON_BYTES_LIMIT,
};
use rebellion_render::encyclopedia_assets::{
    MAX_ENCYCLOPEDIA_IMAGE_BYTES, MAX_ENCYCLOPEDIA_NON_IMAGE_BYTES,
};
use rebellion_render::inspect_encyclopedia_bytes;

use crate::encyclopedia_fetch::EncyclopediaFetchError;
use crate::encyclopedia_session::{
    prepare_encyclopedia_session, EncyclopediaBytes, EncyclopediaSession,
};

const LOOSE_ROOT: &str = "data/encyclopedia";
const CATALOG_PATH: &str = "catalog.json";
const MANIFEST_PATH: &str = "manifest.json";
const MAX_IMAGE_BATCH: usize = 4;
const MAX_EFFECTIVE_IMAGE_BYTES: u64 = 128 * 1024 * 1024;

#[derive(Debug)]
pub(crate) struct LooseEncyclopediaError {
    code: &'static str,
    path: String,
    detail: String,
}

impl LooseEncyclopediaError {
    fn new(code: &'static str, path: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            code,
            path: path.into(),
            detail: detail.into(),
        }
    }

    #[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
    const fn code(&self) -> &'static str {
        self.code
    }

    #[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
    fn path(&self) -> &str {
        &self.path
    }
}

impl fmt::Display for LooseEncyclopediaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at {}: {}", self.code, self.path, self.detail)
    }
}

impl std::error::Error for LooseEncyclopediaError {}

#[derive(Debug)]
struct ImageRequest {
    bundle_path: String,
    fetch_path: String,
    max_bytes: usize,
}

/// Fetches and validates the loose encyclopedia without mutating any global cache.
///
/// `Ok(None)` is reserved for the one absence state: both metadata requests returned
/// HTTP 404. Every partial namespace and every uncertain transport failure is an error.
pub(crate) async fn prepare_loose_encyclopedia<F, Fut>(
    fetch: &F,
    selected_dats: &HashMap<String, Vec<u8>>,
) -> Result<Option<EncyclopediaSession>, LooseEncyclopediaError>
where
    F: Fn(String, usize) -> Fut,
    Fut: Future<Output = Result<Vec<u8>, EncyclopediaFetchError>>,
{
    let metadata = join_all_local(vec![
        fetch(loose_path(MANIFEST_PATH), MANIFEST_JSON_BYTES_LIMIT),
        fetch(loose_path(CATALOG_PATH), CATALOG_JSON_BYTES_LIMIT),
    ])
    .await;
    let [manifest_result, catalog_result]: [_; 2] = metadata
        .try_into()
        .expect("the fixed metadata request set has two members");

    let (manifest_result, catalog_result) = match (manifest_result, catalog_result) {
        (Err(EncyclopediaFetchError::NotFound), Err(EncyclopediaFetchError::NotFound)) => {
            return Ok(None)
        }
        (Err(EncyclopediaFetchError::NotFound), Err(error)) => {
            return required_fetch(CATALOG_PATH, Err(error)).map(|_| None);
        }
        (Err(error), Err(EncyclopediaFetchError::NotFound)) => {
            return required_fetch(MANIFEST_PATH, Err(error)).map(|_| None);
        }
        results => results,
    };

    let manifest_bytes = required_fetch(MANIFEST_PATH, manifest_result)?;
    let catalog_bytes = required_fetch(CATALOG_PATH, catalog_result)?;
    let manifest =
        parse_manifest(&manifest_bytes).map_err(|error| validation_error(MANIFEST_PATH, &error))?;
    let catalog =
        parse_catalog(&catalog_bytes).map_err(|error| validation_error(CATALOG_PATH, &error))?;

    let image_requests = declared_image_requests(&catalog, &manifest)?;
    let dat_hashes = selected_dat_hashes(&manifest, selected_dats)?;

    let mut retained = EncyclopediaBytes::from([
        (
            CATALOG_PATH.to_owned(),
            Arc::from(catalog_bytes.into_boxed_slice()),
        ),
        (
            MANIFEST_PATH.to_owned(),
            Arc::from(manifest_bytes.into_boxed_slice()),
        ),
    ]);

    for batch in image_requests.chunks(MAX_IMAGE_BATCH) {
        let fetched = join_all_local(
            batch
                .iter()
                .map(|request| fetch(request.fetch_path.clone(), request.max_bytes))
                .collect(),
        )
        .await;
        for (request, result) in batch.iter().zip(fetched) {
            let bytes = required_fetch(&request.bundle_path, result)?;
            retained.insert(
                request.bundle_path.clone(),
                Arc::from(bytes.into_boxed_slice()),
            );
        }
    }

    prepare_encyclopedia_session(retained, &dat_hashes)
        .map(Some)
        .map_err(|error| validation_error(error.path(), &error))
}

#[cfg(target_arch = "wasm32")]
pub(crate) async fn prepare_browser_loose_encyclopedia(
    selected_dats: &HashMap<String, Vec<u8>>,
) -> Result<Option<EncyclopediaSession>, LooseEncyclopediaError> {
    prepare_loose_encyclopedia(
        &|path, max_bytes| async move {
            crate::encyclopedia_fetch::fetch_encyclopedia_file(&path, max_bytes).await
        },
        selected_dats,
    )
    .await
}

#[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
const BROWSER_PROBE_MAGIC: u32 = 0xe117_0000;

#[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BrowserProbeCase {
    Success,
    BothMissing,
    PartialMetadata,
    MalformedCatalog,
    FailedImage,
    OversizedImage,
    DatMismatch,
}

#[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
impl BrowserProbeCase {
    const fn label(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::BothMissing => "both_missing",
            Self::PartialMetadata => "partial_metadata",
            Self::MalformedCatalog => "malformed_catalog",
            Self::FailedImage => "failed_image",
            Self::OversizedImage => "oversized_image",
            Self::DatMismatch => "dat_mismatch",
        }
    }
}

#[cfg(any(test, all(target_arch = "wasm32", feature = "interface-test-fixtures")))]
fn decode_browser_probe_code(code: u32) -> Option<BrowserProbeCase> {
    if code & 0xffff_0000 != BROWSER_PROBE_MAGIC {
        return None;
    }
    Some(match code & 0xffff {
        1 => BrowserProbeCase::Success,
        2 => BrowserProbeCase::BothMissing,
        3 => BrowserProbeCase::PartialMetadata,
        4 => BrowserProbeCase::MalformedCatalog,
        5 => BrowserProbeCase::FailedImage,
        6 => BrowserProbeCase::OversizedImage,
        7 => BrowserProbeCase::DatMismatch,
        _ => return None,
    })
}

#[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
extern "C" {
    fn open_rebellion_interface_fixture_code() -> u32;
    fn open_rebellion_interface_fixture_emit(ptr: *const u8, len: usize);
}

#[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
pub(crate) fn browser_probe_requested() -> Option<BrowserProbeCase> {
    decode_browser_probe_code(unsafe { open_rebellion_interface_fixture_code() })
}

/// Runs the actual status-aware E15 transport and shared E45 preparation, then emits
/// evidence without installing any global cache. This exists only in the isolated
/// `interface-test-fixtures` WASM artifact.
#[cfg(all(target_arch = "wasm32", feature = "interface-test-fixtures"))]
pub(crate) async fn run_browser_probe(case: BrowserProbeCase) {
    const VALID_DAT: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/sources/SYNTHETIC.DAT"
    );

    let selected_dats = HashMap::from([(
        "SYNTHETIC.DAT".to_owned(),
        if case == BrowserProbeCase::DatMismatch {
            b"synthetic mismatched DAT bytes".to_vec()
        } else {
            VALID_DAT.to_vec()
        },
    )]);
    let result = match prepare_browser_loose_encyclopedia(&selected_dats).await {
        Ok(Some(session)) => {
            let retained = session
                .base_bytes()
                .iter()
                .map(|(path, bytes)| {
                    let facts = session
                        .observed_facts()
                        .get(path)
                        .expect("every retained probe member has validated facts");
                    serde_json::json!({
                        "path": path,
                        "byte_len": bytes.len(),
                        "sha256": facts.sha256,
                    })
                })
                .collect::<Vec<_>>();
            serde_json::json!({
                "schema": "open-rebellion-e17-browser-probe-v1",
                "case": case.label(),
                "status": "ready",
                "loader": "prepare_browser_loose_encyclopedia",
                "published": false,
                "generation": session.generation(),
                "source_profile": session.base_manifest().source_profile,
                "catalog_sha256": session.base_manifest().catalog_sha256,
                "topic_ids": session.base_catalog().topics.keys().map(|id| &id.0).collect::<Vec<_>>(),
                "image_ids": session.base_catalog().images.keys().map(|id| &id.0).collect::<Vec<_>>(),
                "retained": retained,
            })
        }
        Ok(None) => serde_json::json!({
            "schema": "open-rebellion-e17-browser-probe-v1",
            "case": case.label(),
            "status": "absent",
            "loader": "prepare_browser_loose_encyclopedia",
            "published": false,
        }),
        Err(error) => serde_json::json!({
            "schema": "open-rebellion-e17-browser-probe-v1",
            "case": case.label(),
            "status": "invalid",
            "loader": "prepare_browser_loose_encyclopedia",
            "published": false,
            "error_code": error.code(),
            "error_path": error.path(),
            "detail": error.to_string(),
        }),
    };
    let encoded = serde_json::to_vec(&result).expect("browser probe evidence serializes");
    unsafe { open_rebellion_interface_fixture_emit(encoded.as_ptr(), encoded.len()) };
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn original_image_cache(
    session: &EncyclopediaSession,
) -> Result<HashMap<String, Vec<u8>>, LooseEncyclopediaError> {
    let mut cache = HashMap::with_capacity(session.base_catalog().images.len());
    for descriptor in session.base_catalog().images.values() {
        let cache_key = descriptor.path.strip_prefix("assets/").ok_or_else(|| {
            LooseEncyclopediaError::new(
                "unsafe_asset_path",
                &descriptor.path,
                "validated browser base image is outside assets/",
            )
        })?;
        let retained = session.base_bytes().get(&descriptor.path).ok_or_else(|| {
            LooseEncyclopediaError::new(
                "missing_runtime_file",
                &descriptor.path,
                "validated session omitted image bytes",
            )
        })?;
        cache.insert(cache_key.to_owned(), retained.to_vec());
    }
    Ok(cache)
}

fn loose_path(bundle_path: &str) -> String {
    format!("{LOOSE_ROOT}/{bundle_path}")
}

fn required_fetch(
    path: &str,
    result: Result<Vec<u8>, EncyclopediaFetchError>,
) -> Result<Vec<u8>, LooseEncyclopediaError> {
    result.map_err(|error| {
        LooseEncyclopediaError::new(
            match error {
                EncyclopediaFetchError::NotFound => "missing_runtime_file",
                EncyclopediaFetchError::ResourceLimit { .. } => "resource_limit:fetch_bytes",
                EncyclopediaFetchError::HttpStatus(_)
                | EncyclopediaFetchError::Transport
                | EncyclopediaFetchError::UnsupportedStreaming => "fetch_failed",
            },
            path,
            error.to_string(),
        )
    })
}

fn validation_error(
    path: impl Into<String>,
    error: &rebellion_data::encyclopedia::EncyclopediaError,
) -> LooseEncyclopediaError {
    LooseEncyclopediaError::new(error.code(), path, error.to_string())
}

fn declared_image_requests(
    catalog: &EncyclopediaCatalog,
    manifest: &EncyclopediaManifest,
) -> Result<Vec<ImageRequest>, LooseEncyclopediaError> {
    let mut expected_files = BTreeSet::from([CATALOG_PATH.to_owned()]);
    let mut requests = BTreeMap::new();
    let mut aggregate = 0_u64;

    for descriptor in catalog.images.values() {
        if !expected_files.insert(descriptor.path.clone()) {
            return Err(LooseEncyclopediaError::new(
                "asset_identity_collision",
                &descriptor.path,
                "multiple image identities share one loose-file path",
            ));
        }
        if descriptor.byte_length > MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64 {
            return Err(LooseEncyclopediaError::new(
                "resource_limit:image_bytes",
                &descriptor.path,
                "declared image length exceeds the 32 MiB transport limit",
            ));
        }
        aggregate = aggregate
            .checked_add(descriptor.byte_length)
            .ok_or_else(|| {
                LooseEncyclopediaError::new(
                    "resource_limit:effective_image_bytes",
                    "$.images",
                    "declared image byte total overflowed",
                )
            })?;
        if aggregate > MAX_EFFECTIVE_IMAGE_BYTES {
            return Err(LooseEncyclopediaError::new(
                "resource_limit:effective_image_bytes",
                "$.images",
                "declared image bytes exceed the 128 MiB asset-set limit",
            ));
        }
        let max_bytes = usize::try_from(descriptor.byte_length).map_err(|_| {
            LooseEncyclopediaError::new(
                "resource_limit:image_bytes",
                &descriptor.path,
                "declared image length does not fit usize",
            )
        })?;
        requests.insert(
            descriptor.path.clone(),
            ImageRequest {
                bundle_path: descriptor.path.clone(),
                fetch_path: loose_path(&descriptor.path),
                max_bytes,
            },
        );
    }

    let manifest_files: BTreeSet<_> = manifest.files.keys().cloned().collect();
    if manifest_files != expected_files {
        return Err(LooseEncyclopediaError::new(
            "manifest_file_set_mismatch",
            "$.files",
            "manifest and catalog-required loose-file sets differ",
        ));
    }

    Ok(requests.into_values().collect())
}

fn selected_dat_hashes(
    manifest: &EncyclopediaManifest,
    selected_dats: &HashMap<String, Vec<u8>>,
) -> Result<BTreeMap<String, String>, LooseEncyclopediaError> {
    selected_dat_hashes_with_limit(manifest, selected_dats, MAX_ENCYCLOPEDIA_NON_IMAGE_BYTES)
}

fn selected_dat_hashes_with_limit(
    manifest: &EncyclopediaManifest,
    selected_dats: &HashMap<String, Vec<u8>>,
    max_bytes: usize,
) -> Result<BTreeMap<String, String>, LooseEncyclopediaError> {
    let mut hashes = BTreeMap::new();
    let mut selected_by_folded = BTreeMap::<String, (&str, &[u8])>::new();
    for (basename, bytes) in selected_dats {
        let folded = basename.to_ascii_lowercase();
        if selected_by_folded
            .insert(folded, (basename.as_str(), bytes.as_slice()))
            .is_some()
        {
            return Err(LooseEncyclopediaError::new(
                "binding_source_mismatch",
                basename,
                "selected DAT basenames collide case-insensitively",
            ));
        }
    }

    for source in &manifest.binding_sources {
        let folded = source.basename.to_ascii_lowercase();
        let (actual_basename, bytes) = selected_by_folded.get(&folded).ok_or_else(|| {
            LooseEncyclopediaError::new(
                "binding_source_mismatch",
                &source.basename,
                "required selected DAT bytes are absent",
            )
        })?;
        if bytes.len() > max_bytes {
            return Err(LooseEncyclopediaError::new(
                "resource_limit:binding_source_bytes",
                *actual_basename,
                format!("selected DAT exceeds the {max_bytes}-byte verification limit"),
            ));
        }
        let inspected = inspect_encyclopedia_bytes(bytes, None).map_err(|detail| {
            LooseEncyclopediaError::new(
                "resource_limit:binding_source_bytes",
                *actual_basename,
                detail,
            )
        })?;
        if inspected.sha256 != source.sha256 {
            return Err(LooseEncyclopediaError::new(
                "binding_source_mismatch",
                &source.basename,
                "selected DAT bytes do not match the manifest binding source",
            ));
        }
        hashes.insert((*actual_basename).to_owned(), inspected.sha256);
    }
    Ok(hashes)
}

async fn join_all_local<F>(futures: Vec<F>) -> Vec<F::Output>
where
    F: Future,
{
    let mut futures: Vec<Option<Pin<Box<F>>>> = futures
        .into_iter()
        .map(|future| Some(Box::pin(future)))
        .collect();
    let mut outputs: Vec<Option<F::Output>> = std::iter::repeat_with(|| None)
        .take(futures.len())
        .collect();

    poll_fn(move |context| {
        let mut complete = true;
        for (future, output) in futures.iter_mut().zip(outputs.iter_mut()) {
            if output.is_some() {
                continue;
            }
            let pending = future.as_mut().expect("unfinished future remains present");
            match pending.as_mut().poll(context) {
                Poll::Ready(value) => {
                    *output = Some(value);
                    *future = None;
                }
                Poll::Pending => complete = false,
            }
        }
        if complete {
            Poll::Ready(
                outputs
                    .iter_mut()
                    .map(|output| output.take().expect("completed future has output"))
                    .collect(),
            )
        } else {
            Poll::Pending
        }
    })
    .await
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::future::Future;
    use std::io::{self, BufRead, BufReader, Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::pin::Pin;
    use std::rc::Rc;
    use std::task::{Context, Poll, Waker};

    use serde_json::Value;

    use super::*;

    const VALID_CATALOG: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/catalog.json");
    const VALID_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/manifest.json");
    const VALID_IMAGE_1: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001"
    );
    const VALID_IMAGE_2: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.002"
    );
    const VALID_IMAGE_3: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.003"
    );
    const VALID_DAT: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/sources/SYNTHETIC.DAT"
    );
    const CONTROLLED_HTTP_REQUEST_HEADER_BYTES_LIMIT: usize = 16 * 1024;

    fn read_controlled_http_request_headers(stream: &TcpStream) -> io::Result<String> {
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut request_line = None;
        let mut total_bytes = 0_usize;

        loop {
            let remaining = CONTROLLED_HTTP_REQUEST_HEADER_BYTES_LIMIT
                .checked_sub(total_bytes)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "controlled HTTP request headers exceed the byte limit",
                    )
                })?;
            let mut line = Vec::new();
            let read = reader
                .by_ref()
                .take(
                    u64::try_from(remaining)
                        .unwrap_or(u64::MAX)
                        .saturating_add(1),
                )
                .read_until(b'\n', &mut line)?;
            if read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "controlled HTTP request ended before the header terminator",
                ));
            }
            total_bytes = total_bytes.checked_add(read).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "controlled HTTP request header byte count overflowed",
                )
            })?;
            if total_bytes > CONTROLLED_HTTP_REQUEST_HEADER_BYTES_LIMIT {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "controlled HTTP request headers exceed the byte limit",
                ));
            }
            if request_line.is_none() {
                request_line = Some(String::from_utf8(line.clone()).map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("controlled HTTP request line is not UTF-8: {error}"),
                    )
                })?);
            }
            if line == b"\r\n" || line == b"\n" {
                return request_line.ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "controlled HTTP request has no request line",
                    )
                });
            }
        }
    }

    #[test]
    fn browser_probe_codes_are_reserved_and_cover_every_http_outcome() {
        assert_eq!(BrowserProbeCase::Success.label(), "success");
        assert_eq!(decode_browser_probe_code(0), None);
        assert_eq!(decode_browser_probe_code(1), None);
        assert_eq!(decode_browser_probe_code(BROWSER_PROBE_MAGIC), None);
        assert_eq!(
            (1..=7)
                .map(|case| decode_browser_probe_code(BROWSER_PROBE_MAGIC | case).unwrap())
                .collect::<Vec<_>>(),
            vec![
                BrowserProbeCase::Success,
                BrowserProbeCase::BothMissing,
                BrowserProbeCase::PartialMetadata,
                BrowserProbeCase::MalformedCatalog,
                BrowserProbeCase::FailedImage,
                BrowserProbeCase::OversizedImage,
                BrowserProbeCase::DatMismatch,
            ]
        );
        assert_eq!(decode_browser_probe_code(BROWSER_PROBE_MAGIC | 8), None);
    }

    #[derive(Clone)]
    enum FakeResponse {
        Bytes(Vec<u8>),
        NotFound,
        HttpStatus(u16),
        Transport,
    }

    #[derive(Default)]
    struct FetchState {
        active: usize,
        max_active: usize,
        requests: Vec<(String, usize)>,
        responses: BTreeMap<String, FakeResponse>,
    }

    #[derive(Clone, Default)]
    struct FakeFetcher(Rc<RefCell<FetchState>>);

    impl FakeFetcher {
        fn with_valid_bundle() -> Self {
            let fetcher = Self::default();
            let mut state = fetcher.0.borrow_mut();
            state.responses = BTreeMap::from([
                (
                    loose_path(CATALOG_PATH),
                    FakeResponse::Bytes(VALID_CATALOG.to_vec()),
                ),
                (
                    loose_path(MANIFEST_PATH),
                    FakeResponse::Bytes(VALID_MANIFEST.to_vec()),
                ),
                (
                    loose_path("assets/EDATA.001"),
                    FakeResponse::Bytes(VALID_IMAGE_1.to_vec()),
                ),
                (
                    loose_path("assets/EDATA.002"),
                    FakeResponse::Bytes(VALID_IMAGE_2.to_vec()),
                ),
                (
                    loose_path("assets/EDATA.003"),
                    FakeResponse::Bytes(VALID_IMAGE_3.to_vec()),
                ),
            ]);
            drop(state);
            fetcher
        }

        fn fetch(&self, path: String, max_bytes: usize) -> FakeFuture {
            FakeFuture {
                state: Rc::clone(&self.0),
                path,
                max_bytes,
                started: false,
            }
        }
    }

    struct FakeFuture {
        state: Rc<RefCell<FetchState>>,
        path: String,
        max_bytes: usize,
        started: bool,
    }

    impl Future for FakeFuture {
        type Output = Result<Vec<u8>, EncyclopediaFetchError>;

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
            if !self.started {
                self.started = true;
                let mut state = self.state.borrow_mut();
                state.active += 1;
                state.max_active = state.max_active.max(state.active);
                state.requests.push((self.path.clone(), self.max_bytes));
                context.waker().wake_by_ref();
                return Poll::Pending;
            }

            let mut state = self.state.borrow_mut();
            state.active -= 1;
            let response = state
                .responses
                .get(&self.path)
                .cloned()
                .unwrap_or(FakeResponse::NotFound);
            Poll::Ready(match response {
                FakeResponse::Bytes(bytes) if bytes.len() > self.max_bytes => {
                    Err(EncyclopediaFetchError::ResourceLimit {
                        max_bytes: self.max_bytes,
                    })
                }
                FakeResponse::Bytes(bytes) => Ok(bytes),
                FakeResponse::NotFound => Err(EncyclopediaFetchError::NotFound),
                FakeResponse::HttpStatus(status) => Err(EncyclopediaFetchError::HttpStatus(status)),
                FakeResponse::Transport => Err(EncyclopediaFetchError::Transport),
            })
        }
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        let mut context = Context::from_waker(Waker::noop());
        let mut future = Box::pin(future);
        loop {
            match Pin::as_mut(&mut future).poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    fn valid_dats() -> HashMap<String, Vec<u8>> {
        HashMap::from([("synthetic.dat".to_owned(), VALID_DAT.to_vec())])
    }

    fn http_get(
        origin: &str,
        request_path: &str,
        max_bytes: usize,
    ) -> Result<Vec<u8>, EncyclopediaFetchError> {
        let address = origin.strip_prefix("http://").unwrap();
        let mut stream = TcpStream::connect(address).map_err(|error| {
            eprintln!("controlled HTTP connect failed for {request_path}: {error}");
            EncyclopediaFetchError::Transport
        })?;
        write!(
            stream,
            "GET /{request_path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n"
        )
        .map_err(|error| {
            eprintln!("controlled HTTP request write failed for {request_path}: {error}");
            EncyclopediaFetchError::Transport
        })?;
        let mut reader = BufReader::new(stream);
        let mut status = String::new();
        reader.read_line(&mut status).map_err(|error| {
            eprintln!("controlled HTTP status read failed for {request_path}: {error}");
            EncyclopediaFetchError::Transport
        })?;
        let status_code = status
            .split_ascii_whitespace()
            .nth(1)
            .and_then(|value| value.parse::<u16>().ok())
            .ok_or_else(|| {
                eprintln!("controlled HTTP invalid status for {request_path}: {status:?}");
                EncyclopediaFetchError::Transport
            })?;
        let mut declared_length = None;
        loop {
            let mut header = String::new();
            reader.read_line(&mut header).map_err(|error| {
                eprintln!("controlled HTTP header read failed for {request_path}: {error}");
                EncyclopediaFetchError::Transport
            })?;
            if header == "\r\n" {
                break;
            }
            if let Some(value) = header
                .strip_prefix("Content-Length: ")
                .and_then(|value| value.trim().parse::<usize>().ok())
            {
                declared_length = Some(value);
            }
        }
        if status_code == 404 {
            return Err(EncyclopediaFetchError::NotFound);
        }
        if !(200..300).contains(&status_code) {
            return Err(EncyclopediaFetchError::HttpStatus(status_code));
        }
        if declared_length.is_some_and(|length| length > max_bytes) {
            return Err(EncyclopediaFetchError::ResourceLimit { max_bytes });
        }
        let take_limit = u64::try_from(max_bytes)
            .unwrap_or(u64::MAX)
            .saturating_add(1);
        let mut bytes = Vec::new();
        reader
            .take(take_limit)
            .read_to_end(&mut bytes)
            .map_err(|error| {
                eprintln!("controlled HTTP body read failed for {request_path}: {error}");
                EncyclopediaFetchError::Transport
            })?;
        if bytes.len() > max_bytes {
            return Err(EncyclopediaFetchError::ResourceLimit { max_bytes });
        }
        Ok(bytes)
    }

    #[test]
    fn controlled_http_bundle_prepares_without_a_viewer() {
        let responses = BTreeMap::from([
            ("/data/encyclopedia/catalog.json", VALID_CATALOG),
            ("/data/encyclopedia/manifest.json", VALID_MANIFEST),
            ("/data/encyclopedia/assets/EDATA.001", VALID_IMAGE_1),
            ("/data/encyclopedia/assets/EDATA.002", VALID_IMAGE_2),
            ("/data/encyclopedia/assets/EDATA.003", VALID_IMAGE_3),
        ]);
        let listener = TcpListener::bind("127.0.0.1:0")
            .unwrap_or_else(|error| panic!("controlled HTTP listener bind failed: {error}"));
        let address = listener
            .local_addr()
            .unwrap_or_else(|error| panic!("controlled HTTP listener address failed: {error}"));
        let server = std::thread::spawn(move || {
            let mut observed = Vec::new();
            for connection in listener.incoming().take(responses.len()) {
                let mut stream = connection
                    .unwrap_or_else(|error| panic!("controlled HTTP accept failed: {error}"));
                let request =
                    read_controlled_http_request_headers(&stream).unwrap_or_else(|error| {
                        panic!("controlled HTTP request header read failed: {error}")
                    });
                let request_path = request.split_ascii_whitespace().nth(1).unwrap().to_owned();
                observed.push(request_path.clone());
                let bytes = responses[request_path.as_str()];
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    bytes.len()
                )
                .unwrap_or_else(|error| {
                    panic!("controlled HTTP response header write failed: {error}")
                });
                stream.write_all(bytes).unwrap_or_else(|error| {
                    panic!("controlled HTTP response body write failed: {error}")
                });
                stream.flush().unwrap_or_else(|error| {
                    panic!("controlled HTTP response flush failed: {error}")
                });
            }
            observed
        });
        let origin = format!("http://{address}");

        let session = block_on(prepare_loose_encyclopedia(
            &|path, max_bytes| std::future::ready(http_get(&origin, &path, max_bytes)),
            &valid_dats(),
        ))
        .unwrap()
        .unwrap();

        assert_eq!(session.base_catalog().images.len(), 3);
        assert_eq!(
            server.join().unwrap(),
            vec![
                "/data/encyclopedia/manifest.json",
                "/data/encyclopedia/catalog.json",
                "/data/encyclopedia/assets/EDATA.001",
                "/data/encyclopedia/assets/EDATA.002",
                "/data/encyclopedia/assets/EDATA.003",
            ]
        );
    }

    fn prepare(
        fetcher: &FakeFetcher,
    ) -> Result<Option<EncyclopediaSession>, LooseEncyclopediaError> {
        block_on(prepare_loose_encyclopedia(
            &|path, max_bytes| fetcher.fetch(path, max_bytes),
            &valid_dats(),
        ))
    }

    #[test]
    fn both_missing_metadata_files_mean_the_loose_namespace_is_absent() {
        let fetcher = FakeFetcher::default();

        let result = prepare(&fetcher).expect("two 404 responses are the one valid absence state");

        assert!(result.is_none());
        let requests = &fetcher.0.borrow().requests;
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].0, loose_path(MANIFEST_PATH));
        assert_eq!(requests[1].0, loose_path(CATALOG_PATH));
    }

    #[test]
    fn partial_http_and_transport_metadata_fail_instead_of_becoming_absence() {
        for (path, response, expected_code, expected_path) in [
            (
                CATALOG_PATH,
                FakeResponse::Bytes(VALID_CATALOG.to_vec()),
                "missing_runtime_file",
                MANIFEST_PATH,
            ),
            (
                MANIFEST_PATH,
                FakeResponse::Bytes(VALID_MANIFEST.to_vec()),
                "missing_runtime_file",
                CATALOG_PATH,
            ),
            (
                CATALOG_PATH,
                FakeResponse::HttpStatus(403),
                "fetch_failed",
                CATALOG_PATH,
            ),
            (
                CATALOG_PATH,
                FakeResponse::Transport,
                "fetch_failed",
                CATALOG_PATH,
            ),
        ] {
            let fetcher = FakeFetcher::default();
            fetcher
                .0
                .borrow_mut()
                .responses
                .insert(loose_path(path), response);

            let error = prepare(&fetcher).unwrap_err();

            assert_eq!(error.code(), expected_code);
            assert_eq!(error.path(), expected_path);
        }
    }

    #[test]
    fn manifest_and_catalog_are_fetched_before_exactly_allowlisted_images() {
        let fetcher = FakeFetcher::with_valid_bundle();

        let session = prepare(&fetcher).unwrap().unwrap();
        let image_cache = original_image_cache(&session).unwrap();

        assert_eq!(session.base_bytes().len(), 5);
        assert_eq!(
            image_cache,
            HashMap::from([
                ("EDATA.001".to_owned(), VALID_IMAGE_1.to_vec()),
                ("EDATA.002".to_owned(), VALID_IMAGE_2.to_vec()),
                ("EDATA.003".to_owned(), VALID_IMAGE_3.to_vec()),
            ]),
            "the renderer cache is projected only from exact validated retained bytes",
        );
        let state = fetcher.0.borrow();
        assert_eq!(
            state.max_active, 3,
            "three declared images share one bounded batch"
        );
        assert_eq!(
            state
                .requests
                .iter()
                .map(|request| request.0.as_str())
                .collect::<Vec<_>>(),
            vec![
                "data/encyclopedia/manifest.json",
                "data/encyclopedia/catalog.json",
                "data/encyclopedia/assets/EDATA.001",
                "data/encyclopedia/assets/EDATA.002",
                "data/encyclopedia/assets/EDATA.003",
            ]
        );
        assert_eq!(state.requests[2].1, VALID_IMAGE_1.len());
        assert_eq!(state.requests[3].1, VALID_IMAGE_2.len());
        assert_eq!(state.requests[4].1, VALID_IMAGE_3.len());
    }

    #[test]
    fn production_loader_caps_nine_declared_images_at_four_in_flight() {
        let fetcher = FakeFetcher::with_valid_bundle();
        let mut catalog: Value = serde_json::from_slice(VALID_CATALOG).unwrap();
        let template = catalog["images"]["edata:1"].clone();
        let mut manifest: Value = serde_json::from_slice(VALID_MANIFEST).unwrap();
        for index in 4..=9 {
            let image_id = format!("edata:{index}");
            let image_path = format!("assets/SYNTHETIC.{index:03}");
            let mut descriptor = template.clone();
            descriptor["path"] = Value::from(image_path.clone());
            catalog["images"][&image_id] = descriptor;
            manifest["files"][&image_path] =
                Value::from(catalog["images"][&image_id]["sha256"].as_str().unwrap());
            catalog["topics"]["original:60001"]["localized"][&format!("10{}", index + 32)] = serde_json::json!({
                "title": format!("Synthetic image {index}"),
                "body": "Contributor-written batching fixture.",
                "image_id": image_id,
            });
            fetcher.0.borrow_mut().responses.insert(
                loose_path(&image_path),
                FakeResponse::Bytes(VALID_IMAGE_1.to_vec()),
            );
        }
        let catalog_bytes = serde_json::to_vec(&catalog).unwrap();
        let catalog_digest = inspect_encyclopedia_bytes(&catalog_bytes, None)
            .unwrap()
            .sha256;
        manifest["catalog_sha256"] = Value::from(catalog_digest.clone());
        manifest["files"][CATALOG_PATH] = Value::from(catalog_digest);
        {
            let mut state = fetcher.0.borrow_mut();
            state
                .responses
                .insert(loose_path(CATALOG_PATH), FakeResponse::Bytes(catalog_bytes));
            state.responses.insert(
                loose_path(MANIFEST_PATH),
                FakeResponse::Bytes(serde_json::to_vec(&manifest).unwrap()),
            );
        }

        let session = prepare(&fetcher).unwrap().unwrap();

        assert_eq!(session.base_catalog().images.len(), 9);
        assert_eq!(session.base_bytes().len(), 11);
        let state = fetcher.0.borrow();
        assert_eq!(state.max_active, 4);
        assert_eq!(
            state
                .requests
                .iter()
                .map(|request| request.0.as_str())
                .collect::<Vec<_>>(),
            vec![
                "data/encyclopedia/manifest.json",
                "data/encyclopedia/catalog.json",
                "data/encyclopedia/assets/EDATA.001",
                "data/encyclopedia/assets/EDATA.002",
                "data/encyclopedia/assets/EDATA.003",
                "data/encyclopedia/assets/SYNTHETIC.004",
                "data/encyclopedia/assets/SYNTHETIC.005",
                "data/encyclopedia/assets/SYNTHETIC.006",
                "data/encyclopedia/assets/SYNTHETIC.007",
                "data/encyclopedia/assets/SYNTHETIC.008",
                "data/encyclopedia/assets/SYNTHETIC.009",
            ]
        );
    }

    #[test]
    fn html_catalog_failed_images_and_dat_mismatches_publish_no_session() {
        let html = FakeFetcher::with_valid_bundle();
        html.0.borrow_mut().responses.insert(
            loose_path(CATALOG_PATH),
            FakeResponse::Bytes(b"<!doctype html>".to_vec()),
        );
        assert_eq!(prepare(&html).unwrap_err().code(), "invalid_json");
        assert_eq!(
            html.0.borrow().requests.len(),
            2,
            "invalid metadata fetches no art"
        );

        let failed_image = FakeFetcher::with_valid_bundle();
        failed_image
            .0
            .borrow_mut()
            .responses
            .insert(loose_path("assets/EDATA.002"), FakeResponse::Transport);
        let error = prepare(&failed_image).unwrap_err();
        assert_eq!(error.code(), "fetch_failed");
        assert_eq!(error.path(), "assets/EDATA.002");

        let oversized_response = FakeFetcher::with_valid_bundle();
        let mut too_many_bytes = VALID_IMAGE_1.to_vec();
        too_many_bytes.push(0);
        oversized_response.0.borrow_mut().responses.insert(
            loose_path("assets/EDATA.001"),
            FakeResponse::Bytes(too_many_bytes),
        );
        let error = prepare(&oversized_response).unwrap_err();
        assert_eq!(error.code(), "resource_limit:fetch_bytes");
        assert_eq!(error.path(), "assets/EDATA.001");

        let corrupt_image = FakeFetcher::with_valid_bundle();
        let mut corrupt = VALID_IMAGE_1.to_vec();
        corrupt[0] = b'X';
        corrupt_image
            .0
            .borrow_mut()
            .responses
            .insert(loose_path("assets/EDATA.001"), FakeResponse::Bytes(corrupt));
        let error = prepare(&corrupt_image).unwrap_err();
        assert_eq!(error.code(), "invalid_image");
        assert_eq!(error.path(), "assets/EDATA.001");

        let wrong_dat = FakeFetcher::with_valid_bundle();
        let error = block_on(prepare_loose_encyclopedia(
            &|path, max_bytes| wrong_dat.fetch(path, max_bytes),
            &HashMap::from([("SYNTHETIC.DAT".to_owned(), b"wrong".to_vec())]),
        ))
        .unwrap_err();
        assert_eq!(error.code(), "binding_source_mismatch");
        assert_eq!(
            wrong_dat.0.borrow().requests.len(),
            2,
            "DAT mismatch precedes art fetch"
        );

        let manifest = parse_manifest(VALID_MANIFEST).unwrap();
        assert_eq!(
            selected_dat_hashes_with_limit(&manifest, &valid_dats(), VALID_DAT.len())
                .unwrap()
                .len(),
            1,
            "the exact selected-DAT verification boundary is accepted",
        );
        let oversized_dat = HashMap::from([("SYNTHETIC.DAT".to_owned(), vec![0; 5])]);
        let error = selected_dat_hashes_with_limit(&manifest, &oversized_dat, 4).unwrap_err();
        assert_eq!(error.code(), "resource_limit:binding_source_bytes");
        assert_eq!(error.path(), "SYNTHETIC.DAT");
    }

    #[test]
    fn every_manifest_binding_source_requires_present_matching_selected_bytes() {
        const SECOND_DAT: &[u8] = b"second synthetic selected DAT";

        let mut manifest_value: Value = serde_json::from_slice(VALID_MANIFEST).unwrap();
        manifest_value["binding_sources"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({
                "basename": "SECOND.DAT",
                "sha256": inspect_encyclopedia_bytes(SECOND_DAT, None).unwrap().sha256,
            }));
        let manifest = parse_manifest(&serde_json::to_vec(&manifest_value).unwrap()).unwrap();

        let missing = selected_dat_hashes(&manifest, &valid_dats()).unwrap_err();
        assert_eq!(missing.code(), "binding_source_mismatch");
        assert_eq!(missing.path(), "SECOND.DAT");

        let mut wrong = valid_dats();
        wrong.insert("SECOND.DAT".to_owned(), b"wrong selected bytes".to_vec());
        let mismatch = selected_dat_hashes(&manifest, &wrong).unwrap_err();
        assert_eq!(mismatch.code(), "binding_source_mismatch");
        assert_eq!(mismatch.path(), "SECOND.DAT");

        let mut complete = valid_dats();
        complete.insert("SECOND.DAT".to_owned(), SECOND_DAT.to_vec());
        assert_eq!(selected_dat_hashes(&manifest, &complete).unwrap().len(), 2);
    }

    #[test]
    fn manifest_closure_and_declared_image_budgets_fail_before_image_fetch() {
        let extra = FakeFetcher::with_valid_bundle();
        let mut manifest: Value = serde_json::from_slice(VALID_MANIFEST).unwrap();
        manifest["files"]["assets/UNLISTED.999"] = Value::from("0".repeat(64));
        extra.0.borrow_mut().responses.insert(
            loose_path(MANIFEST_PATH),
            FakeResponse::Bytes(serde_json::to_vec(&manifest).unwrap()),
        );
        let error = prepare(&extra).unwrap_err();
        assert_eq!(error.code(), "manifest_file_set_mismatch");
        assert_eq!(extra.0.borrow().requests.len(), 2);

        let catalog = parse_catalog(VALID_CATALOG).unwrap();
        let manifest = parse_manifest(VALID_MANIFEST).unwrap();
        let mut oversized = catalog.clone();
        oversized.images.get_mut("edata:1").unwrap().byte_length =
            MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64 + 1;
        let error = declared_image_requests(&oversized, &manifest).unwrap_err();
        assert_eq!(error.code(), "resource_limit:image_bytes");

        let mut aggregate = catalog;
        for descriptor in aggregate.images.values_mut() {
            descriptor.byte_length = MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64;
        }
        let template = aggregate.images["edata:1"].clone();
        let mut fourth = template.clone();
        fourth.path = "assets/EDATA.004".to_owned();
        aggregate.images.insert(
            rebellion_data::encyclopedia::BaseImageId("edata:4".to_owned()),
            fourth,
        );
        let mut aggregate_manifest = manifest;
        aggregate_manifest
            .files
            .insert("assets/EDATA.004".to_owned(), "0".repeat(64));
        assert_eq!(
            declared_image_requests(&aggregate, &aggregate_manifest)
                .unwrap()
                .len(),
            4,
            "four declared 32 MiB images are the exact 128 MiB boundary"
        );

        let mut fifth = template;
        fifth.path = "assets/EDATA.005".to_owned();
        aggregate.images.insert(
            rebellion_data::encyclopedia::BaseImageId("edata:5".to_owned()),
            fifth,
        );
        aggregate_manifest
            .files
            .insert("assets/EDATA.005".to_owned(), "0".repeat(64));
        let error = declared_image_requests(&aggregate, &aggregate_manifest).unwrap_err();
        assert_eq!(error.code(), "resource_limit:effective_image_bytes");
    }
}
