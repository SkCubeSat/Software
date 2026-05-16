#!/usr/bin/env python3

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.


"""
Wrapper for creating a HTTP based Kubos service
"""

from flask import Flask, Request, Response
from graphql_server.flask.views import GraphQLView as BaseGraphQLView


def _create_view_class(user_context):
    """Create a GraphQLView subclass with custom context."""

    class GraphQLView(BaseGraphQLView):
        def get_context(self, request: Request, response: Response):
            # Merge user context with request/response
            ctx = {"request": request, "response": response}
            ctx.update(user_context)
            return ctx

    return GraphQLView


def start(config, schema, context={}):
    """
    Creates flask based graphql and graphiql endpoints
    """

    app = Flask(__name__)
    app.debug = True

    # Create a custom view class that includes the user's context
    CustomGraphQLView = _create_view_class(context)

    app.add_url_rule(
        "/",
        view_func=CustomGraphQLView.as_view("graphql", schema=schema, graphql_ide=None),
    )

    app.add_url_rule(
        "/graphiql",
        view_func=CustomGraphQLView.as_view(
            "graphiql", schema=schema, graphql_ide="graphiql"
        ),
    )

    app.run(config.ip, config.port)
