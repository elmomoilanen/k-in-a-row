# Google Cloud Run deployment

## For the first time

After having created a project in Google Cloud and enabled billing for it, make sure that APIs IAM, Artifact Registry and Cloud Run are enabled. Create a new service account, e.g. with a name `terraform-deploy-account`, and give it the following roles:

- Cloud Build Service Account
- Cloud Run Admin
- Project IAM Admin
- Service Account Admin
- Service Account User
- Storage Object Admin
- Artifact Registry Writer

After the first deployment, excess permissions (suggested by Google) can be removed from this service account.

When the new service account has been created, create a key for it (service account credentials). This service account will be used to create the cloud infrastructure.

Create a repository in Artifact Registry for this project and make sure that its name aligns with Terraform variables.

Furthermore, create a bucket in Cloud Storage that matches the backend bucket name in the `main.tf`. Set it to private and thus not available to the public internet. The bucket name must also be changed if the deployment is targeted for a new environment (i.e., practically always when following these instructions).

## Terraform local

Deployment commands can be run locally, within this `gcp` directory, although following instructions doesn't cover the mandatory step of pushing the backend image to Google Artifact Registry.

File `variables.tf` lists required Terraform variables. Make sure that insensitive variables are listed in `environments/prod.tfvars` and others saved as environment variables (e.g., client_url must be saved as TF_VAR_client_url). Service account credentials must also be available in the current shell environment similarly to the following example

```bash
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account-credentials-file
```

Do not include this file in version control!

Run Terraform commands in the following order, skipping the initialisation step if it has been done earlier

```bash
terraform init
```

and plan the deployment with

```bash
terraform plan -input=false -var-file=environments/prod.tfvars
```

and if satisfied with the plan results, complete the deployment with

```bash
terraform apply -input=false -auto-approve -var-file=environments/prod.tfvars
```

Now wait for the cloud infrastructure to be created. On success, file `outputs.tf` determines what of the results will be printed to the shell (e.g., URL of the Cloud Run service).

## Terraform pipeline

GitHub workflow `deploy-be.yml` uploads a new revision to Google Cloud Run for every versioned push event to the main branch. For this pipeline to succeed following variables must have been stored as secrets in GitHub's Actions secrets (notice that only some of them are really critically sensitive):

- GOOGLE_PROJECT_ID
- GOOGLE_REGION
- GOOGLE_IMAGE_NAME
- GOOGLE_SERVICE_NAME (just some descriptive name)
- CLIENT_URL (frontend service prod URL)
- GOOGLE_SERVICE_ACCOUNT_FILE (service account credentials, sensitive)

The first five of these are listed in `variables.tf` and some may have values defined under the folder `environments`. Make sure that these correspond with the values stored in GitHub's Actions secrets.
