// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af -
// kopium version: 0.14.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec defines the desired state of Gateway.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "gateway.networking.k8s.io", version = "v1beta1", kind = "Gateway", plural = "gateways")]
#[kube(namespaced)]
#[kube(status = "GatewayStatus")]
pub struct GatewaySpec {
    /// Addresses requested for this Gateway. This is optional and behavior can depend on the implementation. If a value is set in the spec and the requested address is invalid or unavailable, the implementation MUST indicate this in the associated entry in GatewayStatus.Addresses. 
    ///  The Addresses field represents a request for the address(es) on the "outside of the Gateway", that traffic bound for this Gateway will use. This could be the IP address or hostname of an external load balancer or other networking infrastructure, or some other address that traffic will be sent to. 
    ///  The .listener.hostname field is used to route traffic that has already arrived at the Gateway to the correct in-cluster destination. 
    ///  If no Addresses are specified, the implementation MAY schedule the Gateway in an implementation-specific manner, assigning an appropriate set of Addresses. 
    ///  The implementation MUST bind all Listeners to every GatewayAddress that it assigns to the Gateway and add a corresponding entry in GatewayStatus.Addresses. 
    ///  Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayAddresses>>,
    /// GatewayClassName used for this Gateway. This is the name of a GatewayClass resource.
    #[serde(rename = "gatewayClassName")]
    pub gateway_class_name: String,
    /// Listeners associated with this Gateway. Listeners define logical endpoints that are bound on this Gateway's addresses. At least one Listener MUST be specified. 
    ///  Each listener in a Gateway must have a unique combination of Hostname, Port, and Protocol. 
    ///  An implementation MAY group Listeners by Port and then collapse each group of Listeners into a single Listener if the implementation determines that the Listeners in the group are "compatible". An implementation MAY also group together and collapse compatible Listeners belonging to different Gateways. 
    ///  For example, an implementation might consider Listeners to be compatible with each other if all of the following conditions are met: 
    ///  1. Either each Listener within the group specifies the "HTTP" Protocol or each Listener within the group specifies either the "HTTPS" or "TLS" Protocol. 
    ///  2. Each Listener within the group specifies a Hostname that is unique within the group. 
    ///  3. As a special case, one Listener within a group may omit Hostname, in which case this Listener matches when no other Listener matches. 
    ///  If the implementation does collapse compatible Listeners, the hostname provided in the incoming client request MUST be matched to a Listener to find the correct set of Routes. The incoming hostname MUST be matched using the Hostname field for each Listener in order of most to least specific. That is, exact matches must be processed before wildcard matches. 
    ///  If this field specifies multiple Listeners that have the same Port value but are not compatible, the implementation must raise a "Conflicted" condition in the Listener status. 
    ///  Support: Core
    pub listeners: Vec<GatewayListeners>,
}

/// GatewayAddress describes an address that can be bound to a Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayAddresses {
    /// Type of the address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Value of the address. The validity of the values will depend on the type and support by the controller. 
    ///  Examples: `1.2.3.4`, `128::1`, `my-ip-address`.
    pub value: String,
}

/// Listener embodies the concept of a logical endpoint where a Gateway accepts network connections.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayListeners {
    /// AllowedRoutes defines the types of routes that MAY be attached to a Listener and the trusted namespaces where those Route resources MAY be present. 
    ///  Although a client request may match multiple route rules, only one rule may ultimately receive the request. Matching precedence MUST be determined in order of the following criteria: 
    ///  * The most specific match as defined by the Route type. * The oldest Route based on creation timestamp. For example, a Route with a creation timestamp of "2020-09-08 01:02:03" is given precedence over a Route with a creation timestamp of "2020-09-08 01:02:04". * If everything else is equivalent, the Route appearing first in alphabetical order (namespace/name) should be given precedence. For example, foo/bar is given precedence over foo/baz. 
    ///  All valid rules within a Route attached to this Listener should be implemented. Invalid Route rules can be ignored (sometimes that will mean the full Route). If a Route rule transitions from valid to invalid, support for that Route rule should be dropped to ensure consistency. For example, even if a filter specified by a Route rule is invalid, the rest of the rules within that Route should still be supported. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRoutes")]
    pub allowed_routes: Option<GatewayListenersAllowedRoutes>,
    /// Hostname specifies the virtual hostname to match for protocol types that define this concept. When unspecified, all hostnames are matched. This field is ignored for protocols that don't require hostname based matching. 
    ///  Implementations MUST apply Hostname matching appropriately for each of the following protocols: 
    ///  * TLS: The Listener Hostname MUST match the SNI. * HTTP: The Listener Hostname MUST match the Host header of the request. * HTTPS: The Listener Hostname SHOULD match at both the TLS and HTTP protocol layers as described above. If an implementation does not ensure that both the SNI and Host header match the Listener hostname, it MUST clearly document that. 
    ///  For HTTPRoute and TLSRoute resources, there is an interaction with the `spec.hostnames` array. When both listener and route specify hostnames, there MUST be an intersection between the values for a Route to be accepted. For more information, refer to the Route specific Hostnames documentation. 
    ///  Hostnames that are prefixed with a wildcard label (`*.`) are interpreted as a suffix match. That means that a match for `*.example.com` would match both `test.example.com`, and `foo.test.example.com`, but not `example.com`. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Name is the name of the Listener. This name MUST be unique within a Gateway. 
    ///  Support: Core
    pub name: String,
    /// Port is the network port. Multiple listeners may use the same port, subject to the Listener compatibility rules. 
    ///  Support: Core
    pub port: i32,
    /// Protocol specifies the network protocol this listener expects to receive. 
    ///  Support: Core
    pub protocol: String,
    /// TLS is the TLS configuration for the Listener. This field is required if the Protocol field is "HTTPS" or "TLS". It is invalid to set this field if the Protocol field is "HTTP", "TCP", or "UDP". 
    ///  The association of SNIs to Certificate defined in GatewayTLSConfig is defined based on the Hostname field for this listener. 
    ///  The GatewayClass MUST use the longest matching SNI out of all available certificates for any TLS handshake. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayListenersTls>,
}

/// AllowedRoutes defines the types of routes that MAY be attached to a Listener and the trusted namespaces where those Route resources MAY be present. 
///  Although a client request may match multiple route rules, only one rule may ultimately receive the request. Matching precedence MUST be determined in order of the following criteria: 
///  * The most specific match as defined by the Route type. * The oldest Route based on creation timestamp. For example, a Route with a creation timestamp of "2020-09-08 01:02:03" is given precedence over a Route with a creation timestamp of "2020-09-08 01:02:04". * If everything else is equivalent, the Route appearing first in alphabetical order (namespace/name) should be given precedence. For example, foo/bar is given precedence over foo/baz. 
///  All valid rules within a Route attached to this Listener should be implemented. Invalid Route rules can be ignored (sometimes that will mean the full Route). If a Route rule transitions from valid to invalid, support for that Route rule should be dropped to ensure consistency. For example, even if a filter specified by a Route rule is invalid, the rest of the rules within that Route should still be supported. 
///  Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayListenersAllowedRoutes {
    /// Kinds specifies the groups and kinds of Routes that are allowed to bind to this Gateway Listener. When unspecified or empty, the kinds of Routes selected are determined using the Listener protocol. 
    ///  A RouteGroupKind MUST correspond to kinds of Routes that are compatible with the application protocol specified in the Listener's Protocol field. If an implementation does not support or recognize this resource type, it MUST set the "ResolvedRefs" condition to False for this Listener with the "InvalidRouteKinds" reason. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<GatewayListenersAllowedRoutesKinds>>,
    /// Namespaces indicates namespaces from which Routes may be attached to this Listener. This is restricted to the namespace of this Gateway by default. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<GatewayListenersAllowedRoutesNamespaces>,
}

/// RouteGroupKind indicates the group and kind of a Route resource.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayListenersAllowedRoutesKinds {
    /// Group is the group of the Route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the kind of the Route.
    pub kind: String,
}

/// Namespaces indicates namespaces from which Routes may be attached to this Listener. This is restricted to the namespace of this Gateway by default. 
///  Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayListenersAllowedRoutesNamespaces {
    /// From indicates where Routes will be selected for this Gateway. Possible values are: * All: Routes in all namespaces may be used by this Gateway. * Selector: Routes in namespaces selected by the selector may be used by this Gateway. * Same: Only Routes in the same namespace may be used by this Gateway. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<GatewayListenersAllowedRoutesNamespacesFrom>,
    /// Selector must be specified when From is set to "Selector". In that case, only Routes in Namespaces matching this Selector will be selected by this Gateway. This field is ignored for other values of "From". 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<GatewayListenersAllowedRoutesNamespacesSelector>,
}

/// Namespaces indicates namespaces from which Routes may be attached to this Listener. This is restricted to the namespace of this Gateway by default. 
///  Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum GatewayListenersAllowedRoutesNamespacesFrom {
    All,
    Selector,
    Same,
}

/// Selector must be specified when From is set to "Selector". In that case, only Routes in Namespaces matching this Selector will be selected by this Gateway. This field is ignored for other values of "From". 
///  Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayListenersAllowedRoutesNamespacesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GatewayListenersAllowedRoutesNamespacesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayListenersAllowedRoutesNamespacesSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// TLS is the TLS configuration for the Listener. This field is required if the Protocol field is "HTTPS" or "TLS". It is invalid to set this field if the Protocol field is "HTTP", "TCP", or "UDP". 
///  The association of SNIs to Certificate defined in GatewayTLSConfig is defined based on the Hostname field for this listener. 
///  The GatewayClass MUST use the longest matching SNI out of all available certificates for any TLS handshake. 
///  Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayListenersTls {
    /// CertificateRefs contains a series of references to Kubernetes objects that contains TLS certificates and private keys. These certificates are used to establish a TLS handshake for requests that match the hostname of the associated listener. 
    ///  A single CertificateRef to a Kubernetes Secret has "Core" support. Implementations MAY choose to support attaching multiple certificates to a Listener, but this behavior is implementation-specific. 
    ///  References to a resource in different namespace are invalid UNLESS there is a ReferenceGrant in the target namespace that allows the certificate to be attached. If a ReferenceGrant does not allow this reference, the "ResolvedRefs" condition MUST be set to False for this listener with the "RefNotPermitted" reason. 
    ///  This field is required to have at least one element when the mode is set to "Terminate" (default) and is optional otherwise. 
    ///  CertificateRefs can reference to standard Kubernetes resources, i.e. Secret, or implementation-specific custom resources. 
    ///  Support: Core - A single reference to a Kubernetes Secret of type kubernetes.io/tls 
    ///  Support: Implementation-specific (More than one reference or other resource types)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateRefs")]
    pub certificate_refs: Option<Vec<GatewayListenersTlsCertificateRefs>>,
    /// Mode defines the TLS behavior for the TLS session initiated by the client. There are two possible modes: 
    ///  - Terminate: The TLS session between the downstream client and the Gateway is terminated at the Gateway. This mode requires certificateRefs to be set and contain at least one element. - Passthrough: The TLS session is NOT terminated by the Gateway. This implies that the Gateway can't decipher the TLS stream except for the ClientHello message of the TLS protocol. CertificateRefs field is ignored in this mode. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GatewayListenersTlsMode>,
    /// Options are a list of key/value pairs to enable extended TLS configuration for each implementation. For example, configuring the minimum TLS version or supported cipher suites. 
    ///  A set of common keys MAY be defined by the API in the future. To avoid any ambiguity, implementation-specific definitions MUST use domain-prefixed names, such as `example.com/my-custom-option`. Un-prefixed names are reserved for key names defined by Gateway API. 
    ///  Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
}

/// SecretObjectReference identifies an API object including its namespace, defaulting to Secret. 
///  The API object must be valid in the cluster; the Group and Kind must be registered in the cluster for this reference to be valid. 
///  References to objects with invalid Group and Kind are not valid, and must be rejected by the implementation, with appropriate Conditions set on the containing object.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayListenersTlsCertificateRefs {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io". When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent. For example "Secret".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the backend. When unspecified, the local namespace is inferred. 
    ///  Note that when a namespace is specified, a ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// TLS is the TLS configuration for the Listener. This field is required if the Protocol field is "HTTPS" or "TLS". It is invalid to set this field if the Protocol field is "HTTP", "TCP", or "UDP". 
///  The association of SNIs to Certificate defined in GatewayTLSConfig is defined based on the Hostname field for this listener. 
///  The GatewayClass MUST use the longest matching SNI out of all available certificates for any TLS handshake. 
///  Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum GatewayListenersTlsMode {
    Terminate,
    Passthrough,
}

/// Status defines the current state of Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayStatus {
    /// Addresses lists the IP addresses that have actually been bound to the Gateway. These addresses may differ from the addresses in the Spec, e.g. if the Gateway automatically assigns an address from a reserved pool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayStatusAddresses>>,
    /// Conditions describe the current conditions of the Gateway. 
    ///  Implementations should prefer to express Gateway conditions using the `GatewayConditionType` and `GatewayConditionReason` constants so that operators and tools can converge on a common vocabulary to describe Gateway state. 
    ///  Known condition types are: 
    ///  * "Accepted" * "Programmed" * "Ready"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<GatewayStatusConditions>>,
    /// Listeners provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<GatewayStatusListeners>>,
}

/// GatewayAddress describes an address that can be bound to a Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayStatusAddresses {
    /// Type of the address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Value of the address. The validity of the values will depend on the type and support by the controller. 
    ///  Examples: `1.2.3.4`, `128::1`, `my-ip-address`.
    pub value: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: GatewayStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum GatewayStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// ListenerStatus is the status associated with a Listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayStatusListeners {
    /// AttachedRoutes represents the total number of Routes that have been successfully attached to this Listener.
    #[serde(rename = "attachedRoutes")]
    pub attached_routes: i32,
    /// Conditions describe the current condition of this listener.
    pub conditions: Vec<GatewayStatusListenersConditions>,
    /// Name is the name of the Listener that this status corresponds to.
    pub name: String,
    /// SupportedKinds is the list indicating the Kinds supported by this listener. This MUST represent the kinds an implementation supports for that Listener configuration. 
    ///  If kinds are specified in Spec that are not supported, they MUST NOT appear in this list and an implementation MUST set the "ResolvedRefs" condition to "False" with the "InvalidRouteKinds" reason. If both valid and invalid Route kinds are specified, the implementation MUST reference the valid Route kinds that have been specified.
    #[serde(rename = "supportedKinds")]
    pub supported_kinds: Vec<GatewayStatusListenersSupportedKinds>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayStatusListenersConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: GatewayStatusListenersConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum GatewayStatusListenersConditionsStatus {
    True,
    False,
    Unknown,
}

/// RouteGroupKind indicates the group and kind of a Route resource.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct GatewayStatusListenersSupportedKinds {
    /// Group is the group of the Route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the kind of the Route.
    pub kind: String,
}

