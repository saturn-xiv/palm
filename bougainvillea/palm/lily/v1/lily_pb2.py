# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: palm/lily/v1/lily.proto
# Protobuf Python Version: 5.27.2
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    2,
    '',
    'palm/lily/v1/lily.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import duration_pb2 as google_dot_protobuf_dot_duration__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x17palm/lily/v1/lily.proto\x12\x0cpalm.lily.v1\x1a\x1egoogle/protobuf/duration.proto\"\x06\n\x04\x42ook\"\x0b\n\tSlideshow\"\t\n\x07\x41rticle\"/\n\x07Section\x12$\n\x05items\x18\x01 \x03(\x0b\x32\x15.palm.lily.v1.Section\"\xb1\x03\n\x0eTeXLiveRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\"\n\x05style\x18\x02 \x01(\x0e\x32\x13.palm.lily.v1.Style\x12\x37\n\x06\x66ormat\x18\x03 \x01(\x0e\x32\'.palm.lily.v1.TeXLiveTask.Output.Format\x12&\n\x03ttl\x18\t \x01(\x0b\x32\x19.google.protobuf.Duration\x12\"\n\x04\x62ook\x18\x0b \x01(\x0b\x32\x12.palm.lily.v1.BookH\x00\x12(\n\x07\x61rticle\x18\x0c \x01(\x0b\x32\x15.palm.lily.v1.ArticleH\x00\x12,\n\tslideshow\x18\r \x01(\x0b\x32\x17.palm.lily.v1.SlideshowH\x00\x12\r\n\x05\x65ntry\x18\x62 \x01(\x0c\x12\x42\n\x0b\x61ttachments\x18\x63 \x03(\x0b\x32-.palm.lily.v1.TeXLiveRequest.AttachmentsEntry\x1a\x32\n\x10\x41ttachmentsEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\x0c:\x02\x38\x01\x42\t\n\x07Payload\"1\n\x0fTeXLiveResponse\x12\x0e\n\x06\x62ucket\x18\x01 \x01(\t\x12\x0e\n\x06object\x18\x02 \x01(\t\"\xb9\x02\n\x0bTeXLiveTask\x12\r\n\x05\x65ntry\x18\x01 \x01(\x0c\x12?\n\x0b\x61ttachments\x18\x03 \x03(\x0b\x32*.palm.lily.v1.TeXLiveTask.AttachmentsEntry\x12\x30\n\x06output\x18\t \x01(\x0b\x32 .palm.lily.v1.TeXLiveTask.Output\x1a\x32\n\x10\x41ttachmentsEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\x0c:\x02\x38\x01\x1at\n\x06Output\x12\x37\n\x06\x66ormat\x18\x01 \x01(\x0e\x32\'.palm.lily.v1.TeXLiveTask.Output.Format\x12\x0e\n\x06\x62ucket\x18\x02 \x01(\t\x12\x0e\n\x06object\x18\x03 \x01(\t\"\x11\n\x06\x46ormat\x12\x07\n\x03Pdf\x10\x00\"\xc8\x02\n\nPandocTask\x12-\n\x05input\x18\x01 \x01(\x0b\x32\x1e.palm.lily.v1.PandocTask.Input\x12/\n\x06output\x18\x02 \x01(\x0b\x32\x1f.palm.lily.v1.PandocTask.Output\x1aI\n\x05Input\x12/\n\x06\x66ormat\x18\x01 \x01(\x0e\x32\x1f.palm.lily.v1.PandocTask.Format\x12\x0f\n\x07payload\x18\x02 \x01(\x0c\x1aY\n\x06Output\x12/\n\x06\x66ormat\x18\x01 \x01(\x0e\x32\x1f.palm.lily.v1.PandocTask.Format\x12\x0e\n\x06\x62ucket\x18\x02 \x01(\t\x12\x0e\n\x06object\x18\x03 \x01(\t\"4\n\x06\x46ormat\x12\t\n\x05Plain\x10\x00\x12\x0c\n\x08Markdown\x10\x01\x12\x07\n\x03Pdf\x10\x02\x12\x08\n\x04\x44ocx\x10\x03*\'\n\x05Style\x12\t\n\x05\x41PA_7\x10\x00\x12\t\n\x05MLA_8\x10\n\x12\x08\n\x04\x43MOS\x10\x14\x32Q\n\x07TeXLive\x12\x46\n\x05ToPdf\x12\x1c.palm.lily.v1.TeXLiveRequest\x1a\x1d.palm.lily.v1.TeXLiveResponse\"\x00\x42\x62\n*com.github.saturn_xiv.palm.plugins.lily.v1P\x01Z2github.com/saturn-xiv/palm/atropa/lily/services/v2b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'palm.lily.v1.lily_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n*com.github.saturn_xiv.palm.plugins.lily.v1P\001Z2github.com/saturn-xiv/palm/atropa/lily/services/v2'
  _globals['_TEXLIVEREQUEST_ATTACHMENTSENTRY']._loaded_options = None
  _globals['_TEXLIVEREQUEST_ATTACHMENTSENTRY']._serialized_options = b'8\001'
  _globals['_TEXLIVETASK_ATTACHMENTSENTRY']._loaded_options = None
  _globals['_TEXLIVETASK_ATTACHMENTSENTRY']._serialized_options = b'8\001'
  _globals['_STYLE']._serialized_start=1288
  _globals['_STYLE']._serialized_end=1327
  _globals['_BOOK']._serialized_start=73
  _globals['_BOOK']._serialized_end=79
  _globals['_SLIDESHOW']._serialized_start=81
  _globals['_SLIDESHOW']._serialized_end=92
  _globals['_ARTICLE']._serialized_start=94
  _globals['_ARTICLE']._serialized_end=103
  _globals['_SECTION']._serialized_start=105
  _globals['_SECTION']._serialized_end=152
  _globals['_TEXLIVEREQUEST']._serialized_start=155
  _globals['_TEXLIVEREQUEST']._serialized_end=588
  _globals['_TEXLIVEREQUEST_ATTACHMENTSENTRY']._serialized_start=527
  _globals['_TEXLIVEREQUEST_ATTACHMENTSENTRY']._serialized_end=577
  _globals['_TEXLIVERESPONSE']._serialized_start=590
  _globals['_TEXLIVERESPONSE']._serialized_end=639
  _globals['_TEXLIVETASK']._serialized_start=642
  _globals['_TEXLIVETASK']._serialized_end=955
  _globals['_TEXLIVETASK_ATTACHMENTSENTRY']._serialized_start=527
  _globals['_TEXLIVETASK_ATTACHMENTSENTRY']._serialized_end=577
  _globals['_TEXLIVETASK_OUTPUT']._serialized_start=839
  _globals['_TEXLIVETASK_OUTPUT']._serialized_end=955
  _globals['_TEXLIVETASK_OUTPUT_FORMAT']._serialized_start=938
  _globals['_TEXLIVETASK_OUTPUT_FORMAT']._serialized_end=955
  _globals['_PANDOCTASK']._serialized_start=958
  _globals['_PANDOCTASK']._serialized_end=1286
  _globals['_PANDOCTASK_INPUT']._serialized_start=1068
  _globals['_PANDOCTASK_INPUT']._serialized_end=1141
  _globals['_PANDOCTASK_OUTPUT']._serialized_start=1143
  _globals['_PANDOCTASK_OUTPUT']._serialized_end=1232
  _globals['_PANDOCTASK_FORMAT']._serialized_start=1234
  _globals['_PANDOCTASK_FORMAT']._serialized_end=1286
  _globals['_TEXLIVE']._serialized_start=1329
  _globals['_TEXLIVE']._serialized_end=1410
# @@protoc_insertion_point(module_scope)
