import grpcWeb from 'grpc-web';

import { PageServicePromiseClient } from "../protocols/CmsServiceClientPb";
import {GRPC_HOST} from '.'

function indexPage() {
    const service = new PageServicePromiseClient(GRPC_HOST, null, null);
}
