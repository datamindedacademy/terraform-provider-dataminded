// Copyright (c) HashiCorp, Inc.
// SPDX-License-Identifier: MPL-2.0

package functions

import (
	"context"
	"fmt"
	"strings"

	"github.com/hashicorp/terraform-plugin-framework/attr"
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

var roleParseResultAttrTypes = map[string]attr.Type{
	"name": types.StringType,
	"role": types.StringType,
}

var roleMapReturnType = types.ObjectType{
	AttrTypes: roleParseResultAttrTypes,
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
			ElementType: roleMapReturnType,
		},
	}
}

func (r ConfigParser) Run(ctx context.Context, req function.RunRequest, resp *function.RunResponse) {
	var data string

	resp.Error = function.ConcatFuncErrors(req.Arguments.Get(ctx, &data))

	if resp.Error != nil {
		return
	}

	parsedConfig, err := parseChapterConfig(data)
	if err != nil {
		resp.Error = function.ConcatFuncErrors(function.NewFuncError(err.Error()))
		return
	}

	value := make(map[string]attr.Value)
	for chapter, members := range parsedConfig {
		for _, member := range members {
			if member.Role == "" {
				member.Role = "Contributor"
			}
			roleValue := map[string]attr.Value{
				"name": types.StringValue(member.Name),
				"role": types.StringValue(member.Role),
			}
			val, err := types.ObjectValue(roleParseResultAttrTypes, roleValue)
			if err != nil {
				resp.Error = function.ConcatFuncErrors(function.FuncErrorFromDiags(ctx, err))
				return
			}

			value[fmt.Sprintf("%s-%s", strings.ToLower(chapter), strings.ToLower(member.Name))] = val

		}
	}

	returnValue, diag := types.MapValue(roleMapReturnType, value)

	if diag.HasError() {
		resp.Error = function.ConcatFuncErrors(function.FuncErrorFromDiags(ctx, diag))
		return
	}

	resp.Error = function.ConcatFuncErrors(resp.Result.Set(ctx, returnValue))
}

type ChapterMember struct {
	Name string `yaml:"name"`
	Role string `yaml:"role,omitempty"`
}

type ChapterConfig map[string][]ChapterMember

func parseChapterConfig(data string) (ChapterConfig, error) {
	var parsedConfig ChapterConfig

	// Parse the data into the ChapterConfig struct
	err := yaml.Unmarshal([]byte(data), &parsedConfig)
	if err != nil {
		return ChapterConfig{}, err
	}
	return parsedConfig, nil
}
