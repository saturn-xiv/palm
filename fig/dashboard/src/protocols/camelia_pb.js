// source: camelia.proto
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
var azalea_pb = require('./azalea_pb.js');
goog.object.extend(proto, azalea_pb);
goog.exportSymbol('proto.palm.camelia.v1.AttachmentIndexResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.AttachmentIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.camelia.v1.AttachmentResourceRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.AttachmentShowRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.AttachmentShowResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.AttachmentUpdateRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.AttachmentUploadUrlRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.AttachmentUploadUrlResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.BaiduProfile', null, global);
goog.exportSymbol('proto.palm.camelia.v1.GoogleProfile', null, global);
goog.exportSymbol('proto.palm.camelia.v1.IndexNowProfile', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LeaveWordCreateRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LeaveWordIndexResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LeaveWordIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LeaveWordIndexResponse.Item.Status', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LocaleByLangRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LocaleByLangResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LocaleIndexResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LocaleIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.camelia.v1.LocaleSetRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.Menu', null, global);
goog.exportSymbol('proto.palm.camelia.v1.Operation', null, global);
goog.exportSymbol('proto.palm.camelia.v1.Permission', null, global);
goog.exportSymbol('proto.palm.camelia.v1.ReCaptchaProfile', null, global);
goog.exportSymbol('proto.palm.camelia.v1.Resource', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteLayoutResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteLayoutResponse.Author', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteLayoutResponse.GabCode', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteLayoutResponse.IcpCode', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteSetFaviconRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteSetInfoRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteSetKeywordsRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteStatusResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteStatusResponse.OpenSearch', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteStatusResponse.PostgreSql', null, global);
goog.exportSymbol('proto.palm.camelia.v1.SiteStatusResponse.Redis', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserCancelRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserCancelResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserCancelResponse.PayloadCase', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserChangePasswordRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserEmailRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserEmailRequest.UserCase', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserIndexResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserIndexResponse.Item', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserIndexResponse.Item.Type', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserInfo', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserLogsResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserLogsResponse.Item', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserLogsResponse.Item.Level', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserOauth2State', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserResetPasswordRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserSetLocationRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserSetPasswordRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserSetProfileRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserSignInByPasswordRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserSignInByPasswordRequest.WhoCase', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserSignInResponse', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserSignUpByEmailRequest', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserTokenAction', null, global);
goog.exportSymbol('proto.palm.camelia.v1.UserTokenRequest', null, global);
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
proto.palm.camelia.v1.UserInfo = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserInfo, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserInfo.displayName = 'proto.palm.camelia.v1.UserInfo';
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
proto.palm.camelia.v1.UserOauth2State = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserOauth2State, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserOauth2State.displayName = 'proto.palm.camelia.v1.UserOauth2State';
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
proto.palm.camelia.v1.Resource = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.Resource, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.Resource.displayName = 'proto.palm.camelia.v1.Resource';
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
proto.palm.camelia.v1.Permission = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.Permission, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.Permission.displayName = 'proto.palm.camelia.v1.Permission';
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
proto.palm.camelia.v1.Menu = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.Menu.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.Menu, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.Menu.displayName = 'proto.palm.camelia.v1.Menu';
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
proto.palm.camelia.v1.UserIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.UserIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.UserIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserIndexResponse.displayName = 'proto.palm.camelia.v1.UserIndexResponse';
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
proto.palm.camelia.v1.UserIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserIndexResponse.Item.displayName = 'proto.palm.camelia.v1.UserIndexResponse.Item';
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
proto.palm.camelia.v1.UserCancelRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserCancelRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserCancelRequest.displayName = 'proto.palm.camelia.v1.UserCancelRequest';
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
proto.palm.camelia.v1.UserCancelResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.palm.camelia.v1.UserCancelResponse.oneofGroups_);
};
goog.inherits(proto.palm.camelia.v1.UserCancelResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserCancelResponse.displayName = 'proto.palm.camelia.v1.UserCancelResponse';
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
proto.palm.camelia.v1.UserSetProfileRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserSetProfileRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserSetProfileRequest.displayName = 'proto.palm.camelia.v1.UserSetProfileRequest';
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
proto.palm.camelia.v1.UserSetLocationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserSetLocationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserSetLocationRequest.displayName = 'proto.palm.camelia.v1.UserSetLocationRequest';
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
proto.palm.camelia.v1.UserChangePasswordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserChangePasswordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserChangePasswordRequest.displayName = 'proto.palm.camelia.v1.UserChangePasswordRequest';
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
proto.palm.camelia.v1.UserSetPasswordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserSetPasswordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserSetPasswordRequest.displayName = 'proto.palm.camelia.v1.UserSetPasswordRequest';
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
proto.palm.camelia.v1.UserResetPasswordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserResetPasswordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserResetPasswordRequest.displayName = 'proto.palm.camelia.v1.UserResetPasswordRequest';
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
proto.palm.camelia.v1.UserEmailRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.palm.camelia.v1.UserEmailRequest.oneofGroups_);
};
goog.inherits(proto.palm.camelia.v1.UserEmailRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserEmailRequest.displayName = 'proto.palm.camelia.v1.UserEmailRequest';
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
proto.palm.camelia.v1.UserTokenRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserTokenRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserTokenRequest.displayName = 'proto.palm.camelia.v1.UserTokenRequest';
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
proto.palm.camelia.v1.UserSignInResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.UserSignInResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.UserSignInResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserSignInResponse.displayName = 'proto.palm.camelia.v1.UserSignInResponse';
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
proto.palm.camelia.v1.UserSignInByPasswordRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_);
};
goog.inherits(proto.palm.camelia.v1.UserSignInByPasswordRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserSignInByPasswordRequest.displayName = 'proto.palm.camelia.v1.UserSignInByPasswordRequest';
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
proto.palm.camelia.v1.UserSignUpByEmailRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserSignUpByEmailRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserSignUpByEmailRequest.displayName = 'proto.palm.camelia.v1.UserSignUpByEmailRequest';
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
proto.palm.camelia.v1.UserLogsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.UserLogsResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.UserLogsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserLogsResponse.displayName = 'proto.palm.camelia.v1.UserLogsResponse';
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
proto.palm.camelia.v1.UserLogsResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.UserLogsResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.UserLogsResponse.Item.displayName = 'proto.palm.camelia.v1.UserLogsResponse.Item';
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
proto.palm.camelia.v1.AttachmentShowRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.AttachmentShowRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.AttachmentShowRequest.displayName = 'proto.palm.camelia.v1.AttachmentShowRequest';
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
proto.palm.camelia.v1.AttachmentShowResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.AttachmentShowResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.AttachmentShowResponse.displayName = 'proto.palm.camelia.v1.AttachmentShowResponse';
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
proto.palm.camelia.v1.AttachmentResourceRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.AttachmentResourceRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.AttachmentResourceRequest.displayName = 'proto.palm.camelia.v1.AttachmentResourceRequest';
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
proto.palm.camelia.v1.AttachmentUpdateRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.AttachmentUpdateRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.AttachmentUpdateRequest.displayName = 'proto.palm.camelia.v1.AttachmentUpdateRequest';
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
proto.palm.camelia.v1.AttachmentUploadUrlRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.AttachmentUploadUrlRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.AttachmentUploadUrlRequest.displayName = 'proto.palm.camelia.v1.AttachmentUploadUrlRequest';
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
proto.palm.camelia.v1.AttachmentUploadUrlResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.AttachmentUploadUrlResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.AttachmentUploadUrlResponse.displayName = 'proto.palm.camelia.v1.AttachmentUploadUrlResponse';
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
proto.palm.camelia.v1.AttachmentIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.AttachmentIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.AttachmentIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.AttachmentIndexResponse.displayName = 'proto.palm.camelia.v1.AttachmentIndexResponse';
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
proto.palm.camelia.v1.AttachmentIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.AttachmentIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.AttachmentIndexResponse.Item.displayName = 'proto.palm.camelia.v1.AttachmentIndexResponse.Item';
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
proto.palm.camelia.v1.LeaveWordCreateRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.LeaveWordCreateRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.LeaveWordCreateRequest.displayName = 'proto.palm.camelia.v1.LeaveWordCreateRequest';
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
proto.palm.camelia.v1.LeaveWordIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.LeaveWordIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.LeaveWordIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.LeaveWordIndexResponse.displayName = 'proto.palm.camelia.v1.LeaveWordIndexResponse';
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
proto.palm.camelia.v1.LeaveWordIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.LeaveWordIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.LeaveWordIndexResponse.Item.displayName = 'proto.palm.camelia.v1.LeaveWordIndexResponse.Item';
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
proto.palm.camelia.v1.LocaleByLangRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.LocaleByLangRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.LocaleByLangRequest.displayName = 'proto.palm.camelia.v1.LocaleByLangRequest';
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
proto.palm.camelia.v1.LocaleSetRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.LocaleSetRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.LocaleSetRequest.displayName = 'proto.palm.camelia.v1.LocaleSetRequest';
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
proto.palm.camelia.v1.LocaleIndexResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.LocaleIndexResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.LocaleIndexResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.LocaleIndexResponse.displayName = 'proto.palm.camelia.v1.LocaleIndexResponse';
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
proto.palm.camelia.v1.LocaleIndexResponse.Item = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.LocaleIndexResponse.Item, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.LocaleIndexResponse.Item.displayName = 'proto.palm.camelia.v1.LocaleIndexResponse.Item';
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
proto.palm.camelia.v1.LocaleByLangResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.LocaleByLangResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.LocaleByLangResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.LocaleByLangResponse.displayName = 'proto.palm.camelia.v1.LocaleByLangResponse';
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
proto.palm.camelia.v1.BaiduProfile = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.BaiduProfile, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.BaiduProfile.displayName = 'proto.palm.camelia.v1.BaiduProfile';
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
proto.palm.camelia.v1.IndexNowProfile = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.IndexNowProfile, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.IndexNowProfile.displayName = 'proto.palm.camelia.v1.IndexNowProfile';
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
proto.palm.camelia.v1.GoogleProfile = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.GoogleProfile, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.GoogleProfile.displayName = 'proto.palm.camelia.v1.GoogleProfile';
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
proto.palm.camelia.v1.ReCaptchaProfile = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.ReCaptchaProfile, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.ReCaptchaProfile.displayName = 'proto.palm.camelia.v1.ReCaptchaProfile';
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
proto.palm.camelia.v1.SiteSetFaviconRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.SiteSetFaviconRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteSetFaviconRequest.displayName = 'proto.palm.camelia.v1.SiteSetFaviconRequest';
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
proto.palm.camelia.v1.SiteLayoutResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.SiteLayoutResponse.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.SiteLayoutResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteLayoutResponse.displayName = 'proto.palm.camelia.v1.SiteLayoutResponse';
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
proto.palm.camelia.v1.SiteLayoutResponse.Author = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.SiteLayoutResponse.Author, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteLayoutResponse.Author.displayName = 'proto.palm.camelia.v1.SiteLayoutResponse.Author';
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
proto.palm.camelia.v1.SiteLayoutResponse.GabCode = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.SiteLayoutResponse.GabCode, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteLayoutResponse.GabCode.displayName = 'proto.palm.camelia.v1.SiteLayoutResponse.GabCode';
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
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.SiteLayoutResponse.IcpCode, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.displayName = 'proto.palm.camelia.v1.SiteLayoutResponse.IcpCode';
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
proto.palm.camelia.v1.SiteSetKeywordsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.SiteSetKeywordsRequest.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.SiteSetKeywordsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteSetKeywordsRequest.displayName = 'proto.palm.camelia.v1.SiteSetKeywordsRequest';
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
proto.palm.camelia.v1.SiteSetInfoRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.SiteSetInfoRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteSetInfoRequest.displayName = 'proto.palm.camelia.v1.SiteSetInfoRequest';
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
proto.palm.camelia.v1.SiteStatusResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.SiteStatusResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteStatusResponse.displayName = 'proto.palm.camelia.v1.SiteStatusResponse';
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
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.SiteStatusResponse.PostgreSql, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.displayName = 'proto.palm.camelia.v1.SiteStatusResponse.PostgreSql';
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
proto.palm.camelia.v1.SiteStatusResponse.Redis = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.palm.camelia.v1.SiteStatusResponse.Redis.repeatedFields_, null);
};
goog.inherits(proto.palm.camelia.v1.SiteStatusResponse.Redis, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteStatusResponse.Redis.displayName = 'proto.palm.camelia.v1.SiteStatusResponse.Redis';
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
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.palm.camelia.v1.SiteStatusResponse.OpenSearch, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.displayName = 'proto.palm.camelia.v1.SiteStatusResponse.OpenSearch';
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
proto.palm.camelia.v1.UserInfo.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserInfo.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserInfo} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserInfo.toObject = function(includeInstance, msg) {
  var f, obj = {
    uid: jspb.Message.getFieldWithDefault(msg, 1, ""),
    name: jspb.Message.getFieldWithDefault(msg, 2, ""),
    avatar: jspb.Message.getFieldWithDefault(msg, 3, ""),
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
 * @return {!proto.palm.camelia.v1.UserInfo}
 */
proto.palm.camelia.v1.UserInfo.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserInfo;
  return proto.palm.camelia.v1.UserInfo.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserInfo} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserInfo}
 */
proto.palm.camelia.v1.UserInfo.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setUid(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setAvatar(value);
      break;
    case 99:
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
proto.palm.camelia.v1.UserInfo.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserInfo.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserInfo} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserInfo.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getUid();
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
  f = /** @type {string} */ (jspb.Message.getField(message, 3));
  if (f != null) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getDeletedAt();
  if (f != null) {
    writer.writeMessage(
      99,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional string uid = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserInfo.prototype.getUid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserInfo} returns this
 */
proto.palm.camelia.v1.UserInfo.prototype.setUid = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserInfo.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserInfo} returns this
 */
proto.palm.camelia.v1.UserInfo.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string avatar = 3;
 * @return {string}
 */
proto.palm.camelia.v1.UserInfo.prototype.getAvatar = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserInfo} returns this
 */
proto.palm.camelia.v1.UserInfo.prototype.setAvatar = function(value) {
  return jspb.Message.setField(this, 3, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserInfo} returns this
 */
proto.palm.camelia.v1.UserInfo.prototype.clearAvatar = function() {
  return jspb.Message.setField(this, 3, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserInfo.prototype.hasAvatar = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional google.protobuf.Timestamp deleted_at = 99;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.UserInfo.prototype.getDeletedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 99));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.UserInfo} returns this
*/
proto.palm.camelia.v1.UserInfo.prototype.setDeletedAt = function(value) {
  return jspb.Message.setWrapperField(this, 99, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.UserInfo} returns this
 */
proto.palm.camelia.v1.UserInfo.prototype.clearDeletedAt = function() {
  return this.setDeletedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserInfo.prototype.hasDeletedAt = function() {
  return jspb.Message.getField(this, 99) != null;
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
proto.palm.camelia.v1.UserOauth2State.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserOauth2State.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserOauth2State} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserOauth2State.toObject = function(includeInstance, msg) {
  var f, obj = {
    user: jspb.Message.getFieldWithDefault(msg, 1, 0),
    code: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.palm.camelia.v1.UserOauth2State}
 */
proto.palm.camelia.v1.UserOauth2State.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserOauth2State;
  return proto.palm.camelia.v1.UserOauth2State.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserOauth2State} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserOauth2State}
 */
proto.palm.camelia.v1.UserOauth2State.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setUser(value);
      break;
    case 2:
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
proto.palm.camelia.v1.UserOauth2State.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserOauth2State.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserOauth2State} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserOauth2State.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = /** @type {number} */ (jspb.Message.getField(message, 1));
  if (f != null) {
    writer.writeInt64(
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
};


/**
 * optional int64 user = 1;
 * @return {number}
 */
proto.palm.camelia.v1.UserOauth2State.prototype.getUser = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.UserOauth2State} returns this
 */
proto.palm.camelia.v1.UserOauth2State.prototype.setUser = function(value) {
  return jspb.Message.setField(this, 1, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserOauth2State} returns this
 */
proto.palm.camelia.v1.UserOauth2State.prototype.clearUser = function() {
  return jspb.Message.setField(this, 1, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserOauth2State.prototype.hasUser = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string code = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserOauth2State.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserOauth2State} returns this
 */
proto.palm.camelia.v1.UserOauth2State.prototype.setCode = function(value) {
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
proto.palm.camelia.v1.Resource.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.Resource.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.Resource} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.Resource.toObject = function(includeInstance, msg) {
  var f, obj = {
    type: jspb.Message.getFieldWithDefault(msg, 1, ""),
    id: jspb.Message.getFieldWithDefault(msg, 2, 0)
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
 * @return {!proto.palm.camelia.v1.Resource}
 */
proto.palm.camelia.v1.Resource.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.Resource;
  return proto.palm.camelia.v1.Resource.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.Resource} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.Resource}
 */
proto.palm.camelia.v1.Resource.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setType(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
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
proto.palm.camelia.v1.Resource.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.Resource.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.Resource} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.Resource.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getType();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 2));
  if (f != null) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional string type = 1;
 * @return {string}
 */
proto.palm.camelia.v1.Resource.prototype.getType = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.Resource} returns this
 */
proto.palm.camelia.v1.Resource.prototype.setType = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 id = 2;
 * @return {number}
 */
proto.palm.camelia.v1.Resource.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.Resource} returns this
 */
proto.palm.camelia.v1.Resource.prototype.setId = function(value) {
  return jspb.Message.setField(this, 2, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.Resource} returns this
 */
proto.palm.camelia.v1.Resource.prototype.clearId = function() {
  return jspb.Message.setField(this, 2, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.Resource.prototype.hasId = function() {
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
proto.palm.camelia.v1.Permission.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.Permission.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.Permission} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.Permission.toObject = function(includeInstance, msg) {
  var f, obj = {
    operation: jspb.Message.getFieldWithDefault(msg, 1, ""),
    resource: (f = msg.getResource()) && proto.palm.camelia.v1.Resource.toObject(includeInstance, f)
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
 * @return {!proto.palm.camelia.v1.Permission}
 */
proto.palm.camelia.v1.Permission.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.Permission;
  return proto.palm.camelia.v1.Permission.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.Permission} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.Permission}
 */
proto.palm.camelia.v1.Permission.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setOperation(value);
      break;
    case 2:
      var value = new proto.palm.camelia.v1.Resource;
      reader.readMessage(value,proto.palm.camelia.v1.Resource.deserializeBinaryFromReader);
      msg.setResource(value);
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
proto.palm.camelia.v1.Permission.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.Permission.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.Permission} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.Permission.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getOperation();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getResource();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.camelia.v1.Resource.serializeBinaryToWriter
    );
  }
};


/**
 * optional string operation = 1;
 * @return {string}
 */
proto.palm.camelia.v1.Permission.prototype.getOperation = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.Permission} returns this
 */
proto.palm.camelia.v1.Permission.prototype.setOperation = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional Resource resource = 2;
 * @return {?proto.palm.camelia.v1.Resource}
 */
proto.palm.camelia.v1.Permission.prototype.getResource = function() {
  return /** @type{?proto.palm.camelia.v1.Resource} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.Resource, 2));
};


/**
 * @param {?proto.palm.camelia.v1.Resource|undefined} value
 * @return {!proto.palm.camelia.v1.Permission} returns this
*/
proto.palm.camelia.v1.Permission.prototype.setResource = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.Permission} returns this
 */
proto.palm.camelia.v1.Permission.prototype.clearResource = function() {
  return this.setResource(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.Permission.prototype.hasResource = function() {
  return jspb.Message.getField(this, 2) != null;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.Menu.repeatedFields_ = [9];



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
proto.palm.camelia.v1.Menu.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.Menu.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.Menu} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.Menu.toObject = function(includeInstance, msg) {
  var f, obj = {
    key: jspb.Message.getFieldWithDefault(msg, 1, ""),
    label: jspb.Message.getFieldWithDefault(msg, 2, ""),
    path: jspb.Message.getFieldWithDefault(msg, 3, ""),
    childrenList: jspb.Message.toObjectList(msg.getChildrenList(),
    proto.palm.camelia.v1.Menu.toObject, includeInstance)
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
 * @return {!proto.palm.camelia.v1.Menu}
 */
proto.palm.camelia.v1.Menu.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.Menu;
  return proto.palm.camelia.v1.Menu.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.Menu} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.Menu}
 */
proto.palm.camelia.v1.Menu.deserializeBinaryFromReader = function(msg, reader) {
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
      var value = /** @type {string} */ (reader.readString());
      msg.setLabel(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setPath(value);
      break;
    case 9:
      var value = new proto.palm.camelia.v1.Menu;
      reader.readMessage(value,proto.palm.camelia.v1.Menu.deserializeBinaryFromReader);
      msg.addChildren(value);
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
proto.palm.camelia.v1.Menu.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.Menu.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.Menu} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.Menu.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getKey();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getLabel();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getPath();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getChildrenList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      9,
      f,
      proto.palm.camelia.v1.Menu.serializeBinaryToWriter
    );
  }
};


/**
 * optional string key = 1;
 * @return {string}
 */
proto.palm.camelia.v1.Menu.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.Menu} returns this
 */
proto.palm.camelia.v1.Menu.prototype.setKey = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string label = 2;
 * @return {string}
 */
proto.palm.camelia.v1.Menu.prototype.getLabel = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.Menu} returns this
 */
proto.palm.camelia.v1.Menu.prototype.setLabel = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string path = 3;
 * @return {string}
 */
proto.palm.camelia.v1.Menu.prototype.getPath = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.Menu} returns this
 */
proto.palm.camelia.v1.Menu.prototype.setPath = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * repeated Menu children = 9;
 * @return {!Array<!proto.palm.camelia.v1.Menu>}
 */
proto.palm.camelia.v1.Menu.prototype.getChildrenList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.Menu>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.Menu, 9));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.Menu>} value
 * @return {!proto.palm.camelia.v1.Menu} returns this
*/
proto.palm.camelia.v1.Menu.prototype.setChildrenList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 9, value);
};


/**
 * @param {!proto.palm.camelia.v1.Menu=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.Menu}
 */
proto.palm.camelia.v1.Menu.prototype.addChildren = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 9, opt_value, proto.palm.camelia.v1.Menu, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.Menu} returns this
 */
proto.palm.camelia.v1.Menu.prototype.clearChildrenList = function() {
  return this.setChildrenList([]);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.UserIndexResponse.repeatedFields_ = [1];



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
proto.palm.camelia.v1.UserIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.camelia.v1.UserIndexResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && azalea_pb.Pagination.toObject(includeInstance, f)
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
 * @return {!proto.palm.camelia.v1.UserIndexResponse}
 */
proto.palm.camelia.v1.UserIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserIndexResponse;
  return proto.palm.camelia.v1.UserIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserIndexResponse}
 */
proto.palm.camelia.v1.UserIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.camelia.v1.UserIndexResponse.Item;
      reader.readMessage(value,proto.palm.camelia.v1.UserIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new azalea_pb.Pagination;
      reader.readMessage(value,azalea_pb.Pagination.deserializeBinaryFromReader);
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
proto.palm.camelia.v1.UserIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.camelia.v1.UserIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      azalea_pb.Pagination.serializeBinaryToWriter
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
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    type: jspb.Message.getFieldWithDefault(msg, 1, 0),
    id: jspb.Message.getFieldWithDefault(msg, 2, ""),
    name: jspb.Message.getFieldWithDefault(msg, 11, ""),
    lang: jspb.Message.getFieldWithDefault(msg, 12, ""),
    timezone: jspb.Message.getFieldWithDefault(msg, 13, ""),
    avatar: jspb.Message.getFieldWithDefault(msg, 19, "")
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
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserIndexResponse.Item;
  return proto.palm.camelia.v1.UserIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.palm.camelia.v1.UserIndexResponse.Item.Type} */ (reader.readEnum());
      msg.setType(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 11:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 12:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 13:
      var value = /** @type {string} */ (reader.readString());
      msg.setTimezone(value);
      break;
    case 19:
      var value = /** @type {string} */ (reader.readString());
      msg.setAvatar(value);
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
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getType();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 11));
  if (f != null) {
    writer.writeString(
      11,
      f
    );
  }
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      12,
      f
    );
  }
  f = message.getTimezone();
  if (f.length > 0) {
    writer.writeString(
      13,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 19));
  if (f != null) {
    writer.writeString(
      19,
      f
    );
  }
};


/**
 * @enum {number}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.Type = {
  EMAIL: 0,
  PHONE: 1,
  GOOGLE: 2,
  WECHATMINIPROGRAM: 11,
  WECHATOAUTH2: 12
};

/**
 * optional Type type = 1;
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item.Type}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.getType = function() {
  return /** @type {!proto.palm.camelia.v1.UserIndexResponse.Item.Type} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.palm.camelia.v1.UserIndexResponse.Item.Type} value
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.setType = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional string id = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string name = 11;
 * @return {string}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 11, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.setName = function(value) {
  return jspb.Message.setField(this, 11, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.clearName = function() {
  return jspb.Message.setField(this, 11, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.hasName = function() {
  return jspb.Message.getField(this, 11) != null;
};


/**
 * optional string lang = 12;
 * @return {string}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 12, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 12, value);
};


/**
 * optional string timezone = 13;
 * @return {string}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.getTimezone = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 13, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.setTimezone = function(value) {
  return jspb.Message.setProto3StringField(this, 13, value);
};


/**
 * optional string avatar = 19;
 * @return {string}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.getAvatar = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 19, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.setAvatar = function(value) {
  return jspb.Message.setField(this, 19, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.clearAvatar = function() {
  return jspb.Message.setField(this, 19, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserIndexResponse.Item.prototype.hasAvatar = function() {
  return jspb.Message.getField(this, 19) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.camelia.v1.UserIndexResponse.Item>}
 */
proto.palm.camelia.v1.UserIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.UserIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.UserIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.UserIndexResponse.Item>} value
 * @return {!proto.palm.camelia.v1.UserIndexResponse} returns this
*/
proto.palm.camelia.v1.UserIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.camelia.v1.UserIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item}
 */
proto.palm.camelia.v1.UserIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.camelia.v1.UserIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.UserIndexResponse} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional palm.azalea.v1.Pagination pagination = 2;
 * @return {?proto.palm.azalea.v1.Pagination}
 */
proto.palm.camelia.v1.UserIndexResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.azalea.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, azalea_pb.Pagination, 2));
};


/**
 * @param {?proto.palm.azalea.v1.Pagination|undefined} value
 * @return {!proto.palm.camelia.v1.UserIndexResponse} returns this
*/
proto.palm.camelia.v1.UserIndexResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.UserIndexResponse} returns this
 */
proto.palm.camelia.v1.UserIndexResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserIndexResponse.prototype.hasPagination = function() {
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
proto.palm.camelia.v1.UserCancelRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserCancelRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserCancelRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserCancelRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    home: jspb.Message.getFieldWithDefault(msg, 1, ""),
    reason: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.palm.camelia.v1.UserCancelRequest}
 */
proto.palm.camelia.v1.UserCancelRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserCancelRequest;
  return proto.palm.camelia.v1.UserCancelRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserCancelRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserCancelRequest}
 */
proto.palm.camelia.v1.UserCancelRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setHome(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setReason(value);
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
proto.palm.camelia.v1.UserCancelRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserCancelRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserCancelRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserCancelRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getHome();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getReason();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string home = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserCancelRequest.prototype.getHome = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserCancelRequest} returns this
 */
proto.palm.camelia.v1.UserCancelRequest.prototype.setHome = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string reason = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserCancelRequest.prototype.getReason = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserCancelRequest} returns this
 */
proto.palm.camelia.v1.UserCancelRequest.prototype.setReason = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};



/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.palm.camelia.v1.UserCancelResponse.oneofGroups_ = [[1,2]];

/**
 * @enum {number}
 */
proto.palm.camelia.v1.UserCancelResponse.PayloadCase = {
  PAYLOAD_NOT_SET: 0,
  REMINDER: 1,
  REDIRECT_TO: 2
};

/**
 * @return {proto.palm.camelia.v1.UserCancelResponse.PayloadCase}
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.getPayloadCase = function() {
  return /** @type {proto.palm.camelia.v1.UserCancelResponse.PayloadCase} */(jspb.Message.computeOneofCase(this, proto.palm.camelia.v1.UserCancelResponse.oneofGroups_[0]));
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
proto.palm.camelia.v1.UserCancelResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserCancelResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserCancelResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserCancelResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    reminder: jspb.Message.getFieldWithDefault(msg, 1, ""),
    redirectTo: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.palm.camelia.v1.UserCancelResponse}
 */
proto.palm.camelia.v1.UserCancelResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserCancelResponse;
  return proto.palm.camelia.v1.UserCancelResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserCancelResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserCancelResponse}
 */
proto.palm.camelia.v1.UserCancelResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setReminder(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setRedirectTo(value);
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
proto.palm.camelia.v1.UserCancelResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserCancelResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserCancelResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserCancelResponse.serializeBinaryToWriter = function(message, writer) {
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
 * optional string reminder = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.getReminder = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserCancelResponse} returns this
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.setReminder = function(value) {
  return jspb.Message.setOneofField(this, 1, proto.palm.camelia.v1.UserCancelResponse.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserCancelResponse} returns this
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.clearReminder = function() {
  return jspb.Message.setOneofField(this, 1, proto.palm.camelia.v1.UserCancelResponse.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.hasReminder = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string redirect_to = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.getRedirectTo = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserCancelResponse} returns this
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.setRedirectTo = function(value) {
  return jspb.Message.setOneofField(this, 2, proto.palm.camelia.v1.UserCancelResponse.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserCancelResponse} returns this
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.clearRedirectTo = function() {
  return jspb.Message.setOneofField(this, 2, proto.palm.camelia.v1.UserCancelResponse.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserCancelResponse.prototype.hasRedirectTo = function() {
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
proto.palm.camelia.v1.UserSetProfileRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserSetProfileRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserSetProfileRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSetProfileRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    realName: jspb.Message.getFieldWithDefault(msg, 1, ""),
    avatar: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.palm.camelia.v1.UserSetProfileRequest}
 */
proto.palm.camelia.v1.UserSetProfileRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserSetProfileRequest;
  return proto.palm.camelia.v1.UserSetProfileRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserSetProfileRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserSetProfileRequest}
 */
proto.palm.camelia.v1.UserSetProfileRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.UserSetProfileRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserSetProfileRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserSetProfileRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSetProfileRequest.serializeBinaryToWriter = function(message, writer) {
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
};


/**
 * optional string real_name = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserSetProfileRequest.prototype.getRealName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSetProfileRequest} returns this
 */
proto.palm.camelia.v1.UserSetProfileRequest.prototype.setRealName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string avatar = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserSetProfileRequest.prototype.getAvatar = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSetProfileRequest} returns this
 */
proto.palm.camelia.v1.UserSetProfileRequest.prototype.setAvatar = function(value) {
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
proto.palm.camelia.v1.UserSetLocationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserSetLocationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserSetLocationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSetLocationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    lang: jspb.Message.getFieldWithDefault(msg, 1, ""),
    timezone: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.palm.camelia.v1.UserSetLocationRequest}
 */
proto.palm.camelia.v1.UserSetLocationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserSetLocationRequest;
  return proto.palm.camelia.v1.UserSetLocationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserSetLocationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserSetLocationRequest}
 */
proto.palm.camelia.v1.UserSetLocationRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.UserSetLocationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserSetLocationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserSetLocationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSetLocationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getTimezone();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string lang = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserSetLocationRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSetLocationRequest} returns this
 */
proto.palm.camelia.v1.UserSetLocationRequest.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string timezone = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserSetLocationRequest.prototype.getTimezone = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSetLocationRequest} returns this
 */
proto.palm.camelia.v1.UserSetLocationRequest.prototype.setTimezone = function(value) {
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
proto.palm.camelia.v1.UserChangePasswordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserChangePasswordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserChangePasswordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserChangePasswordRequest.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.UserChangePasswordRequest}
 */
proto.palm.camelia.v1.UserChangePasswordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserChangePasswordRequest;
  return proto.palm.camelia.v1.UserChangePasswordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserChangePasswordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserChangePasswordRequest}
 */
proto.palm.camelia.v1.UserChangePasswordRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.UserChangePasswordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserChangePasswordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserChangePasswordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserChangePasswordRequest.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.UserChangePasswordRequest.prototype.getCurrentPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserChangePasswordRequest} returns this
 */
proto.palm.camelia.v1.UserChangePasswordRequest.prototype.setCurrentPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string new_password = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserChangePasswordRequest.prototype.getNewPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserChangePasswordRequest} returns this
 */
proto.palm.camelia.v1.UserChangePasswordRequest.prototype.setNewPassword = function(value) {
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
proto.palm.camelia.v1.UserSetPasswordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserSetPasswordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserSetPasswordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSetPasswordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    providerType: jspb.Message.getFieldWithDefault(msg, 1, 0),
    providerId: jspb.Message.getFieldWithDefault(msg, 2, 0),
    password: jspb.Message.getFieldWithDefault(msg, 3, "")
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
 * @return {!proto.palm.camelia.v1.UserSetPasswordRequest}
 */
proto.palm.camelia.v1.UserSetPasswordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserSetPasswordRequest;
  return proto.palm.camelia.v1.UserSetPasswordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserSetPasswordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserSetPasswordRequest}
 */
proto.palm.camelia.v1.UserSetPasswordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.palm.camelia.v1.UserIndexResponse.Item.Type} */ (reader.readEnum());
      msg.setProviderType(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setProviderId(value);
      break;
    case 3:
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
proto.palm.camelia.v1.UserSetPasswordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserSetPasswordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserSetPasswordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSetPasswordRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getProviderType();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getProviderId();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getPassword();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
};


/**
 * optional UserIndexResponse.Item.Type provider_type = 1;
 * @return {!proto.palm.camelia.v1.UserIndexResponse.Item.Type}
 */
proto.palm.camelia.v1.UserSetPasswordRequest.prototype.getProviderType = function() {
  return /** @type {!proto.palm.camelia.v1.UserIndexResponse.Item.Type} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.palm.camelia.v1.UserIndexResponse.Item.Type} value
 * @return {!proto.palm.camelia.v1.UserSetPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSetPasswordRequest.prototype.setProviderType = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional int64 provider_id = 2;
 * @return {number}
 */
proto.palm.camelia.v1.UserSetPasswordRequest.prototype.getProviderId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.UserSetPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSetPasswordRequest.prototype.setProviderId = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional string password = 3;
 * @return {string}
 */
proto.palm.camelia.v1.UserSetPasswordRequest.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSetPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSetPasswordRequest.prototype.setPassword = function(value) {
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
proto.palm.camelia.v1.UserResetPasswordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserResetPasswordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserResetPasswordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserResetPasswordRequest.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.UserResetPasswordRequest}
 */
proto.palm.camelia.v1.UserResetPasswordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserResetPasswordRequest;
  return proto.palm.camelia.v1.UserResetPasswordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserResetPasswordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserResetPasswordRequest}
 */
proto.palm.camelia.v1.UserResetPasswordRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.UserResetPasswordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserResetPasswordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserResetPasswordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserResetPasswordRequest.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.UserResetPasswordRequest.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserResetPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserResetPasswordRequest.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string password = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserResetPasswordRequest.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserResetPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserResetPasswordRequest.prototype.setPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};



/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.palm.camelia.v1.UserEmailRequest.oneofGroups_ = [[1,2]];

/**
 * @enum {number}
 */
proto.palm.camelia.v1.UserEmailRequest.UserCase = {
  USER_NOT_SET: 0,
  NICKNAME: 1,
  EMAIL: 2
};

/**
 * @return {proto.palm.camelia.v1.UserEmailRequest.UserCase}
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.getUserCase = function() {
  return /** @type {proto.palm.camelia.v1.UserEmailRequest.UserCase} */(jspb.Message.computeOneofCase(this, proto.palm.camelia.v1.UserEmailRequest.oneofGroups_[0]));
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
proto.palm.camelia.v1.UserEmailRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserEmailRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserEmailRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserEmailRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    nickname: jspb.Message.getFieldWithDefault(msg, 1, ""),
    email: jspb.Message.getFieldWithDefault(msg, 2, ""),
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
 * @return {!proto.palm.camelia.v1.UserEmailRequest}
 */
proto.palm.camelia.v1.UserEmailRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserEmailRequest;
  return proto.palm.camelia.v1.UserEmailRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserEmailRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserEmailRequest}
 */
proto.palm.camelia.v1.UserEmailRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setNickname(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setEmail(value);
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
proto.palm.camelia.v1.UserEmailRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserEmailRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserEmailRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserEmailRequest.serializeBinaryToWriter = function(message, writer) {
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
  f = message.getHome();
  if (f.length > 0) {
    writer.writeString(
      9,
      f
    );
  }
};


/**
 * optional string nickname = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.getNickname = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserEmailRequest} returns this
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.setNickname = function(value) {
  return jspb.Message.setOneofField(this, 1, proto.palm.camelia.v1.UserEmailRequest.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserEmailRequest} returns this
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.clearNickname = function() {
  return jspb.Message.setOneofField(this, 1, proto.palm.camelia.v1.UserEmailRequest.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.hasNickname = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string email = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.getEmail = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserEmailRequest} returns this
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.setEmail = function(value) {
  return jspb.Message.setOneofField(this, 2, proto.palm.camelia.v1.UserEmailRequest.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserEmailRequest} returns this
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.clearEmail = function() {
  return jspb.Message.setOneofField(this, 2, proto.palm.camelia.v1.UserEmailRequest.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.hasEmail = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional string home = 9;
 * @return {string}
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.getHome = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 9, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserEmailRequest} returns this
 */
proto.palm.camelia.v1.UserEmailRequest.prototype.setHome = function(value) {
  return jspb.Message.setProto3StringField(this, 9, value);
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
proto.palm.camelia.v1.UserTokenRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserTokenRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserTokenRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserTokenRequest.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.UserTokenRequest}
 */
proto.palm.camelia.v1.UserTokenRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserTokenRequest;
  return proto.palm.camelia.v1.UserTokenRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserTokenRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserTokenRequest}
 */
proto.palm.camelia.v1.UserTokenRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.UserTokenRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserTokenRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserTokenRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserTokenRequest.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.UserTokenRequest.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserTokenRequest} returns this
 */
proto.palm.camelia.v1.UserTokenRequest.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.UserSignInResponse.repeatedFields_ = [11,12,13];



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
proto.palm.camelia.v1.UserSignInResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserSignInResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserSignInResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSignInResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    token: jspb.Message.getFieldWithDefault(msg, 1, ""),
    user: (f = msg.getUser()) && proto.palm.camelia.v1.UserIndexResponse.Item.toObject(includeInstance, f),
    rolesList: (f = jspb.Message.getRepeatedField(msg, 11)) == null ? undefined : f,
    permissionsList: jspb.Message.toObjectList(msg.getPermissionsList(),
    proto.palm.camelia.v1.Permission.toObject, includeInstance),
    menusList: jspb.Message.toObjectList(msg.getMenusList(),
    proto.palm.camelia.v1.Menu.toObject, includeInstance)
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
 * @return {!proto.palm.camelia.v1.UserSignInResponse}
 */
proto.palm.camelia.v1.UserSignInResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserSignInResponse;
  return proto.palm.camelia.v1.UserSignInResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserSignInResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserSignInResponse}
 */
proto.palm.camelia.v1.UserSignInResponse.deserializeBinaryFromReader = function(msg, reader) {
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
      var value = new proto.palm.camelia.v1.UserIndexResponse.Item;
      reader.readMessage(value,proto.palm.camelia.v1.UserIndexResponse.Item.deserializeBinaryFromReader);
      msg.setUser(value);
      break;
    case 11:
      var value = /** @type {string} */ (reader.readString());
      msg.addRoles(value);
      break;
    case 12:
      var value = new proto.palm.camelia.v1.Permission;
      reader.readMessage(value,proto.palm.camelia.v1.Permission.deserializeBinaryFromReader);
      msg.addPermissions(value);
      break;
    case 13:
      var value = new proto.palm.camelia.v1.Menu;
      reader.readMessage(value,proto.palm.camelia.v1.Menu.deserializeBinaryFromReader);
      msg.addMenus(value);
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
proto.palm.camelia.v1.UserSignInResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserSignInResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserSignInResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSignInResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getToken();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getUser();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.camelia.v1.UserIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getRolesList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      11,
      f
    );
  }
  f = message.getPermissionsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      12,
      f,
      proto.palm.camelia.v1.Permission.serializeBinaryToWriter
    );
  }
  f = message.getMenusList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      13,
      f,
      proto.palm.camelia.v1.Menu.serializeBinaryToWriter
    );
  }
};


/**
 * optional string token = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.getToken = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.setToken = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional UserIndexResponse.Item user = 2;
 * @return {?proto.palm.camelia.v1.UserIndexResponse.Item}
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.getUser = function() {
  return /** @type{?proto.palm.camelia.v1.UserIndexResponse.Item} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.UserIndexResponse.Item, 2));
};


/**
 * @param {?proto.palm.camelia.v1.UserIndexResponse.Item|undefined} value
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
*/
proto.palm.camelia.v1.UserSignInResponse.prototype.setUser = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.clearUser = function() {
  return this.setUser(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.hasUser = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * repeated string roles = 11;
 * @return {!Array<string>}
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.getRolesList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 11));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.setRolesList = function(value) {
  return jspb.Message.setField(this, 11, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.addRoles = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 11, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.clearRolesList = function() {
  return this.setRolesList([]);
};


/**
 * repeated Permission permissions = 12;
 * @return {!Array<!proto.palm.camelia.v1.Permission>}
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.getPermissionsList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.Permission>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.Permission, 12));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.Permission>} value
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
*/
proto.palm.camelia.v1.UserSignInResponse.prototype.setPermissionsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 12, value);
};


/**
 * @param {!proto.palm.camelia.v1.Permission=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.Permission}
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.addPermissions = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 12, opt_value, proto.palm.camelia.v1.Permission, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.clearPermissionsList = function() {
  return this.setPermissionsList([]);
};


/**
 * repeated Menu menus = 13;
 * @return {!Array<!proto.palm.camelia.v1.Menu>}
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.getMenusList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.Menu>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.Menu, 13));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.Menu>} value
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
*/
proto.palm.camelia.v1.UserSignInResponse.prototype.setMenusList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 13, value);
};


/**
 * @param {!proto.palm.camelia.v1.Menu=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.Menu}
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.addMenus = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 13, opt_value, proto.palm.camelia.v1.Menu, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.UserSignInResponse} returns this
 */
proto.palm.camelia.v1.UserSignInResponse.prototype.clearMenusList = function() {
  return this.setMenusList([]);
};



/**
 * Oneof group definitions for this message. Each group defines the field
 * numbers belonging to that group. When of these fields' value is set, all
 * other fields in the group are cleared. During deserialization, if multiple
 * fields are encountered for a group, only the last value seen will be kept.
 * @private {!Array<!Array<number>>}
 * @const
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_ = [[1,2,3]];

/**
 * @enum {number}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.WhoCase = {
  WHO_NOT_SET: 0,
  NICKNAME: 1,
  EMAIL: 2,
  PHONE: 3
};

/**
 * @return {proto.palm.camelia.v1.UserSignInByPasswordRequest.WhoCase}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.getWhoCase = function() {
  return /** @type {proto.palm.camelia.v1.UserSignInByPasswordRequest.WhoCase} */(jspb.Message.computeOneofCase(this, proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_[0]));
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
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserSignInByPasswordRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserSignInByPasswordRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    nickname: jspb.Message.getFieldWithDefault(msg, 1, ""),
    email: jspb.Message.getFieldWithDefault(msg, 2, ""),
    phone: jspb.Message.getFieldWithDefault(msg, 3, ""),
    password: jspb.Message.getFieldWithDefault(msg, 9, ""),
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
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserSignInByPasswordRequest;
  return proto.palm.camelia.v1.UserSignInByPasswordRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserSignInByPasswordRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setNickname(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setEmail(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setPhone(value);
      break;
    case 9:
      var value = /** @type {string} */ (reader.readString());
      msg.setPassword(value);
      break;
    case 11:
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
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserSignInByPasswordRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserSignInByPasswordRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.serializeBinaryToWriter = function(message, writer) {
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
  f = /** @type {string} */ (jspb.Message.getField(message, 3));
  if (f != null) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getPassword();
  if (f.length > 0) {
    writer.writeString(
      9,
      f
    );
  }
  f = message.getTtl();
  if (f != null) {
    writer.writeMessage(
      11,
      f,
      google_protobuf_duration_pb.Duration.serializeBinaryToWriter
    );
  }
};


/**
 * optional string nickname = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.getNickname = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.setNickname = function(value) {
  return jspb.Message.setOneofField(this, 1, proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.clearNickname = function() {
  return jspb.Message.setOneofField(this, 1, proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.hasNickname = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string email = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.getEmail = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.setEmail = function(value) {
  return jspb.Message.setOneofField(this, 2, proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.clearEmail = function() {
  return jspb.Message.setOneofField(this, 2, proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.hasEmail = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional string phone = 3;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.getPhone = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.setPhone = function(value) {
  return jspb.Message.setOneofField(this, 3, proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_[0], value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.clearPhone = function() {
  return jspb.Message.setOneofField(this, 3, proto.palm.camelia.v1.UserSignInByPasswordRequest.oneofGroups_[0], undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.hasPhone = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional string password = 9;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 9, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.setPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 9, value);
};


/**
 * optional google.protobuf.Duration ttl = 11;
 * @return {?proto.google.protobuf.Duration}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.getTtl = function() {
  return /** @type{?proto.google.protobuf.Duration} */ (
    jspb.Message.getWrapperField(this, google_protobuf_duration_pb.Duration, 11));
};


/**
 * @param {?proto.google.protobuf.Duration|undefined} value
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
*/
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.setTtl = function(value) {
  return jspb.Message.setWrapperField(this, 11, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.UserSignInByPasswordRequest} returns this
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.clearTtl = function() {
  return this.setTtl(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserSignInByPasswordRequest.prototype.hasTtl = function() {
  return jspb.Message.getField(this, 11) != null;
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
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserSignUpByEmailRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserSignUpByEmailRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    realName: jspb.Message.getFieldWithDefault(msg, 1, ""),
    email: jspb.Message.getFieldWithDefault(msg, 2, ""),
    nickname: jspb.Message.getFieldWithDefault(msg, 3, ""),
    password: jspb.Message.getFieldWithDefault(msg, 4, ""),
    timezone: jspb.Message.getFieldWithDefault(msg, 11, ""),
    home: jspb.Message.getFieldWithDefault(msg, 19, "")
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
 * @return {!proto.palm.camelia.v1.UserSignUpByEmailRequest}
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserSignUpByEmailRequest;
  return proto.palm.camelia.v1.UserSignUpByEmailRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserSignUpByEmailRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserSignUpByEmailRequest}
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.deserializeBinaryFromReader = function(msg, reader) {
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
    case 11:
      var value = /** @type {string} */ (reader.readString());
      msg.setTimezone(value);
      break;
    case 19:
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
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserSignUpByEmailRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserSignUpByEmailRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.serializeBinaryToWriter = function(message, writer) {
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
  f = message.getTimezone();
  if (f.length > 0) {
    writer.writeString(
      11,
      f
    );
  }
  f = message.getHome();
  if (f.length > 0) {
    writer.writeString(
      19,
      f
    );
  }
};


/**
 * optional string real_name = 1;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.getRealName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.setRealName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string email = 2;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.getEmail = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.setEmail = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string nickname = 3;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.getNickname = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.setNickname = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string password = 4;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.getPassword = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.setPassword = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string timezone = 11;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.getTimezone = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 11, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.setTimezone = function(value) {
  return jspb.Message.setProto3StringField(this, 11, value);
};


/**
 * optional string home = 19;
 * @return {string}
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.getHome = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 19, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserSignUpByEmailRequest} returns this
 */
proto.palm.camelia.v1.UserSignUpByEmailRequest.prototype.setHome = function(value) {
  return jspb.Message.setProto3StringField(this, 19, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.UserLogsResponse.repeatedFields_ = [1];



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
proto.palm.camelia.v1.UserLogsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserLogsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserLogsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserLogsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.camelia.v1.UserLogsResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && azalea_pb.Pagination.toObject(includeInstance, f)
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
 * @return {!proto.palm.camelia.v1.UserLogsResponse}
 */
proto.palm.camelia.v1.UserLogsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserLogsResponse;
  return proto.palm.camelia.v1.UserLogsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserLogsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserLogsResponse}
 */
proto.palm.camelia.v1.UserLogsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.camelia.v1.UserLogsResponse.Item;
      reader.readMessage(value,proto.palm.camelia.v1.UserLogsResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new azalea_pb.Pagination;
      reader.readMessage(value,azalea_pb.Pagination.deserializeBinaryFromReader);
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
proto.palm.camelia.v1.UserLogsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserLogsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserLogsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserLogsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.camelia.v1.UserLogsResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      azalea_pb.Pagination.serializeBinaryToWriter
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
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.UserLogsResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.UserLogsResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserLogsResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    level: jspb.Message.getFieldWithDefault(msg, 2, 0),
    ip: jspb.Message.getFieldWithDefault(msg, 3, ""),
    message: jspb.Message.getFieldWithDefault(msg, 4, ""),
    resource: (f = msg.getResource()) && proto.palm.camelia.v1.Resource.toObject(includeInstance, f),
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
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.UserLogsResponse.Item;
  return proto.palm.camelia.v1.UserLogsResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.UserLogsResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {!proto.palm.camelia.v1.UserLogsResponse.Item.Level} */ (reader.readEnum());
      msg.setLevel(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setIp(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setMessage(value);
      break;
    case 5:
      var value = new proto.palm.camelia.v1.Resource;
      reader.readMessage(value,proto.palm.camelia.v1.Resource.deserializeBinaryFromReader);
      msg.setResource(value);
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
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.UserLogsResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.UserLogsResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.UserLogsResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getLevel();
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
  f = message.getMessage();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getResource();
  if (f != null) {
    writer.writeMessage(
      5,
      f,
      proto.palm.camelia.v1.Resource.serializeBinaryToWriter
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
proto.palm.camelia.v1.UserLogsResponse.Item.Level = {
  DEBUG: 0,
  INFO: 1,
  WARN: 2,
  ERROR: 3
};

/**
 * optional int64 id = 1;
 * @return {number}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item} returns this
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional Level level = 2;
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item.Level}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.getLevel = function() {
  return /** @type {!proto.palm.camelia.v1.UserLogsResponse.Item.Level} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.palm.camelia.v1.UserLogsResponse.Item.Level} value
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item} returns this
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.setLevel = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional string ip = 3;
 * @return {string}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.getIp = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item} returns this
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.setIp = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string message = 4;
 * @return {string}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item} returns this
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional Resource resource = 5;
 * @return {?proto.palm.camelia.v1.Resource}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.getResource = function() {
  return /** @type{?proto.palm.camelia.v1.Resource} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.Resource, 5));
};


/**
 * @param {?proto.palm.camelia.v1.Resource|undefined} value
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item} returns this
*/
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.setResource = function(value) {
  return jspb.Message.setWrapperField(this, 5, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item} returns this
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.clearResource = function() {
  return this.setResource(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.hasResource = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional google.protobuf.Timestamp created_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item} returns this
*/
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item} returns this
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserLogsResponse.Item.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.camelia.v1.UserLogsResponse.Item>}
 */
proto.palm.camelia.v1.UserLogsResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.UserLogsResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.UserLogsResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.UserLogsResponse.Item>} value
 * @return {!proto.palm.camelia.v1.UserLogsResponse} returns this
*/
proto.palm.camelia.v1.UserLogsResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.camelia.v1.UserLogsResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.UserLogsResponse.Item}
 */
proto.palm.camelia.v1.UserLogsResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.camelia.v1.UserLogsResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.UserLogsResponse} returns this
 */
proto.palm.camelia.v1.UserLogsResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional palm.azalea.v1.Pagination pagination = 2;
 * @return {?proto.palm.azalea.v1.Pagination}
 */
proto.palm.camelia.v1.UserLogsResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.azalea.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, azalea_pb.Pagination, 2));
};


/**
 * @param {?proto.palm.azalea.v1.Pagination|undefined} value
 * @return {!proto.palm.camelia.v1.UserLogsResponse} returns this
*/
proto.palm.camelia.v1.UserLogsResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.UserLogsResponse} returns this
 */
proto.palm.camelia.v1.UserLogsResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.UserLogsResponse.prototype.hasPagination = function() {
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
proto.palm.camelia.v1.AttachmentShowRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.AttachmentShowRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.AttachmentShowRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentShowRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
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
 * @return {!proto.palm.camelia.v1.AttachmentShowRequest}
 */
proto.palm.camelia.v1.AttachmentShowRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.AttachmentShowRequest;
  return proto.palm.camelia.v1.AttachmentShowRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.AttachmentShowRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.AttachmentShowRequest}
 */
proto.palm.camelia.v1.AttachmentShowRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
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
proto.palm.camelia.v1.AttachmentShowRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.AttachmentShowRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.AttachmentShowRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentShowRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeInt64(
      1,
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
 * optional int64 id = 1;
 * @return {number}
 */
proto.palm.camelia.v1.AttachmentShowRequest.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.AttachmentShowRequest} returns this
 */
proto.palm.camelia.v1.AttachmentShowRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional google.protobuf.Duration ttl = 9;
 * @return {?proto.google.protobuf.Duration}
 */
proto.palm.camelia.v1.AttachmentShowRequest.prototype.getTtl = function() {
  return /** @type{?proto.google.protobuf.Duration} */ (
    jspb.Message.getWrapperField(this, google_protobuf_duration_pb.Duration, 9));
};


/**
 * @param {?proto.google.protobuf.Duration|undefined} value
 * @return {!proto.palm.camelia.v1.AttachmentShowRequest} returns this
*/
proto.palm.camelia.v1.AttachmentShowRequest.prototype.setTtl = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.AttachmentShowRequest} returns this
 */
proto.palm.camelia.v1.AttachmentShowRequest.prototype.clearTtl = function() {
  return this.setTtl(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.AttachmentShowRequest.prototype.hasTtl = function() {
  return jspb.Message.getField(this, 9) != null;
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
proto.palm.camelia.v1.AttachmentShowResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.AttachmentShowResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.AttachmentShowResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentShowResponse.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.AttachmentShowResponse}
 */
proto.palm.camelia.v1.AttachmentShowResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.AttachmentShowResponse;
  return proto.palm.camelia.v1.AttachmentShowResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.AttachmentShowResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.AttachmentShowResponse}
 */
proto.palm.camelia.v1.AttachmentShowResponse.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.AttachmentShowResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.AttachmentShowResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.AttachmentShowResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentShowResponse.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.AttachmentShowResponse.prototype.getUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentShowResponse} returns this
 */
proto.palm.camelia.v1.AttachmentShowResponse.prototype.setUrl = function(value) {
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
proto.palm.camelia.v1.AttachmentResourceRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.AttachmentResourceRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.AttachmentResourceRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentResourceRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    resourceType: jspb.Message.getFieldWithDefault(msg, 2, ""),
    resourceId: jspb.Message.getFieldWithDefault(msg, 3, 0)
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
 * @return {!proto.palm.camelia.v1.AttachmentResourceRequest}
 */
proto.palm.camelia.v1.AttachmentResourceRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.AttachmentResourceRequest;
  return proto.palm.camelia.v1.AttachmentResourceRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.AttachmentResourceRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.AttachmentResourceRequest}
 */
proto.palm.camelia.v1.AttachmentResourceRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setResourceType(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setResourceId(value);
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
proto.palm.camelia.v1.AttachmentResourceRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.AttachmentResourceRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.AttachmentResourceRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentResourceRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getResourceType();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getResourceId();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional int64 id = 1;
 * @return {number}
 */
proto.palm.camelia.v1.AttachmentResourceRequest.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.AttachmentResourceRequest} returns this
 */
proto.palm.camelia.v1.AttachmentResourceRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string resource_type = 2;
 * @return {string}
 */
proto.palm.camelia.v1.AttachmentResourceRequest.prototype.getResourceType = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentResourceRequest} returns this
 */
proto.palm.camelia.v1.AttachmentResourceRequest.prototype.setResourceType = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional int64 resource_id = 3;
 * @return {number}
 */
proto.palm.camelia.v1.AttachmentResourceRequest.prototype.getResourceId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.AttachmentResourceRequest} returns this
 */
proto.palm.camelia.v1.AttachmentResourceRequest.prototype.setResourceId = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
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
proto.palm.camelia.v1.AttachmentUpdateRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.AttachmentUpdateRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.AttachmentUpdateRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentUpdateRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    title: jspb.Message.getFieldWithDefault(msg, 2, "")
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
 * @return {!proto.palm.camelia.v1.AttachmentUpdateRequest}
 */
proto.palm.camelia.v1.AttachmentUpdateRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.AttachmentUpdateRequest;
  return proto.palm.camelia.v1.AttachmentUpdateRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.AttachmentUpdateRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.AttachmentUpdateRequest}
 */
proto.palm.camelia.v1.AttachmentUpdateRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setTitle(value);
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
proto.palm.camelia.v1.AttachmentUpdateRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.AttachmentUpdateRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.AttachmentUpdateRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentUpdateRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeInt64(
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
};


/**
 * optional int64 id = 1;
 * @return {number}
 */
proto.palm.camelia.v1.AttachmentUpdateRequest.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.AttachmentUpdateRequest} returns this
 */
proto.palm.camelia.v1.AttachmentUpdateRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string title = 2;
 * @return {string}
 */
proto.palm.camelia.v1.AttachmentUpdateRequest.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentUpdateRequest} returns this
 */
proto.palm.camelia.v1.AttachmentUpdateRequest.prototype.setTitle = function(value) {
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
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.AttachmentUploadUrlRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    title: jspb.Message.getFieldWithDefault(msg, 1, ""),
    contentType: jspb.Message.getFieldWithDefault(msg, 2, ""),
    size: jspb.Message.getFieldWithDefault(msg, 3, 0),
    pb_public: jspb.Message.getBooleanFieldWithDefault(msg, 11, false),
    expirationDays: jspb.Message.getFieldWithDefault(msg, 12, 0)
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
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlRequest}
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.AttachmentUploadUrlRequest;
  return proto.palm.camelia.v1.AttachmentUploadUrlRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlRequest}
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.deserializeBinaryFromReader = function(msg, reader) {
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
      msg.setContentType(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readUint64());
      msg.setSize(value);
      break;
    case 11:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setPublic(value);
      break;
    case 12:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setExpirationDays(value);
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
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.AttachmentUploadUrlRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTitle();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getContentType();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getSize();
  if (f !== 0) {
    writer.writeUint64(
      3,
      f
    );
  }
  f = message.getPublic();
  if (f) {
    writer.writeBool(
      11,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 12));
  if (f != null) {
    writer.writeInt32(
      12,
      f
    );
  }
};


/**
 * optional string title = 1;
 * @return {string}
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} returns this
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.setTitle = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string content_type = 2;
 * @return {string}
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.getContentType = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} returns this
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.setContentType = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional uint64 size = 3;
 * @return {number}
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.getSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} returns this
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.setSize = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional bool public = 11;
 * @return {boolean}
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.getPublic = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 11, false));
};


/**
 * @param {boolean} value
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} returns this
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.setPublic = function(value) {
  return jspb.Message.setProto3BooleanField(this, 11, value);
};


/**
 * optional int32 expiration_days = 12;
 * @return {number}
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.getExpirationDays = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 12, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} returns this
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.setExpirationDays = function(value) {
  return jspb.Message.setField(this, 12, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlRequest} returns this
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.clearExpirationDays = function() {
  return jspb.Message.setField(this, 12, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.AttachmentUploadUrlRequest.prototype.hasExpirationDays = function() {
  return jspb.Message.getField(this, 12) != null;
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
proto.palm.camelia.v1.AttachmentUploadUrlResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.AttachmentUploadUrlResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.AttachmentUploadUrlResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentUploadUrlResponse.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlResponse}
 */
proto.palm.camelia.v1.AttachmentUploadUrlResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.AttachmentUploadUrlResponse;
  return proto.palm.camelia.v1.AttachmentUploadUrlResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.AttachmentUploadUrlResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlResponse}
 */
proto.palm.camelia.v1.AttachmentUploadUrlResponse.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.AttachmentUploadUrlResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.AttachmentUploadUrlResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.AttachmentUploadUrlResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentUploadUrlResponse.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.AttachmentUploadUrlResponse.prototype.getUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentUploadUrlResponse} returns this
 */
proto.palm.camelia.v1.AttachmentUploadUrlResponse.prototype.setUrl = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.AttachmentIndexResponse.repeatedFields_ = [1];



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
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.AttachmentIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.AttachmentIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.camelia.v1.AttachmentIndexResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && azalea_pb.Pagination.toObject(includeInstance, f)
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
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.AttachmentIndexResponse;
  return proto.palm.camelia.v1.AttachmentIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.AttachmentIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.camelia.v1.AttachmentIndexResponse.Item;
      reader.readMessage(value,proto.palm.camelia.v1.AttachmentIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new azalea_pb.Pagination;
      reader.readMessage(value,azalea_pb.Pagination.deserializeBinaryFromReader);
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
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.AttachmentIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.AttachmentIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.camelia.v1.AttachmentIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      azalea_pb.Pagination.serializeBinaryToWriter
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
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.AttachmentIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    bucket: jspb.Message.getFieldWithDefault(msg, 2, ""),
    name: jspb.Message.getFieldWithDefault(msg, 3, ""),
    title: jspb.Message.getFieldWithDefault(msg, 4, ""),
    size: jspb.Message.getFieldWithDefault(msg, 5, 0),
    contentType: jspb.Message.getFieldWithDefault(msg, 6, ""),
    publishedAt: (f = msg.getPublishedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
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
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.AttachmentIndexResponse.Item;
  return proto.palm.camelia.v1.AttachmentIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
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
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.AttachmentIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeInt64(
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
 * optional int64 id = 1;
 * @return {number}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string bucket = 2;
 * @return {string}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getBucket = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setBucket = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string name = 3;
 * @return {string}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string title = 4;
 * @return {string}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setTitle = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional int64 size = 5;
 * @return {number}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getSize = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setSize = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};


/**
 * optional string content_type = 6;
 * @return {string}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getContentType = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setContentType = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};


/**
 * optional google.protobuf.Timestamp published_at = 7;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getPublishedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 7));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
*/
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setPublishedAt = function(value) {
  return jspb.Message.setWrapperField(this, 7, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.clearPublishedAt = function() {
  return this.setPublishedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.hasPublishedAt = function() {
  return jspb.Message.getField(this, 7) != null;
};


/**
 * optional google.protobuf.Timestamp deleted_at = 8;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getDeletedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 8));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
*/
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setDeletedAt = function(value) {
  return jspb.Message.setWrapperField(this, 8, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.clearDeletedAt = function() {
  return this.setDeletedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.hasDeletedAt = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional google.protobuf.Timestamp updated_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
*/
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.Item.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.camelia.v1.AttachmentIndexResponse.Item>}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.AttachmentIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.AttachmentIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.AttachmentIndexResponse.Item>} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse} returns this
*/
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.camelia.v1.AttachmentIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse.Item}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.camelia.v1.AttachmentIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional palm.azalea.v1.Pagination pagination = 2;
 * @return {?proto.palm.azalea.v1.Pagination}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.azalea.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, azalea_pb.Pagination, 2));
};


/**
 * @param {?proto.palm.azalea.v1.Pagination|undefined} value
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse} returns this
*/
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.AttachmentIndexResponse} returns this
 */
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.AttachmentIndexResponse.prototype.hasPagination = function() {
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
proto.palm.camelia.v1.LeaveWordCreateRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.LeaveWordCreateRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.LeaveWordCreateRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LeaveWordCreateRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    body: jspb.Message.getFieldWithDefault(msg, 1, ""),
    editor: jspb.Message.getFieldWithDefault(msg, 2, 0)
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
 * @return {!proto.palm.camelia.v1.LeaveWordCreateRequest}
 */
proto.palm.camelia.v1.LeaveWordCreateRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.LeaveWordCreateRequest;
  return proto.palm.camelia.v1.LeaveWordCreateRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.LeaveWordCreateRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.LeaveWordCreateRequest}
 */
proto.palm.camelia.v1.LeaveWordCreateRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setBody(value);
      break;
    case 2:
      var value = /** @type {!proto.palm.azalea.v1.TextEditor} */ (reader.readEnum());
      msg.setEditor(value);
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
proto.palm.camelia.v1.LeaveWordCreateRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.LeaveWordCreateRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.LeaveWordCreateRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LeaveWordCreateRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getBody();
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
};


/**
 * optional string body = 1;
 * @return {string}
 */
proto.palm.camelia.v1.LeaveWordCreateRequest.prototype.getBody = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LeaveWordCreateRequest} returns this
 */
proto.palm.camelia.v1.LeaveWordCreateRequest.prototype.setBody = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional palm.azalea.v1.TextEditor editor = 2;
 * @return {!proto.palm.azalea.v1.TextEditor}
 */
proto.palm.camelia.v1.LeaveWordCreateRequest.prototype.getEditor = function() {
  return /** @type {!proto.palm.azalea.v1.TextEditor} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.palm.azalea.v1.TextEditor} value
 * @return {!proto.palm.camelia.v1.LeaveWordCreateRequest} returns this
 */
proto.palm.camelia.v1.LeaveWordCreateRequest.prototype.setEditor = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.repeatedFields_ = [1];



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
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.LeaveWordIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.LeaveWordIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.camelia.v1.LeaveWordIndexResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && azalea_pb.Pagination.toObject(includeInstance, f)
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
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.LeaveWordIndexResponse;
  return proto.palm.camelia.v1.LeaveWordIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.LeaveWordIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.camelia.v1.LeaveWordIndexResponse.Item;
      reader.readMessage(value,proto.palm.camelia.v1.LeaveWordIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new azalea_pb.Pagination;
      reader.readMessage(value,azalea_pb.Pagination.deserializeBinaryFromReader);
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
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.LeaveWordIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.LeaveWordIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.camelia.v1.LeaveWordIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      azalea_pb.Pagination.serializeBinaryToWriter
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
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.LeaveWordIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    lang: jspb.Message.getFieldWithDefault(msg, 2, ""),
    ip: jspb.Message.getFieldWithDefault(msg, 3, ""),
    body: jspb.Message.getFieldWithDefault(msg, 4, ""),
    editor: jspb.Message.getFieldWithDefault(msg, 5, 0),
    status: jspb.Message.getFieldWithDefault(msg, 6, 0),
    publishedAt: (f = msg.getPublishedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
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
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.LeaveWordIndexResponse.Item;
  return proto.palm.camelia.v1.LeaveWordIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setLang(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setIp(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setBody(value);
      break;
    case 5:
      var value = /** @type {!proto.palm.azalea.v1.TextEditor} */ (reader.readEnum());
      msg.setEditor(value);
      break;
    case 6:
      var value = /** @type {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item.Status} */ (reader.readEnum());
      msg.setStatus(value);
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
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.LeaveWordIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeInt64(
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
  f = message.getIp();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getBody();
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
  f = message.getStatus();
  if (f !== 0.0) {
    writer.writeEnum(
      6,
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
 * @enum {number}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.Status = {
  PENDING: 0,
  PROCESSING: 2,
  CLOSED: 3
};

/**
 * optional int64 id = 1;
 * @return {number}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string lang = 2;
 * @return {string}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string ip = 3;
 * @return {string}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getIp = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setIp = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string body = 4;
 * @return {string}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getBody = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setBody = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional palm.azalea.v1.TextEditor editor = 5;
 * @return {!proto.palm.azalea.v1.TextEditor}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getEditor = function() {
  return /** @type {!proto.palm.azalea.v1.TextEditor} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {!proto.palm.azalea.v1.TextEditor} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setEditor = function(value) {
  return jspb.Message.setProto3EnumField(this, 5, value);
};


/**
 * optional Status status = 6;
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item.Status}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getStatus = function() {
  return /** @type {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item.Status} */ (jspb.Message.getFieldWithDefault(this, 6, 0));
};


/**
 * @param {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item.Status} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setStatus = function(value) {
  return jspb.Message.setProto3EnumField(this, 6, value);
};


/**
 * optional google.protobuf.Timestamp published_at = 7;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getPublishedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 7));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
*/
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setPublishedAt = function(value) {
  return jspb.Message.setWrapperField(this, 7, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.clearPublishedAt = function() {
  return this.setPublishedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.hasPublishedAt = function() {
  return jspb.Message.getField(this, 7) != null;
};


/**
 * optional google.protobuf.Timestamp deleted_at = 8;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getDeletedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 8));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
*/
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setDeletedAt = function(value) {
  return jspb.Message.setWrapperField(this, 8, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.clearDeletedAt = function() {
  return this.setDeletedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.hasDeletedAt = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional google.protobuf.Timestamp updated_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
*/
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.Item.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.camelia.v1.LeaveWordIndexResponse.Item>}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.LeaveWordIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.LeaveWordIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.LeaveWordIndexResponse.Item>} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse} returns this
*/
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse.Item}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.camelia.v1.LeaveWordIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional palm.azalea.v1.Pagination pagination = 2;
 * @return {?proto.palm.azalea.v1.Pagination}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.azalea.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, azalea_pb.Pagination, 2));
};


/**
 * @param {?proto.palm.azalea.v1.Pagination|undefined} value
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse} returns this
*/
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.LeaveWordIndexResponse} returns this
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.LeaveWordIndexResponse.prototype.hasPagination = function() {
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
proto.palm.camelia.v1.LocaleByLangRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.LocaleByLangRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.LocaleByLangRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleByLangRequest.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.LocaleByLangRequest}
 */
proto.palm.camelia.v1.LocaleByLangRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.LocaleByLangRequest;
  return proto.palm.camelia.v1.LocaleByLangRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.LocaleByLangRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.LocaleByLangRequest}
 */
proto.palm.camelia.v1.LocaleByLangRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.LocaleByLangRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.LocaleByLangRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.LocaleByLangRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleByLangRequest.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.LocaleByLangRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LocaleByLangRequest} returns this
 */
proto.palm.camelia.v1.LocaleByLangRequest.prototype.setLang = function(value) {
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
proto.palm.camelia.v1.LocaleSetRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.LocaleSetRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.LocaleSetRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleSetRequest.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.LocaleSetRequest}
 */
proto.palm.camelia.v1.LocaleSetRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.LocaleSetRequest;
  return proto.palm.camelia.v1.LocaleSetRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.LocaleSetRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.LocaleSetRequest}
 */
proto.palm.camelia.v1.LocaleSetRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.LocaleSetRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.LocaleSetRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.LocaleSetRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleSetRequest.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.LocaleSetRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LocaleSetRequest} returns this
 */
proto.palm.camelia.v1.LocaleSetRequest.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string code = 2;
 * @return {string}
 */
proto.palm.camelia.v1.LocaleSetRequest.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LocaleSetRequest} returns this
 */
proto.palm.camelia.v1.LocaleSetRequest.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string message = 3;
 * @return {string}
 */
proto.palm.camelia.v1.LocaleSetRequest.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LocaleSetRequest} returns this
 */
proto.palm.camelia.v1.LocaleSetRequest.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.LocaleIndexResponse.repeatedFields_ = [1];



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
proto.palm.camelia.v1.LocaleIndexResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.LocaleIndexResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.LocaleIndexResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleIndexResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.camelia.v1.LocaleIndexResponse.Item.toObject, includeInstance),
    pagination: (f = msg.getPagination()) && azalea_pb.Pagination.toObject(includeInstance, f)
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
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse}
 */
proto.palm.camelia.v1.LocaleIndexResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.LocaleIndexResponse;
  return proto.palm.camelia.v1.LocaleIndexResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.LocaleIndexResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse}
 */
proto.palm.camelia.v1.LocaleIndexResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.camelia.v1.LocaleIndexResponse.Item;
      reader.readMessage(value,proto.palm.camelia.v1.LocaleIndexResponse.Item.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    case 2:
      var value = new azalea_pb.Pagination;
      reader.readMessage(value,azalea_pb.Pagination.deserializeBinaryFromReader);
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
proto.palm.camelia.v1.LocaleIndexResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.LocaleIndexResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.LocaleIndexResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleIndexResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.camelia.v1.LocaleIndexResponse.Item.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      azalea_pb.Pagination.serializeBinaryToWriter
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
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.LocaleIndexResponse.Item.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.LocaleIndexResponse.Item} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item}
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.LocaleIndexResponse.Item;
  return proto.palm.camelia.v1.LocaleIndexResponse.Item.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.LocaleIndexResponse.Item} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item}
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
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
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.LocaleIndexResponse.Item.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.LocaleIndexResponse.Item} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f !== 0) {
    writer.writeInt64(
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
 * optional int64 id = 1;
 * @return {number}
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.setId = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional string lang = 2;
 * @return {string}
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string code = 3;
 * @return {string}
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string message = 4;
 * @return {string}
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional google.protobuf.Timestamp updated_at = 9;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 9));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item} returns this
*/
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 9, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item} returns this
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.LocaleIndexResponse.Item.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * repeated Item items = 1;
 * @return {!Array<!proto.palm.camelia.v1.LocaleIndexResponse.Item>}
 */
proto.palm.camelia.v1.LocaleIndexResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.LocaleIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.LocaleIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.LocaleIndexResponse.Item>} value
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse} returns this
*/
proto.palm.camelia.v1.LocaleIndexResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.camelia.v1.LocaleIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item}
 */
proto.palm.camelia.v1.LocaleIndexResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.camelia.v1.LocaleIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse} returns this
 */
proto.palm.camelia.v1.LocaleIndexResponse.prototype.clearItemsList = function() {
  return this.setItemsList([]);
};


/**
 * optional palm.azalea.v1.Pagination pagination = 2;
 * @return {?proto.palm.azalea.v1.Pagination}
 */
proto.palm.camelia.v1.LocaleIndexResponse.prototype.getPagination = function() {
  return /** @type{?proto.palm.azalea.v1.Pagination} */ (
    jspb.Message.getWrapperField(this, azalea_pb.Pagination, 2));
};


/**
 * @param {?proto.palm.azalea.v1.Pagination|undefined} value
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse} returns this
*/
proto.palm.camelia.v1.LocaleIndexResponse.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse} returns this
 */
proto.palm.camelia.v1.LocaleIndexResponse.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.LocaleIndexResponse.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.LocaleByLangResponse.repeatedFields_ = [1];



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
proto.palm.camelia.v1.LocaleByLangResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.LocaleByLangResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.LocaleByLangResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleByLangResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.palm.camelia.v1.LocaleIndexResponse.Item.toObject, includeInstance)
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
 * @return {!proto.palm.camelia.v1.LocaleByLangResponse}
 */
proto.palm.camelia.v1.LocaleByLangResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.LocaleByLangResponse;
  return proto.palm.camelia.v1.LocaleByLangResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.LocaleByLangResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.LocaleByLangResponse}
 */
proto.palm.camelia.v1.LocaleByLangResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.camelia.v1.LocaleIndexResponse.Item;
      reader.readMessage(value,proto.palm.camelia.v1.LocaleIndexResponse.Item.deserializeBinaryFromReader);
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
proto.palm.camelia.v1.LocaleByLangResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.LocaleByLangResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.LocaleByLangResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.LocaleByLangResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.palm.camelia.v1.LocaleIndexResponse.Item.serializeBinaryToWriter
    );
  }
};


/**
 * repeated LocaleIndexResponse.Item items = 1;
 * @return {!Array<!proto.palm.camelia.v1.LocaleIndexResponse.Item>}
 */
proto.palm.camelia.v1.LocaleByLangResponse.prototype.getItemsList = function() {
  return /** @type{!Array<!proto.palm.camelia.v1.LocaleIndexResponse.Item>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.palm.camelia.v1.LocaleIndexResponse.Item, 1));
};


/**
 * @param {!Array<!proto.palm.camelia.v1.LocaleIndexResponse.Item>} value
 * @return {!proto.palm.camelia.v1.LocaleByLangResponse} returns this
*/
proto.palm.camelia.v1.LocaleByLangResponse.prototype.setItemsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.palm.camelia.v1.LocaleIndexResponse.Item=} opt_value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.LocaleIndexResponse.Item}
 */
proto.palm.camelia.v1.LocaleByLangResponse.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.palm.camelia.v1.LocaleIndexResponse.Item, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.LocaleByLangResponse} returns this
 */
proto.palm.camelia.v1.LocaleByLangResponse.prototype.clearItemsList = function() {
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
proto.palm.camelia.v1.BaiduProfile.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.BaiduProfile.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.BaiduProfile} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.BaiduProfile.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.BaiduProfile}
 */
proto.palm.camelia.v1.BaiduProfile.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.BaiduProfile;
  return proto.palm.camelia.v1.BaiduProfile.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.BaiduProfile} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.BaiduProfile}
 */
proto.palm.camelia.v1.BaiduProfile.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.BaiduProfile.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.BaiduProfile.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.BaiduProfile} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.BaiduProfile.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.BaiduProfile.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.BaiduProfile} returns this
 */
proto.palm.camelia.v1.BaiduProfile.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string content = 2;
 * @return {string}
 */
proto.palm.camelia.v1.BaiduProfile.prototype.getContent = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.BaiduProfile} returns this
 */
proto.palm.camelia.v1.BaiduProfile.prototype.setContent = function(value) {
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
proto.palm.camelia.v1.IndexNowProfile.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.IndexNowProfile.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.IndexNowProfile} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.IndexNowProfile.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.IndexNowProfile}
 */
proto.palm.camelia.v1.IndexNowProfile.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.IndexNowProfile;
  return proto.palm.camelia.v1.IndexNowProfile.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.IndexNowProfile} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.IndexNowProfile}
 */
proto.palm.camelia.v1.IndexNowProfile.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.IndexNowProfile.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.IndexNowProfile.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.IndexNowProfile} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.IndexNowProfile.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.IndexNowProfile.prototype.getKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.IndexNowProfile} returns this
 */
proto.palm.camelia.v1.IndexNowProfile.prototype.setKey = function(value) {
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
proto.palm.camelia.v1.GoogleProfile.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.GoogleProfile.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.GoogleProfile} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.GoogleProfile.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.GoogleProfile}
 */
proto.palm.camelia.v1.GoogleProfile.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.GoogleProfile;
  return proto.palm.camelia.v1.GoogleProfile.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.GoogleProfile} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.GoogleProfile}
 */
proto.palm.camelia.v1.GoogleProfile.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.GoogleProfile.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.GoogleProfile.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.GoogleProfile} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.GoogleProfile.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.GoogleProfile.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.GoogleProfile} returns this
 */
proto.palm.camelia.v1.GoogleProfile.prototype.setCode = function(value) {
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
proto.palm.camelia.v1.ReCaptchaProfile.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.ReCaptchaProfile.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.ReCaptchaProfile} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.ReCaptchaProfile.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.ReCaptchaProfile}
 */
proto.palm.camelia.v1.ReCaptchaProfile.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.ReCaptchaProfile;
  return proto.palm.camelia.v1.ReCaptchaProfile.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.ReCaptchaProfile} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.ReCaptchaProfile}
 */
proto.palm.camelia.v1.ReCaptchaProfile.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.ReCaptchaProfile.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.ReCaptchaProfile.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.ReCaptchaProfile} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.ReCaptchaProfile.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.ReCaptchaProfile.prototype.getSiteKey = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.ReCaptchaProfile} returns this
 */
proto.palm.camelia.v1.ReCaptchaProfile.prototype.setSiteKey = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string secret = 2;
 * @return {string}
 */
proto.palm.camelia.v1.ReCaptchaProfile.prototype.getSecret = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.ReCaptchaProfile} returns this
 */
proto.palm.camelia.v1.ReCaptchaProfile.prototype.setSecret = function(value) {
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
proto.palm.camelia.v1.SiteSetFaviconRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteSetFaviconRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteSetFaviconRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteSetFaviconRequest.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.SiteSetFaviconRequest}
 */
proto.palm.camelia.v1.SiteSetFaviconRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteSetFaviconRequest;
  return proto.palm.camelia.v1.SiteSetFaviconRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteSetFaviconRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteSetFaviconRequest}
 */
proto.palm.camelia.v1.SiteSetFaviconRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.SiteSetFaviconRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteSetFaviconRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteSetFaviconRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteSetFaviconRequest.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.SiteSetFaviconRequest.prototype.getUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteSetFaviconRequest} returns this
 */
proto.palm.camelia.v1.SiteSetFaviconRequest.prototype.setUrl = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.SiteLayoutResponse.repeatedFields_ = [6,97];



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
proto.palm.camelia.v1.SiteLayoutResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteLayoutResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteLayoutResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    lang: jspb.Message.getFieldWithDefault(msg, 1, ""),
    title: jspb.Message.getFieldWithDefault(msg, 2, ""),
    subhead: jspb.Message.getFieldWithDefault(msg, 3, ""),
    author: (f = msg.getAuthor()) && proto.palm.camelia.v1.SiteLayoutResponse.Author.toObject(includeInstance, f),
    description: jspb.Message.getFieldWithDefault(msg, 5, ""),
    keywordsList: (f = jspb.Message.getRepeatedField(msg, 6)) == null ? undefined : f,
    favicon: jspb.Message.getFieldWithDefault(msg, 8, ""),
    copyright: jspb.Message.getFieldWithDefault(msg, 9, ""),
    gab: (f = msg.getGab()) && proto.palm.camelia.v1.SiteLayoutResponse.GabCode.toObject(includeInstance, f),
    icp: (f = msg.getIcp()) && proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.toObject(includeInstance, f),
    languagesList: (f = jspb.Message.getRepeatedField(msg, 97)) == null ? undefined : f,
    version: jspb.Message.getFieldWithDefault(msg, 98, ""),
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
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse}
 */
proto.palm.camelia.v1.SiteLayoutResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteLayoutResponse;
  return proto.palm.camelia.v1.SiteLayoutResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse}
 */
proto.palm.camelia.v1.SiteLayoutResponse.deserializeBinaryFromReader = function(msg, reader) {
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
      msg.setTitle(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setSubhead(value);
      break;
    case 4:
      var value = new proto.palm.camelia.v1.SiteLayoutResponse.Author;
      reader.readMessage(value,proto.palm.camelia.v1.SiteLayoutResponse.Author.deserializeBinaryFromReader);
      msg.setAuthor(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setDescription(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.addKeywords(value);
      break;
    case 8:
      var value = /** @type {string} */ (reader.readString());
      msg.setFavicon(value);
      break;
    case 9:
      var value = /** @type {string} */ (reader.readString());
      msg.setCopyright(value);
      break;
    case 11:
      var value = new proto.palm.camelia.v1.SiteLayoutResponse.GabCode;
      reader.readMessage(value,proto.palm.camelia.v1.SiteLayoutResponse.GabCode.deserializeBinaryFromReader);
      msg.setGab(value);
      break;
    case 12:
      var value = new proto.palm.camelia.v1.SiteLayoutResponse.IcpCode;
      reader.readMessage(value,proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.deserializeBinaryFromReader);
      msg.setIcp(value);
      break;
    case 97:
      var value = /** @type {string} */ (reader.readString());
      msg.addLanguages(value);
      break;
    case 98:
      var value = /** @type {string} */ (reader.readString());
      msg.setVersion(value);
      break;
    case 99:
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
proto.palm.camelia.v1.SiteLayoutResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteLayoutResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteLayoutResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
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
  f = message.getSubhead();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getAuthor();
  if (f != null) {
    writer.writeMessage(
      4,
      f,
      proto.palm.camelia.v1.SiteLayoutResponse.Author.serializeBinaryToWriter
    );
  }
  f = message.getDescription();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = message.getKeywordsList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      6,
      f
    );
  }
  f = message.getFavicon();
  if (f.length > 0) {
    writer.writeString(
      8,
      f
    );
  }
  f = message.getCopyright();
  if (f.length > 0) {
    writer.writeString(
      9,
      f
    );
  }
  f = message.getGab();
  if (f != null) {
    writer.writeMessage(
      11,
      f,
      proto.palm.camelia.v1.SiteLayoutResponse.GabCode.serializeBinaryToWriter
    );
  }
  f = message.getIcp();
  if (f != null) {
    writer.writeMessage(
      12,
      f,
      proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.serializeBinaryToWriter
    );
  }
  f = message.getLanguagesList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      97,
      f
    );
  }
  f = message.getVersion();
  if (f.length > 0) {
    writer.writeString(
      98,
      f
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      99,
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
proto.palm.camelia.v1.SiteLayoutResponse.Author.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteLayoutResponse.Author.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.Author} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteLayoutResponse.Author.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.Author}
 */
proto.palm.camelia.v1.SiteLayoutResponse.Author.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteLayoutResponse.Author;
  return proto.palm.camelia.v1.SiteLayoutResponse.Author.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.Author} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.Author}
 */
proto.palm.camelia.v1.SiteLayoutResponse.Author.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.SiteLayoutResponse.Author.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteLayoutResponse.Author.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.Author} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteLayoutResponse.Author.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.SiteLayoutResponse.Author.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.Author} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.Author.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string email = 2;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.Author.prototype.getEmail = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.Author} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.Author.prototype.setEmail = function(value) {
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
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteLayoutResponse.GabCode.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.GabCode} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.GabCode}
 */
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteLayoutResponse.GabCode;
  return proto.palm.camelia.v1.SiteLayoutResponse.GabCode.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.GabCode} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.GabCode}
 */
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteLayoutResponse.GabCode.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.GabCode} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.GabCode} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.GabCode} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.GabCode.prototype.setName = function(value) {
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
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.IcpCode} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.IcpCode}
 */
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteLayoutResponse.IcpCode;
  return proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.IcpCode} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.IcpCode}
 */
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteLayoutResponse.IcpCode} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.prototype.getCode = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse.IcpCode} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.IcpCode.prototype.setCode = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string lang = 1;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string title = 2;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setTitle = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string subhead = 3;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getSubhead = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setSubhead = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional Author author = 4;
 * @return {?proto.palm.camelia.v1.SiteLayoutResponse.Author}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getAuthor = function() {
  return /** @type{?proto.palm.camelia.v1.SiteLayoutResponse.Author} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.SiteLayoutResponse.Author, 4));
};


/**
 * @param {?proto.palm.camelia.v1.SiteLayoutResponse.Author|undefined} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
*/
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setAuthor = function(value) {
  return jspb.Message.setWrapperField(this, 4, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.clearAuthor = function() {
  return this.setAuthor(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.hasAuthor = function() {
  return jspb.Message.getField(this, 4) != null;
};


/**
 * optional string description = 5;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getDescription = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setDescription = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * repeated string keywords = 6;
 * @return {!Array<string>}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getKeywordsList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 6));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setKeywordsList = function(value) {
  return jspb.Message.setField(this, 6, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.addKeywords = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 6, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.clearKeywordsList = function() {
  return this.setKeywordsList([]);
};


/**
 * optional string favicon = 8;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getFavicon = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 8, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setFavicon = function(value) {
  return jspb.Message.setProto3StringField(this, 8, value);
};


/**
 * optional string copyright = 9;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getCopyright = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 9, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setCopyright = function(value) {
  return jspb.Message.setProto3StringField(this, 9, value);
};


/**
 * optional GabCode gab = 11;
 * @return {?proto.palm.camelia.v1.SiteLayoutResponse.GabCode}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getGab = function() {
  return /** @type{?proto.palm.camelia.v1.SiteLayoutResponse.GabCode} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.SiteLayoutResponse.GabCode, 11));
};


/**
 * @param {?proto.palm.camelia.v1.SiteLayoutResponse.GabCode|undefined} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
*/
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setGab = function(value) {
  return jspb.Message.setWrapperField(this, 11, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.clearGab = function() {
  return this.setGab(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.hasGab = function() {
  return jspb.Message.getField(this, 11) != null;
};


/**
 * optional IcpCode icp = 12;
 * @return {?proto.palm.camelia.v1.SiteLayoutResponse.IcpCode}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getIcp = function() {
  return /** @type{?proto.palm.camelia.v1.SiteLayoutResponse.IcpCode} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.SiteLayoutResponse.IcpCode, 12));
};


/**
 * @param {?proto.palm.camelia.v1.SiteLayoutResponse.IcpCode|undefined} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
*/
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setIcp = function(value) {
  return jspb.Message.setWrapperField(this, 12, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.clearIcp = function() {
  return this.setIcp(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.hasIcp = function() {
  return jspb.Message.getField(this, 12) != null;
};


/**
 * repeated string languages = 97;
 * @return {!Array<string>}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getLanguagesList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 97));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setLanguagesList = function(value) {
  return jspb.Message.setField(this, 97, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.addLanguages = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 97, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.clearLanguagesList = function() {
  return this.setLanguagesList([]);
};


/**
 * optional string version = 98;
 * @return {string}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getVersion = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 98, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setVersion = function(value) {
  return jspb.Message.setProto3StringField(this, 98, value);
};


/**
 * optional google.protobuf.Timestamp created_at = 99;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 99));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
*/
proto.palm.camelia.v1.SiteLayoutResponse.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 99, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.SiteLayoutResponse} returns this
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.SiteLayoutResponse.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 99) != null;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.SiteSetKeywordsRequest.repeatedFields_ = [1];



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
proto.palm.camelia.v1.SiteSetKeywordsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteSetKeywordsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteSetKeywordsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteSetKeywordsRequest.toObject = function(includeInstance, msg) {
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
 * @return {!proto.palm.camelia.v1.SiteSetKeywordsRequest}
 */
proto.palm.camelia.v1.SiteSetKeywordsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteSetKeywordsRequest;
  return proto.palm.camelia.v1.SiteSetKeywordsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteSetKeywordsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteSetKeywordsRequest}
 */
proto.palm.camelia.v1.SiteSetKeywordsRequest.deserializeBinaryFromReader = function(msg, reader) {
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
proto.palm.camelia.v1.SiteSetKeywordsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteSetKeywordsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteSetKeywordsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteSetKeywordsRequest.serializeBinaryToWriter = function(message, writer) {
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
proto.palm.camelia.v1.SiteSetKeywordsRequest.prototype.getItemsList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 1));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.palm.camelia.v1.SiteSetKeywordsRequest} returns this
 */
proto.palm.camelia.v1.SiteSetKeywordsRequest.prototype.setItemsList = function(value) {
  return jspb.Message.setField(this, 1, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.SiteSetKeywordsRequest} returns this
 */
proto.palm.camelia.v1.SiteSetKeywordsRequest.prototype.addItems = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 1, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.SiteSetKeywordsRequest} returns this
 */
proto.palm.camelia.v1.SiteSetKeywordsRequest.prototype.clearItemsList = function() {
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
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteSetInfoRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteSetInfoRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteSetInfoRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    lang: jspb.Message.getFieldWithDefault(msg, 1, ""),
    title: jspb.Message.getFieldWithDefault(msg, 2, ""),
    subhead: jspb.Message.getFieldWithDefault(msg, 3, ""),
    description: jspb.Message.getFieldWithDefault(msg, 4, ""),
    copyright: jspb.Message.getFieldWithDefault(msg, 5, "")
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
 * @return {!proto.palm.camelia.v1.SiteSetInfoRequest}
 */
proto.palm.camelia.v1.SiteSetInfoRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteSetInfoRequest;
  return proto.palm.camelia.v1.SiteSetInfoRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteSetInfoRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteSetInfoRequest}
 */
proto.palm.camelia.v1.SiteSetInfoRequest.deserializeBinaryFromReader = function(msg, reader) {
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
      msg.setTitle(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setSubhead(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setDescription(value);
      break;
    case 5:
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
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteSetInfoRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteSetInfoRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteSetInfoRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getLang();
  if (f.length > 0) {
    writer.writeString(
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
  f = message.getSubhead();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getDescription();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getCopyright();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
};


/**
 * optional string lang = 1;
 * @return {string}
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.getLang = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteSetInfoRequest} returns this
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.setLang = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string title = 2;
 * @return {string}
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.getTitle = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteSetInfoRequest} returns this
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.setTitle = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string subhead = 3;
 * @return {string}
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.getSubhead = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteSetInfoRequest} returns this
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.setSubhead = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string description = 4;
 * @return {string}
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.getDescription = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteSetInfoRequest} returns this
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.setDescription = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string copyright = 5;
 * @return {string}
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.getCopyright = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteSetInfoRequest} returns this
 */
proto.palm.camelia.v1.SiteSetInfoRequest.prototype.setCopyright = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
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
proto.palm.camelia.v1.SiteStatusResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteStatusResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteStatusResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteStatusResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
    postgresql: (f = msg.getPostgresql()) && proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.toObject(includeInstance, f),
    redis: (f = msg.getRedis()) && proto.palm.camelia.v1.SiteStatusResponse.Redis.toObject(includeInstance, f),
    opensearch: (f = msg.getOpensearch()) && proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.toObject(includeInstance, f)
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
 * @return {!proto.palm.camelia.v1.SiteStatusResponse}
 */
proto.palm.camelia.v1.SiteStatusResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteStatusResponse;
  return proto.palm.camelia.v1.SiteStatusResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteStatusResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse}
 */
proto.palm.camelia.v1.SiteStatusResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.palm.camelia.v1.SiteStatusResponse.PostgreSql;
      reader.readMessage(value,proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.deserializeBinaryFromReader);
      msg.setPostgresql(value);
      break;
    case 2:
      var value = new proto.palm.camelia.v1.SiteStatusResponse.Redis;
      reader.readMessage(value,proto.palm.camelia.v1.SiteStatusResponse.Redis.deserializeBinaryFromReader);
      msg.setRedis(value);
      break;
    case 3:
      var value = new proto.palm.camelia.v1.SiteStatusResponse.OpenSearch;
      reader.readMessage(value,proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.deserializeBinaryFromReader);
      msg.setOpensearch(value);
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
proto.palm.camelia.v1.SiteStatusResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteStatusResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteStatusResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteStatusResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPostgresql();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.serializeBinaryToWriter
    );
  }
  f = message.getRedis();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.palm.camelia.v1.SiteStatusResponse.Redis.serializeBinaryToWriter
    );
  }
  f = message.getOpensearch();
  if (f != null) {
    writer.writeMessage(
      3,
      f,
      proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.serializeBinaryToWriter
    );
  }
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.repeatedFields_ = [1];



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
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.toObject = function(includeInstance, msg) {
  var f, obj = {
    migrationsList: (f = jspb.Message.getRepeatedField(msg, 1)) == null ? undefined : f,
    version: jspb.Message.getFieldWithDefault(msg, 9, "")
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
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql}
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteStatusResponse.PostgreSql;
  return proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql}
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.addMigrations(value);
      break;
    case 9:
      var value = /** @type {string} */ (reader.readString());
      msg.setVersion(value);
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
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getMigrationsList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      1,
      f
    );
  }
  f = message.getVersion();
  if (f.length > 0) {
    writer.writeString(
      9,
      f
    );
  }
};


/**
 * repeated string migrations = 1;
 * @return {!Array<string>}
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.prototype.getMigrationsList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 1));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.prototype.setMigrationsList = function(value) {
  return jspb.Message.setField(this, 1, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.prototype.addMigrations = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 1, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.prototype.clearMigrationsList = function() {
  return this.setMigrationsList([]);
};


/**
 * optional string version = 9;
 * @return {string}
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.prototype.getVersion = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 9, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.PostgreSql} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.PostgreSql.prototype.setVersion = function(value) {
  return jspb.Message.setProto3StringField(this, 9, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.repeatedFields_ = [1];



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
proto.palm.camelia.v1.SiteStatusResponse.Redis.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteStatusResponse.Redis.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.Redis} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.toObject = function(includeInstance, msg) {
  var f, obj = {
    nodesList: (f = jspb.Message.getRepeatedField(msg, 1)) == null ? undefined : f
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
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.Redis}
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteStatusResponse.Redis;
  return proto.palm.camelia.v1.SiteStatusResponse.Redis.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.Redis} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.Redis}
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.addNodes(value);
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
proto.palm.camelia.v1.SiteStatusResponse.Redis.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteStatusResponse.Redis.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.Redis} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getNodesList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      1,
      f
    );
  }
};


/**
 * repeated string nodes = 1;
 * @return {!Array<string>}
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.prototype.getNodesList = function() {
  return /** @type {!Array<string>} */ (jspb.Message.getRepeatedField(this, 1));
};


/**
 * @param {!Array<string>} value
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.Redis} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.prototype.setNodesList = function(value) {
  return jspb.Message.setField(this, 1, value || []);
};


/**
 * @param {string} value
 * @param {number=} opt_index
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.Redis} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.prototype.addNodes = function(value, opt_index) {
  return jspb.Message.addToRepeatedField(this, 1, value, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.Redis} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.Redis.prototype.clearNodesList = function() {
  return this.setNodesList([]);
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
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.prototype.toObject = function(opt_includeInstance) {
  return proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.OpenSearch} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.toObject = function(includeInstance, msg) {
  var f, obj = {
    version: jspb.Message.getFieldWithDefault(msg, 1, "")
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
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.OpenSearch}
 */
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.palm.camelia.v1.SiteStatusResponse.OpenSearch;
  return proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.OpenSearch} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.OpenSearch}
 */
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setVersion(value);
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
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.palm.camelia.v1.SiteStatusResponse.OpenSearch} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getVersion();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string version = 1;
 * @return {string}
 */
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.prototype.getVersion = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.palm.camelia.v1.SiteStatusResponse.OpenSearch} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.OpenSearch.prototype.setVersion = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional PostgreSql postgresql = 1;
 * @return {?proto.palm.camelia.v1.SiteStatusResponse.PostgreSql}
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.getPostgresql = function() {
  return /** @type{?proto.palm.camelia.v1.SiteStatusResponse.PostgreSql} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.SiteStatusResponse.PostgreSql, 1));
};


/**
 * @param {?proto.palm.camelia.v1.SiteStatusResponse.PostgreSql|undefined} value
 * @return {!proto.palm.camelia.v1.SiteStatusResponse} returns this
*/
proto.palm.camelia.v1.SiteStatusResponse.prototype.setPostgresql = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.clearPostgresql = function() {
  return this.setPostgresql(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.hasPostgresql = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional Redis redis = 2;
 * @return {?proto.palm.camelia.v1.SiteStatusResponse.Redis}
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.getRedis = function() {
  return /** @type{?proto.palm.camelia.v1.SiteStatusResponse.Redis} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.SiteStatusResponse.Redis, 2));
};


/**
 * @param {?proto.palm.camelia.v1.SiteStatusResponse.Redis|undefined} value
 * @return {!proto.palm.camelia.v1.SiteStatusResponse} returns this
*/
proto.palm.camelia.v1.SiteStatusResponse.prototype.setRedis = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.clearRedis = function() {
  return this.setRedis(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.hasRedis = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional OpenSearch opensearch = 3;
 * @return {?proto.palm.camelia.v1.SiteStatusResponse.OpenSearch}
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.getOpensearch = function() {
  return /** @type{?proto.palm.camelia.v1.SiteStatusResponse.OpenSearch} */ (
    jspb.Message.getWrapperField(this, proto.palm.camelia.v1.SiteStatusResponse.OpenSearch, 3));
};


/**
 * @param {?proto.palm.camelia.v1.SiteStatusResponse.OpenSearch|undefined} value
 * @return {!proto.palm.camelia.v1.SiteStatusResponse} returns this
*/
proto.palm.camelia.v1.SiteStatusResponse.prototype.setOpensearch = function(value) {
  return jspb.Message.setWrapperField(this, 3, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.palm.camelia.v1.SiteStatusResponse} returns this
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.clearOpensearch = function() {
  return this.setOpensearch(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.palm.camelia.v1.SiteStatusResponse.prototype.hasOpensearch = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * @enum {number}
 */
proto.palm.camelia.v1.UserTokenAction = {
  SIGNIN: 0,
  CONFIRM: 1,
  RESETPASSWORD: 2,
  UNLOCK: 3,
  CANCEL: 9
};

/**
 * @enum {number}
 */
proto.palm.camelia.v1.Operation = {
  VIEW: 0,
  READ: 1,
  WRITE: 2,
  MANAGE: 3
};

goog.object.extend(exports, proto.palm.camelia.v1);
