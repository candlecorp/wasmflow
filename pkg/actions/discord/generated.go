// Code generated by @apexlang/codegen. DO NOT EDIT.

package discord

import (
	"github.com/nanobus/nanobus/pkg/actions"
	"github.com/nanobus/nanobus/pkg/expr"
	"github.com/nanobus/nanobus/pkg/resource"
)

type CodecRef string

// Sends a message to a Discord channel using it channel ID.
type SendMessageConfig struct {
	Resource  resource.Ref    `json:"resource" yaml:"resource" msgpack:"resource" mapstructure:"resource" validate:"required"`
	ChannelID *expr.ValueExpr `json:"channelId" yaml:"channelId" msgpack:"channelId" mapstructure:"channelId" validate:"required"`
	Content   *expr.ValueExpr `json:"content" yaml:"content" msgpack:"content" mapstructure:"content" validate:"required"`
}

func SendMessage() (string, actions.Loader) {
	return "@discord/send_message", SendMessageLoader
}
