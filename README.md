# Terraform provider workshop

This repository contains the code for the Terraform provider workshop.


Steps:

1. Setup local environment: make changes in `.terraformrc` (link to docs).
2. Create a new shell, and start the API: `make api`.
3. Complete the provider implementation for the `chapter` resource. Have a look at the `user` resource for inspiration.
4. Complete the provider implementaiton for the `chatper_member` resource.
5. (Optional) complete the provider implementation for the provider-defined function `parse_chapter_config`. 

You can test your implementation on the infrastructure defined in `main.tf`. 