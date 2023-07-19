use serde;

// Based on
// https://web.archive.org/web/20230602040457/https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect#understanding-the-oidc-token
#[derive(serde::Serialize, serde::Deserialize)]
struct Claim {
    // Mandatory(?) standard claims
    /// Audience
    /// By default, this is the URL of the repository owner, such as the organization that owns the repository. This is the only claim that can be customized. You can set a custom audience with a toolkit command: core.getIDToken(audience)
    aud: String,
    /// Issuer
    /// The issuer of the OIDC token: https://token.actions.githubusercontent.com
    iss: String,
    /// Subject
    /// Defines the subject claim that is to be validated by the cloud provider. This setting is essential for making sure that access tokens are only allocated in a predictable way.
    sub: String,

    // Additional standard claims
    /// Algorithm
    /// The algorithm used by the OIDC provider.
    alg: String,
    /// Expires at
    /// Identifies the expiry time of the JWT.
    exp: String,
    /// Issued at
    /// The time when the JWT was issued.
    iat: String,
    /// JWT token identifier
    /// Unique identifier for the OIDC token.
    jti: String,
    /// Key identifier
    /// Unique key for the OIDC token.
    kid: String,
    /// Not before
    /// JWT is not valid for use before this time.
    nbf: String,
    /// Type
    /// Describes the type of token. This is a JSON Web Token (JWT).
    typ: String,

    // GitHub-specific claims
    /// The personal account that initiated the workflow run.
    actor: String,
    /// The ID of personal account that initiated the workflow run.
    actor_id: String,
    /// The target branch of the pull request in a workflow run.
    base_ref: String,
    /// The name of the environment used by the job. To include the environment claim you must reference an environment.
    environment: String,
    /// The name of the event that triggered the workflow run.
    event_name: String,
    /// The source branch of the pull request in a workflow run.
    head_ref: String,
    /// For jobs using a reusable workflow, the ref path to the reusable workflow. For more information, see "Using OpenID Connect with reusable workflows."
    job_workflow_ref: String,
    /// For jobs using a reusable workflow, the commit SHA for the reusable workflow file.
    job_workflow_sha: String,
    /// (Reference) The git ref that triggered the workflow run.
    /// Called "ref" in the raw claim, but we can't use that because it's a Rust keyword.
    #[serde(rename="ref")]
    git_ref: String,
    /// The type of ref, for example: "branch".
    ref_type: String,
    /// The visibility of the repository where the workflow is running. Accepts the following values: internal, private, or public.
    repository_visibility: String,
    /// The repository from where the workflow is running.
    repository: String,
    /// The ID of the repository from where the workflow is running.
    repository_id: String,
    /// The name of the organization in which the repository is stored.
    repository_owner: String,
    /// The ID of the organization in which the repository is stored.
    repository_owner_id: String,
    /// The ID of the workflow run that triggered the workflow.
    run_id: String,
    /// The number of times this workflow has been run.
    run_number: String,
    /// The number of times this workflow run has been retried.
    run_attempt: String,
    /// The type of runner used by the job. Accepts the following values: github-hosted or self-hosted.
    runner_environment: String,
    /// The name of the workflow.
    workflow: String,
    /// The ref path to the workflow. For example, octocat/hello-world/.github/workflows/my-workflow.yml@refs/heads/my_branch.
    workflow_ref: String,
    /// The commit SHA for the workflow file.
    workflow_sha: String,
}
