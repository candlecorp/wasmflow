// Code generated by NanoBus codegen utilities. DO NOT EDIT.

// deno-lint-ignore-file no-explicit-any no-unused-vars ban-unused-ignore
import {
  Component,
  DataExpr,
  Handler,
  ResourceRef,
  Step,
  ValueExpr
} from "../nanobus.ts";

export interface RouterV1Config {
  routes: AddRoute[];
}

export function RouterV1(config: RouterV1Config): Component<RouterV1Config> {
  return {
    uses: "nanobus.transport.http.router/v1",
    with: config
  };
}

export interface AddRoute {
  method: string;
  uri: string;
  encoding?: string;
  raw?: boolean;
  handler: Handler;
}