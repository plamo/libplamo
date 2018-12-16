#pragma once

#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum PlamoScheme {
  Http,
  Https,
} PlamoScheme;

typedef enum PlamoHttpMethod {
  Get,
  Head,
  Post,
  Put,
  Delete,
  Connect,
  Options,
  Trace,
} PlamoHttpMethod;

typedef struct PlamoString PlamoString;
PlamoString* plamo_string_new(const char *value);
void plamo_string_destroy(PlamoString plamo_string);
const char* plamo_string_get_char(const PlamoString *plamo_string);

typedef struct PlamoStringArray PlamoStringArray;
PlamoStringArray* plamo_string_array_new(void);
void plamo_string_array_destroy(PlamoStringArray *plamo_string_array);
void plamo_string_array_for_each(const PlamoStringArray *plamo_string_array, void (*callback)(const char*));
const char* plamo_string_array_get_at(const PlamoStringArray *plamo_string_array, size_t index);
const char* plamo_string_array_get_first(const PlamoStringArray *plamo_string_array);
const char* plamo_string_array_get_last(const PlamoStringArray *plamo_string_array);
void plamo_string_array_add(PlamoStringArray *plamo_string_array, const char* value);
bool plamo_string_array_remove_at(PlamoStringArray *plamo_string_array, size_t index);

typedef struct PlamoByteArray PlamoByteArray;
PlamoByteArray* plamo_byte_array_new(const unsigned char *body, size_t length);
void plamo_byte_array_destroy(PlamoByteArray *plamo_byte_array);
const unsigned char* plamo_byte_array_get_body(const PlamoByteArray *plamo_byte_array);
size_t plamo_byte_array_get_body_size(const PlamoByteArray *plamo_byte_array);

typedef struct PlamoHttpHeader PlamoHttpHeader;
PlamoHttpHeader* plamo_http_header_new(void);
void plamo_http_header_destroy(PlamoHttpHeader *plamo_http_header);
const PlamoStringArray* plamo_http_header_get(const PlamoHttpHeader *plamo_http_header, const char *key);
void plamo_http_header_add(PlamoHttpHeader *plamo_http_header, const char *key, const char *value);
bool plamo_http_header_remove(PlamoHttpHeader *plamo_http_header, const char *key);

typedef struct PlamoHttpQuery PlamoHttpQuery;
PlamoHttpQuery* plamo_http_query_new(void);
void plamo_http_query_destroy(PlamoHttpQuery *plamo_http_query);
const PlamoStringArray* plamo_http_query_get(const PlamoHttpQuery *plamo_http_query, const char *key);
void plamo_http_query_add(PlamoHttpQuery *plamo_http_query, const char *key, const char *value);
bool plamo_http_query_remove(PlamoHttpQuery *plamo_http_query, const char *key);

typedef struct PlamoRequest {
  PlamoHttpMethod method;
  PlamoScheme scheme;
  const PlamoString *path;
  const PlamoString *version;
  const PlamoHttpQuery *query;
  const PlamoHttpHeader *header;
  const PlamoByteArray *body;
} PlamoRequest;
PlamoRequest* plamo_request_new(PlamoHttpMethod method, PlamoScheme scheme, const char *path, const char *version, const PlamoByteArray *body);
void plamo_request_destroy(PlamoRequest *plamo_request);

typedef struct PlamoResponse {
  unsigned int status_code;
  const PlamoHttpHeader *header;
  const PlamoByteArray *body;
} PlamoResponse;
PlamoResponse* plamo_response_new(void);
void plamo_response_destroy(PlamoResponse *plamo_response);

typedef struct PlamoMiddleware {
  const void *body;
  bool (**callback)(const void*, const PlamoRequest*, PlamoResponse*);
} PlamoMiddleware;
PlamoMiddleware* plamo_middleware_new(const void *body, bool (*callback)(const void*, const PlamoRequest*, PlamoResponse*));
void plamo_middleware_destroy(PlamoMiddleware *plamo_middleware);

typedef struct PlamoApp PlamoApp;
PlamoApp* plamo_app_new(void);
void plamo_app_destroy(PlamoApp *plamo_app);
void plamo_app_add_middleware(PlamoApp *plamo_app, const PlamoMiddleware *plamo_middleware);
PlamoResponse* plamo_app_execute(const PlamoApp *plamo_app, const PlamoRequest *plamo_request);

#ifdef __cplusplus
}
#endif
