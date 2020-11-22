// source: bot_manager.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

goog.exportSymbol('proto.services.BotInfo', null, global);
goog.exportSymbol('proto.services.BotInfoList', null, global);
goog.exportSymbol('proto.services.BotInfoListRequest', null, global);
goog.exportSymbol('proto.services.CurrentPosition', null, global);
goog.exportSymbol('proto.services.Strategy', null, global);
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
proto.services.BotInfo = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.services.BotInfo, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.services.BotInfo.displayName = 'proto.services.BotInfo';
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
proto.services.CurrentPosition = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.services.CurrentPosition, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.services.CurrentPosition.displayName = 'proto.services.CurrentPosition';
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
proto.services.BotInfoList = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.services.BotInfoList.repeatedFields_, null);
};
goog.inherits(proto.services.BotInfoList, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.services.BotInfoList.displayName = 'proto.services.BotInfoList';
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
proto.services.BotInfoListRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.services.BotInfoListRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.services.BotInfoListRequest.displayName = 'proto.services.BotInfoListRequest';
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
proto.services.BotInfo.prototype.toObject = function(opt_includeInstance) {
  return proto.services.BotInfo.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.services.BotInfo} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.services.BotInfo.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, ""),
    strategy: jspb.Message.getFieldWithDefault(msg, 2, 0),
    name: jspb.Message.getFieldWithDefault(msg, 3, ""),
    basecurrency: jspb.Message.getFieldWithDefault(msg, 4, ""),
    desc: jspb.Message.getFieldWithDefault(msg, 5, ""),
    config: jspb.Message.getFieldWithDefault(msg, 6, "")
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
 * @return {!proto.services.BotInfo}
 */
proto.services.BotInfo.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.services.BotInfo;
  return proto.services.BotInfo.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.services.BotInfo} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.services.BotInfo}
 */
proto.services.BotInfo.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {!proto.services.Strategy} */ (reader.readEnum());
      msg.setStrategy(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setBasecurrency(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setDesc(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setConfig(value);
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
proto.services.BotInfo.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.services.BotInfo.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.services.BotInfo} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.services.BotInfo.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getStrategy();
  if (f !== 0.0) {
    writer.writeEnum(
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
  f = message.getBasecurrency();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getDesc();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = message.getConfig();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.services.BotInfo.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.services.BotInfo} returns this
 */
proto.services.BotInfo.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional Strategy strategy = 2;
 * @return {!proto.services.Strategy}
 */
proto.services.BotInfo.prototype.getStrategy = function() {
  return /** @type {!proto.services.Strategy} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.services.Strategy} value
 * @return {!proto.services.BotInfo} returns this
 */
proto.services.BotInfo.prototype.setStrategy = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional string name = 3;
 * @return {string}
 */
proto.services.BotInfo.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.services.BotInfo} returns this
 */
proto.services.BotInfo.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string baseCurrency = 4;
 * @return {string}
 */
proto.services.BotInfo.prototype.getBasecurrency = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.services.BotInfo} returns this
 */
proto.services.BotInfo.prototype.setBasecurrency = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string desc = 5;
 * @return {string}
 */
proto.services.BotInfo.prototype.getDesc = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.services.BotInfo} returns this
 */
proto.services.BotInfo.prototype.setDesc = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * optional string config = 6;
 * @return {string}
 */
proto.services.BotInfo.prototype.getConfig = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.services.BotInfo} returns this
 */
proto.services.BotInfo.prototype.setConfig = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
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
proto.services.CurrentPosition.prototype.toObject = function(opt_includeInstance) {
  return proto.services.CurrentPosition.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.services.CurrentPosition} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.services.CurrentPosition.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, ""),
    botid: jspb.Message.getFieldWithDefault(msg, 2, ""),
    symbol: jspb.Message.getFieldWithDefault(msg, 3, ""),
    tradingamount: jspb.Message.getFloatingPointFieldWithDefault(msg, 4, 0.0),
    profitamount: jspb.Message.getFloatingPointFieldWithDefault(msg, 5, 0.0),
    profitpercent: jspb.Message.getFloatingPointFieldWithDefault(msg, 6, 0.0)
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
 * @return {!proto.services.CurrentPosition}
 */
proto.services.CurrentPosition.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.services.CurrentPosition;
  return proto.services.CurrentPosition.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.services.CurrentPosition} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.services.CurrentPosition}
 */
proto.services.CurrentPosition.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setBotid(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setSymbol(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setTradingamount(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setProfitamount(value);
      break;
    case 6:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setProfitpercent(value);
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
proto.services.CurrentPosition.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.services.CurrentPosition.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.services.CurrentPosition} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.services.CurrentPosition.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getBotid();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getSymbol();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getTradingamount();
  if (f !== 0.0) {
    writer.writeDouble(
      4,
      f
    );
  }
  f = message.getProfitamount();
  if (f !== 0.0) {
    writer.writeDouble(
      5,
      f
    );
  }
  f = message.getProfitpercent();
  if (f !== 0.0) {
    writer.writeDouble(
      6,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.services.CurrentPosition.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.services.CurrentPosition} returns this
 */
proto.services.CurrentPosition.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string botId = 2;
 * @return {string}
 */
proto.services.CurrentPosition.prototype.getBotid = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.services.CurrentPosition} returns this
 */
proto.services.CurrentPosition.prototype.setBotid = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string symbol = 3;
 * @return {string}
 */
proto.services.CurrentPosition.prototype.getSymbol = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.services.CurrentPosition} returns this
 */
proto.services.CurrentPosition.prototype.setSymbol = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional double tradingAmount = 4;
 * @return {number}
 */
proto.services.CurrentPosition.prototype.getTradingamount = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 4, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.services.CurrentPosition} returns this
 */
proto.services.CurrentPosition.prototype.setTradingamount = function(value) {
  return jspb.Message.setProto3FloatField(this, 4, value);
};


/**
 * optional double profitAmount = 5;
 * @return {number}
 */
proto.services.CurrentPosition.prototype.getProfitamount = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 5, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.services.CurrentPosition} returns this
 */
proto.services.CurrentPosition.prototype.setProfitamount = function(value) {
  return jspb.Message.setProto3FloatField(this, 5, value);
};


/**
 * optional double profitPercent = 6;
 * @return {number}
 */
proto.services.CurrentPosition.prototype.getProfitpercent = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 6, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.services.CurrentPosition} returns this
 */
proto.services.CurrentPosition.prototype.setProfitpercent = function(value) {
  return jspb.Message.setProto3FloatField(this, 6, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.services.BotInfoList.repeatedFields_ = [1];



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
proto.services.BotInfoList.prototype.toObject = function(opt_includeInstance) {
  return proto.services.BotInfoList.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.services.BotInfoList} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.services.BotInfoList.toObject = function(includeInstance, msg) {
  var f, obj = {
    botsList: jspb.Message.toObjectList(msg.getBotsList(),
    proto.services.BotInfo.toObject, includeInstance)
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
 * @return {!proto.services.BotInfoList}
 */
proto.services.BotInfoList.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.services.BotInfoList;
  return proto.services.BotInfoList.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.services.BotInfoList} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.services.BotInfoList}
 */
proto.services.BotInfoList.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.services.BotInfo;
      reader.readMessage(value,proto.services.BotInfo.deserializeBinaryFromReader);
      msg.addBots(value);
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
proto.services.BotInfoList.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.services.BotInfoList.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.services.BotInfoList} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.services.BotInfoList.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getBotsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.services.BotInfo.serializeBinaryToWriter
    );
  }
};


/**
 * repeated BotInfo bots = 1;
 * @return {!Array<!proto.services.BotInfo>}
 */
proto.services.BotInfoList.prototype.getBotsList = function() {
  return /** @type{!Array<!proto.services.BotInfo>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.services.BotInfo, 1));
};


/**
 * @param {!Array<!proto.services.BotInfo>} value
 * @return {!proto.services.BotInfoList} returns this
*/
proto.services.BotInfoList.prototype.setBotsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.services.BotInfo=} opt_value
 * @param {number=} opt_index
 * @return {!proto.services.BotInfo}
 */
proto.services.BotInfoList.prototype.addBots = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.services.BotInfo, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.services.BotInfoList} returns this
 */
proto.services.BotInfoList.prototype.clearBotsList = function() {
  return this.setBotsList([]);
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
proto.services.BotInfoListRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.services.BotInfoListRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.services.BotInfoListRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.services.BotInfoListRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    offset: jspb.Message.getFieldWithDefault(msg, 1, 0),
    limit: jspb.Message.getFieldWithDefault(msg, 2, 0)
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
 * @return {!proto.services.BotInfoListRequest}
 */
proto.services.BotInfoListRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.services.BotInfoListRequest;
  return proto.services.BotInfoListRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.services.BotInfoListRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.services.BotInfoListRequest}
 */
proto.services.BotInfoListRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setOffset(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setLimit(value);
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
proto.services.BotInfoListRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.services.BotInfoListRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.services.BotInfoListRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.services.BotInfoListRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getOffset();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getLimit();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional int64 offset = 1;
 * @return {number}
 */
proto.services.BotInfoListRequest.prototype.getOffset = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.services.BotInfoListRequest} returns this
 */
proto.services.BotInfoListRequest.prototype.setOffset = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 limit = 2;
 * @return {number}
 */
proto.services.BotInfoListRequest.prototype.getLimit = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.services.BotInfoListRequest} returns this
 */
proto.services.BotInfoListRequest.prototype.setLimit = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * @enum {number}
 */
proto.services.Strategy = {
  TRAILING: 0
};

goog.object.extend(exports, proto.services);
