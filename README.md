# Terraform provider workshop

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/datamindedacademy/terraform-provider-dataminded)

This repository contains the code for the Terraform provider workshop. In this workshop, you will implement a Terraform provider plugin for the `dataminded` API. The API exposes three resources: users, chapters, and chapter members. 
## Step-by-Step Guide

The steps you need to complete are:

1. Set up your local environment: make changes in [`.terraformrc`](https://developer.hashicorp.com/terraform/cli/config/config-file#development-overrides-for-provider-developers) as described in the documentation.
2. Create a new shell, and start the API: `make api`.
3. Complete the provider implementation for the `chapter` resource. Have a look at the `user` resource for inspiration.
4. Complete the provider implementation for the `chapter_member` resource.
5. (Optional) complete the provider implementation for the provider-defined function `parse_chapter_config`. 

For each of the resources that you have to complete, a separate README.md is provided with more detailed instructions in the `internal/services` folder. 

You can test your implementation on the infrastructure defined in `main.tf`. 