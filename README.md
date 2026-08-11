# Implementing a Terraform provider

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/datamindedacademy/terraform-provider-dataminded)

📚 A workshop brought to you by the [Dataminded Academy].

A Terraform provider is the plugin layer that translates declarative configuration into
API calls and reconciles the result with state. This workshop examines that layer by
implementing one.

The subject is the `dataminded` API, a service managing users, chapters, and chapter
membership. A complete `user` resource is supplied as a reference implementation; the
remaining resources are yours to write. In doing so you will encounter resource schemas,
the CRUD lifecycle and the drift detection that gives `terraform plan` its meaning,
attribute validation, and provider-defined functions. Each is verified through Terraform's
acceptance testing framework.

## Prerequisites

Codespaces requires nothing beyond the badge above. A local setup requires
[Go](https://go.dev/dl/) 1.25+, [Terraform](https://developer.hashicorp.com/terraform/install)
1.8+ (provider-defined functions were introduced in 1.8), and
[Docker](https://docs.docker.com/get-docker/) to run the API.

## Getting started

Compile the provider and direct Terraform to the resulting binary:

```bash
make build
export TF_CLI_CONFIG_FILE=$PWD/.terraformrc
```

Terraform ordinarily retrieves providers from a registry. This one exists only locally, so
the committed [`.terraformrc`](.terraformrc) declares a
[`dev_overrides`](https://developer.hashicorp.com/terraform/cli/config/config-file#development-overrides-for-provider-developers)
block redirecting `hashicorp.com/dev/dataminded` to the `./bin` directory that `make build`
writes to. That path is relative, so Terraform must be invoked from the repository root.

Start the API in a second terminal and leave it running:

```bash
make api
```

It listens on `http://localhost:3000` and serves interactive OpenAPI documentation for the
endpoints under consideration; the specification itself is at `/api.json`. Should the port
be occupied, use `make api PORT=3001` and set `port = 3001` in the `provider` block of
`main.tf`.

Verify the configuration with `terraform plan`. Terraform reports that development
overrides are in effect, which is expected, then fails on the unimplemented `chapter`
resource. That failure is the first exercise.

Note that `dev_overrides` bypasses `terraform init` and produces no lock file, so
`terraform plan` and `terraform apply` are invoked directly. Because the override resolves
to a compiled binary rather than to source, `make build` must be re-run after each change.

## Exercises

| # | Exercise | Subject | Instructions |
| - | --- | --- | --- |
| 1 | Chapter resource | Schema and the CRUD lifecycle | [`internal/services/chapter`](internal/services/chapter/README.md) |
| 2 | Chapter member resource | The same, with a validated optional `role` attribute | [`internal/services/chapter_member`](internal/services/chapter_member/README.md) |
| 3 | Provider-defined function (optional) | `chapter_config_parser`, replacing the equivalent HCL | [`internal/services/functions`](internal/services/functions/README.md) |

The completed `user` resource in [`internal/services/user`](internal/services/user) is
worth reading first, as exercises 1 and 2 follow its structure. Verify your work with
`make testacc`, or apply it to the configuration in [`main.tf`](main.tf).

## Repository structure

Implementation work is confined to `internal/services/`. The HTTP client in
`internal/dataminded_api/` is supplied, so that attention rests on the provider framework
rather than on request plumbing.

```
main.tf                      configuration describing chapters and members
internal/provider/           registers resources and functions with the provider
internal/services/<name>/    the resource under implementation: schema and CRUD
internal/dataminded_api/     supplied HTTP client
api/                         the service the provider addresses
```

## Reference

- [Terraform plugin framework](https://developer.hashicorp.com/terraform/plugin/framework)
- [Provider-defined functions](https://developer.hashicorp.com/terraform/plugin/framework/functions)
- [Acceptance testing](https://developer.hashicorp.com/terraform/plugin/testing/acceptance-tests)
- Generated provider documentation in [`docs/`](docs/)

[Dataminded Academy]: https://www.dataminded.academy/
