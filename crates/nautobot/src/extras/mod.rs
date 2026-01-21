//! extras endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.extras().computed_fields().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::{Error, Result};
use crate::pagination::Page;
use crate::query::QueryBuilder;
use crate::resource::Resource;

/// ComputedField model.
pub type ComputedField = crate::models::ComputedField;
/// ConfigContextSchema model.
pub type ConfigContextSchema = crate::models::ConfigContextSchema;
/// ConfigContext model.
pub type ConfigContext = crate::models::ConfigContext;
/// ContactAssociation model.
pub type ContactAssociation = crate::models::ContactAssociation;
/// Contact model.
pub type Contact = crate::models::Contact;
/// ContentType model.
pub type ContentType = crate::models::ContentType;
/// CustomFieldChoice model.
pub type CustomFieldChoice = crate::models::CustomFieldChoice;
/// CustomField model.
pub type CustomField = crate::models::CustomField;
/// CustomLink model.
pub type CustomLink = crate::models::CustomLink;
/// DynamicGroupMembership model.
pub type DynamicGroupMembership = crate::models::DynamicGroupMembership;
/// DynamicGroup model.
pub type DynamicGroup = crate::models::DynamicGroup;
/// ExportTemplate model.
pub type ExportTemplate = crate::models::ExportTemplate;
/// ExternalIntegration model.
pub type ExternalIntegration = crate::models::ExternalIntegration;
/// FileProxy model.
pub type FileProxy = crate::models::FileProxy;
/// GitRepository model.
pub type GitRepository = crate::models::GitRepository;
/// GitRepositorySyncResponse model.
pub type GitRepositorySyncResponse = crate::models::GitRepositorySyncResponse;
/// GraphQlQuery model.
pub type GraphQlQuery = crate::models::GraphQlQuery;
/// GraphQlQueryInputRequest model.
pub type GraphQlQueryInputRequest = crate::models::GraphQlQueryInputRequest;
/// GraphQlQueryOutput model.
pub type GraphQlQueryOutput = crate::models::GraphQlQueryOutput;
/// ImageAttachment model.
pub type ImageAttachment = crate::models::ImageAttachment;
/// JobButton model.
pub type JobButton = crate::models::JobButton;
/// JobHook model.
pub type JobHook = crate::models::JobHook;
/// JobLogEntry model.
pub type JobLogEntry = crate::models::JobLogEntry;
/// JobInputRequest model.
pub type JobInputRequest = crate::models::JobInputRequest;
/// JobRunResponse model.
pub type JobRunResponse = crate::models::JobRunResponse;
/// JobVariable model.
pub type JobVariable = crate::models::JobVariable;
/// JobQueueAssignment model.
pub type JobQueueAssignment = crate::models::JobQueueAssignment;
/// JobQueue model.
pub type JobQueue = crate::models::JobQueue;
/// JobResult model.
pub type JobResult = crate::models::JobResult;
/// Job model.
pub type Job = crate::models::Job;
/// MetadataChoice model.
pub type MetadataChoice = crate::models::MetadataChoice;
/// MetadataType model.
pub type MetadataType = crate::models::MetadataType;
/// Note model.
pub type Note = crate::models::Note;
/// NoteInputRequest model.
pub type NoteInputRequest = crate::models::NoteInputRequest;
/// ObjectChange model.
pub type ObjectChange = crate::models::ObjectChange;
/// ObjectMetadata model.
pub type ObjectMetadata = crate::models::ObjectMetadata;
/// RelationshipAssociation model.
pub type RelationshipAssociation = crate::models::RelationshipAssociation;
/// Relationship model.
pub type Relationship = crate::models::Relationship;
/// Role model.
pub type Role = crate::models::Role;
/// SavedView model.
pub type SavedView = crate::models::SavedView;
/// ScheduledJob model.
pub type ScheduledJob = crate::models::ScheduledJob;
/// SecretsGroupAssociation model.
pub type SecretsGroupAssociation = crate::models::SecretsGroupAssociation;
/// SecretsGroup model.
pub type SecretsGroup = crate::models::SecretsGroup;
/// Secret model.
pub type Secret = crate::models::Secret;
/// Secrets check response model.
pub type SecretsCheckResponse = crate::models::ExtrasSecretsCheckRetrieve200Response;
/// StaticGroupAssociation model.
pub type StaticGroupAssociation = crate::models::StaticGroupAssociation;
/// Status model.
pub type Status = crate::models::Status;
/// Tag model.
pub type Tag = crate::models::Tag;
/// Team model.
pub type Team = crate::models::Team;
/// UserSavedViewAssociation model.
pub type UserSavedViewAssociation = crate::models::UserSavedViewAssociation;
/// Webhook model.
pub type Webhook = crate::models::Webhook;

/// resource for extras/computed-fields/.
pub type ComputedFieldsApi = Resource<crate::models::ComputedField>;
/// resource for extras/config-context-schemas/.
pub type ConfigContextSchemasApi = Resource<crate::models::ConfigContextSchema>;
/// resource for extras/config-contexts/.
pub type ConfigContextsApi = Resource<crate::models::ConfigContext>;
/// resource for extras/contact-associations/.
pub type ContactAssociationsApi = Resource<crate::models::ContactAssociation>;
/// resource for extras/contacts/.
pub type ContactsApi = Resource<crate::models::Contact>;
/// resource for extras/content-types/.
pub type ContentTypesApi = Resource<crate::models::ContentType>;
/// resource for extras/custom-field-choices/.
pub type CustomFieldChoicesApi = Resource<crate::models::CustomFieldChoice>;
/// resource for extras/custom-fields/.
pub type CustomFieldsApi = Resource<crate::models::CustomField>;
/// resource for extras/custom-links/.
pub type CustomLinksApi = Resource<crate::models::CustomLink>;
/// resource for extras/dynamic-group-memberships/.
pub type DynamicGroupMembershipsApi = Resource<crate::models::DynamicGroupMembership>;
/// resource for extras/dynamic-groups/.
pub type DynamicGroupsApi = Resource<crate::models::DynamicGroup>;
/// resource for extras/export-templates/.
pub type ExportTemplatesApi = Resource<crate::models::ExportTemplate>;
/// resource for extras/external-integrations/.
pub type ExternalIntegrationsApi = Resource<crate::models::ExternalIntegration>;
/// resource for extras/file-proxies/.
pub type FileProxiesApi = Resource<crate::models::FileProxy>;
/// resource for extras/git-repositories/.
pub type GitRepositoriesApi = Resource<crate::models::GitRepository>;
/// resource for extras/graphql-queries/.
pub type GraphqlQueriesApi = Resource<crate::models::GraphQlQuery>;
/// resource for extras/image-attachments/.
pub type ImageAttachmentsApi = Resource<crate::models::ImageAttachment>;
/// resource for extras/job-buttons/.
pub type JobButtonsApi = Resource<crate::models::JobButton>;
/// resource for extras/job-hooks/.
pub type JobHooksApi = Resource<crate::models::JobHook>;
/// resource for extras/job-logs/.
pub type JobLogsApi = Resource<crate::models::JobLogEntry>;
/// resource for extras/job-queue-assignments/.
pub type JobQueueAssignmentsApi = Resource<crate::models::JobQueueAssignment>;
/// resource for extras/job-queues/.
pub type JobQueuesApi = Resource<crate::models::JobQueue>;
/// resource for extras/job-results/.
pub type JobResultsApi = Resource<crate::models::JobResult>;
/// resource for extras/jobs/.
pub type JobsApi = Resource<crate::models::Job>;
/// resource for extras/metadata-choices/.
pub type MetadataChoicesApi = Resource<crate::models::MetadataChoice>;
/// resource for extras/metadata-types/.
pub type MetadataTypesApi = Resource<crate::models::MetadataType>;
/// resource for extras/notes/.
pub type NotesApi = Resource<crate::models::Note>;
/// resource for extras/object-changes/.
pub type ObjectChangesApi = Resource<crate::models::ObjectChange>;
/// resource for extras/object-metadata/.
pub type ObjectMetadataApi = Resource<crate::models::ObjectMetadata>;
/// resource for extras/relationship-associations/.
pub type RelationshipAssociationsApi = Resource<crate::models::RelationshipAssociation>;
/// resource for extras/relationships/.
pub type RelationshipsApi = Resource<crate::models::Relationship>;
/// resource for extras/roles/.
pub type RolesApi = Resource<crate::models::Role>;
/// resource for extras/saved-views/.
pub type SavedViewsApi = Resource<crate::models::SavedView>;
/// resource for extras/scheduled-jobs/.
pub type ScheduledJobsApi = Resource<crate::models::ScheduledJob>;
/// resource for extras/secrets-groups-associations/.
pub type SecretsGroupsAssociationsApi = Resource<crate::models::SecretsGroupAssociation>;
/// resource for extras/secrets-groups/.
pub type SecretsGroupsApi = Resource<crate::models::SecretsGroup>;
/// resource for extras/secrets/.
pub type SecretsApi = Resource<crate::models::Secret>;
/// resource for extras/static-group-associations/.
pub type StaticGroupAssociationsApi = Resource<crate::models::StaticGroupAssociation>;
/// resource for extras/statuses/.
pub type StatusesApi = Resource<crate::models::Status>;
/// resource for extras/tags/.
pub type TagsApi = Resource<crate::models::Tag>;
/// resource for extras/teams/.
pub type TeamsApi = Resource<crate::models::Team>;
/// resource for extras/user-saved-view-associations/.
pub type UserSavedViewAssociationsApi = Resource<crate::models::UserSavedViewAssociation>;
/// resource for extras/webhooks/.
pub type WebhooksApi = Resource<crate::models::Webhook>;

/// api for extras endpoints.
#[derive(Clone)]
pub struct ExtrasApi {
    client: Client,
}

impl ExtrasApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the computed fields resource.
    pub fn computed_fields(&self) -> ComputedFieldsApi {
        Resource::new(self.client.clone(), "extras/computed-fields/")
    }

    /// returns the config context schemas resource.
    pub fn config_context_schemas(&self) -> ConfigContextSchemasApi {
        Resource::new(self.client.clone(), "extras/config-context-schemas/")
    }

    /// returns the config contexts resource.
    pub fn config_contexts(&self) -> ConfigContextsApi {
        Resource::new(self.client.clone(), "extras/config-contexts/")
    }

    /// returns the contact associations resource.
    pub fn contact_associations(&self) -> ContactAssociationsApi {
        Resource::new(self.client.clone(), "extras/contact-associations/")
    }

    /// returns the contacts resource.
    pub fn contacts(&self) -> ContactsApi {
        Resource::new(self.client.clone(), "extras/contacts/")
    }

    /// returns the content types resource.
    pub fn content_types(&self) -> ContentTypesApi {
        Resource::new(self.client.clone(), "extras/content-types/")
    }

    /// returns the custom field choices resource.
    pub fn custom_field_choices(&self) -> CustomFieldChoicesApi {
        Resource::new(self.client.clone(), "extras/custom-field-choices/")
    }

    /// returns the custom fields resource.
    pub fn custom_fields(&self) -> CustomFieldsApi {
        Resource::new(self.client.clone(), "extras/custom-fields/")
    }

    /// returns the custom links resource.
    pub fn custom_links(&self) -> CustomLinksApi {
        Resource::new(self.client.clone(), "extras/custom-links/")
    }

    /// returns the dynamic group memberships resource.
    pub fn dynamic_group_memberships(&self) -> DynamicGroupMembershipsApi {
        Resource::new(self.client.clone(), "extras/dynamic-group-memberships/")
    }

    /// returns the dynamic groups resource.
    pub fn dynamic_groups(&self) -> DynamicGroupsApi {
        Resource::new(self.client.clone(), "extras/dynamic-groups/")
    }

    /// returns the export templates resource.
    pub fn export_templates(&self) -> ExportTemplatesApi {
        Resource::new(self.client.clone(), "extras/export-templates/")
    }

    /// returns the external integrations resource.
    pub fn external_integrations(&self) -> ExternalIntegrationsApi {
        Resource::new(self.client.clone(), "extras/external-integrations/")
    }

    /// returns the file proxies resource.
    pub fn file_proxies(&self) -> FileProxiesApi {
        Resource::new(self.client.clone(), "extras/file-proxies/")
    }

    /// returns the git repositories resource.
    pub fn git_repositories(&self) -> GitRepositoriesApi {
        Resource::new(self.client.clone(), "extras/git-repositories/")
    }

    /// returns the graphql queries resource.
    pub fn graphql_queries(&self) -> GraphqlQueriesApi {
        Resource::new(self.client.clone(), "extras/graphql-queries/")
    }

    /// returns the image attachments resource.
    pub fn image_attachments(&self) -> ImageAttachmentsApi {
        Resource::new(self.client.clone(), "extras/image-attachments/")
    }

    /// returns the job buttons resource.
    pub fn job_buttons(&self) -> JobButtonsApi {
        Resource::new(self.client.clone(), "extras/job-buttons/")
    }

    /// returns the job hooks resource.
    pub fn job_hooks(&self) -> JobHooksApi {
        Resource::new(self.client.clone(), "extras/job-hooks/")
    }

    /// returns the job logs resource.
    pub fn job_logs(&self) -> JobLogsApi {
        Resource::new(self.client.clone(), "extras/job-logs/")
    }

    /// returns the job queue assignments resource.
    pub fn job_queue_assignments(&self) -> JobQueueAssignmentsApi {
        Resource::new(self.client.clone(), "extras/job-queue-assignments/")
    }

    /// returns the job queues resource.
    pub fn job_queues(&self) -> JobQueuesApi {
        Resource::new(self.client.clone(), "extras/job-queues/")
    }

    /// returns the job results resource.
    pub fn job_results(&self) -> JobResultsApi {
        Resource::new(self.client.clone(), "extras/job-results/")
    }

    /// returns the jobs resource.
    pub fn jobs(&self) -> JobsApi {
        Resource::new(self.client.clone(), "extras/jobs/")
    }

    /// returns the metadata choices resource.
    pub fn metadata_choices(&self) -> MetadataChoicesApi {
        Resource::new(self.client.clone(), "extras/metadata-choices/")
    }

    /// returns the metadata types resource.
    pub fn metadata_types(&self) -> MetadataTypesApi {
        Resource::new(self.client.clone(), "extras/metadata-types/")
    }

    /// returns the notes resource.
    pub fn notes(&self) -> NotesApi {
        Resource::new(self.client.clone(), "extras/notes/")
    }

    /// returns the object changes resource.
    pub fn object_changes(&self) -> ObjectChangesApi {
        Resource::new(self.client.clone(), "extras/object-changes/")
    }

    /// returns the object metadata resource.
    pub fn object_metadata(&self) -> ObjectMetadataApi {
        Resource::new(self.client.clone(), "extras/object-metadata/")
    }

    /// returns the relationship associations resource.
    pub fn relationship_associations(&self) -> RelationshipAssociationsApi {
        Resource::new(self.client.clone(), "extras/relationship-associations/")
    }

    /// returns the relationships resource.
    pub fn relationships(&self) -> RelationshipsApi {
        Resource::new(self.client.clone(), "extras/relationships/")
    }

    /// returns the roles resource.
    pub fn roles(&self) -> RolesApi {
        Resource::new(self.client.clone(), "extras/roles/")
    }

    /// returns the saved views resource.
    pub fn saved_views(&self) -> SavedViewsApi {
        Resource::new(self.client.clone(), "extras/saved-views/")
    }

    /// returns the scheduled jobs resource.
    pub fn scheduled_jobs(&self) -> ScheduledJobsApi {
        Resource::new(self.client.clone(), "extras/scheduled-jobs/")
    }

    /// returns the secrets groups associations resource.
    pub fn secrets_groups_associations(&self) -> SecretsGroupsAssociationsApi {
        Resource::new(self.client.clone(), "extras/secrets-groups-associations/")
    }

    /// returns the secrets groups resource.
    pub fn secrets_groups(&self) -> SecretsGroupsApi {
        Resource::new(self.client.clone(), "extras/secrets-groups/")
    }

    /// returns the secrets resource.
    pub fn secrets(&self) -> SecretsApi {
        Resource::new(self.client.clone(), "extras/secrets/")
    }

    /// returns the static group associations resource.
    pub fn static_group_associations(&self) -> StaticGroupAssociationsApi {
        Resource::new(self.client.clone(), "extras/static-group-associations/")
    }

    /// returns the statuses resource.
    pub fn statuses(&self) -> StatusesApi {
        Resource::new(self.client.clone(), "extras/statuses/")
    }

    /// returns the tags resource.
    pub fn tags(&self) -> TagsApi {
        Resource::new(self.client.clone(), "extras/tags/")
    }

    /// returns the teams resource.
    pub fn teams(&self) -> TeamsApi {
        Resource::new(self.client.clone(), "extras/teams/")
    }

    /// returns the user saved view associations resource.
    pub fn user_saved_view_associations(&self) -> UserSavedViewAssociationsApi {
        Resource::new(self.client.clone(), "extras/user-saved-view-associations/")
    }

    /// returns the webhooks resource.
    pub fn webhooks(&self) -> WebhooksApi {
        Resource::new(self.client.clone(), "extras/webhooks/")
    }

    /// download a file proxy payload by id.
    pub async fn file_proxy_download(&self, id: &str) -> Result<Vec<u8>> {
        let url = self
            .client
            .config()
            .build_url(&format!("extras/file-proxies/{}/download/", id))?;
        let response = self.client.http_client().get(url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.bytes().await.map_err(Error::from)?.to_vec())
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(Error::from_response(status, body))
        }
    }

    /// sync a git repository by id.
    pub async fn git_repository_sync(&self, id: &str) -> Result<GitRepositorySyncResponse> {
        self.client
            .post(
                &format!("extras/git-repositories/{}/sync/", id),
                &serde_json::json!({}),
            )
            .await
    }

    /// run a stored graphql query by id.
    pub async fn graphql_query_run(
        &self,
        id: &str,
        request: &GraphQlQueryInputRequest,
    ) -> Result<GraphQlQueryOutput> {
        self.client
            .post(&format!("extras/graphql-queries/{}/run/", id), request)
            .await
    }

    /// fetch job result logs by id.
    pub async fn job_result_logs(&self, id: &str) -> Result<JobResult> {
        self.client
            .get(&format!("extras/job-results/{}/logs/", id))
            .await
    }

    /// run a job by id.
    pub async fn job_run(&self, id: &str, request: &JobInputRequest) -> Result<JobRunResponse> {
        self.client
            .post(&format!("extras/jobs/{}/run/", id), request)
            .await
    }

    /// run a job by name.
    pub async fn job_run_by_name(
        &self,
        name: &str,
        request: &JobInputRequest,
    ) -> Result<JobRunResponse> {
        self.client
            .post(&format!("extras/jobs/{}/run/", name), request)
            .await
    }

    /// fetch a job by name.
    pub async fn job_by_name(&self, name: &str) -> Result<Job> {
        self.client.get(&format!("extras/jobs/{}/", name)).await
    }

    /// fetch job notes by name.
    pub async fn job_notes_by_name(
        &self,
        name: &str,
        query: Option<QueryBuilder>,
    ) -> Result<Page<Note>> {
        let query = query.unwrap_or_default();
        self.client
            .get_with_params(&format!("extras/jobs/{}/notes/", name), &query)
            .await
    }

    /// create a job note by name.
    pub async fn create_job_note_by_name(
        &self,
        name: &str,
        request: &NoteInputRequest,
    ) -> Result<Note> {
        self.client
            .post(&format!("extras/jobs/{}/notes/", name), request)
            .await
    }

    /// fetch job variables by id.
    pub async fn job_variables(&self, id: &str) -> Result<crate::models::PaginatedJobVariableList> {
        self.client
            .get(&format!("extras/jobs/{}/variables/", id))
            .await
    }

    /// fetch job variables by name.
    pub async fn job_variables_by_name(
        &self,
        name: &str,
    ) -> Result<crate::models::PaginatedJobVariableList> {
        self.client
            .get(&format!("extras/jobs/{}/variables/", name))
            .await
    }

    /// approve a scheduled job.
    pub async fn scheduled_job_approve(&self, id: &str) -> Result<ScheduledJob> {
        self.client
            .post(
                &format!("extras/scheduled-jobs/{}/approve/", id),
                &serde_json::json!({}),
            )
            .await
    }

    /// deny a scheduled job.
    pub async fn scheduled_job_deny(&self, id: &str) -> Result<ScheduledJob> {
        self.client
            .post(
                &format!("extras/scheduled-jobs/{}/deny/", id),
                &serde_json::json!({}),
            )
            .await
    }

    /// dry-run a scheduled job.
    pub async fn scheduled_job_dry_run(&self, id: &str) -> Result<JobResult> {
        self.client
            .post(
                &format!("extras/scheduled-jobs/{}/dry-run/", id),
                &serde_json::json!({}),
            )
            .await
    }

    /// check that a secret's value is accessible.
    pub async fn secret_check(&self, id: &str) -> Result<SecretsCheckResponse> {
        self.client
            .get(&format!("extras/secrets/{}/check/", id))
            .await
    }

    /// resolve a dynamic group membership snapshot.
    pub async fn dynamic_group_members(&self, id: &str) -> Result<DynamicGroup> {
        self.client
            .get(&format!("extras/dynamic-groups/{}/members/", id))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::GET, Method::POST, MockServer};
    use serde_json::json;

    fn test_client() -> Client {
        let config = ClientConfig::new("https://nautobot.example.com", "token");
        Client::new(config).unwrap()
    }

    fn assert_path<T>(resource: Resource<T>, expected: &str)
    where
        T: serde::de::DeserializeOwned,
    {
        let paginator = resource.paginate(None).unwrap();
        assert_eq!(paginator.next_url(), Some(expected));
    }

    #[test]
    fn extras_accessors_return_expected_paths() {
        let api = ExtrasApi::new(test_client());

        assert_path(api.computed_fields(), "extras/computed-fields/");
        assert_path(api.config_context_schemas(), "extras/config-context-schemas/");
        assert_path(api.config_contexts(), "extras/config-contexts/");
        assert_path(api.contact_associations(), "extras/contact-associations/");
        assert_path(api.contacts(), "extras/contacts/");
        assert_path(api.content_types(), "extras/content-types/");
        assert_path(api.custom_field_choices(), "extras/custom-field-choices/");
        assert_path(api.custom_fields(), "extras/custom-fields/");
        assert_path(api.custom_links(), "extras/custom-links/");
        assert_path(
            api.dynamic_group_memberships(),
            "extras/dynamic-group-memberships/",
        );
        assert_path(api.dynamic_groups(), "extras/dynamic-groups/");
        assert_path(api.export_templates(), "extras/export-templates/");
        assert_path(api.external_integrations(), "extras/external-integrations/");
        assert_path(api.file_proxies(), "extras/file-proxies/");
        assert_path(api.git_repositories(), "extras/git-repositories/");
        assert_path(api.graphql_queries(), "extras/graphql-queries/");
        assert_path(api.image_attachments(), "extras/image-attachments/");
        assert_path(api.job_buttons(), "extras/job-buttons/");
        assert_path(api.job_hooks(), "extras/job-hooks/");
        assert_path(api.job_logs(), "extras/job-logs/");
        assert_path(
            api.job_queue_assignments(),
            "extras/job-queue-assignments/",
        );
        assert_path(api.job_queues(), "extras/job-queues/");
        assert_path(api.job_results(), "extras/job-results/");
        assert_path(api.jobs(), "extras/jobs/");
        assert_path(api.metadata_choices(), "extras/metadata-choices/");
        assert_path(api.metadata_types(), "extras/metadata-types/");
        assert_path(api.notes(), "extras/notes/");
        assert_path(api.object_changes(), "extras/object-changes/");
        assert_path(api.object_metadata(), "extras/object-metadata/");
        assert_path(
            api.relationship_associations(),
            "extras/relationship-associations/",
        );
        assert_path(api.relationships(), "extras/relationships/");
        assert_path(api.roles(), "extras/roles/");
        assert_path(api.saved_views(), "extras/saved-views/");
        assert_path(api.scheduled_jobs(), "extras/scheduled-jobs/");
        assert_path(
            api.secrets_groups_associations(),
            "extras/secrets-groups-associations/",
        );
        assert_path(api.secrets_groups(), "extras/secrets-groups/");
        assert_path(api.secrets(), "extras/secrets/");
        assert_path(
            api.static_group_associations(),
            "extras/static-group-associations/",
        );
        assert_path(api.statuses(), "extras/statuses/");
        assert_path(api.tags(), "extras/tags/");
        assert_path(api.teams(), "extras/teams/");
        assert_path(
            api.user_saved_view_associations(),
            "extras/user-saved-view-associations/",
        );
        assert_path(api.webhooks(), "extras/webhooks/");
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn extras_helpers_hit_expected_paths() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = ExtrasApi::new(client);

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/extras/file-proxies/123/download/");
            then.status(200).body("blob");
        });
        server.mock(|when, then| {
            when.method(POST)
                .path("/api/extras/git-repositories/123/sync/");
            then.status(200).json_body(json!({}));
        });
        server.mock(|when, then| {
            when.method(POST)
                .path("/api/extras/graphql-queries/123/run/");
            then.status(200)
                .json_body(json!({ "data": { "ok": true } }));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/extras/job-results/123/logs/");
            then.status(200).json_body(json!({ "name": "job" }));
        });
        server.mock(|when, then| {
            when.method(POST).path("/api/extras/jobs/123/run/");
            then.status(200).json_body(json!({}));
        });
        server.mock(|when, then| {
            when.method(POST).path("/api/extras/jobs/my-job/run/");
            then.status(200).json_body(json!({}));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/extras/jobs/my-job/");
            then.status(200).json_body(json!({
                "grouping": "group",
                "name": "my-job",
                "default_job_queue": {}
            }));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/extras/jobs/my-job/notes/");
            then.status(200)
                .json_body(json!({ "count": 0, "next": null, "previous": null, "results": [] }));
        });
        server.mock(|when, then| {
            when.method(POST)
                .path("/api/extras/jobs/my-job/notes/")
                .json_body(json!({ "note": "note" }));
            then.status(201).json_body(json!({
                "assigned_object_type": "extras.job",
                "assigned_object_id": "00000000-0000-0000-0000-000000000000",
                "note": "note"
            }));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/extras/jobs/123/variables/");
            then.status(200)
                .json_body(json!({ "count": 0, "next": null, "previous": null, "results": [] }));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/extras/jobs/my-job/variables/");
            then.status(200)
                .json_body(json!({ "count": 0, "next": null, "previous": null, "results": [] }));
        });
        server.mock(|when, then| {
            when.method(POST)
                .path("/api/extras/scheduled-jobs/123/approve/");
            then.status(200).json_body(json!({
                "name": "scheduled",
                "task": "task",
                "interval": "immediately",
                "start_time": "2024-01-01T00:00:00Z"
            }));
        });
        server.mock(|when, then| {
            when.method(POST)
                .path("/api/extras/scheduled-jobs/123/deny/");
            then.status(200).json_body(json!({
                "name": "scheduled",
                "task": "task",
                "interval": "immediately",
                "start_time": "2024-01-01T00:00:00Z"
            }));
        });
        server.mock(|when, then| {
            when.method(POST)
                .path("/api/extras/scheduled-jobs/123/dry-run/");
            then.status(200).json_body(json!({ "name": "job" }));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/extras/secrets/123/check/");
            then.status(200)
                .json_body(json!({ "result": true, "message": "ok" }));
        });
        server.mock(|when, then| {
            when.method(GET)
                .path("/api/extras/dynamic-groups/123/members/");
            then.status(200)
                .json_body(json!({ "content_type": "dcim.device", "name": "group" }));
        });

        let blob = api.file_proxy_download("123").await.unwrap();
        assert_eq!(blob, b"blob".to_vec());

        let _ = api.git_repository_sync("123").await.unwrap();

        let gql = api
            .graphql_query_run("123", &GraphQlQueryInputRequest::new())
            .await
            .unwrap();
        assert_eq!(gql.data.unwrap().get("ok"), Some(&json!(true)));

        let logs = api.job_result_logs("123").await.unwrap();
        assert_eq!(logs.name, "job");

        let _ = api.job_run("123", &JobInputRequest::new()).await.unwrap();
        let _ = api
            .job_run_by_name("my-job", &JobInputRequest::new())
            .await
            .unwrap();
        let job = api.job_by_name("my-job").await.unwrap();
        assert_eq!(job.name, "my-job");
        let notes = api.job_notes_by_name("my-job", None).await.unwrap();
        assert!(notes.results.is_empty());
        let note = api
            .create_job_note_by_name("my-job", &NoteInputRequest::new("note".to_string()))
            .await
            .unwrap();
        assert_eq!(note.note, "note");

        let vars = api.job_variables("123").await.unwrap();
        assert_eq!(vars.count, 0);
        let vars = api.job_variables_by_name("my-job").await.unwrap();
        assert_eq!(vars.count, 0);

        let approved = api.scheduled_job_approve("123").await.unwrap();
        assert_eq!(approved.name, "scheduled");
        let denied = api.scheduled_job_deny("123").await.unwrap();
        assert_eq!(denied.name, "scheduled");
        let dry_run = api.scheduled_job_dry_run("123").await.unwrap();
        assert_eq!(dry_run.name, "job");

        let check = api.secret_check("123").await.unwrap();
        assert_eq!(check.result, Some(true));

        let members = api.dynamic_group_members("123").await.unwrap();
        assert_eq!(members.name, "group");
    }
}
