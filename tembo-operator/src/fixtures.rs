//! Helper methods only available for tests
use crate::{
    apis::coredb_types::{CoreDB, CoreDBSpec, CoreDBStatus},
    Context, Result, COREDB_FINALIZER,
};
use assert_json_diff::assert_json_include;
use http::{Request, Response};
use k8s_openapi::api::core::v1::{Pod, Secret};
use kube::{
    api::ObjectMeta, api::TypeMeta, client::Body, core::ObjectList, runtime::events::Recorder,
    Client, Resource, ResourceExt,
};
use std::sync::Arc;

impl CoreDB {
    /// A CoreDB that will cause the reconciler to fail
    pub fn illegal() -> Self {
        let mut d = CoreDB::new("illegal", CoreDBSpec::default());
        d.meta_mut().namespace = Some("default".into());
        d
    }

    /// A normal test CoreDB
    pub fn test() -> Self {
        let mut d = CoreDB::new("testdb", CoreDBSpec::default());
        d.meta_mut().namespace = Some("testns".into());
        d.meta_mut().uid = Some("752d59ef-2671-4890-9feb-0097459b18c8".into());
        d.spec.replicas = 1;
        // Need to figure out how to mock websocket
        // in order to unit test a feature using kube exec
        d.spec.postgresExporterEnabled = false;
        d
    }

    /// Modify coredb to be set to paused
    pub fn needs_stop(mut self) -> Self {
        self.spec.stop = true;
        self
    }

    /// Modify coredb to set a deletion timestamp
    pub fn needs_delete(mut self) -> Self {
        use chrono::prelude::{DateTime, TimeZone, Utc};
        let now: DateTime<Utc> = Utc.with_ymd_and_hms(2017, 4, 2, 12, 50, 32).unwrap();
        use k8s_openapi::apimachinery::pkg::apis::meta::v1::Time;
        self.meta_mut().deletion_timestamp = Some(Time(now));
        self
    }

    /// Modify a coredb to have the expected finalizer
    pub fn finalized(mut self) -> Self {
        self.finalizers_mut().push(COREDB_FINALIZER.to_string());
        self
    }

    /// Modify a coredb to have an expected status
    pub fn with_status(mut self, status: CoreDBStatus) -> Self {
        self.status = Some(status);
        self
    }
}
// We wrap tower_test::mock::Handle
type ApiServerHandle = tower_test::mock::Handle<Request<Body>, Response<Body>>;
pub struct ApiServerVerifier(ApiServerHandle);

/// Scenarios we test for in ApiServerVerifier
pub enum Scenario {
    /// objects without finalizers will get a finalizer applied (and not call the apply loop)
    FinalizerCreation(CoreDB),
    /// objects that do not fail and do not cause publishes will only patch
    StatusPatch(CoreDB),
    /// finalized objects with hide set causes both an event and then a hide patch
    EventPublishThenStatusPatch(String, CoreDB),
    /// finalized objects "with errors" (i.e. the "illegal" object) will short circuit the apply loop
    RadioSilence,
    /// objects with a deletion timestamp will run the cleanup loop sending event and removing the finalizer
    Cleanup(String, CoreDB),
}

pub async fn timeout_after_1s(handle: tokio::task::JoinHandle<()>) {
    tokio::time::timeout(std::time::Duration::from_secs(1), handle)
        .await
        .expect("timeout on mock apiserver")
        .expect("scenario succeeded")
}

/// Create a responder + verifier object that deals with the main reconcile scenarios
///
impl ApiServerVerifier {
    /// Tests only get to run specific scenarios that has matching handlers
    ///
    /// This setup makes it easy to handle multiple requests by chaining handlers together.
    ///
    /// NB: If the controller is making more calls than we are handling in the scenario,
    /// you then typically see a `KubeError(Service(Closed(())))` from the reconciler.
    ///
    /// You should await the `JoinHandle` (with a timeout) from this function to ensure that the
    /// scenario runs to completion (i.e. all expected calls were responded to),
    /// using the timeout to catch missing api calls to Kubernetes.
    pub fn run(self, scenario: Scenario) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            // moving self => one scenario per test
            match scenario {
                Scenario::FinalizerCreation(cdb) => self.handle_finalizer_creation(&cdb).await,
                Scenario::StatusPatch(cdb) => self.handle_coredb_patch(&cdb).await,
                Scenario::EventPublishThenStatusPatch(reason, cdb) => {
                    self.handle_event_create(reason)
                        .await
                        .unwrap()
                        .handle_coredb_patch(&cdb)
                        .await
                }
                Scenario::RadioSilence => Ok(self),
                Scenario::Cleanup(reason, cdb) => {
                    self.handle_event_create(reason)
                        .await
                        .unwrap()
                        .handle_finalizer_removal(cdb)
                        .await
                }
            }
            .expect("scenario completed without errors");
        })
    }

    // chainable scenario handlers
    async fn handle_finalizer_creation(mut self, cdb: &CoreDB) -> Result<Self> {
        let (request, send) = self.0.next_request().await.expect("service not called");
        let coredb = cdb.clone();
        // We expect a json patch to the specified document adding our finalizer
        assert_eq!(request.method(), http::Method::PATCH);
        assert_eq!(
            request.uri().to_string(),
            format!(
                "/apis/coredb.io/v1alpha1/namespaces/testns/coredbs/{}?",
                coredb.name_any()
            )
        );
        let expected_patch = serde_json::json!([
            { "op": "test", "path": "/metadata/finalizers", "value": null },
            { "op": "add", "path": "/metadata/finalizers", "value": vec![COREDB_FINALIZER] }
        ]);
        let req_body = request.into_body().collect_bytes().await.unwrap();
        let runtime_patch: serde_json::Value =
            serde_json::from_slice(&req_body).expect("valid document from runtime");
        assert_json_include!(actual: runtime_patch, expected: expected_patch);

        let response = serde_json::to_vec(&coredb.finalized()).unwrap(); // respond as the apiserver would have
        send.send_response(Response::builder().body(Body::from(response)).unwrap());
        Ok(self)
    }

    async fn handle_finalizer_removal(mut self, cdb: CoreDB) -> Result<Self> {
        let (request, send) = self.0.next_request().await.expect("service not called");
        let coredb = cdb.clone();
        // We expect a json patch to the specified document removing our finalizer (at index 0)
        assert_eq!(request.method(), http::Method::PATCH);
        assert_eq!(
            request.uri().to_string(),
            format!(
                "/apis/coredb.io/v1alpha1/namespaces/testns/coredbs/{}?",
                coredb.name_any()
            )
        );
        let expected_patch = serde_json::json!([
            { "op": "test", "path": "/metadata/finalizers/0", "value": COREDB_FINALIZER },
            { "op": "remove", "path": "/metadata/finalizers/0" }
        ]);
        let req_body = request.into_body().collect_bytes().await.unwrap();
        let runtime_patch: serde_json::Value =
            serde_json::from_slice(&req_body).expect("valid document from runtime");
        assert_json_include!(actual: runtime_patch, expected: expected_patch);

        let response = serde_json::to_vec(&coredb).unwrap(); // respond as the apiserver would have
        send.send_response(Response::builder().body(Body::from(response)).unwrap());
        Ok(self)
    }

    async fn handle_coredb_patch(mut self, coredb: &CoreDB) -> Result<Self> {
        let (request, send) = self.0.next_request().await.expect("service not called");
        let coredb = coredb.clone();
        assert_eq!(request.method(), http::Method::GET);
        assert_eq!(
            request.uri().to_string(),
            format!("/api/v1/namespaces/testns/secrets?&labelSelector=app%3Dcoredb")
        );

        // We need to send an empty ObjectList<Secret> back as our response
        let obj: ObjectList<Secret> = ObjectList {
            metadata: Default::default(),
            items: vec![],
            types: TypeMeta::default(),
        };
        let response = serde_json::to_vec(&obj).unwrap();
        send.send_response(Response::builder().body(Body::from(response)).unwrap());

        // After the GET on Secrets, we expect a PATCH to Secret
        let (request, send) = self.0.next_request().await.expect("service not called");
        assert_eq!(request.method(), http::Method::PATCH);
        assert_eq!(
                request.uri().to_string(),
                format!(
                    "/api/v1/namespaces/testns/secrets/testdb-connection?&force=true&fieldManager=cntrlr"
                )
            );
        send.send_response(Response::builder().body(request.into_body()).unwrap());

        // After the PATCH to Secret, we expect a PATCH to StatefulSet
        let (request, send) = self.0.next_request().await.expect("service not called");
        assert_eq!(request.method(), http::Method::PATCH);
        assert_eq!(
                request.uri().to_string(),
                format!(
                    "/apis/apps/v1/namespaces/testns/statefulsets/testdb?&force=true&fieldManager=cntrlr"
                )
            );
        send.send_response(Response::builder().body(request.into_body()).unwrap());

        // After the PATCH to StatefulSet, we expect a PATCH to Service
        let (request, send) = self.0.next_request().await.expect("service not called");
        assert_eq!(request.method(), http::Method::PATCH);
        assert_eq!(
            request.uri().to_string(),
            format!("/api/v1/namespaces/testns/services/testdb?&force=true&fieldManager=cntrlr")
        );
        send.send_response(Response::builder().body(request.into_body()).unwrap());

        // After the PATCH to Service, we expect a GET to Pods
        let (request, send) = self.0.next_request().await.expect("service not called");
        assert_eq!(request.method(), http::Method::GET);
        assert_eq!(
            request.uri().to_string(),
            format!("/api/v1/namespaces/testns/pods?&labelSelector=statefulset%3Dtestdb")
        );

        // We need to send an ObjectList<Pod> back as our response
        let pod: Pod = Pod {
            metadata: ObjectMeta {
                name: Some("testdb-0".to_string()),
                namespace: Some("testns".to_string()),
                ..ObjectMeta::default()
            },
            ..Pod::default()
        };
        let obj: ObjectList<Pod> = ObjectList {
            metadata: Default::default(),
            items: vec![pod],
            types: TypeMeta::default(),
        };
        let response = serde_json::to_vec(&obj).unwrap();
        send.send_response(Response::builder().body(Body::from(response)).unwrap());

        // expecting to get a PATCH request to update CoreDB resource
        let (request, send) = self.0.next_request().await.expect("service not called");
        assert_eq!(request.method(), http::Method::PATCH);
        assert_eq!(
                request.uri().to_string(),
                format!(
                    "/apis/coredb.io/v1alpha1/namespaces/testns/coredbs/{}/status?&force=true&fieldManager=cntrlr",
                    coredb.name_any()
                )
            );
        let req_body = request.into_body().collect_bytes().await.unwrap();
        let json: serde_json::Value =
            serde_json::from_slice(&req_body).expect("patch_status object is json");
        let status_json = json.get("status").expect("status object").clone();
        let status: CoreDBStatus =
            serde_json::from_value(status_json).expect("contains valid status");
        assert!(
            status.running,
            "CoreDB::test says the status isn't running, but it was expected to be running."
        );

        let response = serde_json::to_vec(&coredb.with_status(status)).unwrap();
        // pass through coredb "patch accepted"
        send.send_response(Response::builder().body(Body::from(response)).unwrap());
        Ok(self)
    }

    async fn handle_event_create(mut self, reason: String) -> Result<Self> {
        let (request, send) = self.0.next_request().await.expect("service not called");
        assert_eq!(request.method(), http::Method::POST);
        assert_eq!(
            request.uri().to_string(),
            format!("/apis/events.k8s.io/v1/namespaces/default/events?")
        );
        // verify the event reason matches the expected
        let req_body = request.into_body().collect_bytes().await.unwrap();
        let postdata: serde_json::Value =
            serde_json::from_slice(&req_body).expect("valid event from runtime");
        dbg!("postdata for event: {}", postdata.clone());
        assert_eq!(
            postdata.get("reason").unwrap().as_str().map(String::from),
            Some(reason)
        );
        // then pass through the body
        send.send_response(Response::builder().body(Body::from(req_body)).unwrap());
        Ok(self)
    }
}

impl Context {
    // Create a test context with a mocked kube client, locally registered metrics and default diagnostics
    pub fn test() -> (Arc<Self>, ApiServerVerifier) {
        let (mock_service, handle) = tower_test::mock::pair::<Request<Body>, Response<Body>>();
        let mock_client = Client::new(mock_service, "default");
        let mock_recorder = Recorder::new(mock_client.clone(), "doc-ctrl-test".into());
        let ctx = Self {
            client: mock_client,
            metrics: Arc::default(),
            diagnostics: Arc::default(),
            recorder: mock_recorder,
        };
        (Arc::new(ctx), ApiServerVerifier(handle))
    }
}
