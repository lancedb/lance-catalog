/*
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package com.lancedb.lance.catalog.client.apache.api;

import com.lancedb.lance.catalog.client.apache.ApiClient;
import com.lancedb.lance.catalog.client.apache.ApiException;
import com.lancedb.lance.catalog.client.apache.BaseApi;
import com.lancedb.lance.catalog.client.apache.Configuration;
import com.lancedb.lance.catalog.client.apache.Pair;
import com.lancedb.lance.catalog.client.apache.model.CreateNamespaceRequest;
import com.lancedb.lance.catalog.client.apache.model.CreateNamespaceResponse;

import com.fasterxml.jackson.core.type.TypeReference;

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.StringJoiner;

@javax.annotation.Generated(
    value = "org.openapitools.codegen.languages.JavaClientCodegen",
    comments = "Generator version: 7.12.0")
public class NamespaceApi extends BaseApi {

  public NamespaceApi() {
    super(Configuration.getDefaultApiClient());
  }

  public NamespaceApi(ApiClient apiClient) {
    super(apiClient);
  }

  /**
   * Create a new namespace. A catalog can manage one or more namespaces. A namespace is used to
   * manage one or more tables. There are three modes when trying to create a namespace: * CREATE:
   * Create the namespace if it does not exist. If a namespace of the same name already exists, the
   * operation fails with 400. * EXIST_OK: Create the namespace if it does not exist. If a namespace
   * of the same name already exists, the operation succeeds and the existing namespace is kept. *
   * OVERWRITE: Create the namespace if it does not exist. If a namespace of the same name already
   * exists, the existing namespace is dropped and a new namespace with this name with no table is
   * created.
   *
   * @param createNamespaceRequest (required)
   * @return CreateNamespaceResponse
   * @throws ApiException if fails to make API call
   */
  public CreateNamespaceResponse createNamespace(CreateNamespaceRequest createNamespaceRequest)
      throws ApiException {
    return this.createNamespace(createNamespaceRequest, Collections.emptyMap());
  }

  /**
   * Create a new namespace. A catalog can manage one or more namespaces. A namespace is used to
   * manage one or more tables. There are three modes when trying to create a namespace: * CREATE:
   * Create the namespace if it does not exist. If a namespace of the same name already exists, the
   * operation fails with 400. * EXIST_OK: Create the namespace if it does not exist. If a namespace
   * of the same name already exists, the operation succeeds and the existing namespace is kept. *
   * OVERWRITE: Create the namespace if it does not exist. If a namespace of the same name already
   * exists, the existing namespace is dropped and a new namespace with this name with no table is
   * created.
   *
   * @param createNamespaceRequest (required)
   * @param additionalHeaders additionalHeaders for this call
   * @return CreateNamespaceResponse
   * @throws ApiException if fails to make API call
   */
  public CreateNamespaceResponse createNamespace(
      CreateNamespaceRequest createNamespaceRequest, Map<String, String> additionalHeaders)
      throws ApiException {
    Object localVarPostBody = createNamespaceRequest;

    // verify the required parameter 'createNamespaceRequest' is set
    if (createNamespaceRequest == null) {
      throw new ApiException(
          400,
          "Missing the required parameter 'createNamespaceRequest' when calling createNamespace");
    }

    // create path and map variables
    String localVarPath = "/v1/namespaces";

    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    List<Pair> localVarQueryParams = new ArrayList<Pair>();
    List<Pair> localVarCollectionQueryParams = new ArrayList<Pair>();
    Map<String, String> localVarHeaderParams = new HashMap<String, String>();
    Map<String, String> localVarCookieParams = new HashMap<String, String>();
    Map<String, Object> localVarFormParams = new HashMap<String, Object>();

    localVarHeaderParams.putAll(additionalHeaders);

    final String[] localVarAccepts = {"application/json"};
    final String localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);

    final String[] localVarContentTypes = {"application/json"};
    final String localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);

    String[] localVarAuthNames = new String[] {};

    TypeReference<CreateNamespaceResponse> localVarReturnType =
        new TypeReference<CreateNamespaceResponse>() {};
    return apiClient.invokeAPI(
        localVarPath,
        "POST",
        localVarQueryParams,
        localVarCollectionQueryParams,
        localVarQueryStringJoiner.toString(),
        localVarPostBody,
        localVarHeaderParams,
        localVarCookieParams,
        localVarFormParams,
        localVarAccept,
        localVarContentType,
        localVarAuthNames,
        localVarReturnType);
  }

  @Override
  public <T> T invokeAPI(
      String url,
      String method,
      Object request,
      TypeReference<T> returnType,
      Map<String, String> additionalHeaders)
      throws ApiException {
    String localVarPath = url.replace(apiClient.getBaseURL(), "");
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    List<Pair> localVarQueryParams = new ArrayList<Pair>();
    List<Pair> localVarCollectionQueryParams = new ArrayList<Pair>();
    Map<String, String> localVarHeaderParams = new HashMap<String, String>();
    Map<String, String> localVarCookieParams = new HashMap<String, String>();
    Map<String, Object> localVarFormParams = new HashMap<String, Object>();

    localVarHeaderParams.putAll(additionalHeaders);

    final String[] localVarAccepts = {"application/json"};
    final String localVarAccept = apiClient.selectHeaderAccept(localVarAccepts);

    final String[] localVarContentTypes = {"application/json"};
    final String localVarContentType = apiClient.selectHeaderContentType(localVarContentTypes);

    String[] localVarAuthNames = new String[] {};

    return apiClient.invokeAPI(
        localVarPath,
        method,
        localVarQueryParams,
        localVarCollectionQueryParams,
        localVarQueryStringJoiner.toString(),
        request,
        localVarHeaderParams,
        localVarCookieParams,
        localVarFormParams,
        localVarAccept,
        localVarContentType,
        localVarAuthNames,
        returnType);
  }
}
