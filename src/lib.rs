#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Visibility {
    Internal,
    Private,
    Public,
    #[serde(untagged)]
    Other(String),
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum RunnerEnvironment {
    GithubHosted,
    SelfHosted,
    #[serde(untagged)]
    Other(String),
}

/// Based on
/// https://web.archive.org/web/20230602040457/https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect#understanding-the-oidc-token
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct Claims {
    // Mandatory(?) standard claims
    /// Audience
    /// By default, this is the URL of the repository owner, such as the organization that owns the repository. This is the only claim that can be customized. You can set a custom audience with a toolkit command: core.getIDToken(audience)
    pub aud: String,
    /// Issuer
    /// The issuer of the OIDC token: https://token.actions.githubusercontent.com
    pub iss: String,
    /// Subject
    /// Defines the subject claim that is to be validated by the cloud provider. This setting is essential for making sure that access tokens are only allocated in a predictable way.
    pub sub: String,

    /// Expires at
    /// Identifies the expiry time of the JWT.
    pub exp: f64,
    /// Issued at
    /// The time when the JWT was issued.
    pub iat: f64,
    /// JWT token identifier
    /// Unique identifier for the OIDC token.
    pub jti: String,
    /// Not before
    /// JWT is not valid for use before this time.
    pub nbf: f64,

    // GitHub-specific claims
    /// The personal account that initiated the workflow run.
    pub actor: String,
    /// The ID of personal account that initiated the workflow run.
    pub actor_id: String,
    /// The target branch of the pull request in a workflow run.
    pub base_ref: String,
    /// The name of the environment used by the job. To include the environment claim you must reference an environment.
    pub environment: Option<String>,
    /// The name of the event that triggered the workflow run.
    pub event_name: String,
    /// The source branch of the pull request in a workflow run.
    pub head_ref: String,
    /// For jobs using a reusable workflow, the ref path to the reusable workflow. For more information, see "Using OpenID Connect with reusable workflows."
    pub job_workflow_ref: Option<String>,
    /// For jobs using a reusable workflow, the commit SHA for the reusable workflow file.
    pub job_workflow_sha: Option<String>,
    /// (Reference) The git ref that triggered the workflow run.
    /// Called "ref" in the raw claim, but we can't use that because it's a Rust keyword.
    #[serde(rename = "ref")]
    pub git_ref: String,
    /// The type of ref, for example: "branch".
    pub ref_type: String,
    /// The visibility of the repository where the workflow is running. Accepts the following values: internal, private, or public.
    pub repository_visibility: Visibility,
    /// The repository from where the workflow is running.
    pub repository: String,
    /// The ID of the repository from where the workflow is running.
    pub repository_id: String,
    /// The name of the organization in which the repository is stored.
    pub repository_owner: String,
    /// The ID of the organization in which the repository is stored.
    pub repository_owner_id: String,
    /// The ID of the workflow run that triggered the workflow.
    pub run_id: String,
    /// The number of times this workflow has been run.
    pub run_number: String,
    /// The number of times this workflow run has been retried.
    pub run_attempt: String,
    /// The type of runner used by the job. Accepts the following values: github-hosted or self-hosted.
    pub runner_environment: RunnerEnvironment,
    /// The name of the workflow.
    pub workflow: String,
    /// The ref path to the workflow. For example, octocat/hello-world/.github/workflows/my-workflow.yml@refs/heads/my_branch.
    pub workflow_ref: String,
    /// The commit SHA for the workflow file.
    pub workflow_sha: String,
}

impl Claims {
    /// Fill in all the fields of a claim set.
    /// May be useful for testing, but does not resemble GitHub-issued tokens very closely, and the fields are likely to require further adjustment by the caller.
    pub fn make_dummy() -> Self {
        Self {
            aud: "".into(),
            iss: "".into(),
            sub: "".into(),
            exp: 33247274880f64,
            iat: 1690366107f64,
            jti: "".into(),
            nbf: 1690366107f64,
            actor: "".into(),
            actor_id: "".into(),
            base_ref: "".into(),
            environment: None,
            event_name: "".into(),
            head_ref: "".into(),
            job_workflow_ref: None,
            job_workflow_sha: None,
            git_ref: "refs/heads/main".into(),
            ref_type: "branch".into(),
            repository_visibility: Visibility::Public,
            repository: "".into(),
            repository_id: "".into(),
            repository_owner: "".into(),
            repository_owner_id: "".into(),
            run_id: "".into(),
            run_number: "".into(),
            run_attempt: "1".into(),
            runner_environment: RunnerEnvironment::GithubHosted,
            workflow: "".into(),
            workflow_ref: "".into(),
            workflow_sha: "".into(),
        }
    }
}
