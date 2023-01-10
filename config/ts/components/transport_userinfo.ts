// Code generated by NanoBus codegen utilities. DO NOT EDIT.

// deno-lint-ignore-file no-explicit-any no-unused-vars ban-unused-ignore
import {
  CodecRef,
  Component,
  DataExpr,
  Handler,
  ResourceRef,
  Step,
  ValueExpr
} from "../nanobus.ts";

export interface UserInfoV1Config {
  userInfoUrl: string;
}

export function UserInfoV1(
  config: UserInfoV1Config
): Component<UserInfoV1Config> {
  return {
    uses: "nanobus.filter.userinfo/v1",
    with: config
  };
}
