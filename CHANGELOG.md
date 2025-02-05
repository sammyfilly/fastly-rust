# Changelog

## [v4.1.1](https://github.com/fastly/fastly-rust/releases/tag/release/v4.1.1) (2023-09-01)

**Enhancements:**

- feat(events): support extra created_at filters.

## [v4.1.0](https://github.com/fastly/fastly-rust/releases/tag/release/v4.1.0) (2023-09-01)

**Bug fixes:**

- fix(user-agent): use consistent naming to other clients.

**Enhancements:**

- feat(backend): support share_key field.
- feat(events): support extra created_at filters.
- feat(logging/newrelic): add OTLP endpoints.
- feat(tls/subscriptions): support self_managed_http_challenge field.

**Documentation:**

- doc(secretstore): correct description for GET endpoint.

## [v4.0.0](https://github.com/fastly/fastly-rust/releases/tag/release/v4.0.0) (2023-07-31)

**Breaking:**

The following restructures have helped resolve some issues with our OpenAPI schemas but as a side-effect this has resulted in a break to our API client interface as different types are now being generated.

- refactor: general restructure OpenAPI schemas.
- refactor(domain): remove explicit schema type for 'any'.

**Bug fixes:**

- fix: change response `version` type to string.
- fix(cache_settings): change response `stale_ttl` and `ttl` types to strings.
- fix(header): change response `ignore_if_set` and `priority` types to strings.
- fix(logging): change response `period` and `gzip_level` types to strings.
- fix(pool): change response `use_tls`, `max_conn_default`, `first_byte_timeout`, `quorum` and `tls_check_cert` types to strings.
- fix(request_settings): change response `bypass_busy_wait`, `force_miss`, `force_ssl`, `geo_headers`, `max_stale_age` and `timer_support` types to strings.
- fix(response_object): change response `status` type to string.

## [v3.0.2](https://github.com/fastly/fastly-rust/releases/tag/release/v3.0.2) (2023-07-13)

**Bug fixes:**

- fix(logging_gcs): set expected default value for 'path'.
- fix(origin_inspector_historical): use correct type for 'values'.
- fix(tls_subscriptions): fix argument order for deleting globalsign email challenge.

## [v3.0.1](https://github.com/fastly/fastly-rust/releases/tag/release/v3.0.1) (2023-07-12)

**Bug fixes:**

- fix(billing): rename response field 'lines' to 'line_items'.
- fix(billing): restructure response models like 'aria_invoice_id'.
- fix(billing): make 'sent_at' nullable.

## [v3.0.0](https://github.com/fastly/fastly-rust/releases/tag/release/v3.0.0) (2023-07-06)

Substantial changes were made to the underlying OpenAPI specification that produces this API client. These changes have resulted in multiple new endpoints being supported as well as multiple breaking type changes and so we're publishing these changes as a new major release.

**Enhancements:**

- feat(apex_redirect): support all endpoints.
- feat(contact): support 'create' endpoint.
- feat(content): support 'check status' endpoint.
- feat(director): support 'update' endpoint.
- feat(domain_inspector): support all endpoints.
- feat(events): support all endpoints.
- feat(iam_roles): support 'add permissions' endpoint.
- feat(iam_roles): support 'create role' endpoint.
- feat(iam_roles): support 'delete permissions' endpoint.
- feat(iam_roles): support 'update role' endpoint.
- feat(iam_services): support 'add services' endpoint.
- feat(iam_services): support 'create service group' endpoint.
- feat(iam_services): support 'remove services' endpoint.
- feat(iam_services): support 'update service group' endpoint.
- feat(iam_users): support 'add members' endpoint.
- feat(iam_users): support 'add roles' endpoint.
- feat(iam_users): support 'add service groups' endpoint.
- feat(iam_users): support 'create user group' endpoint.
- feat(iam_users): support 'remove members' endpoint.
- feat(iam_users): support 'remove roles' endpoint.
- feat(iam_users): support 'remove service groups' endpoint.
- feat(iam_users): support 'update user group' endpoint.
- feat(legacy_waf): support all endpoints.
- feat(logging_kafka): support 'update' endpoint.
- feat(logging_kinesis): support 'update' endpoint.
- feat(origin_inspector): support all endpoints.
- feat(package): support 'upload' endpoint.
- feat(realtime): support all endpoints.
- feat(request_settings): support 'create' endpoint.
- feat(response_object): support 'create' endpoint.
- feat(response_object): support 'update' endpoint.
- feat(secret_store): support all endpoints.
- feat(service_authorizations): support 'delete' endpoint.
- feat(service_authorizations): support 'update' endpoint.
- feat(snippet): support 'update versioned snippet' endpoint.
- feat(sudo): support 'request sudo access' endpoint.
- feat(tokens): support 'revoke multiple tokens' endpoint.
- feat(tokens): support 'create token' endpoint.
- feat(waf_active_rules): support 'delete' endpoint.

**Bug fixes:**

- fix(acl_entry): generate NEGATED enum.

## [v2.5.0](https://github.com/fastly/fastly-rust/releases/tag/release/v2.5.0) (2023-07-05)

**Enhancements:**

- feat(vcl): support all vcl endpoints.

**Bug fixes:**

- fix(snippet): dynamic field switched from int to string.

## [v2.4.0](https://github.com/fastly/fastly-rust/releases/tag/release/v2.4.0) (2023-06-27)

**Enhancements:**

- feat(rate_limiter): implement POST/PUT endpoints.

**Bug fixes:**

- fix(automation_tokens): generate explicit error response type.
- remove(historical_stats): remove feature as generated code is invalid.

## [v2.3.0](https://github.com/fastly/fastly-rust/releases/tag/release/v2.3.0) (2023-06-23)

**Enhancements:**

- feat(historical_stats): implement historical stats.

## [v2.2.1](https://github.com/fastly/fastly-rust/releases/tag/release/v2.2.1) (2023-06-21)

**Bug fixes:**

- fix(tls_activation): add tls_configuration and tls_domains.
- fix(tls_subscription): add tls_configuration and common name.

## [v2.2.0](https://github.com/fastly/fastly-rust/releases/tag/release/v2.2.0) (2023-06-20)

**Enhancements:**

- feat(realtime_measurements): add billable request processing time.
- feat(tokens): add support for the 'get token' endpoint.

**Bug fixes:**

- fix(config): add realtime hostname.
- fix(historical_stats): generate field results model.
- fix(kv_store): remove the 'force' property from the 'delete store' endpoint.
- feat(realtime_measurements): rename object store to kv store.

## [v2.1.1](https://github.com/fastly/fastly-rust/releases/tag/release/v2.1.1) (2023-05-22)

**Bug fixes:**

- fix(acl): change `version` from int to string.
- fix(acl): add missing methods for `service_id` and `service_version` properties.
- fix(backend): make `ssl_check_cert` nullable.
- fix(purge): skip URL escape for `surrogate_key` param.
- fix(snippets): change `priority` and `version` from int to string.
- fix(snippets): add missing methods for `service_id` and `service_version` properties.

## [v2.1.0](https://github.com/fastly/fastly-rust/releases/tag/release/v2.1.0) (2023-05-17)

**Enhancements:**

- feat(config_store): add Config Store endpoints.

## [v2.0.0](https://github.com/fastly/fastly-rust/releases/tag/release/v2.0.0) (2023-05-16)

**Breaking changes:**

- breaking(object_store): rename to kv_store

**Enhancements:**

- feat(dictionary_item): add 'bulk' PATCH endpoint.
- feat(package): add `files_hash` metadata property.
- feat(tls_certificates): add `filter[in_use]` parameter.

## [v1.3.1](https://github.com/fastly/fastly-rust/releases/tag/release/v1.3.1) (2023-04-26)

**Bug fixes:**

- fix(object-store-item): use correct type for key value
- fix(tls-csrs): remove internal endpoint

## [v1.3.0](https://github.com/fastly/fastly-rust/releases/tag/release/v1.3.0) (2023-04-03)

**Bug fixes:**

- fix(purge): avoid CGI escape URL parameter

**Enhancements:**

- feat(domain-ownership): list API endpoint
- feat(object-store): items API endpoints
- feat(realtime): additional DDoS properties

## [v1.2.0](https://github.com/fastly/fastly-rust/releases/tag/release/v1.2.0) (2023-03-21)

**Bug fixes:**

- fix(purge): switch authentication type to 'token'

**Enhancements:**

- feat(events): implement 'filter_created_at' property
- feat(mutual-authentication): implement 'include' property
- feat(object-store): implement new Object Store API endpoints
- feat(settings): implement Service Settings 'update' endpoint

**Documentation:**

- docs(backend): keepalive_time
- docs(pop): region, shield, latitude, longitude
- docs(product-enablement): brotli_compression
- docs(resource): terminology
- docs(results): fanout properties
- docs(tls/subscriptions): new 'failed' state
- docs(user): 'login' modification note removed

## [v1.0.0](https://github.com/fastly/fastly-rust/releases/tag/release/v1.0.0) (2022-12-15)

**Enhancements:**

* New interface from code-generated API client [#43](https://github.com/fastly/fastly-rust/pull/3) 
  * [Blog post: Better Fastly API clients with OpenAPI Generator](https://dev.to/fastly/better-fastly-api-clients-with-openapi-generator-3lno)
  * [Documentation](https://github.com/fastly/fastly-rust#documentation-for-api-endpoints)
  * [Unsupported API endpoints](https://github.com/fastly/fastly-rust#issues)
