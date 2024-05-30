// Copyright (c) HashiCorp, Inc.
// SPDX-License-Identifier: MPL-2.0

package functions

import (
	"context"

	"github.com/hashicorp/terraform-plugin-framework/function"
	"github.com/hashicorp/terraform-plugin-framework/types"
	"gopkg.in/yaml.v2"
)

var (
	_ function.Function = ConfigParser{}
)

func NewConfigParser() function.Function {
	return ConfigParser{}
}

type ConfigParser struct{}

func (r ConfigParser) Metadata(_ context.Context, req function.MetadataRequest, resp *function.MetadataResponse) {
	resp.Name = "chapter_config_parser"
}

func (r ConfigParser) Definition(_ context.Context, _ function.DefinitionRequest, resp *function.DefinitionResponse) {
	resp.Definition = function.Definition{
		Summary:             "Parse chapter configuration",
		MarkdownDescription: "Parse chapter configuration and return the parsed data as an object.",
		Parameters: []function.Parameter{
			function.StringParameter{
				Name:                "input",
				MarkdownDescription: "The input data to parse.",
			},
		},
		Return: function.MapReturn{
			ElementType: types.StringType,
		},
	}
}

func (r ConfigParser) Run(ctx context.Context, req function.RunRequest, resp *function.RunResponse) {
	var data string

	resp.Error = function.ConcatFuncErrors(req.Arguments.Get(ctx, &data))

	if resp.Error != nil {
		return
	}

	// This returns the arguments received via the req.Arguments.Get(ctx, &data) to the result. It's basically an echo function.
	// The types defined below can help you parse a yaml file into Go objects.
	resp.Error = function.ConcatFuncErrors(resp.Result.Set(ctx, data))
}

type ChapterMember struct {
	Name string `yaml:"name"`
	Role string `yaml:"role,omitempty"`
}

type ChapterConfig map[string][]ChapterMember

func parseChapterConfig(data string) (ChapterConfig, error) { //nolint:golint,unused
	var parsedConfig ChapterConfig

	// Parse the data into the ChapterConfig struct
	err := yaml.Unmarshal([]byte(data), &parsedConfig)
	if err != nil {
		return ChapterConfig{}, err
	}
	return parsedConfig, nil
}
