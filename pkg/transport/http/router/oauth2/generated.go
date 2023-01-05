// Code generated by @apexlang/codegen. DO NOT EDIT.

package oauth2

import (
	"encoding/json"
	"fmt"

	"github.com/nanobus/nanobus/pkg/handler"
	"github.com/nanobus/nanobus/pkg/transport/http/router"
)

type OAuth2V1Config struct {
	LoginPath    string           `json:"loginPath" yaml:"loginPath" msgpack:"loginPath" mapstructure:"loginPath" validate:"required"`
	CallbackPath string           `json:"callbackPath" yaml:"callbackPath" msgpack:"callbackPath" mapstructure:"callbackPath" validate:"required"`
	ClientID     string           `json:"clientId" yaml:"clientId" msgpack:"clientId" mapstructure:"clientId" validate:"required"`
	ClientSecret string           `json:"clientSecret" yaml:"clientSecret" msgpack:"clientSecret" mapstructure:"clientSecret" validate:"required"`
	Endpoint     Endpoint         `json:"endpoint" yaml:"endpoint" msgpack:"endpoint" mapstructure:"endpoint"`
	CallbackURL  string           `json:"callbackUrl" yaml:"callbackUrl" msgpack:"callbackUrl" mapstructure:"callbackUrl" validate:"required"`
	RedirectURL  string           `json:"redirectUrl" yaml:"redirectUrl" msgpack:"redirectUrl" mapstructure:"redirectUrl" validate:"required"`
	CookieDomain string           `json:"cookieDomain" yaml:"cookieDomain" msgpack:"cookieDomain" mapstructure:"cookieDomain" validate:"required"`
	Scopes       []string         `json:"scopes,omitempty" yaml:"scopes,omitempty" msgpack:"scopes,omitempty" mapstructure:"scopes" validate:"dive"`
	Handler      *handler.Handler `json:"handler,omitempty" yaml:"handler,omitempty" msgpack:"handler,omitempty" mapstructure:"handler"`
}

func OAuth2V1() (string, router.Loader) {
	return "nanobus.transport.http.oauth2/v1", OAuth2V1Loader
}

type Endpoint struct {
	AuthURL     string `json:"authUrl" yaml:"authUrl" msgpack:"authUrl" mapstructure:"authUrl" validate:"required"`
	TokenURL    string `json:"tokenUrl" yaml:"tokenUrl" msgpack:"tokenUrl" mapstructure:"tokenUrl" validate:"required"`
	UserInfoURL string `json:"userInfoUrl" yaml:"userInfoUrl" msgpack:"userInfoUrl" mapstructure:"userInfoUrl" validate:"required"`
	// AuthStyle optionally specifies how the endpoint wants the client ID & client
	// secret sent.
	AuthStyle AuthStyle `json:"authStyle" yaml:"authStyle" msgpack:"authStyle" mapstructure:"authStyle"`
}

// AuthStyle represents how requests for tokens are authenticated to the server.
type AuthStyle int32

const (
	// AuthStyleAutoDetect means to auto-detect which authentication style the
	// provider wants by trying both ways and caching the successful way for the
	// future.
	AuthStyleAutoDetect AuthStyle = 0
	// AuthStyleInParams sends the "client_id" and "client_secret" in the POST body as
	// application/x-www-form-urlencoded parameters.
	AuthStyleInParams AuthStyle = 1
	// AuthStyleInHeader sends the client_id and client_password using HTTP Basic
	// Authorization. This is an optional style described in the OAuth2 RFC 6749
	// section 2.3.1.
	AuthStyleInHeader AuthStyle = 2
)

var toStringAuthStyle = map[AuthStyle]string{
	AuthStyleAutoDetect: "auto-detect",
	AuthStyleInParams:   "inparams",
	AuthStyleInHeader:   "inheader",
}

var toIDAuthStyle = map[string]AuthStyle{
	"auto-detect": AuthStyleAutoDetect,
	"inparams":    AuthStyleInParams,
	"inheader":    AuthStyleInHeader,
}

func (e AuthStyle) String() string {
	str, ok := toStringAuthStyle[e]
	if !ok {
		return "unknown"
	}
	return str
}

func (e *AuthStyle) FromString(str string) error {
	var ok bool
	*e, ok = toIDAuthStyle[str]
	if !ok {
		return fmt.Errorf("unknown value %q for AuthStyle", str)
	}
	return nil
}

// MarshalJSON marshals the enum as a quoted json string
func (e AuthStyle) MarshalJSON() ([]byte, error) {
	return json.Marshal(e.String())
}

// UnmarshalJSON unmashals a quoted json string to the enum value
func (e *AuthStyle) UnmarshalJSON(b []byte) error {
	var str string
	err := json.Unmarshal(b, &str)
	if err != nil {
		return err
	}
	return e.FromString(str)
}
