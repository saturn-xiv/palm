import grpc from 'grpc-web';
import {Empty} from 'google-protobuf/google/protobuf/empty_pb';

import { PageServiceClient } from "../protocols/CmsServiceClientPb";
import {GRPC_HOST} from '.'

export const index_page = () => {
    const request = new Empty();

    const service = new PageServiceClient(GRPC_HOST, null, null);
    const call = service.index(request, {'custom-header-1': 'value1'}, (_err: grpc.RpcError, _response: Empty) => {
        // TODO
        console.log('done.');
    });
    call.on('status', (_status: grpc.Status) => {
        // TODO
    });
}

