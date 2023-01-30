// Code generated by NanoBus codegen utilities. DO NOT EDIT.

// deno-lint-ignore-file no-explicit-any no-unused-vars ban-unused-ignore
import {
  CodecRef,
  Component,
  DataExpr,
  Handler,
  Entity,
  ResourceRef,
  Step,
  ValueExpr
} from "../nanobus.ts";

export interface InvokeBindingConfig {
  // The name of the Dapr client resource.
  resource?: ResourceRef;
  // Name of binding to invoke.
  binding: string;
  // Name of the operation type for the binding to invoke.
  operation: string;
  // The configured codec to use for encoding the message.
  codec?: CodecRef;
  // The arguments for the codec, if any.
  codecArgs?: any[];
  // Data is the input data sent.
  data?: DataExpr;
  // Metadata is the input binding metadata.
  metadata?: DataExpr;
}

export function InvokeBinding(
  config: InvokeBindingConfig
): Component<InvokeBindingConfig> {
  return {
    uses: "@dapr/invoke_binding",
    with: config
  };
}

export interface PublishConfig {
  // The name of the Dapr client resource.
  resource?: ResourceRef;
  // Name of pubsub to invoke.
  pubsub: string;
  // Topic is the name of the topic to publish to.
  topic: string;
  // The configured codec to use for encoding the message.
  codec?: CodecRef;
  // The arguments for the codec, if any.
  codecArgs?: any[];
  // optional value to use for the message key (is supported).
  key?: ValueExpr;
  // The input bindings sent.
  data?: DataExpr;
  // The input binding metadata.
  metadata?: DataExpr;
  // Enables/disables propogating the distributed tracing context (e.g. W3C
  // TraceContext standard).
  propogateTracing?: boolean;
}

export function Publish(config: PublishConfig): Component<PublishConfig> {
  return {
    uses: "@dapr/publish",
    with: config
  };
}

export interface DeleteStateConfig {
  // The name of the Dapr client resource.
  resource?: ResourceRef;
  // Name of state store to invoke.
  store: string;
  // The key to delete.
  key: ValueExpr;
  // Etag value of the item to delete
  etag?: ValueExpr;
  // The desired concurrency level
  concurrency?: Concurrency;
  // The desired consistency level
  consistency?: Consistency;
}

export function DeleteState(
  config: DeleteStateConfig
): Component<DeleteStateConfig> {
  return {
    uses: "@dapr/delete_state",
    with: config
  };
}

export interface GetStateConfig {
  // The name of the Dapr client resource.
  resource?: ResourceRef;
  // Name of state store to invoke.
  store: string;
  // The key to get.
  key: ValueExpr;
  // The configured codec to use for decoding the state.
  codec?: CodecRef;
  // The arguments for the codec, if any.
  codecArgs?: any[];
  // The error to return if the key is not found.
  notFoundError: string;
  // The desired concurrency level
  concurrency?: Concurrency;
  // The desired consistency level
  consistency?: Consistency;
}

export function GetState(config: GetStateConfig): Component<GetStateConfig> {
  return {
    uses: "@dapr/get_state",
    with: config
  };
}

export interface SetStateConfig {
  // The name of the Dapr client resource.
  resource?: ResourceRef;
  // Name of state store to invoke.
  store: string;
  // The configured codec to use for encoding the state.
  codec?: CodecRef;
  // The arguments for the codec, if any.
  codecArgs?: any[];
  // The items to set in the store.
  items: SetStateItem[];
}

export function SetState(config: SetStateConfig): Component<SetStateConfig> {
  return {
    uses: "@dapr/set_state",
    with: config
  };
}

export interface SetStateItem {
  // The key of the item to set.
  key: ValueExpr;
  // an option expression to evaluate a.
  forEach?: ValueExpr;
  // Optional data expression to tranform the data to set.
  value?: DataExpr;
  // Etag value of the item to set
  etag?: ValueExpr;
  // Optional data expression for the key's metadata.
  metadata?: DataExpr;
  // The desired concurrency level
  concurrency?: Concurrency;
  // The desired consistency level
  consistency?: Consistency;
}

export interface InvokeActorConfig {
  // The name of the Dapr client resource.
  resource?: ResourceRef;
  // The actor handler (type::method)
  handler: Handler;
  // The actor identifier
  id: ValueExpr;
  // The input sent.
  data?: DataExpr;
  // The configured codec to use for encoding the message.
  codec?: CodecRef;
  // The arguments for the codec, if any.
  codecArgs?: any[];
}

export function InvokeActor(
  config: InvokeActorConfig
): Component<InvokeActorConfig> {
  return {
    uses: "@dapr/invoke_actor",
    with: config
  };
}

// TODO
export enum Concurrency {
  // Undefined value for state concurrency
  Undefined = "undefined",
  // First write concurrency value
  FirstWrite = "firstWrite",
  // Last write concurrency value
  LastWrite = "lastWrite"
}

// TODO
export enum Consistency {
  // Undefined value for state consistency
  Undefined = "undefined",
  // Eventual state consistency value
  Eventual = "eventual",
  // Strong state consistency value
  Strong = "strong"
}
