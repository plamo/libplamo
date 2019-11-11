#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

  typedef enum PlamoScheme {
    PlamoSchemeHttp,
    PlamoSchemeHttps,
  } PlamoScheme;

  typedef enum PlamoHttpVersion {
    PlamoHttpVersionHttp09,
    PlamoHttpVersionHttp10,
    PlamoHttpVersionHttp11,
    PlamoHttpVersionHttp20,
  } PlamoHttpVersion;

  typedef struct PlamoString PlamoString;
  PlamoString* plamo_string_new(const char *value);
  void plamo_string_destroy(PlamoString *plamo_string);
  const char* plamo_string_get_char(const PlamoString *plamo_string);

  typedef struct PlamoStringArray PlamoStringArray;
  PlamoStringArray* plamo_string_array_new(void);
  void plamo_string_array_destroy(PlamoStringArray *plamo_string_array);
  size_t plamo_string_array_length(const PlamoStringArray *plamo_string_array);
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
  void plamo_http_header_for_each(PlamoHttpHeader *plamo_http_header, void (*callback)(const char*, const char*));
  PlamoStringArray* plamo_http_header_get(PlamoHttpHeader *plamo_http_header, const char *key);
  void plamo_http_header_add(PlamoHttpHeader *plamo_http_header, const char *key, const char *value);
  bool plamo_http_header_remove(PlamoHttpHeader *plamo_http_header, const char *key);

  typedef struct PlamoHttpQuery PlamoHttpQuery;
  PlamoHttpQuery* plamo_http_query_new(void);
  void plamo_http_query_destroy(PlamoHttpQuery *plamo_http_query);
  void plamo_http_query_for_each(PlamoHttpQuery *plamo_http_query, void (*callback)(const char*, const char*));
  PlamoStringArray* plamo_http_query_get(PlamoHttpQuery *plamo_http_query, const char *key);
  void plamo_http_query_add(PlamoHttpQuery *plamo_http_query, const char *key, const char *value);
  bool plamo_http_query_remove(PlamoHttpQuery *plamo_http_query, const char *key);

  typedef size_t PlamoDefinedHttpMethod;
  #define PLAMO_HTTP_METHOD_GET SIZE_MAX
  #define PLAMO_HTTP_METHOD_POST SIZE_MAX - 1
  #define PLAMO_HTTP_METHOD_PUT SIZE_MAX - 2
  #define PLAMO_HTTP_METHOD_DELETE SIZE_MAX - 3
  #define PLAMO_HTTP_METHOD_HEAD SIZE_MAX - 4
  #define PLAMO_HTTP_METHOD_CONNECT SIZE_MAX - 5
  #define PLAMO_HTTP_METHOD_OPTIONS SIZE_MAX - 6
  #define PLAMO_HTTP_METHOD_TRACE SIZE_MAX - 7
  #define PLAMO_HTTP_METHOD_PATCH SIZE_MAX - 8
  typedef union PlamoHttpMethod {
    PlamoDefinedHttpMethod defined_http_method;
    char *undefined_http_method;
  } PlamoHttpMethod;
  PlamoHttpMethod plamo_http_method_new(size_t method);
  void plamo_http_method_destroy(PlamoHttpMethod *plamo_http_method);

  typedef struct PlamoRequest {
    PlamoScheme scheme;
    PlamoHttpVersion version;
    PlamoHttpMethod method;
    PlamoString *path;
    PlamoHttpQuery *query;
    PlamoHttpHeader *header;
    PlamoByteArray *body;
  } PlamoRequest;
  PlamoRequest* plamo_request_new(PlamoScheme scheme, PlamoHttpVersion version, PlamoHttpMethod method, const char *path, PlamoHttpQuery *query, PlamoHttpHeader *header, PlamoByteArray *body);
  void plamo_request_destroy(PlamoRequest *plamo_request);

  typedef struct PlamoResponse {
    unsigned int status_code;
    PlamoHttpHeader *header;
    PlamoByteArray *body;
  } PlamoResponse;
  PlamoResponse* plamo_response_new(void);
  void plamo_response_destroy(PlamoResponse *plamo_response);

  typedef struct PlamoMiddleware {
    const void *config;
    void (*callback)(const void*, const PlamoRequest*, PlamoResponse*);
  } PlamoMiddleware;
  PlamoMiddleware* plamo_middleware_new(const void *config, void (*callback)(const void*, const PlamoRequest*, PlamoResponse*));
  void plamo_middleware_destroy(PlamoMiddleware *plamo_middleware);

  typedef struct PlamoApp PlamoApp;
  PlamoApp* plamo_app_new(void);
  void plamo_app_destroy(PlamoApp *plamo_app);
  void plamo_app_add_middleware(PlamoApp *plamo_app, const PlamoMiddleware *plamo_middleware);
  PlamoResponse* plamo_app_execute(const PlamoApp *plamo_app, const PlamoRequest *plamo_request);

#ifdef __cplusplus
}
#endif
