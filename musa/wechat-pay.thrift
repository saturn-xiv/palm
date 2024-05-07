namespace cpp musa.v1.wechat_pay
namespace java com.github.saturn_xiv.palm.plugins.musa.v1.wechat_pay

enum Currency {
    CNY = 1,
}

struct Amount {
    1:required i32 total,
    2:required Currency currency,
}

enum NotifyAction {
    TRANSCATION = 1,
    REFUND = 2,
}

struct PrepayRequest {
    1:required string app_id,
    
    11:optional string payer_open_id,
    12:required Amount amount,

    98:required string description,
    99:required string notify_host,
}

struct ResponseError {
    1:required string code,
    2:required string message,
}

enum TarType {
    GZIP = 1,
}

struct BillDate {
    1:required i16 year,
    2:required i8 month,
    3:required i8 day,
}

struct TradeResponse {
    1:required string trade_state,
    2:required string trade_state_desc,
}

service Native {
    NativeQrCodeUrlResponse create_prepay(1:PrepayRequest request);
    NativeQrCodeUrlResponse execute_prepay(1:string out_trade_no, 2:PrepayRequest request);
}

struct NativeQrCodeUrlResponse {
    1:required string url,
    2:required string out_trade_no,
}

service Jsapi {
    JsapiPrepayIdResponse create_prepay(1:PrepayRequest request);
    JsapiPrepayIdResponse execute_prepay(1:string out_trade_no, 2:PrepayRequest request);
    TradeResponse query_order_by_out_trade_no(1:string out_trade_no);
    TradeResponse query_order_by_transaction_id(1:string transaction_id);
    void close_order( 1:string out_trade_no, 2:string reason);
}

struct JsapiPrepayIdResponse {
    1:required string app_id,
    2:required string out_trade_no,

    11:required string time_stamp,
    12:required string nonce_str,
    13:required string package_,
    14:required string sign_type,
    15:required string pay_sign,
}

service Bill {
    binary trade(1:TradeBillType bill_type, 2:BillDate bill_date);
    binary fund_flow(1:FundFlowAccountType account_type, 2:BillDate bill_date);
}

enum TradeBillType {
    ALL = 1,
    SUCCESS = 2,
    REFUND = 3,
}

enum FundFlowAccountType {
    BASIC = 1,
    OPERATION = 2,
    FEES = 3,
}

service Refund {
    QueryRefundResponse create(1:string out_trade_no, 2:CreateRefundAmount amount, 3:string reason, 4:string notify_host);
    QueryRefundResponse query(1:string out_refund_no);
}

struct CreateRefundAmount {
    1:required i32 total,
    2:required i32 refund,
    9:required Currency currency,
}
struct QueryRefundResponse {
    1:required string out_refund_no,
    
    11:required string channel,
    12:required string status,
    13:required string user_received_account,

    99:required string create_time,
}

service Transfer {
    ExecuteTransferBatchResponse execute_batch(1:string app_id, 2:string out_batch_no, 3:string name, 4: string remark, 5:list<ExecuteTransferBatchRequestDetail> details, 6:string scene_id);
    ExecuteTransferBatchResponse create_batch(1:string app_id, 2:string name, 3: string remark, 4:list<ExecuteTransferBatchRequestDetail> details, 5:string scene_id);
    QueryTransferBatchResponse query_batch(1:string out_batch_no, 2:QueryBatchTransferDetailStatus detail_status, 3:i32 offset, 4:i32 limit);
    QueryTransferDetailResponse query_detail(1:string out_batch_no, 2:string out_detail_no);
    binary get_bill_receipt(1:string out_batch_no);
    binary get_electronic_receipt(1:TransferElectronicReceiptAcceptType accept_type, 2:string out_batch_no, 3:string out_detail_no);
}

struct ExecuteTransferBatchRequestDetail {
    1:required string open_id,
    2:required string username,
    3:required i64 amount,
    4:required string remark,
}


struct ExecuteTransferBatchResponse {
    1:required string out_batch_no,
    2:required list<ExecuteTransferBatchResponseDetail> details,
    9:required ExecuteTransferBatchResponseStatus status,
}

union ExecuteTransferBatchResponseStatus {
    1:ExecuteTransferBatchResponseSucceeded succeeded,
    2:ResponseError error,
}

struct ExecuteTransferBatchResponseSucceeded {
    1:required string batch_id,
    2:required string create_time,
}

struct ExecuteTransferBatchResponseDetail {
    1:required string open_id,
    2:required string out_detail_no,
}

struct QueryTransferBatchResponse {
    1:required string app_id,
    2:required string merchant_id,
    3:required string transfer_scene_id,

    11:required string batch_id,
    12:required string out_batch_no,
    13:required string batch_status,
    14:required string batch_type,
    15:required string batch_name,
    16:required string batch_remark,

    21:optional string close_reason,
    22:required i64 total_amount,
    23:required i32 total_num,
    24:optional string create_time,
    25:optional string update_time,
    26:optional i64 success_amount,
    27:optional i32 success_num,
    28:optional i64 fail_amount,
    29:optional i32 fail_num,

    99:list<QueryTransferBatchResponseDetail> details,
}

struct QueryTransferBatchResponseDetail {
    1:required string detail_id,
    2:required string out_detail_no,
    3:required string status,
}

enum QueryBatchTransferDetailStatus {
    ALL = 1,
    SUCCESS = 2,
    WAIT_PAY = 3,
    FAIL = 9,
}

enum TransferElectronicReceiptAcceptType {
    BATCH_TRANSFER = 1,
    TRANSFER_TO_POCKET = 2,
    TRANSFER_TO_BANK = 3,
}

struct QueryTransferDetailResponse {
    1:required string app_id,
    2:required string merchant_id,
    3:required string open_id,
    9:optional string user_name,

    11:required string batch_id,
    12:required string out_batch_no,
    13:required string out_detail_no,
    14:required string detail_id,
    15:required string detail_status,
    16:required i64 transfer_amount,
    17:required string transfer_remark,
    18:optional string fail_reason,
    19:required string initiate_time,
    20:required string update_time,
}
