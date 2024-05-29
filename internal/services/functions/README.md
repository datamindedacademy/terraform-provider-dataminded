# Chapter Config Parser

Instead of using HCL Kung-Fu to flatten the yaml configuration of chapters and users and create the `chapter_member` resources, try to complete the provider-defined function `chapter_config_parser`. 

Steps to implement the provider-defined function:

1. Complete the `run` method, the most important method in the `Function` interface. The function expects a yaml-formatted string, which you can parse with the [`yaml`](https://pkg.go.dev/gopkg.in/yaml.v3) package. Then write Go code that translates the parsed yaml into a Terraform `function.MapReturn` value. The keys of the map should be a unique value that identifies the `chapter_member` in your Terraform code. The value of the map should be a `types.ObjectType` with two attributes: "name" and "role", which you will use to configure the `chapter_member` resource. 
2. Test your implementation by running `terraform plan` on the infrastructure defined in the `main.tf` file. 