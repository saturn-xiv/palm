import logging
import json
import requests

import google.oauth2.credentials
import google_auth_oauthlib.flow
import googleapiclient.discovery
from grpc import StatusCode

from dahlia.protocols import oauth2_pb2_grpc, oauth2_pb2

logger = logging.getLogger(__name__)

# https://developers.google.com/identity/protocols/oauth2/web-server#python_3


class Server(oauth2_pb2_grpc.GoogleServicer):
    def __init__(self, client_secret_file):
        self.client_secret_file = client_secret_file

    def SignInUrl(self, request, context):
        flow = google_auth_oauthlib.flow.Flow.from_client_secrets_file(
            self.client_secret_file, scopes=request.scopes)
        flow.redirect_uri = request.redirect_uri
        authorization_url, state = flow.authorization_url(
            access_type='offline', include_granted_scopes='true')
        return oauth2_pb2.GoogleSignInUrlResponse(authorization_url=authorization_url, state=state)

    def FetchToken(self, request, context):
        flow = google_auth_oauthlib.flow.Flow.from_client_secrets_file(
            self.client_secret_file, scopes=request.scopes, state=request.state)
        flow.fetch_token(authorization_response=request.authorization_response)
        credentials = flow.credentials
        return oauth2_pb2.GoogleFetchTokenResponse(
            credentials=oauth2_pb2.GoogleFetchTokenResponse.Credentials(
                token=credentials.token,
                refresh_token=credentials.refresh_token,
                granted_scopes=credentials.granted_scopes,
            ),
        )

    # https://developers.google.com/identity/account-linking/oauth-linking#handle_userinfo_requests
    # https://googleapis.github.io/google-api-python-client/docs/dyn/oauth2_v2.userinfo.html
    def UserInfo(self, request, context):
        credentials = self._credentials(request)
        user_info_service = googleapiclient.discovery.build(
            'oauth2', 'v2', credentials=credentials)
        user_info = user_info_service.userinfo().get().execute()
        return oauth2_pb2.GoogleUserInfoResponse(
            sub=user_info.get('id'),
            email=user_info.get('email'),
            family_name=user_info.get('family_name'),
            gender=user_info.get('gender'),
            given_name=user_info.get('given_name'),
            hosted_domain=user_info.get('hd'),
            link=user_info.get('link'),
            locale=user_info.get('locale'),
            name=user_info.get('name'),
            picture=user_info.get('picture'),
            verified_email=user_info.get('verified_email'),
        )

    def Revoke(self, request, context):
        credentials = self._credentials(request)
        revoke = requests.post('https://oauth2.googleapis.com/revoke',
                               params={'token': credentials.token},
                               headers={'content-type': 'application/x-www-form-urlencoded'})
        if revoke.status_code != 200:
            context.abort(StatusCode.INTERNAL, "revoke credentials failed")
            return
        logger.info("credentials successfully revoked.")
        return oauth2_pb2.GoogleRevokeResponse()

    def _credentials(self, credentials):
        with open(self.client_secret_file, 'r') as f:
            client_config = json.load(f)['web']
        return google.oauth2.credentials.Credentials(
            refresh_token=credentials.refresh_token,
            scopes=credentials.granted_scopes,
            token=credentials.token,
            client_id=client_config.get('client_id'),
            client_secret=client_config.get('client_secret'),
            token_uri=client_config.get('token_uri'))
