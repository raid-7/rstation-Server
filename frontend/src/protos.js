/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
import * as $protobuf from "protobufjs/minimal";

// Common aliases
const $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
const $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

export const rstation = $root.rstation = (() => {

    /**
     * Namespace rstation.
     * @exports rstation
     * @namespace
     */
    const rstation = {};

    rstation.Measurement = (function() {

        /**
         * Properties of a Measurement.
         * @memberof rstation
         * @interface IMeasurement
         * @property {string|null} [sensor] Measurement sensor
         * @property {number|null} [value] Measurement value
         * @property {number|Long|null} [timestampUs] Measurement timestampUs
         */

        /**
         * Constructs a new Measurement.
         * @memberof rstation
         * @classdesc Represents a Measurement.
         * @implements IMeasurement
         * @constructor
         * @param {rstation.IMeasurement=} [properties] Properties to set
         */
        function Measurement(properties) {
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Measurement sensor.
         * @member {string} sensor
         * @memberof rstation.Measurement
         * @instance
         */
        Measurement.prototype.sensor = "";

        /**
         * Measurement value.
         * @member {number} value
         * @memberof rstation.Measurement
         * @instance
         */
        Measurement.prototype.value = 0;

        /**
         * Measurement timestampUs.
         * @member {number|Long} timestampUs
         * @memberof rstation.Measurement
         * @instance
         */
        Measurement.prototype.timestampUs = $util.Long ? $util.Long.fromBits(0,0,true) : 0;

        /**
         * Creates a new Measurement instance using the specified properties.
         * @function create
         * @memberof rstation.Measurement
         * @static
         * @param {rstation.IMeasurement=} [properties] Properties to set
         * @returns {rstation.Measurement} Measurement instance
         */
        Measurement.create = function create(properties) {
            return new Measurement(properties);
        };

        /**
         * Encodes the specified Measurement message. Does not implicitly {@link rstation.Measurement.verify|verify} messages.
         * @function encode
         * @memberof rstation.Measurement
         * @static
         * @param {rstation.IMeasurement} message Measurement message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Measurement.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.sensor != null && Object.hasOwnProperty.call(message, "sensor"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.sensor);
            if (message.value != null && Object.hasOwnProperty.call(message, "value"))
                writer.uint32(/* id 2, wireType 5 =*/21).float(message.value);
            if (message.timestampUs != null && Object.hasOwnProperty.call(message, "timestampUs"))
                writer.uint32(/* id 3, wireType 0 =*/24).uint64(message.timestampUs);
            return writer;
        };

        /**
         * Encodes the specified Measurement message, length delimited. Does not implicitly {@link rstation.Measurement.verify|verify} messages.
         * @function encodeDelimited
         * @memberof rstation.Measurement
         * @static
         * @param {rstation.IMeasurement} message Measurement message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Measurement.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a Measurement message from the specified reader or buffer.
         * @function decode
         * @memberof rstation.Measurement
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {rstation.Measurement} Measurement
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Measurement.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.rstation.Measurement();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                case 1: {
                        message.sensor = reader.string();
                        break;
                    }
                case 2: {
                        message.value = reader.float();
                        break;
                    }
                case 3: {
                        message.timestampUs = reader.uint64();
                        break;
                    }
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a Measurement message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof rstation.Measurement
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {rstation.Measurement} Measurement
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Measurement.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a Measurement message.
         * @function verify
         * @memberof rstation.Measurement
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Measurement.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.sensor != null && message.hasOwnProperty("sensor"))
                if (!$util.isString(message.sensor))
                    return "sensor: string expected";
            if (message.value != null && message.hasOwnProperty("value"))
                if (typeof message.value !== "number")
                    return "value: number expected";
            if (message.timestampUs != null && message.hasOwnProperty("timestampUs"))
                if (!$util.isInteger(message.timestampUs) && !(message.timestampUs && $util.isInteger(message.timestampUs.low) && $util.isInteger(message.timestampUs.high)))
                    return "timestampUs: integer|Long expected";
            return null;
        };

        /**
         * Creates a Measurement message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof rstation.Measurement
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {rstation.Measurement} Measurement
         */
        Measurement.fromObject = function fromObject(object) {
            if (object instanceof $root.rstation.Measurement)
                return object;
            let message = new $root.rstation.Measurement();
            if (object.sensor != null)
                message.sensor = String(object.sensor);
            if (object.value != null)
                message.value = Number(object.value);
            if (object.timestampUs != null)
                if ($util.Long)
                    (message.timestampUs = $util.Long.fromValue(object.timestampUs)).unsigned = true;
                else if (typeof object.timestampUs === "string")
                    message.timestampUs = parseInt(object.timestampUs, 10);
                else if (typeof object.timestampUs === "number")
                    message.timestampUs = object.timestampUs;
                else if (typeof object.timestampUs === "object")
                    message.timestampUs = new $util.LongBits(object.timestampUs.low >>> 0, object.timestampUs.high >>> 0).toNumber(true);
            return message;
        };

        /**
         * Creates a plain object from a Measurement message. Also converts values to other types if specified.
         * @function toObject
         * @memberof rstation.Measurement
         * @static
         * @param {rstation.Measurement} message Measurement
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Measurement.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            let object = {};
            if (options.defaults) {
                object.sensor = "";
                object.value = 0;
                if ($util.Long) {
                    let long = new $util.Long(0, 0, true);
                    object.timestampUs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.timestampUs = options.longs === String ? "0" : 0;
            }
            if (message.sensor != null && message.hasOwnProperty("sensor"))
                object.sensor = message.sensor;
            if (message.value != null && message.hasOwnProperty("value"))
                object.value = options.json && !isFinite(message.value) ? String(message.value) : message.value;
            if (message.timestampUs != null && message.hasOwnProperty("timestampUs"))
                if (typeof message.timestampUs === "number")
                    object.timestampUs = options.longs === String ? String(message.timestampUs) : message.timestampUs;
                else
                    object.timestampUs = options.longs === String ? $util.Long.prototype.toString.call(message.timestampUs) : options.longs === Number ? new $util.LongBits(message.timestampUs.low >>> 0, message.timestampUs.high >>> 0).toNumber(true) : message.timestampUs;
            return object;
        };

        /**
         * Converts this Measurement to JSON.
         * @function toJSON
         * @memberof rstation.Measurement
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Measurement.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        /**
         * Gets the default type url for Measurement
         * @function getTypeUrl
         * @memberof rstation.Measurement
         * @static
         * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns {string} The default type url
         */
        Measurement.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
            if (typeUrlPrefix === undefined) {
                typeUrlPrefix = "type.googleapis.com";
            }
            return typeUrlPrefix + "/rstation.Measurement";
        };

        return Measurement;
    })();

    rstation.MeasurementSet = (function() {

        /**
         * Properties of a MeasurementSet.
         * @memberof rstation
         * @interface IMeasurementSet
         * @property {Array.<rstation.IMeasurement>|null} [measurements] MeasurementSet measurements
         */

        /**
         * Constructs a new MeasurementSet.
         * @memberof rstation
         * @classdesc Represents a MeasurementSet.
         * @implements IMeasurementSet
         * @constructor
         * @param {rstation.IMeasurementSet=} [properties] Properties to set
         */
        function MeasurementSet(properties) {
            this.measurements = [];
            if (properties)
                for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * MeasurementSet measurements.
         * @member {Array.<rstation.IMeasurement>} measurements
         * @memberof rstation.MeasurementSet
         * @instance
         */
        MeasurementSet.prototype.measurements = $util.emptyArray;

        /**
         * Creates a new MeasurementSet instance using the specified properties.
         * @function create
         * @memberof rstation.MeasurementSet
         * @static
         * @param {rstation.IMeasurementSet=} [properties] Properties to set
         * @returns {rstation.MeasurementSet} MeasurementSet instance
         */
        MeasurementSet.create = function create(properties) {
            return new MeasurementSet(properties);
        };

        /**
         * Encodes the specified MeasurementSet message. Does not implicitly {@link rstation.MeasurementSet.verify|verify} messages.
         * @function encode
         * @memberof rstation.MeasurementSet
         * @static
         * @param {rstation.IMeasurementSet} message MeasurementSet message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        MeasurementSet.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.measurements != null && message.measurements.length)
                for (let i = 0; i < message.measurements.length; ++i)
                    $root.rstation.Measurement.encode(message.measurements[i], writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
            return writer;
        };

        /**
         * Encodes the specified MeasurementSet message, length delimited. Does not implicitly {@link rstation.MeasurementSet.verify|verify} messages.
         * @function encodeDelimited
         * @memberof rstation.MeasurementSet
         * @static
         * @param {rstation.IMeasurementSet} message MeasurementSet message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        MeasurementSet.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a MeasurementSet message from the specified reader or buffer.
         * @function decode
         * @memberof rstation.MeasurementSet
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {rstation.MeasurementSet} MeasurementSet
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        MeasurementSet.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            let end = length === undefined ? reader.len : reader.pos + length, message = new $root.rstation.MeasurementSet();
            while (reader.pos < end) {
                let tag = reader.uint32();
                switch (tag >>> 3) {
                case 1: {
                        if (!(message.measurements && message.measurements.length))
                            message.measurements = [];
                        message.measurements.push($root.rstation.Measurement.decode(reader, reader.uint32()));
                        break;
                    }
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a MeasurementSet message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof rstation.MeasurementSet
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {rstation.MeasurementSet} MeasurementSet
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        MeasurementSet.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a MeasurementSet message.
         * @function verify
         * @memberof rstation.MeasurementSet
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        MeasurementSet.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.measurements != null && message.hasOwnProperty("measurements")) {
                if (!Array.isArray(message.measurements))
                    return "measurements: array expected";
                for (let i = 0; i < message.measurements.length; ++i) {
                    let error = $root.rstation.Measurement.verify(message.measurements[i]);
                    if (error)
                        return "measurements." + error;
                }
            }
            return null;
        };

        /**
         * Creates a MeasurementSet message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof rstation.MeasurementSet
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {rstation.MeasurementSet} MeasurementSet
         */
        MeasurementSet.fromObject = function fromObject(object) {
            if (object instanceof $root.rstation.MeasurementSet)
                return object;
            let message = new $root.rstation.MeasurementSet();
            if (object.measurements) {
                if (!Array.isArray(object.measurements))
                    throw TypeError(".rstation.MeasurementSet.measurements: array expected");
                message.measurements = [];
                for (let i = 0; i < object.measurements.length; ++i) {
                    if (typeof object.measurements[i] !== "object")
                        throw TypeError(".rstation.MeasurementSet.measurements: object expected");
                    message.measurements[i] = $root.rstation.Measurement.fromObject(object.measurements[i]);
                }
            }
            return message;
        };

        /**
         * Creates a plain object from a MeasurementSet message. Also converts values to other types if specified.
         * @function toObject
         * @memberof rstation.MeasurementSet
         * @static
         * @param {rstation.MeasurementSet} message MeasurementSet
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        MeasurementSet.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            let object = {};
            if (options.arrays || options.defaults)
                object.measurements = [];
            if (message.measurements && message.measurements.length) {
                object.measurements = [];
                for (let j = 0; j < message.measurements.length; ++j)
                    object.measurements[j] = $root.rstation.Measurement.toObject(message.measurements[j], options);
            }
            return object;
        };

        /**
         * Converts this MeasurementSet to JSON.
         * @function toJSON
         * @memberof rstation.MeasurementSet
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        MeasurementSet.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        /**
         * Gets the default type url for MeasurementSet
         * @function getTypeUrl
         * @memberof rstation.MeasurementSet
         * @static
         * @param {string} [typeUrlPrefix] your custom typeUrlPrefix(default "type.googleapis.com")
         * @returns {string} The default type url
         */
        MeasurementSet.getTypeUrl = function getTypeUrl(typeUrlPrefix) {
            if (typeUrlPrefix === undefined) {
                typeUrlPrefix = "type.googleapis.com";
            }
            return typeUrlPrefix + "/rstation.MeasurementSet";
        };

        return MeasurementSet;
    })();

    return rstation;
})();

export { $root as default };
