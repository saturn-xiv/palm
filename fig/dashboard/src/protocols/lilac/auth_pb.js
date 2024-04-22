// source: auth.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

var jspb = require('google-protobuf');
var goog = jspb;
var global =
    (typeof globalThis !== 'undefined' && globalThis) ||
    (typeof window !== 'undefined' && window) ||
    (typeof global !== 'undefined' && global) ||
    (typeof self !== 'undefined' && self) ||
    (function () { return this; }).call(null) ||
    Function('return this')();

var google_protobuf_duration_pb = require('google-protobuf/google/protobuf/duration_pb.js');
goog.object.extend(proto, google_protobuf_duration_pb);
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
goog.object.extend(proto, google_protobuf_empty_pb);
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
goog.object.extend(proto, google_protobuf_timestamp_pb);
var rbac_pb = require('./rbac_pb.js');
goog.object.extend(proto, rbac_pb);
goog.exportSymbol('proto.palm.lilac.auth.v1.CategoryCreateRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.CategoryEditRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.CategoryIndexResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.CategoryIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.Editor', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.EmailUserQuery', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.EmailUserQuery.ByCase', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.IdRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.KvGetRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.KvGetResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.KvSetRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LeaveWordCreateRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LeaveWordIndexResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LocaleByLangRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LocaleByLangResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LocaleByLangResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LocaleIndexResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LocaleIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.LocaleSetRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.NotificationCreateRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.NotificationIndexResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.NotificationIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.Pager', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.Pagination', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteAuthors', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteAuthors.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteBaidu', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteFavicon', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteGabCode', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteGoogle', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteGoogleReCaptcha', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteIcpCode', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteIndexNow', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteInfo', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.SiteKeywords', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.TagCreateRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.TagEditRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.TagIndexResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.TagIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserAttachmentsResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserByEmailRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserChangePasswordRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserConfirmByTokenRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserIndexRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserIndexResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserLogsResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserLogsResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserLogsResponse.Item.Level', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserRefreshRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserRefreshResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserResetPasswordRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserSessionsResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserSessionsResponse.Item', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserSetPasswordRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserSignInByEmailRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserSignInResponse', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserSignUpByEmailRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserUnlockByTokenRequest', null, global);
goog.exportSymbol('proto.palm.lilac.auth.v1.UserUpdateProfileRequest', null, global);
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.Pager = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.Pager, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.Pager.displayName = 'proto.palm.lilac.auth.v1.Pager';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.Pagination = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.Pagination, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.Pagination.displayName = 'proto.palm.lilac.auth.v1.Pagination';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.IdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.IdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.IdRequest.displayName = 'proto.palm.lilac.auth.v1.IdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserByEmailRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserByEmailRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserByEmailRequest.displayName = 'proto.palm.lilac.auth.v1.UserByEmailRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.EmailUserQuery = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.palm.lilac.auth.v1.EmailUserQuery.oneofGroups_);
};
goog.inherits(proto.palm.lilac.auth.v1.EmailUserQuery, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.EmailUserQuery.displayName = 'proto.palm.lilac.auth.v1.EmailUserQuery';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserSignInByEmailRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserSignInByEmailRequest.displayName = 'proto.palm.lilac.auth.v1.UserSignInByEmailRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserSignInResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.UserSignInResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserSignInResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserSignInResponse.displayName = 'proto.palm.lilac.auth.v1.UserSignInResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserSignUpByEmailRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.displayName = 'proto.palm.lilac.auth.v1.UserSignUpByEmailRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserConfirmByTokenRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.displayName = 'proto.palm.lilac.auth.v1.UserConfirmByTokenRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserUnlockByTokenRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.displayName = 'proto.palm.lilac.auth.v1.UserUnlockByTokenRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserResetPasswordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserResetPasswordRequest.displayName = 'proto.palm.lilac.auth.v1.UserResetPasswordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserLogsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.UserLogsResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserLogsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserLogsResponse.displayName = 'proto.palm.lilac.auth.v1.UserLogsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserLogsResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserLogsResponse.Item.displayName = 'proto.palm.lilac.auth.v1.UserLogsResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserChangePasswordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserChangePasswordRequest.displayName = 'proto.palm.lilac.auth.v1.UserChangePasswordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserUpdateProfileRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserUpdateProfileRequest.displayName = 'proto.palm.lilac.auth.v1.UserUpdateProfileRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserIndexRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserIndexRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserIndexRequest.displayName = 'proto.palm.lilac.auth.v1.UserIndexRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.UserIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserIndexResponse.displayName = 'proto.palm.lilac.auth.v1.UserIndexResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserIndexResponse.Item.displayName = 'proto.palm.lilac.auth.v1.UserIndexResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.displayName = 'proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserSessionsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.UserSessionsResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserSessionsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserSessionsResponse.displayName = 'proto.palm.lilac.auth.v1.UserSessionsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserSessionsResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserSessionsResponse.Item.displayName = 'proto.palm.lilac.auth.v1.UserSessionsResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserSetPasswordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserSetPasswordRequest.displayName = 'proto.palm.lilac.auth.v1.UserSetPasswordRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.UserAttachmentsResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserAttachmentsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserAttachmentsResponse.displayName = 'proto.palm.lilac.auth.v1.UserAttachmentsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.displayName = 'proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserRefreshRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserRefreshRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserRefreshRequest.displayName = 'proto.palm.lilac.auth.v1.UserRefreshRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.UserRefreshResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.UserRefreshResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.UserRefreshResponse.displayName = 'proto.palm.lilac.auth.v1.UserRefreshResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LocaleByLangRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LocaleByLangRequest.displayName = 'proto.palm.lilac.auth.v1.LocaleByLangRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.LocaleByLangResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LocaleByLangResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LocaleByLangResponse.displayName = 'proto.palm.lilac.auth.v1.LocaleByLangResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LocaleByLangResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.displayName = 'proto.palm.lilac.auth.v1.LocaleByLangResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.LocaleIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LocaleIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LocaleIndexResponse.displayName = 'proto.palm.lilac.auth.v1.LocaleIndexResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LocaleIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.displayName = 'proto.palm.lilac.auth.v1.LocaleIndexResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LocaleSetRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LocaleSetRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LocaleSetRequest.displayName = 'proto.palm.lilac.auth.v1.LocaleSetRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.KvSetRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.KvSetRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.KvSetRequest.displayName = 'proto.palm.lilac.auth.v1.KvSetRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.KvGetRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.KvGetRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.KvGetRequest.displayName = 'proto.palm.lilac.auth.v1.KvGetRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.KvGetResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.KvGetResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.KvGetResponse.displayName = 'proto.palm.lilac.auth.v1.KvGetResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteIcpCode = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteIcpCode, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteIcpCode.displayName = 'proto.palm.lilac.auth.v1.SiteIcpCode';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteGabCode = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteGabCode, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteGabCode.displayName = 'proto.palm.lilac.auth.v1.SiteGabCode';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteBaidu = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteBaidu, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteBaidu.displayName = 'proto.palm.lilac.auth.v1.SiteBaidu';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteGoogle = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteGoogle, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteGoogle.displayName = 'proto.palm.lilac.auth.v1.SiteGoogle';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteIndexNow = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteIndexNow, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteIndexNow.displayName = 'proto.palm.lilac.auth.v1.SiteIndexNow';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteGoogleReCaptcha, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.displayName = 'proto.palm.lilac.auth.v1.SiteGoogleReCaptcha';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteAuthors = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.SiteAuthors.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteAuthors, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteAuthors.displayName = 'proto.palm.lilac.auth.v1.SiteAuthors';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteAuthors.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteAuthors.Item.displayName = 'proto.palm.lilac.auth.v1.SiteAuthors.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteKeywords = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.SiteKeywords.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteKeywords, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteKeywords.displayName = 'proto.palm.lilac.auth.v1.SiteKeywords';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteInfo = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteInfo, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteInfo.displayName = 'proto.palm.lilac.auth.v1.SiteInfo';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.SiteFavicon = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.SiteFavicon, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.SiteFavicon.displayName = 'proto.palm.lilac.auth.v1.SiteFavicon';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LeaveWordCreateRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LeaveWordCreateRequest.displayName = 'proto.palm.lilac.auth.v1.LeaveWordCreateRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.LeaveWordIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LeaveWordIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LeaveWordIndexResponse.displayName = 'proto.palm.lilac.auth.v1.LeaveWordIndexResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.displayName = 'proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.TagCreateRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.TagCreateRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.TagCreateRequest.displayName = 'proto.palm.lilac.auth.v1.TagCreateRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.TagEditRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.TagEditRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.TagEditRequest.displayName = 'proto.palm.lilac.auth.v1.TagEditRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.TagIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.TagIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.TagIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.TagIndexResponse.displayName = 'proto.palm.lilac.auth.v1.TagIndexResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.TagIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.TagIndexResponse.Item.displayName = 'proto.palm.lilac.auth.v1.TagIndexResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.CategoryCreateRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.CategoryCreateRequest.displayName = 'proto.palm.lilac.auth.v1.CategoryCreateRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.CategoryEditRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.CategoryEditRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.CategoryEditRequest.displayName = 'proto.palm.lilac.auth.v1.CategoryEditRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.CategoryIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.CategoryIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.CategoryIndexResponse.displayName = 'proto.palm.lilac.auth.v1.CategoryIndexResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.CategoryIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.displayName = 'proto.palm.lilac.auth.v1.CategoryIndexResponse.Item';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.NotificationCreateRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.NotificationCreateRequest.displayName = 'proto.palm.lilac.auth.v1.NotificationCreateRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.lilac.auth.v1.NotificationIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.lilac.auth.v1.NotificationIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.NotificationIndexResponse.displayName = 'proto.palm.lilac.auth.v1.NotificationIndexResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.lilac.auth.v1.NotificationIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.displayName = 'proto.palm.lilac.auth.v1.NotificationIndexResponse.Item';
}



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.Pager.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.Pager.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.Pager} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.Pager.toObject = function(includeInstance, msg) {
  var f, obj = {
    page: jspb.Message.getFieldWithDefault(msg, 1, 0),
    size: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.Pager}
 */
proto.palm.lilac.auth.v1.Pager.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.Pager;
  return proto.palm.lilac.auth.v1.Pager.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.Pager} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.Pager}
 */
proto.palm.lilac.auth.v1.Pager.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setSize(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.Pager.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.Pager.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.Pager} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.Pager.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getSize();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional int64 page = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.Pager.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.Pager} returns this
 */
proto.palm.lilac.auth.v1.Pager.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 size = 2;
 * @return {number}
 */
proto.palm.lilac.auth.v1.Pager.prototype.getSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.Pager} returns this
 */
proto.palm.lilac.auth.v1.Pager.prototype.setSize = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.Pagination.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.Pagination.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.Pagination} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.Pagination.toObject = function(includeInstance, msg) {
  var f, obj = {
    page: jspb.Message.getFieldWithDefault(msg, 1, 0),
    size: jspb.Message.getFieldWithDefault(msg, 2, 0),
    total: jspb.Message.getFieldWithDefault(msg, 3, 0),
    hasNext: jspb.Message.getBooleanFieldWithDefault(msg, 8, false),
    hasPrevious: jspb.Message.getBooleanFieldWithDefault(msg, 9, false)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.Pagination}
 */
proto.palm.lilac.auth.v1.Pagination.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.Pagination;
  return proto.palm.lilac.auth.v1.Pagination.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.Pagination} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.Pagination}
 */
proto.palm.lilac.auth.v1.Pagination.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setSize(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotal(value);
      break;
    case 8:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setHasNext(value);
      break;
    case 9:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setHasPrevious(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.Pagination.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.Pagination.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.Pagination} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.Pagination.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getSize();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getTotal();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
  f = message.getHasNext();
  if (f) {
    writer.writeBool(
      8,
      f
    );
  }
  f = message.getHasPrevious();
  if (f) {
    writer.writeBool(
      9,
      f
    );
  }
};


/**
 * optional int64 page = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.Pagination.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.Pagination} returns this
 */
proto.palm.lilac.auth.v1.Pagination.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 size = 2;
 * @return {number}
 */
proto.palm.lilac.auth.v1.Pagination.prototype.getSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.Pagination} returns this
 */
proto.palm.lilac.auth.v1.Pagination.prototype.setSize = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 total = 3;
 * @return {number}
 */
proto.palm.lilac.auth.v1.Pagination.prototype.getTotal = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.Pagination} returns this
 */
proto.palm.lilac.auth.v1.Pagination.prototype.setTotal = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional bool has_next = 8;
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.Pagination.prototype.getHasNext = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 8, false));
};


/**
 * @param {boolean} value
 * @return {!proto.palm.lilac.auth.v1.Pagination} returns this
 */
proto.palm.lilac.auth.v1.Pagination.prototype.setHasNext = function(value) {
  return jspb.Message.setProto3BooleanField(this, 8, value);
};


/**
 * optional bool has_previous = 9;
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.Pagination.prototype.getHasPrevious = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 9, false));
};


/**
 * @param {boolean} value
 * @return {!proto.palm.lilac.auth.v1.Pagination} returns this
 */
proto.palm.lilac.auth.v1.Pagination.prototype.setHasPrevious = function(value) {
  return jspb.Message.setProto3BooleanField(this, 9, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.IdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.IdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.IdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.IdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.IdRequest}
 */
proto.palm.lilac.auth.v1.IdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.IdRequest;
  return proto.palm.lilac.auth.v1.IdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.IdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.IdRequest}
 */
proto.palm.lilac.auth.v1.IdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.IdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.IdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.IdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.IdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.IdRequest.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.IdRequest} returns this
 */
proto.palm.lilac.auth.v1.IdRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserByEmailRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserByEmailRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    user: (f = msg.getUser()) && proto.palm.lilac.auth.v1.EmailUserQuery.toObject(includeInstance, f),
    home: jspb.Message.getFieldWithDefault(msg, 9, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserByEmailRequest}
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserByEmailRequest;
  return proto.palm.lilac.auth.v1.UserByEmailRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserByEmailRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserByEmailRequest}
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.EmailUserQuery;
      reader.readMessage(value,proto.palm.lilac.auth.v1.EmailUserQuery.deserializeBinaryFromReader);
      msg.setUser(value);
      break;
    case 9:
      var value = /** @type {string} */ (reader.readString());
      msg.setHome(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserByEmailRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserByEmailRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUser();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.EmailUserQuery.serializeBinaryToWriter
    );
  }
  f = message.getHome();
  if (f.length > 0) {
    writer.writeString(
      9,
      f
    );
  }
};


/**
 * optional EmailUserQuery user = 1;
 * @return {?proto.palm.lilac.auth.v1.EmailUserQuery}
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.prototype.getUser = function() {
  return /** @type{?proto.palm.lilac.auth.v1.EmailUserQuery} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.EmailUserQuery, 1));
};


/**
 * @param {?proto.palm.lilac.auth.v1.EmailUserQuery|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserByEmailRequest} returns this
*/
proto.palm.lilac.auth.v1.UserByEmailRequest.prototype.setUser = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.prototype.clearUser = function() {
  return this.setUser(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.prototype.hasUser = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string home = 9;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.prototype.getHome = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 9, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserByEmailRequest.prototype.setHome = function(value) {
  return jspb.Message.setProto3StringField(this, 9, value);
};



/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.palm.lilac.auth.v1.EmailUserQuery.oneofGroups_ = [[1,2]];

/**
 * @enum {number}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.ByCase = {
  BY_NOT_SET: 0,
  EMAIL: 1,
  NICKNAME: 2
};

/**
 * @return {proto.palm.lilac.auth.v1.EmailUserQuery.ByCase}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.getByCase = function() {
  return /** @type {proto.palm.lilac.auth.v1.EmailUserQuery.ByCase} */(jspb.Message.computeOneofCase(this, proto.palm.lilac.auth.v1.EmailUserQuery.oneofGroups_[0]));
};



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.EmailUserQuery.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.EmailUserQuery} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.EmailUserQuery.toObject = function(includeInstance, msg) {
  var f, obj = {
    email: jspb.Message.getFieldWithDefault(msg, 1, ""),
    nickname: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.EmailUserQuery}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.EmailUserQuery;
  return proto.palm.lilac.auth.v1.EmailUserQuery.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.EmailUserQuery} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.EmailUserQuery}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setEmail(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setNickname(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.EmailUserQuery.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.EmailUserQuery} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.EmailUserQuery.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = /** @type {string} */ (jspb.Message.getField(message, 1));
  if (f != null) {
    writer.writeString(
      1,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 2));
  if (f != null) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string email = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.getEmail = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.EmailUserQuery} returns this
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.setEmail = function(value) {
  return jspb.Message.setOneofField(this, 1, proto.palm.lilac.auth.v1.EmailUserQuery.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.EmailUserQuery} returns this
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.clearEmail = function() {
  return jspb.Message.setOneofField(this, 1, proto.palm.lilac.auth.v1.EmailUserQuery.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.hasEmail = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string nickname = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.getNickname = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.EmailUserQuery} returns this
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.setNickname = function(value) {
  return jspb.Message.setOneofField(this, 2, proto.palm.lilac.auth.v1.EmailUserQuery.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.EmailUserQuery} returns this
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.clearNickname = function() {
  return jspb.Message.setOneofField(this, 2, proto.palm.lilac.auth.v1.EmailUserQuery.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.EmailUserQuery.prototype.hasNickname = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserSignInByEmailRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    user: (f = msg.getUser()) && proto.palm.lilac.auth.v1.EmailUserQuery.toObject(includeInstance, f),
    password: jspb.Message.getFieldWithDefault(msg, 2, ""),
    ttl: (f = msg.getTtl()) && google_protobuf_duration_pb.Duration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserSignInByEmailRequest;
  return proto.palm.lilac.auth.v1.UserSignInByEmailRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.EmailUserQuery;
      reader.readMessage(value,proto.palm.lilac.auth.v1.EmailUserQuery.deserializeBinaryFromReader);
      msg.setUser(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setPassword(value);
      break;
    case 9:
      var value = new google_protobuf_duration_pb.Duration;
      reader.readMessage(value,google_protobuf_duration_pb.Duration.deserializeBinaryFromReader);
      msg.setTtl(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserSignInByEmailRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUser();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.EmailUserQuery.serializeBinaryToWriter
    );
  }
  f = message.getPassword();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getTtl();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_duration_pb.Duration.serializeBinaryToWriter
    );
  }
};


/**
 * optional EmailUserQuery user = 1;
 * @return {?proto.palm.lilac.auth.v1.EmailUserQuery}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.getUser = function() {
  return /** @type{?proto.palm.lilac.auth.v1.EmailUserQuery} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.EmailUserQuery, 1));
};


/**
 * @param {?proto.palm.lilac.auth.v1.EmailUserQuery|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest} returns this
*/
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.setUser = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.clearUser = function() {
  return this.setUser(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.hasUser = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string password = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.setPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional google.protobuf.Duration ttl = 9;
 * @return {?proto.google.protobuf.Duration}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.getTtl = function() {
  return /** @type{?proto.google.protobuf.Duration} */ (
    jspb.Message.getWrapperField(this, google_protobuf_duration_pb.Duration, 9));
};


/**
 * @param {?proto.google.protobuf.Duration|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest} returns this
*/
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.setTtl = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserSignInByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.clearTtl = function() {
  return this.setTtl(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserSignInByEmailRequest.prototype.hasTtl = function() {
  return jspb.Message.getField(this, 9) != null;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.UserSignInResponse.repeatedFields_ = [21,22];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserSignInResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserSignInResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSignInResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    token: jspb.Message.getFieldWithDefault(msg, 1, ""),
    nickname: jspb.Message.getFieldWithDefault(msg, 11, ""),
    email: jspb.Message.getFieldWithDefault(msg, 12, ""),
    realName: jspb.Message.getFieldWithDefault(msg, 16, ""),
    avatar: jspb.Message.getFieldWithDefault(msg, 17, ""),
    lang: jspb.Message.getFieldWithDefault(msg, 18, ""),
    timezone: jspb.Message.getFieldWithDefault(msg, 19, ""),
    rolesList: jspb.Message.toObjectList(msg.getRolesList(),
    rbac_pb.Role.toObject, includeInstance),
    permissionsList: jspb.Message.toObjectList(msg.getPermissionsList(),
    rbac_pb.Permission.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserSignInResponse;
  return proto.palm.lilac.auth.v1.UserSignInResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserSignInResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setToken(value);
      break;
    case 11:
      var value = /** @type {string} */ (reader.readString());
      msg.setNickname(value);
      break;
    case 12:
      var value = /** @type {string} */ (reader.readString());
      msg.setEmail(value);
      break;
    case 16:
      var value = /** @type {string} */ (reader.readString());
      msg.setRealName(value);
      break;
    case 17:
      var value = /** @type {string} */ (reader.readString());
      msg.setAvatar(value);
      break;
    case 18:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 19:
      var value = /** @type {string} */ (reader.readString());
      msg.setTimezone(value);
      break;
    case 21:
      var value = new rbac_pb.Role;
      reader.readMessage(value,rbac_pb.Role.deserializeBinaryFromReader);
      msg.addRoles(value);
      break;
    case 22:
      var value = new rbac_pb.Permission;
      reader.readMessage(value,rbac_pb.Permission.deserializeBinaryFromReader);
      msg.addPermissions(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserSignInResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserSignInResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSignInResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getNickname();
  if (f.length > 0) {
    writer.writeString(
      11,
      f
    );
  }
  f = message.getEmail();
  if (f.length > 0) {
    writer.writeString(
      12,
      f
    );
  }
  f = message.getRealName();
  if (f.length > 0) {
    writer.writeString(
      16,
      f
    );
  }
  f = message.getAvatar();
  if (f.length > 0) {
    writer.writeString(
      17,
      f
    );
  }
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      18,
      f
    );
  }
  f = message.getTimezone();
  if (f.length > 0) {
    writer.writeString(
      19,
      f
    );
  }
  f = message.getRolesList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      21,
      f,
      rbac_pb.Role.serializeBinaryToWriter
    );
  }
  f = message.getPermissionsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      22,
      f,
      rbac_pb.Permission.serializeBinaryToWriter
    );
  }
};


/**
 * optional string token = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string nickname = 11;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getNickname = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 11, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setNickname = function(value) {
  return jspb.Message.setProto3StringField(this, 11, value);
};


/**
 * optional string email = 12;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getEmail = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 12, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setEmail = function(value) {
  return jspb.Message.setProto3StringField(this, 12, value);
};


/**
 * optional string real_name = 16;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getRealName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 16, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setRealName = function(value) {
  return jspb.Message.setProto3StringField(this, 16, value);
};


/**
 * optional string avatar = 17;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getAvatar = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 17, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setAvatar = function(value) {
  return jspb.Message.setProto3StringField(this, 17, value);
};


/**
 * optional string lang = 18;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 18, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 18, value);
};


/**
 * optional string timezone = 19;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getTimezone = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 19, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setTimezone = function(value) {
  return jspb.Message.setProto3StringField(this, 19, value);
};


/**
 * repeated palm.lilac.rbac.v1.Role roles = 21;
 * @return {!Array<!proto.palm.lilac.rbac.v1.Role>}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getRolesList = function() {
  return /** @type{!Array<!proto.palm.lilac.rbac.v1.Role>} */ (
    jspb.Message.getRepeatedWrapperField(this, rbac_pb.Role, 21));
};


/**
 * @param {!Array<!proto.palm.lilac.rbac.v1.Role>} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
*/
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setRolesList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 21, value);
};


/**
 * @param {!proto.palm.lilac.rbac.v1.Role=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.rbac.v1.Role}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.addRoles = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 21, opt_value, proto.palm.lilac.rbac.v1.Role, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.clearRolesList = function() {
  return this.setRolesList([]);
};


/**
 * repeated palm.lilac.rbac.v1.Permission permissions = 22;
 * @return {!Array<!proto.palm.lilac.rbac.v1.Permission>}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.getPermissionsList = function() {
  return /** @type{!Array<!proto.palm.lilac.rbac.v1.Permission>} */ (
    jspb.Message.getRepeatedWrapperField(this, rbac_pb.Permission, 22));
};


/**
 * @param {!Array<!proto.palm.lilac.rbac.v1.Permission>} value
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
*/
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.setPermissionsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 22, value);
};


/**
 * @param {!proto.palm.lilac.rbac.v1.Permission=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.rbac.v1.Permission}
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.addPermissions = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 22, opt_value, proto.palm.lilac.rbac.v1.Permission, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.UserSignInResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSignInResponse.prototype.clearPermissionsList = function() {
  return this.setPermissionsList([]);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    realName: jspb.Message.getFieldWithDefault(msg, 1, ""),
    email: jspb.Message.getFieldWithDefault(msg, 2, ""),
    nickname: jspb.Message.getFieldWithDefault(msg, 3, ""),
    password: jspb.Message.getFieldWithDefault(msg, 4, ""),
    lang: jspb.Message.getFieldWithDefault(msg, 8, ""),
    timezone: jspb.Message.getFieldWithDefault(msg, 9, ""),
    home: jspb.Message.getFieldWithDefault(msg, 11, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserSignUpByEmailRequest;
  return proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setRealName(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setEmail(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setNickname(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setPassword(value);
      break;
    case 8:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 9:
      var value = /** @type {string} */ (reader.readString());
      msg.setTimezone(value);
      break;
    case 11:
      var value = /** @type {string} */ (reader.readString());
      msg.setHome(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getRealName();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEmail();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getNickname();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getPassword();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      8,
      f
    );
  }
  f = message.getTimezone();
  if (f.length > 0) {
    writer.writeString(
      9,
      f
    );
  }
  f = message.getHome();
  if (f.length > 0) {
    writer.writeString(
      11,
      f
    );
  }
};


/**
 * optional string real_name = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.getRealName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.setRealName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string email = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.getEmail = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.setEmail = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string nickname = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.getNickname = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.setNickname = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string password = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.setPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string lang = 8;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 8, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 8, value);
};


/**
 * optional string timezone = 9;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.getTimezone = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 9, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.setTimezone = function(value) {
  return jspb.Message.setProto3StringField(this, 9, value);
};


/**
 * optional string home = 11;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.getHome = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 11, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSignUpByEmailRequest.prototype.setHome = function(value) {
  return jspb.Message.setProto3StringField(this, 11, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserConfirmByTokenRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    token: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserConfirmByTokenRequest}
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserConfirmByTokenRequest;
  return proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserConfirmByTokenRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserConfirmByTokenRequest}
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setToken(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserConfirmByTokenRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string token = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserConfirmByTokenRequest} returns this
 */
proto.palm.lilac.auth.v1.UserConfirmByTokenRequest.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserUnlockByTokenRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    token: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserUnlockByTokenRequest}
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserUnlockByTokenRequest;
  return proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserUnlockByTokenRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserUnlockByTokenRequest}
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setToken(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserUnlockByTokenRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string token = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserUnlockByTokenRequest} returns this
 */
proto.palm.lilac.auth.v1.UserUnlockByTokenRequest.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserResetPasswordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserResetPasswordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    token: jspb.Message.getFieldWithDefault(msg, 1, ""),
    password: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserResetPasswordRequest}
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserResetPasswordRequest;
  return proto.palm.lilac.auth.v1.UserResetPasswordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserResetPasswordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserResetPasswordRequest}
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setToken(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setPassword(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserResetPasswordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserResetPasswordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getPassword();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string token = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserResetPasswordRequest} returns this
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string password = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserResetPasswordRequest} returns this
 */
proto.palm.lilac.auth.v1.UserResetPasswordRequest.prototype.setPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.UserLogsResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserLogsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserLogsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserLogsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.UserLogsResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && proto.palm.lilac.auth.v1.Pagination.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserLogsResponse;
  return proto.palm.lilac.auth.v1.UserLogsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserLogsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.UserLogsResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.UserLogsResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new proto.palm.lilac.auth.v1.Pagination;
      reader.readMessage(value,proto.palm.lilac.auth.v1.Pagination.deserializeBinaryFromReader);
      msg.setPagination(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserLogsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserLogsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserLogsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.UserLogsResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.lilac.auth.v1.Pagination.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserLogsResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    plugin: jspb.Message.getFieldWithDefault(msg, 2, ""),
    ip: jspb.Message.getFieldWithDefault(msg, 3, ""),
    level: jspb.Message.getFieldWithDefault(msg, 4, 0),
    resource: (f = msg.getResource()) && rbac_pb.Resource.toObject(includeInstance, f),
    message: jspb.Message.getFieldWithDefault(msg, 6, ""),
    createdAt: (f = msg.getCreatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserLogsResponse.Item;
  return proto.palm.lilac.auth.v1.UserLogsResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setPlugin(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setIp(value);
      break;
    case 4:
      var value = /** @type {!proto.palm.lilac.auth.v1.UserLogsResponse.Item.Level} */ (reader.readEnum());
      msg.setLevel(value);
      break;
    case 5:
      var value = new rbac_pb.Resource;
      reader.readMessage(value,rbac_pb.Resource.deserializeBinaryFromReader);
      msg.setResource(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setMessage(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCreatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserLogsResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getPlugin();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getIp();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getLevel();
  if (f !== 0.0) {
    writer.writeEnum(
      4,
      f
    );
  }
  f = message.getResource();
  if (f != null) {
    writer.writeMessage(
      5,
      f,
      rbac_pb.Resource.serializeBinaryToWriter
    );
  }
  f = message.getMessage();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * @enum {number}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.Level = {
  INFO: 0,
  WARN: 1,
  ERROR: 2
};

/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string plugin = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.getPlugin = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.setPlugin = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string ip = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.getIp = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.setIp = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional Level level = 4;
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item.Level}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.getLevel = function() {
  return /** @type {!proto.palm.lilac.auth.v1.UserLogsResponse.Item.Level} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {!proto.palm.lilac.auth.v1.UserLogsResponse.Item.Level} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.setLevel = function(value) {
  return jspb.Message.setProto3EnumField(this, 4, value);
};


/**
 * optional palm.lilac.rbac.v1.Resource resource = 5;
 * @return {?proto.palm.lilac.rbac.v1.Resource}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.getResource = function() {
  return /** @type{?proto.palm.lilac.rbac.v1.Resource} */ (
    jspb.Message.getWrapperField(this, rbac_pb.Resource, 5));
};


/**
 * @param {?proto.palm.lilac.rbac.v1.Resource|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.setResource = function(value) {
  return jspb.Message.setWrapperField(this, 5, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.clearResource = function() {
  return this.setResource(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.hasResource = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional string message = 6;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};


/**
 * optional google.protobuf.Timestamp created_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.Item.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.UserLogsResponse.Item>}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.UserLogsResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.UserLogsResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.UserLogsResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse} returns this
*/
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.UserLogsResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.UserLogsResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional Pagination pagination = 2;
 * @return {?proto.palm.lilac.auth.v1.Pagination}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.lilac.auth.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.Pagination, 2));
};


/**
 * @param {?proto.palm.lilac.auth.v1.Pagination|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse} returns this
*/
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserLogsResponse} returns this
 */
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserLogsResponse.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserChangePasswordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserChangePasswordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    currentPassword: jspb.Message.getFieldWithDefault(msg, 1, ""),
    newPassword: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserChangePasswordRequest}
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserChangePasswordRequest;
  return proto.palm.lilac.auth.v1.UserChangePasswordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserChangePasswordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserChangePasswordRequest}
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCurrentPassword(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setNewPassword(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserChangePasswordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserChangePasswordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCurrentPassword();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getNewPassword();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string current_password = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.prototype.getCurrentPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserChangePasswordRequest} returns this
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.prototype.setCurrentPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string new_password = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.prototype.getNewPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserChangePasswordRequest} returns this
 */
proto.palm.lilac.auth.v1.UserChangePasswordRequest.prototype.setNewPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserUpdateProfileRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    realName: jspb.Message.getFieldWithDefault(msg, 1, ""),
    avatar: jspb.Message.getFieldWithDefault(msg, 2, ""),
    lang: jspb.Message.getFieldWithDefault(msg, 3, ""),
    timezone: jspb.Message.getFieldWithDefault(msg, 4, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest}
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserUpdateProfileRequest;
  return proto.palm.lilac.auth.v1.UserUpdateProfileRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest}
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setRealName(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setAvatar(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setTimezone(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserUpdateProfileRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getRealName();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getAvatar();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getTimezone();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional string real_name = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.getRealName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest} returns this
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.setRealName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string avatar = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.getAvatar = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest} returns this
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.setAvatar = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string lang = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest} returns this
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string timezone = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.getTimezone = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserUpdateProfileRequest} returns this
 */
proto.palm.lilac.auth.v1.UserUpdateProfileRequest.prototype.setTimezone = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserIndexRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserIndexRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserIndexRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserIndexRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    providerType: jspb.Message.getFieldWithDefault(msg, 1, 0),
    pager: (f = msg.getPager()) && proto.palm.lilac.auth.v1.Pager.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserIndexRequest}
 */
proto.palm.lilac.auth.v1.UserIndexRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserIndexRequest;
  return proto.palm.lilac.auth.v1.UserIndexRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserIndexRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserIndexRequest}
 */
proto.palm.lilac.auth.v1.UserIndexRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.palm.lilac.rbac.v1.UserDetail.Provider.Type} */ (reader.readEnum());
      msg.setProviderType(value);
      break;
    case 9:
      var value = new proto.palm.lilac.auth.v1.Pager;
      reader.readMessage(value,proto.palm.lilac.auth.v1.Pager.deserializeBinaryFromReader);
      msg.setPager(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserIndexRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserIndexRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserIndexRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserIndexRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getProviderType();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getPager();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      proto.palm.lilac.auth.v1.Pager.serializeBinaryToWriter
    );
  }
};


/**
 * optional palm.lilac.rbac.v1.UserDetail.Provider.Type provider_type = 1;
 * @return {!proto.palm.lilac.rbac.v1.UserDetail.Provider.Type}
 */
proto.palm.lilac.auth.v1.UserIndexRequest.prototype.getProviderType = function() {
  return /** @type {!proto.palm.lilac.rbac.v1.UserDetail.Provider.Type} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.palm.lilac.rbac.v1.UserDetail.Provider.Type} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexRequest} returns this
 */
proto.palm.lilac.auth.v1.UserIndexRequest.prototype.setProviderType = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional Pager pager = 9;
 * @return {?proto.palm.lilac.auth.v1.Pager}
 */
proto.palm.lilac.auth.v1.UserIndexRequest.prototype.getPager = function() {
  return /** @type{?proto.palm.lilac.auth.v1.Pager} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.Pager, 9));
};


/**
 * @param {?proto.palm.lilac.auth.v1.Pager|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexRequest} returns this
*/
proto.palm.lilac.auth.v1.UserIndexRequest.prototype.setPager = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserIndexRequest} returns this
 */
proto.palm.lilac.auth.v1.UserIndexRequest.prototype.clearPager = function() {
  return this.setPager(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserIndexRequest.prototype.hasPager = function() {
  return jspb.Message.getField(this, 9) != null;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.UserIndexResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.UserIndexResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && proto.palm.lilac.auth.v1.Pagination.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserIndexResponse;
  return proto.palm.lilac.auth.v1.UserIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.UserIndexResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.UserIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new proto.palm.lilac.auth.v1.Pagination;
      reader.readMessage(value,proto.palm.lilac.auth.v1.Pagination.deserializeBinaryFromReader);
      msg.setPagination(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.UserIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.lilac.auth.v1.Pagination.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    detail: (f = msg.getDetail()) && proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.toObject(includeInstance, f),
    lang: jspb.Message.getFieldWithDefault(msg, 2, ""),
    timezone: jspb.Message.getFieldWithDefault(msg, 3, ""),
    signInCount: jspb.Message.getFieldWithDefault(msg, 21, 0),
    currentSignedInIp: jspb.Message.getFieldWithDefault(msg, 22, ""),
    currentSignedInAt: (f = msg.getCurrentSignedInAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    lastSignedInIp: jspb.Message.getFieldWithDefault(msg, 24, ""),
    lastSignedInAt: (f = msg.getLastSignedInAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    lockedAt: (f = msg.getLockedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    deletedAt: (f = msg.getDeletedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserIndexResponse.Item;
  return proto.palm.lilac.auth.v1.UserIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail;
      reader.readMessage(value,proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.deserializeBinaryFromReader);
      msg.setDetail(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setTimezone(value);
      break;
    case 21:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setSignInCount(value);
      break;
    case 22:
      var value = /** @type {string} */ (reader.readString());
      msg.setCurrentSignedInIp(value);
      break;
    case 23:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCurrentSignedInAt(value);
      break;
    case 24:
      var value = /** @type {string} */ (reader.readString());
      msg.setLastSignedInIp(value);
      break;
    case 25:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setLastSignedInAt(value);
      break;
    case 31:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setLockedAt(value);
      break;
    case 32:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setDeletedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDetail();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.serializeBinaryToWriter
    );
  }
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getTimezone();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getSignInCount();
  if (f !== 0) {
    writer.writeUint32(
      21,
      f
    );
  }
  f = message.getCurrentSignedInIp();
  if (f.length > 0) {
    writer.writeString(
      22,
      f
    );
  }
  f = message.getCurrentSignedInAt();
  if (f != null) {
    writer.writeMessage(
      23,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getLastSignedInIp();
  if (f.length > 0) {
    writer.writeString(
      24,
      f
    );
  }
  f = message.getLastSignedInAt();
  if (f != null) {
    writer.writeMessage(
      25,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getLockedAt();
  if (f != null) {
    writer.writeMessage(
      31,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getDeletedAt();
  if (f != null) {
    writer.writeMessage(
      32,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.toObject = function(includeInstance, msg) {
  var f, obj = {
    providerType: jspb.Message.getFieldWithDefault(msg, 1, 0),
    providerId: jspb.Message.getFieldWithDefault(msg, 2, ""),
    name: jspb.Message.getFieldWithDefault(msg, 3, ""),
    avatar: jspb.Message.getFieldWithDefault(msg, 4, ""),
    confirmedAt: (f = msg.getConfirmedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail;
  return proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.palm.lilac.rbac.v1.UserDetail.Provider.Type} */ (reader.readEnum());
      msg.setProviderType(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setProviderId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setAvatar(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setConfirmedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getProviderType();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getProviderId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getAvatar();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getConfirmedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional palm.lilac.rbac.v1.UserDetail.Provider.Type provider_type = 1;
 * @return {!proto.palm.lilac.rbac.v1.UserDetail.Provider.Type}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.getProviderType = function() {
  return /** @type {!proto.palm.lilac.rbac.v1.UserDetail.Provider.Type} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.palm.lilac.rbac.v1.UserDetail.Provider.Type} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.setProviderType = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional string provider_id = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.getProviderId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.setProviderId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string name = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string avatar = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.getAvatar = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.setAvatar = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional google.protobuf.Timestamp confirmed_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.getConfirmedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} returns this
*/
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.setConfirmedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.clearConfirmedAt = function() {
  return this.setConfirmedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail.prototype.hasConfirmedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * optional Detail detail = 1;
 * @return {?proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getDetail = function() {
  return /** @type{?proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail, 1));
};


/**
 * @param {?proto.palm.lilac.auth.v1.UserIndexResponse.Item.Detail|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setDetail = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.clearDetail = function() {
  return this.setDetail(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.hasDetail = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string lang = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string timezone = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getTimezone = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setTimezone = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional uint32 sign_in_count = 21;
 * @return {number}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getSignInCount = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 21, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setSignInCount = function(value) {
  return jspb.Message.setProto3IntField(this, 21, value);
};


/**
 * optional string current_signed_in_ip = 22;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getCurrentSignedInIp = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 22, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setCurrentSignedInIp = function(value) {
  return jspb.Message.setProto3StringField(this, 22, value);
};


/**
 * optional google.protobuf.Timestamp current_signed_in_at = 23;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getCurrentSignedInAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 23));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setCurrentSignedInAt = function(value) {
  return jspb.Message.setWrapperField(this, 23, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.clearCurrentSignedInAt = function() {
  return this.setCurrentSignedInAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.hasCurrentSignedInAt = function() {
  return jspb.Message.getField(this, 23) != null;
};


/**
 * optional string last_signed_in_ip = 24;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getLastSignedInIp = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 24, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setLastSignedInIp = function(value) {
  return jspb.Message.setProto3StringField(this, 24, value);
};


/**
 * optional google.protobuf.Timestamp last_signed_in_at = 25;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getLastSignedInAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 25));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setLastSignedInAt = function(value) {
  return jspb.Message.setWrapperField(this, 25, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.clearLastSignedInAt = function() {
  return this.setLastSignedInAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.hasLastSignedInAt = function() {
  return jspb.Message.getField(this, 25) != null;
};


/**
 * optional google.protobuf.Timestamp locked_at = 31;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getLockedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 31));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setLockedAt = function(value) {
  return jspb.Message.setWrapperField(this, 31, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.clearLockedAt = function() {
  return this.setLockedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.hasLockedAt = function() {
  return jspb.Message.getField(this, 31) != null;
};


/**
 * optional google.protobuf.Timestamp deleted_at = 32;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.getDeletedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 32));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.setDeletedAt = function(value) {
  return jspb.Message.setWrapperField(this, 32, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.clearDeletedAt = function() {
  return this.setDeletedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.Item.prototype.hasDeletedAt = function() {
  return jspb.Message.getField(this, 32) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.UserIndexResponse.Item>}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.UserIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.UserIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.UserIndexResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.UserIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.UserIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional Pagination pagination = 2;
 * @return {?proto.palm.lilac.auth.v1.Pagination}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.lilac.auth.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.Pagination, 2));
};


/**
 * @param {?proto.palm.lilac.auth.v1.Pagination|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserIndexResponse.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserSessionsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserSessionsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.UserSessionsResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && proto.palm.lilac.auth.v1.Pagination.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserSessionsResponse;
  return proto.palm.lilac.auth.v1.UserSessionsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserSessionsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.UserSessionsResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.UserSessionsResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new proto.palm.lilac.auth.v1.Pagination;
      reader.readMessage(value,proto.palm.lilac.auth.v1.Pagination.deserializeBinaryFromReader);
      msg.setPagination(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserSessionsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserSessionsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.UserSessionsResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.lilac.auth.v1.Pagination.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserSessionsResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    user: (f = msg.getUser()) && proto.palm.lilac.auth.v1.UserIndexResponse.Item.toObject(includeInstance, f),
    uid: jspb.Message.getFieldWithDefault(msg, 2, ""),
    ip: jspb.Message.getFieldWithDefault(msg, 3, ""),
    expiredAt: (f = msg.getExpiredAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    createdAt: (f = msg.getCreatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserSessionsResponse.Item;
  return proto.palm.lilac.auth.v1.UserSessionsResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.UserIndexResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.UserIndexResponse.Item.deserializeBinaryFromReader);
      msg.setUser(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setUid(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setIp(value);
      break;
    case 8:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setExpiredAt(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCreatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserSessionsResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUser();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.UserIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getUid();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getIp();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getExpiredAt();
  if (f != null) {
    writer.writeMessage(
      8,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional UserIndexResponse.Item user = 1;
 * @return {?proto.palm.lilac.auth.v1.UserIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.getUser = function() {
  return /** @type{?proto.palm.lilac.auth.v1.UserIndexResponse.Item} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.UserIndexResponse.Item, 1));
};


/**
 * @param {?proto.palm.lilac.auth.v1.UserIndexResponse.Item|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.setUser = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.clearUser = function() {
  return this.setUser(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.hasUser = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string uid = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.getUid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.setUid = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string ip = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.getIp = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.setIp = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional google.protobuf.Timestamp expired_at = 8;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.getExpiredAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 8));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.setExpiredAt = function(value) {
  return jspb.Message.setWrapperField(this, 8, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.clearExpiredAt = function() {
  return this.setExpiredAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.hasExpiredAt = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional google.protobuf.Timestamp created_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.Item.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.UserSessionsResponse.Item>}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.UserSessionsResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.UserSessionsResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.UserSessionsResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse} returns this
*/
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.UserSessionsResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional Pagination pagination = 2;
 * @return {?proto.palm.lilac.auth.v1.Pagination}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.lilac.auth.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.Pagination, 2));
};


/**
 * @param {?proto.palm.lilac.auth.v1.Pagination|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse} returns this
*/
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserSessionsResponse} returns this
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserSessionsResponse.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserSetPasswordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserSetPasswordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    user: jspb.Message.getFieldWithDefault(msg, 1, 0),
    password: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserSetPasswordRequest}
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserSetPasswordRequest;
  return proto.palm.lilac.auth.v1.UserSetPasswordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserSetPasswordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserSetPasswordRequest}
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setUser(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setPassword(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserSetPasswordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserSetPasswordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUser();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getPassword();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional uint32 user = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.prototype.getUser = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.UserSetPasswordRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.prototype.setUser = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string password = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserSetPasswordRequest} returns this
 */
proto.palm.lilac.auth.v1.UserSetPasswordRequest.prototype.setPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserAttachmentsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserAttachmentsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && proto.palm.lilac.auth.v1.Pagination.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserAttachmentsResponse;
  return proto.palm.lilac.auth.v1.UserAttachmentsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserAttachmentsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new proto.palm.lilac.auth.v1.Pagination;
      reader.readMessage(value,proto.palm.lilac.auth.v1.Pagination.deserializeBinaryFromReader);
      msg.setPagination(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserAttachmentsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserAttachmentsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.lilac.auth.v1.Pagination.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    bucket: jspb.Message.getFieldWithDefault(msg, 2, ""),
    name: jspb.Message.getFieldWithDefault(msg, 3, ""),
    title: jspb.Message.getFieldWithDefault(msg, 4, ""),
    size: jspb.Message.getFieldWithDefault(msg, 5, 0),
    contentType: jspb.Message.getFieldWithDefault(msg, 6, ""),
    deletedAt: (f = msg.getDeletedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    updatedAt: (f = msg.getUpdatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item;
  return proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setBucket(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setTitle(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setSize(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setContentType(value);
      break;
    case 8:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setDeletedAt(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setUpdatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getBucket();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getTitle();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getSize();
  if (f !== 0) {
    writer.writeInt64(
      5,
      f
    );
  }
  f = message.getContentType();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
  f = message.getDeletedAt();
  if (f != null) {
    writer.writeMessage(
      8,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getUpdatedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string bucket = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.getBucket = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.setBucket = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string name = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string title = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.setTitle = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional int64 size = 5;
 * @return {number}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.getSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.setSize = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};


/**
 * optional string content_type = 6;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.getContentType = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.setContentType = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};


/**
 * optional google.protobuf.Timestamp deleted_at = 8;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.getDeletedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 8));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.setDeletedAt = function(value) {
  return jspb.Message.setWrapperField(this, 8, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.clearDeletedAt = function() {
  return this.setDeletedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.hasDeletedAt = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional google.protobuf.Timestamp updated_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item>}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse} returns this
*/
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.UserAttachmentsResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional Pagination pagination = 2;
 * @return {?proto.palm.lilac.auth.v1.Pagination}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.lilac.auth.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.Pagination, 2));
};


/**
 * @param {?proto.palm.lilac.auth.v1.Pagination|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse} returns this
*/
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserAttachmentsResponse} returns this
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserAttachmentsResponse.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserRefreshRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserRefreshRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    ttl: (f = msg.getTtl()) && google_protobuf_duration_pb.Duration.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserRefreshRequest}
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserRefreshRequest;
  return proto.palm.lilac.auth.v1.UserRefreshRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserRefreshRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserRefreshRequest}
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new google_protobuf_duration_pb.Duration;
      reader.readMessage(value,google_protobuf_duration_pb.Duration.deserializeBinaryFromReader);
      msg.setTtl(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserRefreshRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserRefreshRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTtl();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      google_protobuf_duration_pb.Duration.serializeBinaryToWriter
    );
  }
};


/**
 * optional google.protobuf.Duration ttl = 1;
 * @return {?proto.google.protobuf.Duration}
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.prototype.getTtl = function() {
  return /** @type{?proto.google.protobuf.Duration} */ (
    jspb.Message.getWrapperField(this, google_protobuf_duration_pb.Duration, 1));
};


/**
 * @param {?proto.google.protobuf.Duration|undefined} value
 * @return {!proto.palm.lilac.auth.v1.UserRefreshRequest} returns this
*/
proto.palm.lilac.auth.v1.UserRefreshRequest.prototype.setTtl = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.UserRefreshRequest} returns this
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.prototype.clearTtl = function() {
  return this.setTtl(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.UserRefreshRequest.prototype.hasTtl = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.UserRefreshResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.UserRefreshResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.UserRefreshResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserRefreshResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    token: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.UserRefreshResponse}
 */
proto.palm.lilac.auth.v1.UserRefreshResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.UserRefreshResponse;
  return proto.palm.lilac.auth.v1.UserRefreshResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.UserRefreshResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.UserRefreshResponse}
 */
proto.palm.lilac.auth.v1.UserRefreshResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setToken(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.UserRefreshResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.UserRefreshResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.UserRefreshResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.UserRefreshResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string token = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.UserRefreshResponse.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.UserRefreshResponse} returns this
 */
proto.palm.lilac.auth.v1.UserRefreshResponse.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LocaleByLangRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    lang: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangRequest}
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LocaleByLangRequest;
  return proto.palm.lilac.auth.v1.LocaleByLangRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangRequest}
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LocaleByLangRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string lang = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangRequest} returns this
 */
proto.palm.lilac.auth.v1.LocaleByLangRequest.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LocaleByLangResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LocaleByLangResponse;
  return proto.palm.lilac.auth.v1.LocaleByLangResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.LocaleByLangResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LocaleByLangResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    code: jspb.Message.getFieldWithDefault(msg, 1, ""),
    message: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LocaleByLangResponse.Item;
  return proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setMessage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getMessage();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string code = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string message = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.Item.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item>}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.LocaleByLangResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse} returns this
*/
proto.palm.lilac.auth.v1.LocaleByLangResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse.Item}
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.LocaleByLangResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.LocaleByLangResponse} returns this
 */
proto.palm.lilac.auth.v1.LocaleByLangResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LocaleIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LocaleIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && proto.palm.lilac.auth.v1.Pagination.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LocaleIndexResponse;
  return proto.palm.lilac.auth.v1.LocaleIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LocaleIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.LocaleIndexResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new proto.palm.lilac.auth.v1.Pagination;
      reader.readMessage(value,proto.palm.lilac.auth.v1.Pagination.deserializeBinaryFromReader);
      msg.setPagination(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LocaleIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LocaleIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.lilac.auth.v1.Pagination.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    lang: jspb.Message.getFieldWithDefault(msg, 2, ""),
    code: jspb.Message.getFieldWithDefault(msg, 3, ""),
    message: jspb.Message.getFieldWithDefault(msg, 4, ""),
    updatedAt: (f = msg.getUpdatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LocaleIndexResponse.Item;
  return proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setMessage(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setUpdatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getMessage();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getUpdatedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string lang = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string code = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string message = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional google.protobuf.Timestamp updated_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.Item.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item>}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.LocaleIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.LocaleIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional Pagination pagination = 2;
 * @return {?proto.palm.lilac.auth.v1.Pagination}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.lilac.auth.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.Pagination, 2));
};


/**
 * @param {?proto.palm.lilac.auth.v1.Pagination|undefined} value
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.LocaleIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.LocaleIndexResponse.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LocaleSetRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LocaleSetRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    lang: jspb.Message.getFieldWithDefault(msg, 1, ""),
    code: jspb.Message.getFieldWithDefault(msg, 2, ""),
    message: jspb.Message.getFieldWithDefault(msg, 3, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LocaleSetRequest}
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LocaleSetRequest;
  return proto.palm.lilac.auth.v1.LocaleSetRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LocaleSetRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LocaleSetRequest}
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setMessage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LocaleSetRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LocaleSetRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getMessage();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
};


/**
 * optional string lang = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleSetRequest} returns this
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string code = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleSetRequest} returns this
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string message = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LocaleSetRequest} returns this
 */
proto.palm.lilac.auth.v1.LocaleSetRequest.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.KvSetRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.KvSetRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.KvSetRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    key: jspb.Message.getFieldWithDefault(msg, 1, ""),
    value: msg.getValue_asB64(),
    encrypt: jspb.Message.getBooleanFieldWithDefault(msg, 3, false)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.KvSetRequest}
 */
proto.palm.lilac.auth.v1.KvSetRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.KvSetRequest;
  return proto.palm.lilac.auth.v1.KvSetRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.KvSetRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.KvSetRequest}
 */
proto.palm.lilac.auth.v1.KvSetRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setKey(value);
      break;
    case 2:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setValue(value);
      break;
    case 3:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setEncrypt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.KvSetRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.KvSetRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.KvSetRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getValue_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      2,
      f
    );
  }
  f = message.getEncrypt();
  if (f) {
    writer.writeBool(
      3,
      f
    );
  }
};


/**
 * optional string key = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.KvSetRequest} returns this
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.setKey = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional bytes value = 2;
 * @return {!(string|Uint8Array)}
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.getValue = function() {
  return /** @type {!(string|Uint8Array)} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * optional bytes value = 2;
 * This is a type-conversion wrapper around `getValue()`
 * @return {string}
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.getValue_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getValue()));
};


/**
 * optional bytes value = 2;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getValue()`
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.getValue_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getValue()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.palm.lilac.auth.v1.KvSetRequest} returns this
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.setValue = function(value) {
  return jspb.Message.setProto3BytesField(this, 2, value);
};


/**
 * optional bool encrypt = 3;
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.getEncrypt = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 3, false));
};


/**
 * @param {boolean} value
 * @return {!proto.palm.lilac.auth.v1.KvSetRequest} returns this
 */
proto.palm.lilac.auth.v1.KvSetRequest.prototype.setEncrypt = function(value) {
  return jspb.Message.setProto3BooleanField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.KvGetRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.KvGetRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.KvGetRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.KvGetRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    key: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.KvGetRequest}
 */
proto.palm.lilac.auth.v1.KvGetRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.KvGetRequest;
  return proto.palm.lilac.auth.v1.KvGetRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.KvGetRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.KvGetRequest}
 */
proto.palm.lilac.auth.v1.KvGetRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setKey(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.KvGetRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.KvGetRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.KvGetRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.KvGetRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string key = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.KvGetRequest.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.KvGetRequest} returns this
 */
proto.palm.lilac.auth.v1.KvGetRequest.prototype.setKey = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.KvGetResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.KvGetResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.KvGetResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.KvGetResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    value: msg.getValue_asB64()
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.KvGetResponse}
 */
proto.palm.lilac.auth.v1.KvGetResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.KvGetResponse;
  return proto.palm.lilac.auth.v1.KvGetResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.KvGetResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.KvGetResponse}
 */
proto.palm.lilac.auth.v1.KvGetResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setValue(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.KvGetResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.KvGetResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.KvGetResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.KvGetResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getValue_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      1,
      f
    );
  }
};


/**
 * optional bytes value = 1;
 * @return {!(string|Uint8Array)}
 */
proto.palm.lilac.auth.v1.KvGetResponse.prototype.getValue = function() {
  return /** @type {!(string|Uint8Array)} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * optional bytes value = 1;
 * This is a type-conversion wrapper around `getValue()`
 * @return {string}
 */
proto.palm.lilac.auth.v1.KvGetResponse.prototype.getValue_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getValue()));
};


/**
 * optional bytes value = 1;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getValue()`
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.KvGetResponse.prototype.getValue_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getValue()));
};


/**
 * @param {!(string|Uint8Array)} value
 * @return {!proto.palm.lilac.auth.v1.KvGetResponse} returns this
 */
proto.palm.lilac.auth.v1.KvGetResponse.prototype.setValue = function(value) {
  return jspb.Message.setProto3BytesField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteIcpCode.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteIcpCode.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteIcpCode} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteIcpCode.toObject = function(includeInstance, msg) {
  var f, obj = {
    code: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteIcpCode}
 */
proto.palm.lilac.auth.v1.SiteIcpCode.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteIcpCode;
  return proto.palm.lilac.auth.v1.SiteIcpCode.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteIcpCode} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteIcpCode}
 */
proto.palm.lilac.auth.v1.SiteIcpCode.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteIcpCode.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteIcpCode.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteIcpCode} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteIcpCode.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string code = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteIcpCode.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteIcpCode} returns this
 */
proto.palm.lilac.auth.v1.SiteIcpCode.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteGabCode.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteGabCode.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteGabCode} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteGabCode.toObject = function(includeInstance, msg) {
  var f, obj = {
    code: jspb.Message.getFieldWithDefault(msg, 1, ""),
    name: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteGabCode}
 */
proto.palm.lilac.auth.v1.SiteGabCode.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteGabCode;
  return proto.palm.lilac.auth.v1.SiteGabCode.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteGabCode} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteGabCode}
 */
proto.palm.lilac.auth.v1.SiteGabCode.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteGabCode.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteGabCode.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteGabCode} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteGabCode.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string code = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteGabCode.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteGabCode} returns this
 */
proto.palm.lilac.auth.v1.SiteGabCode.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteGabCode.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteGabCode} returns this
 */
proto.palm.lilac.auth.v1.SiteGabCode.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteBaidu.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteBaidu.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteBaidu} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteBaidu.toObject = function(includeInstance, msg) {
  var f, obj = {
    code: jspb.Message.getFieldWithDefault(msg, 1, ""),
    content: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteBaidu}
 */
proto.palm.lilac.auth.v1.SiteBaidu.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteBaidu;
  return proto.palm.lilac.auth.v1.SiteBaidu.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteBaidu} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteBaidu}
 */
proto.palm.lilac.auth.v1.SiteBaidu.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setContent(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteBaidu.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteBaidu.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteBaidu} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteBaidu.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getContent();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string code = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteBaidu.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteBaidu} returns this
 */
proto.palm.lilac.auth.v1.SiteBaidu.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string content = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteBaidu.prototype.getContent = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteBaidu} returns this
 */
proto.palm.lilac.auth.v1.SiteBaidu.prototype.setContent = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteGoogle.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteGoogle.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteGoogle} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteGoogle.toObject = function(includeInstance, msg) {
  var f, obj = {
    code: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteGoogle}
 */
proto.palm.lilac.auth.v1.SiteGoogle.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteGoogle;
  return proto.palm.lilac.auth.v1.SiteGoogle.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteGoogle} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteGoogle}
 */
proto.palm.lilac.auth.v1.SiteGoogle.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteGoogle.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteGoogle.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteGoogle} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteGoogle.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string code = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteGoogle.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteGoogle} returns this
 */
proto.palm.lilac.auth.v1.SiteGoogle.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteIndexNow.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteIndexNow.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteIndexNow} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteIndexNow.toObject = function(includeInstance, msg) {
  var f, obj = {
    key: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteIndexNow}
 */
proto.palm.lilac.auth.v1.SiteIndexNow.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteIndexNow;
  return proto.palm.lilac.auth.v1.SiteIndexNow.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteIndexNow} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteIndexNow}
 */
proto.palm.lilac.auth.v1.SiteIndexNow.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setKey(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteIndexNow.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteIndexNow.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteIndexNow} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteIndexNow.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string key = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteIndexNow.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteIndexNow} returns this
 */
proto.palm.lilac.auth.v1.SiteIndexNow.prototype.setKey = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteGoogleReCaptcha} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.toObject = function(includeInstance, msg) {
  var f, obj = {
    siteKey: jspb.Message.getFieldWithDefault(msg, 1, ""),
    secret: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteGoogleReCaptcha}
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteGoogleReCaptcha;
  return proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteGoogleReCaptcha} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteGoogleReCaptcha}
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setSiteKey(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setSecret(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteGoogleReCaptcha} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSiteKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getSecret();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string site_key = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.prototype.getSiteKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteGoogleReCaptcha} returns this
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.prototype.setSiteKey = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string secret = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.prototype.getSecret = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteGoogleReCaptcha} returns this
 */
proto.palm.lilac.auth.v1.SiteGoogleReCaptcha.prototype.setSecret = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.SiteAuthors.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteAuthors.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteAuthors.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteAuthors} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteAuthors.toObject = function(includeInstance, msg) {
  var f, obj = {
    itesList: jspb.Message.toObjectList(msg.getItesList(),
    proto.palm.lilac.auth.v1.SiteAuthors.Item.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors}
 */
proto.palm.lilac.auth.v1.SiteAuthors.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteAuthors;
  return proto.palm.lilac.auth.v1.SiteAuthors.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteAuthors} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors}
 */
proto.palm.lilac.auth.v1.SiteAuthors.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.SiteAuthors.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.SiteAuthors.Item.deserializeBinaryFromReader);
      msg.addItes(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteAuthors.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteAuthors.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteAuthors} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteAuthors.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItesList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.SiteAuthors.Item.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteAuthors.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteAuthors.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    name: jspb.Message.getFieldWithDefault(msg, 1, ""),
    email: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors.Item}
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteAuthors.Item;
  return proto.palm.lilac.auth.v1.SiteAuthors.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteAuthors.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors.Item}
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setEmail(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteAuthors.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteAuthors.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEmail();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string name = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors.Item} returns this
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string email = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.prototype.getEmail = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors.Item} returns this
 */
proto.palm.lilac.auth.v1.SiteAuthors.Item.prototype.setEmail = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * repeated Item ites = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.SiteAuthors.Item>}
 */
proto.palm.lilac.auth.v1.SiteAuthors.prototype.getItesList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.SiteAuthors.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.SiteAuthors.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.SiteAuthors.Item>} value
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors} returns this
*/
proto.palm.lilac.auth.v1.SiteAuthors.prototype.setItesList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.SiteAuthors.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors.Item}
 */
proto.palm.lilac.auth.v1.SiteAuthors.prototype.addItes = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.SiteAuthors.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.SiteAuthors} returns this
 */
proto.palm.lilac.auth.v1.SiteAuthors.prototype.clearItesList = function() {
  return this.setItesList([]);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.SiteKeywords.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteKeywords.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteKeywords.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteKeywords} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteKeywords.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: (f = jspb.Message.getRepeatedField(msg, 1)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteKeywords}
 */
proto.palm.lilac.auth.v1.SiteKeywords.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteKeywords;
  return proto.palm.lilac.auth.v1.SiteKeywords.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteKeywords} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteKeywords}
 */
proto.palm.lilac.auth.v1.SiteKeywords.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.addItems(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteKeywords.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteKeywords.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteKeywords} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteKeywords.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      1,
      f
    );
  }
};


/**
 * repeated string items = 1;
 * @return {!Array<string>}
 */
proto.palm.lilac.auth.v1.SiteKeywords.prototype.getItemsList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 1));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.palm.lilac.auth.v1.SiteKeywords} returns this
 */
proto.palm.lilac.auth.v1.SiteKeywords.prototype.setItemsList = function(value) {
  return jspb.Message.setField(this, 1, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.SiteKeywords} returns this
 */
proto.palm.lilac.auth.v1.SiteKeywords.prototype.addItems = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 1, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.SiteKeywords} returns this
 */
proto.palm.lilac.auth.v1.SiteKeywords.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteInfo.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteInfo} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteInfo.toObject = function(includeInstance, msg) {
  var f, obj = {
    title: jspb.Message.getFieldWithDefault(msg, 1, ""),
    subhead: jspb.Message.getFieldWithDefault(msg, 2, ""),
    description: jspb.Message.getFieldWithDefault(msg, 3, ""),
    copyright: jspb.Message.getFieldWithDefault(msg, 4, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteInfo}
 */
proto.palm.lilac.auth.v1.SiteInfo.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteInfo;
  return proto.palm.lilac.auth.v1.SiteInfo.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteInfo} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteInfo}
 */
proto.palm.lilac.auth.v1.SiteInfo.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setTitle(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setSubhead(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setDescription(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setCopyright(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteInfo.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteInfo} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteInfo.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTitle();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getSubhead();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getDescription();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getCopyright();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional string title = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteInfo} returns this
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.setTitle = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string subhead = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.getSubhead = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteInfo} returns this
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.setSubhead = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string description = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.getDescription = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteInfo} returns this
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.setDescription = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string copyright = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.getCopyright = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteInfo} returns this
 */
proto.palm.lilac.auth.v1.SiteInfo.prototype.setCopyright = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.SiteFavicon.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.SiteFavicon.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.SiteFavicon} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteFavicon.toObject = function(includeInstance, msg) {
  var f, obj = {
    url: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.SiteFavicon}
 */
proto.palm.lilac.auth.v1.SiteFavicon.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.SiteFavicon;
  return proto.palm.lilac.auth.v1.SiteFavicon.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.SiteFavicon} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.SiteFavicon}
 */
proto.palm.lilac.auth.v1.SiteFavicon.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setUrl(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.SiteFavicon.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.SiteFavicon.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.SiteFavicon} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.SiteFavicon.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUrl();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string url = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.SiteFavicon.prototype.getUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.SiteFavicon} returns this
 */
proto.palm.lilac.auth.v1.SiteFavicon.prototype.setUrl = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LeaveWordCreateRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    content: jspb.Message.getFieldWithDefault(msg, 1, ""),
    editor: jspb.Message.getFieldWithDefault(msg, 2, 0),
    ip: jspb.Message.getFieldWithDefault(msg, 3, ""),
    lang: jspb.Message.getFieldWithDefault(msg, 4, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest}
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LeaveWordCreateRequest;
  return proto.palm.lilac.auth.v1.LeaveWordCreateRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest}
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setContent(value);
      break;
    case 2:
      var value = /** @type {!proto.palm.lilac.auth.v1.Editor} */ (reader.readEnum());
      msg.setEditor(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setIp(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LeaveWordCreateRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getContent();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getEditor();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getIp();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional string content = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.getContent = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.setContent = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional Editor editor = 2;
 * @return {!proto.palm.lilac.auth.v1.Editor}
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.getEditor = function() {
  return /** @type {!proto.palm.lilac.auth.v1.Editor} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.palm.lilac.auth.v1.Editor} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.setEditor = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional string ip = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.getIp = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.setIp = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string lang = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordCreateRequest.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LeaveWordIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && proto.palm.lilac.auth.v1.Pagination.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LeaveWordIndexResponse;
  return proto.palm.lilac.auth.v1.LeaveWordIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new proto.palm.lilac.auth.v1.Pagination;
      reader.readMessage(value,proto.palm.lilac.auth.v1.Pagination.deserializeBinaryFromReader);
      msg.setPagination(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LeaveWordIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.lilac.auth.v1.Pagination.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    ip: jspb.Message.getFieldWithDefault(msg, 2, ""),
    lang: jspb.Message.getFieldWithDefault(msg, 3, ""),
    content: jspb.Message.getFieldWithDefault(msg, 4, ""),
    editor: jspb.Message.getFieldWithDefault(msg, 5, 0),
    publishedAt: (f = msg.getPublishedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    deletedAt: (f = msg.getDeletedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    createdAt: (f = msg.getCreatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item;
  return proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setIp(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setContent(value);
      break;
    case 5:
      var value = /** @type {!proto.palm.lilac.auth.v1.Editor} */ (reader.readEnum());
      msg.setEditor(value);
      break;
    case 7:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setPublishedAt(value);
      break;
    case 8:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setDeletedAt(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCreatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getIp();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getContent();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getEditor();
  if (f !== 0.0) {
    writer.writeEnum(
      5,
      f
    );
  }
  f = message.getPublishedAt();
  if (f != null) {
    writer.writeMessage(
      7,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getDeletedAt();
  if (f != null) {
    writer.writeMessage(
      8,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string ip = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.getIp = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.setIp = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string lang = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string content = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.getContent = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.setContent = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional Editor editor = 5;
 * @return {!proto.palm.lilac.auth.v1.Editor}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.getEditor = function() {
  return /** @type {!proto.palm.lilac.auth.v1.Editor} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {!proto.palm.lilac.auth.v1.Editor} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.setEditor = function(value) {
  return jspb.Message.setProto3EnumField(this, 5, value);
};


/**
 * optional google.protobuf.Timestamp published_at = 7;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.getPublishedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 7));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.setPublishedAt = function(value) {
  return jspb.Message.setWrapperField(this, 7, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.clearPublishedAt = function() {
  return this.setPublishedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.hasPublishedAt = function() {
  return jspb.Message.getField(this, 7) != null;
};


/**
 * optional google.protobuf.Timestamp deleted_at = 8;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.getDeletedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 8));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.setDeletedAt = function(value) {
  return jspb.Message.setWrapperField(this, 8, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.clearDeletedAt = function() {
  return this.setDeletedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.hasDeletedAt = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional google.protobuf.Timestamp created_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item>}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.LeaveWordIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional Pagination pagination = 2;
 * @return {?proto.palm.lilac.auth.v1.Pagination}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.lilac.auth.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, proto.palm.lilac.auth.v1.Pagination, 2));
};


/**
 * @param {?proto.palm.lilac.auth.v1.Pagination|undefined} value
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.LeaveWordIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.LeaveWordIndexResponse.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.TagCreateRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.TagCreateRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.TagCreateRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.TagCreateRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    name: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.TagCreateRequest}
 */
proto.palm.lilac.auth.v1.TagCreateRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.TagCreateRequest;
  return proto.palm.lilac.auth.v1.TagCreateRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.TagCreateRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.TagCreateRequest}
 */
proto.palm.lilac.auth.v1.TagCreateRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.TagCreateRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.TagCreateRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.TagCreateRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.TagCreateRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string name = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.TagCreateRequest.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.TagCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.TagCreateRequest.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.TagEditRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.TagEditRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.TagEditRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.TagEditRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    name: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.TagEditRequest}
 */
proto.palm.lilac.auth.v1.TagEditRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.TagEditRequest;
  return proto.palm.lilac.auth.v1.TagEditRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.TagEditRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.TagEditRequest}
 */
proto.palm.lilac.auth.v1.TagEditRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.TagEditRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.TagEditRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.TagEditRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.TagEditRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.TagEditRequest.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.TagEditRequest} returns this
 */
proto.palm.lilac.auth.v1.TagEditRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.TagEditRequest.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.TagEditRequest} returns this
 */
proto.palm.lilac.auth.v1.TagEditRequest.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.TagIndexResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.TagIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.TagIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.TagIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.TagIndexResponse.Item.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.TagIndexResponse;
  return proto.palm.lilac.auth.v1.TagIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.TagIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.TagIndexResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.TagIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.TagIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.TagIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.TagIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.TagIndexResponse.Item.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.TagIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.TagIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    name: jspb.Message.getFieldWithDefault(msg, 2, ""),
    sortOrder: jspb.Message.getFieldWithDefault(msg, 8, 0),
    updatedAt: (f = msg.getUpdatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.TagIndexResponse.Item;
  return proto.palm.lilac.auth.v1.TagIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.TagIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 8:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSortOrder(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setUpdatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.TagIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.TagIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getSortOrder();
  if (f !== 0) {
    writer.writeInt32(
      8,
      f
    );
  }
  f = message.getUpdatedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int32 sort_order = 8;
 * @return {number}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.getSortOrder = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 8, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.setSortOrder = function(value) {
  return jspb.Message.setProto3IntField(this, 8, value);
};


/**
 * optional google.protobuf.Timestamp updated_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.Item.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.TagIndexResponse.Item>}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.TagIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.TagIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.TagIndexResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.TagIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.TagIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.TagIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.TagIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.TagIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.TagIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.CategoryCreateRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.CategoryCreateRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    code: jspb.Message.getFieldWithDefault(msg, 1, ""),
    left: jspb.Message.getFieldWithDefault(msg, 11, 0),
    right: jspb.Message.getFieldWithDefault(msg, 12, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.CategoryCreateRequest}
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.CategoryCreateRequest;
  return proto.palm.lilac.auth.v1.CategoryCreateRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.CategoryCreateRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.CategoryCreateRequest}
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    case 11:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setLeft(value);
      break;
    case 12:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setRight(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.CategoryCreateRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.CategoryCreateRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getLeft();
  if (f !== 0) {
    writer.writeInt32(
      11,
      f
    );
  }
  f = message.getRight();
  if (f !== 0) {
    writer.writeInt32(
      12,
      f
    );
  }
};


/**
 * optional string code = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.CategoryCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int32 left = 11;
 * @return {number}
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.prototype.getLeft = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 11, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.CategoryCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.prototype.setLeft = function(value) {
  return jspb.Message.setProto3IntField(this, 11, value);
};


/**
 * optional int32 right = 12;
 * @return {number}
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.prototype.getRight = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 12, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.CategoryCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.CategoryCreateRequest.prototype.setRight = function(value) {
  return jspb.Message.setProto3IntField(this, 12, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.CategoryEditRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.CategoryEditRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    code: jspb.Message.getFieldWithDefault(msg, 2, ""),
    left: jspb.Message.getFieldWithDefault(msg, 11, 0),
    right: jspb.Message.getFieldWithDefault(msg, 12, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.CategoryEditRequest}
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.CategoryEditRequest;
  return proto.palm.lilac.auth.v1.CategoryEditRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.CategoryEditRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.CategoryEditRequest}
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    case 11:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setLeft(value);
      break;
    case 12:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setRight(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.CategoryEditRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.CategoryEditRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getLeft();
  if (f !== 0) {
    writer.writeInt32(
      11,
      f
    );
  }
  f = message.getRight();
  if (f !== 0) {
    writer.writeInt32(
      12,
      f
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.CategoryEditRequest} returns this
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string code = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.CategoryEditRequest} returns this
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int32 left = 11;
 * @return {number}
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.getLeft = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 11, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.CategoryEditRequest} returns this
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.setLeft = function(value) {
  return jspb.Message.setProto3IntField(this, 11, value);
};


/**
 * optional int32 right = 12;
 * @return {number}
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.getRight = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 12, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.CategoryEditRequest} returns this
 */
proto.palm.lilac.auth.v1.CategoryEditRequest.prototype.setRight = function(value) {
  return jspb.Message.setProto3IntField(this, 12, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.CategoryIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.CategoryIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.CategoryIndexResponse;
  return proto.palm.lilac.auth.v1.CategoryIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.CategoryIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.CategoryIndexResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.CategoryIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.CategoryIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    code: jspb.Message.getFieldWithDefault(msg, 2, ""),
    left: jspb.Message.getFieldWithDefault(msg, 3, 0),
    right: jspb.Message.getFieldWithDefault(msg, 4, 0),
    updatedAt: (f = msg.getUpdatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.CategoryIndexResponse.Item;
  return proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setCode(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setLeft(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setRight(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setUpdatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getCode();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getLeft();
  if (f !== 0) {
    writer.writeInt32(
      3,
      f
    );
  }
  f = message.getRight();
  if (f !== 0) {
    writer.writeInt32(
      4,
      f
    );
  }
  f = message.getUpdatedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string code = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int32 left = 3;
 * @return {number}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.getLeft = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.setLeft = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional int32 right = 4;
 * @return {number}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.getRight = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.setRight = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};


/**
 * optional google.protobuf.Timestamp updated_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.Item.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item>}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.CategoryIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.CategoryIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.CategoryIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.CategoryIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.CategoryIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.NotificationCreateRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.NotificationCreateRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    title: jspb.Message.getFieldWithDefault(msg, 1, ""),
    body: jspb.Message.getFieldWithDefault(msg, 2, ""),
    summary: jspb.Message.getFieldWithDefault(msg, 3, ""),
    url: jspb.Message.getFieldWithDefault(msg, 4, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.NotificationCreateRequest}
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.NotificationCreateRequest;
  return proto.palm.lilac.auth.v1.NotificationCreateRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.NotificationCreateRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.NotificationCreateRequest}
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setTitle(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setBody(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setSummary(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setUrl(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.NotificationCreateRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.NotificationCreateRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTitle();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getBody();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getSummary();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getUrl();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
};


/**
 * optional string title = 1;
 * @return {string}
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.NotificationCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.setTitle = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string body = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.getBody = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.NotificationCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.setBody = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string summary = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.getSummary = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.NotificationCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.setSummary = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string url = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.getUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.NotificationCreateRequest} returns this
 */
proto.palm.lilac.auth.v1.NotificationCreateRequest.prototype.setUrl = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.NotificationIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.NotificationIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.NotificationIndexResponse;
  return proto.palm.lilac.auth.v1.NotificationIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.NotificationIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.lilac.auth.v1.NotificationIndexResponse.Item;
      reader.readMessage(value,proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.NotificationIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.NotificationIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.serializeBinaryToWriter
    );
  }
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    title: jspb.Message.getFieldWithDefault(msg, 2, ""),
    body: jspb.Message.getFieldWithDefault(msg, 3, ""),
    summary: jspb.Message.getFieldWithDefault(msg, 4, ""),
    url: jspb.Message.getFieldWithDefault(msg, 5, ""),
    readAt: (f = msg.getReadAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
    updatedAt: (f = msg.getUpdatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.lilac.auth.v1.NotificationIndexResponse.Item;
  return proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint32());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setTitle(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setBody(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setSummary(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setUrl(value);
      break;
    case 8:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setReadAt(value);
      break;
    case 9:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setUpdatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeUint32(
      1,
      f
    );
  }
  f = message.getTitle();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getBody();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getSummary();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getUrl();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = message.getReadAt();
  if (f != null) {
    writer.writeMessage(
      8,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getUpdatedAt();
  if (f != null) {
    writer.writeMessage(
      9,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional uint32 id = 1;
 * @return {number}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string title = 2;
 * @return {string}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.setTitle = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string body = 3;
 * @return {string}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.getBody = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.setBody = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string summary = 4;
 * @return {string}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.getSummary = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.setSummary = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string url = 5;
 * @return {string}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.getUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.setUrl = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * optional google.protobuf.Timestamp read_at = 8;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.getReadAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 8));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.setReadAt = function(value) {
  return jspb.Message.setWrapperField(this, 8, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.clearReadAt = function() {
  return this.setReadAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.hasReadAt = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional google.protobuf.Timestamp updated_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
*/
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item} returns this
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.Item.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item>}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.lilac.auth.v1.NotificationIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item>} value
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse} returns this
*/
proto.palm.lilac.auth.v1.NotificationIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse.Item}
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.lilac.auth.v1.NotificationIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.lilac.auth.v1.NotificationIndexResponse} returns this
 */
proto.palm.lilac.auth.v1.NotificationIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * @enum {number}
 */
proto.palm.lilac.auth.v1.Editor = {
  TEXTAREA: 0,
  QUILL: 1
};

goog.object.extend(exports, proto.palm.lilac.auth.v1);
