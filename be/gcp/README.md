# Google Cloud Run deployment

## For the first time

After having created a project in Google Cloud and enabled billing for it, make sure that Identity and Access Management (IAM) API and Cloud Run API are enabled. Create a new service account, e.g. with a name `terraform-deploy-account`, and give it the following permissions:

- Cloud Build Service Account
- Cloud Run Admin
- Project IAM Admin
- Service Account Admin
- Service Account User
- Storage Object Admin

When the new service account has been created, create a key for it (service account credentials). This service account will be used to create the cloud infrastructure.

Furthermore, create a bucket in Cloud Storage that matches the backend bucket name in the `main.tf`. Set it to be private and not available to the public internet. The bucket name must also be changed if the deployment is targeted for a new environment (i.e., practically always when following these instructions).

## Terraform local

Deployment commands can be run locally, within this `gcp` directory. Remove the old Terraform state (under .terraform) if needed, but normally this shouldn't be necessary.

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

GitHub workflow `deploy-be.yml` uploads a new revision to Cloud Run for every versioned push event to the main branch. For this pipeline, `GOOGLE_PROJECT_ID` (simply project's id), `GOOGLE_SERVICE_NAME` (just some descriptive name), `GOOGLE_SERVICE_ACCOUNT_FILE` (service account credentials), and `CLIENT_URL` (frontend service production URL) must have been stored as secrets in GitHub's Actions secrets.
