// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af -
// kopium version: 0.14.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// Spec defines the desired state of UDPRoute.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "gateway.networking.k8s.io", version = "v1alpha2", kind = "UDPRoute", plural = "udproutes")]
#[kube(namespaced)]
#[kube(status = "UDPRouteStatus")]
pub struct UDPRouteSpec {
    /// ParentRefs references the resources (usually Gateways) that a Route wants to be attached to. Note that the referenced parent resource needs to allow this for the attachment to be complete. For Gateways, that means the Gateway needs to allow attachment from Routes of this kind and namespace. 
    ///  The only kind of parent resource with "Core" support is Gateway. This API may be extended in the future to support additional kinds of parent resources such as one of the route kinds. 
    ///  It is invalid to reference an identical parent more than once. It is valid to reference multiple distinct sections within the same parent resource, such as 2 Listeners within a Gateway. 
    ///  It is possible to separately reference multiple distinct objects that may be collapsed by an implementation. For example, some implementations may choose to merge compatible Gateway Listeners together. If that is the case, the list of routes attached to those resources should also be merged. 
    ///  Note that for ParentRefs that cross namespace boundaries, there are specific rules. Cross-namespace references are only valid if they are explicitly allowed by something in the namespace they are referring to. For example, Gateway has the AllowedRoutes field, and ReferenceGrant provides a generic way to enable any other kind of cross-namespace reference.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentRefs")]
    pub parent_refs: Option<Vec<UDPRouteParentRefs>>,
    /// Rules are a list of UDP matchers and actions.
    pub rules: Vec<UDPRouteRules>,
}

/// ParentReference identifies an API object (usually a Gateway) that can be considered a parent of this resource (usually a route). The only kind of parent resource with "Core" support is Gateway. This API may be extended in the future to support additional kinds of parent resources, such as HTTPRoute. 
///  The API object must be valid in the cluster; the Group and Kind must be registered in the cluster for this reference to be valid.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct UDPRouteParentRefs {
    /// Group is the group of the referent. When unspecified, "gateway.networking.k8s.io" is inferred. To set the core API group (such as for a "Service" kind referent), Group must be explicitly set to "" (empty string). 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent. 
    ///  Support: Core (Gateway) 
    ///  Support: Implementation-specific (Other Resources)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent. 
    ///  Support: Core
    pub name: String,
    /// Namespace is the namespace of the referent. When unspecified, this refers to the local namespace of the Route. 
    ///  Note that there are specific rules for ParentRefs which cross namespace boundaries. Cross-namespace references are only valid if they are explicitly allowed by something in the namespace they are referring to. For example: Gateway has the AllowedRoutes field, and ReferenceGrant provides a generic way to enable any other kind of cross-namespace reference. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port is the network port this Route targets. It can be interpreted differently based on the type of parent resource. 
    ///  When the parent resource is a Gateway, this targets all listeners listening on the specified port that also support this kind of Route(and select this Route). It's not recommended to set `Port` unless the networking behaviors specified in a Route must apply to a specific port as opposed to a listener(s) whose port(s) may be changed. When both Port and SectionName are specified, the name and port of the selected listener must match both specified values. 
    ///  Implementations MAY choose to support other parent resources. Implementations supporting other types of parent resources MUST clearly document how/if Port is interpreted. 
    ///  For the purpose of status, an attachment is considered successful as long as the parent resource accepts it partially. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway. 
    ///  Support: Extended 
    ///  <gateway:experimental>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// SectionName is the name of a section within the target resource. In the following resources, SectionName is interpreted as the following: 
    ///  * Gateway: Listener Name. When both Port (experimental) and SectionName are specified, the name and port of the selected listener must match both specified values. 
    ///  Implementations MAY choose to support attaching Routes to other resources. If that is the case, they MUST clearly document how SectionName is interpreted. 
    ///  When unspecified (empty string), this will reference the entire resource. For the purpose of status, an attachment is considered successful if at least one section in the parent resource accepts it. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

/// UDPRouteRule is the configuration for a given rule.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct UDPRouteRules {
    /// BackendRefs defines the backend(s) where matching requests should be sent. If unspecified or invalid (refers to a non-existent resource or a Service with no endpoints), the underlying implementation MUST actively reject connection attempts to this backend. Packet drops must respect weight; if an invalid backend is requested to have 80% of the packets, then 80% of packets must be dropped instead. 
    ///  Support: Core for Kubernetes Service 
    ///  Support: Extended for Kubernetes ServiceImport 
    ///  Support: Implementation-specific for any other resource 
    ///  Support for weight: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendRefs")]
    pub backend_refs: Option<Vec<UDPRouteRulesBackendRefs>>,
}

/// BackendRef defines how a Route should forward a request to a Kubernetes resource. 
///  Note that when a namespace is specified, a ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct UDPRouteRulesBackendRefs {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io". When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the Kubernetes resource kind of the referent. For example "Service". 
    ///  Defaults to "Service" when not specified. 
    ///  ExternalName services can refer to CNAME DNS records that may live outside of the cluster and as such are difficult to reason about in terms of conformance. They also may not be safe to forward to (see CVE-2021-25740 for more information). Implementations SHOULD NOT support ExternalName Services. 
    ///  Support: Core (Services with a type other than ExternalName) 
    ///  Support: Implementation-specific (Services with type ExternalName)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the backend. When unspecified, the local namespace is inferred. 
    ///  Note that when a namespace is specified, a ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port specifies the destination port number to use for this resource. Port is required when the referent is a Kubernetes Service. In this case, the port number is the service port number, not the target port. For other resources, destination port might be derived from the referent resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Weight specifies the proportion of requests forwarded to the referenced backend. This is computed as weight/(sum of all weights in this BackendRefs list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports. Weight is not a percentage and the sum of weights does not need to equal 100. 
    ///  If only one backend is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend. If weight is set to 0, no traffic should be forwarded for this entry. If unspecified, weight defaults to 1. 
    ///  Support for this field varies based on the context where used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

/// Status defines the current state of UDPRoute.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct UDPRouteStatus {
    /// Parents is a list of parent resources (usually Gateways) that are associated with the route, and the status of the route with respect to each parent. When this route attaches to a parent, the controller that manages the parent must add an entry to this list when the controller first sees the route and should update the entry as appropriate when the route or gateway is modified. 
    ///  Note that parent references that cannot be resolved by an implementation of this API will not be added to this list. Implementations of this API can only populate Route status for the Gateways/parent resources they are responsible for. 
    ///  A maximum of 32 Gateways will be represented in this list. An empty list means the route has not been attached to any Gateway.
    pub parents: Vec<UDPRouteStatusParents>,
}

/// RouteParentStatus describes the status of a route with respect to an associated Parent.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct UDPRouteStatusParents {
    /// Conditions describes the status of the route with respect to the Gateway. Note that the route's availability is also subject to the Gateway's own status conditions and listener status. 
    ///  If the Route's ParentRef specifies an existing Gateway that supports Routes of this kind AND that Gateway's controller has sufficient access, then that Gateway's controller MUST set the "Accepted" condition on the Route, to indicate whether the route has been accepted or rejected by the Gateway, and why. 
    ///  A Route MUST be considered "Accepted" if at least one of the Route's rules is implemented by the Gateway. 
    ///  There are a number of cases where the "Accepted" condition may not be set due to lack of controller visibility, that includes when: 
    ///  * The Route refers to a non-existent parent. * The Route is of a type that the controller does not support. * The Route is in a namespace the controller does not have access to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<UDPRouteStatusParentsConditions>>,
    /// ControllerName is a domain/path string that indicates the name of the controller that wrote this status. This corresponds with the controllerName field on GatewayClass. 
    ///  Example: "example.net/gateway-controller". 
    ///  The format of this field is DOMAIN "/" PATH, where DOMAIN and PATH are valid Kubernetes names (https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names). 
    ///  Controllers MUST populate this field when writing status. Controllers should ensure that entries to status populated with their ControllerName are cleaned up when they are no longer necessary.
    #[serde(rename = "controllerName")]
    pub controller_name: String,
    /// ParentRef corresponds with a ParentRef in the spec that this RouteParentStatus struct describes the status of.
    #[serde(rename = "parentRef")]
    pub parent_ref: UDPRouteStatusParentsParentRef,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct UDPRouteStatusParentsConditions {
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
    pub status: UDPRouteStatusParentsConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum UDPRouteStatusParentsConditionsStatus {
    True,
    False,
    Unknown,
}

/// ParentRef corresponds with a ParentRef in the spec that this RouteParentStatus struct describes the status of.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct UDPRouteStatusParentsParentRef {
    /// Group is the group of the referent. When unspecified, "gateway.networking.k8s.io" is inferred. To set the core API group (such as for a "Service" kind referent), Group must be explicitly set to "" (empty string). 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent. 
    ///  Support: Core (Gateway) 
    ///  Support: Implementation-specific (Other Resources)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent. 
    ///  Support: Core
    pub name: String,
    /// Namespace is the namespace of the referent. When unspecified, this refers to the local namespace of the Route. 
    ///  Note that there are specific rules for ParentRefs which cross namespace boundaries. Cross-namespace references are only valid if they are explicitly allowed by something in the namespace they are referring to. For example: Gateway has the AllowedRoutes field, and ReferenceGrant provides a generic way to enable any other kind of cross-namespace reference. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port is the network port this Route targets. It can be interpreted differently based on the type of parent resource. 
    ///  When the parent resource is a Gateway, this targets all listeners listening on the specified port that also support this kind of Route(and select this Route). It's not recommended to set `Port` unless the networking behaviors specified in a Route must apply to a specific port as opposed to a listener(s) whose port(s) may be changed. When both Port and SectionName are specified, the name and port of the selected listener must match both specified values. 
    ///  Implementations MAY choose to support other parent resources. Implementations supporting other types of parent resources MUST clearly document how/if Port is interpreted. 
    ///  For the purpose of status, an attachment is considered successful as long as the parent resource accepts it partially. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway. 
    ///  Support: Extended 
    ///  <gateway:experimental>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// SectionName is the name of a section within the target resource. In the following resources, SectionName is interpreted as the following: 
    ///  * Gateway: Listener Name. When both Port (experimental) and SectionName are specified, the name and port of the selected listener must match both specified values. 
    ///  Implementations MAY choose to support attaching Routes to other resources. If that is the case, they MUST clearly document how SectionName is interpreted. 
    ///  When unspecified (empty string), this will reference the entire resource. For the purpose of status, an attachment is considered successful if at least one section in the parent resource accepts it. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway. 
    ///  Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}

