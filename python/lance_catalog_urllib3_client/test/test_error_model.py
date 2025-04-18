# coding: utf-8

"""
    Lance Catalog REST Specification

    **Lance Catalog** is an OpenAPI specification on top of the storage-based Lance format. It provides an integration point for catalog service like Apache Hive MetaStore (HMS), Apache Gravitino, etc. to store and use Lance tables. To integrate, the catalog service implements a **Lance Catalog Adapter**, which is a REST server that converts the Lance catalog requests to native requests against the catalog service. Different tools can integrate with Lance Catalog using the generated OpenAPI clients in various languages, and invoke operations in Lance Catalog to read, write and manage Lance tables in the integrated catalog services. 

    The version of the OpenAPI document: 0.0.1
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from lance_catalog_urllib3_client.models.error_model import ErrorModel

class TestErrorModel(unittest.TestCase):
    """ErrorModel unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> ErrorModel:
        """Test ErrorModel
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `ErrorModel`
        """
        model = ErrorModel()
        if include_optional:
            return ErrorModel(
                type = '/errors/incorrect-user-pass',
                title = 'Incorrect username or password',
                status = 404,
                detail = 'Authentication failed due to incorrect username or password',
                instance = '/login/log/abc123'
            )
        else:
            return ErrorModel(
                type = '/errors/incorrect-user-pass',
        )
        """

    def testErrorModel(self):
        """Test ErrorModel"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
