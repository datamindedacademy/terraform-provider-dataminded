# Build a Terraform provider from scratch

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/datamindedacademy/terraform-provider-dataminded)

📚 A workshop brought to you by the [Data Minded Academy].

Every Terraform resource you have ever written was served by a provider: a plugin that
translates HCL into API calls and keeps the result in state. Most engineers use providers
for years without ever opening one up. This workshop has you write one.

You implement a provider for the `dataminded` API, a small service that manages users,
chapters, and the membership between them. You start from a working `user` resource and
build the rest yourself.

## What you will build

By the end you will have written:

- **Resource schemas** that declare what your HCL is allowed to say
- **The full CRUD lifecycle**, including the drift detection that makes `terraform plan`
  meaningful
- **Attribute validation** with optional attributes and default values
- **A provider-defined function**, the Terraform 1.8+ feature that lets a provider ship
  its own HCL functions

Each piece is verified by Terraform's acceptance testing framework, so you get a real
signal on whether your implementation is correct.

## Prerequisites

Nothing, if you use Codespaces. Click the badge above and skip to
[Getting started](#getting-started).

To work locally you need:

| Tool | Why |
| --- | --- |
| [Go](https://go.dev/dl/) 1.25+ | The provider is written in Go |
| [Terraform](https://developer.hashicorp.com/terraform/install) 1.8+ | Provider-defined functions need 1.8 |
| [Docker](https://docs.docker.com/get-docker/) | Runs the `dataminded` API |

## Getting started

**1. Build the provider and point Terraform at it.**

```bash
make dev
```

Terraform normally downloads providers from a registry. Yours only exists on your machine,
so `make dev` compiles it and writes a `.terraformrc` containing a
[`dev_overrides`](https://developer.hashicorp.com/terraform/cli/config/config-file#development-overrides-for-provider-developers)
block that redirects `hashicorp.com/dev/dataminded` to your local binary.

Export the variable that tells Terraform to read that file (Codespaces does this for you):

```bash
export TF_CLI_CONFIG_FILE=$PWD/.terraformrc
```

**2. Start the API** in a second terminal, and leave it running:

```bash
make api
```

It listens on `http://localhost:3000`, which serves interactive OpenAPI docs for the
endpoints you are wrapping. The raw spec is at `/api.json`.

**3. Check the wiring.**

```bash
terraform plan
```

Terraform warns that development overrides are in effect, which is exactly what you want.
It then fails on the unimplemented `chapter` resource. That is your first exercise.

> With `dev_overrides` you do not run `terraform init`, and you should not commit a lock
> file. Run `terraform plan` and `terraform apply` directly.

## The exercises

Work through them in order. Each has its own README with detailed instructions.

| # | Exercise | You implement | Instructions |
| - | --- | --- | --- |
| 1 | **Chapter resource** | Schema plus Create, Read, Update, Delete | [`internal/services/chapter`](internal/services/chapter/README.md) |
| 2 | **Chapter member resource** | The same, with a validated optional `role` attribute | [`internal/services/chapter_member`](internal/services/chapter_member/README.md) |
| 3 | **Provider-defined function** (optional) | `chapter_config_parser`, replacing some ugly HCL | [`internal/services/functions`](internal/services/functions/README.md) |

The `user` resource in [`internal/services/user`](internal/services/user) is already
complete. Read it first: exercises 1 and 2 follow the same shape.

Verify your work with the acceptance tests:

```bash
make testacc
```

Or exercise the whole thing against the infrastructure in [`main.tf`](main.tf):

```bash
terraform apply
```

## How the pieces fit together

```
main.tf                      your HCL, describing chapters and members
   |
   v
internal/provider/           wires resources and functions into the provider
   |
   v
internal/services/<name>/    the resource you implement: schema + CRUD
   |
   v
internal/dataminded_api/     ready-made HTTP client, no work needed here
   |
   v
api/                         the Rust service the provider talks to
```

You spend your time in `internal/services/`. The API client in `internal/dataminded_api/`
is written for you, so you can focus on the provider framework rather than on HTTP
plumbing.

## Troubleshooting

**`Provider registry.terraform.io/hashicorp/dataminded was not found`**
`TF_CLI_CONFIG_FILE` is not set, or points at the wrong file. Re-run `export
TF_CLI_CONFIG_FILE=$PWD/.terraformrc` in the shell you are running Terraform from.

**`connection refused` on port 3000**
The API is not running. Start it with `make api` in a separate terminal.

**You moved the repo, or changed `GOBIN`**
The generated `.terraformrc` holds an absolute path. Regenerate it:

```bash
make clean dev
```

## Reference

- [Terraform plugin framework docs](https://developer.hashicorp.com/terraform/plugin/framework)
- [Provider-defined functions](https://developer.hashicorp.com/terraform/plugin/framework/functions)
- [Acceptance testing](https://developer.hashicorp.com/terraform/plugin/testing/acceptance-tests)
- Generated provider docs live in [`docs/`](docs/)

[Data Minded Academy]: https://www.dataminded.academy/
