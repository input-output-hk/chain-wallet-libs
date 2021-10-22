// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

import Foundation

// Depending on the consumer's build setup, the low-level FFI code
// might be in a separate module, or it might be compiled inline into
// this module. This is a bit of light hackery to work with both.
#if canImport(jormungandr_walletFFI)
import jormungandr_walletFFI
#endif

fileprivate extension RustBuffer {
    // Allocate a new buffer, copying the contents of a `UInt8` array.
    init(bytes: [UInt8]) {
        let rbuf = bytes.withUnsafeBufferPointer { ptr in
            try! rustCall { ffi_jormungandr_wallet_cc83_rustbuffer_from_bytes(ForeignBytes(bufferPointer: ptr), $0) }
        }
        self.init(capacity: rbuf.capacity, len: rbuf.len, data: rbuf.data)
    }

    // Frees the buffer in place.
    // The buffer must not be used after this is called.
    func deallocate() {
        try! rustCall { ffi_jormungandr_wallet_cc83_rustbuffer_free(self, $0) }
    }
}

fileprivate extension ForeignBytes {
    init(bufferPointer: UnsafeBufferPointer<UInt8>) {
        self.init(len: Int32(bufferPointer.count), data: bufferPointer.baseAddress)
    }
}
// For every type used in the interface, we provide helper methods for conveniently
// lifting and lowering that type from C-compatible data, and for reading and writing
// values of that type in a buffer.

// Helper classes/extensions that don't change.
// Someday, this will be in a libray of its own.

fileprivate extension Data {
    init(rustBuffer: RustBuffer) {
        // TODO: This copies the buffer. Can we read directly from a
        // Rust buffer?
        self.init(bytes: rustBuffer.data!, count: Int(rustBuffer.len))
    }
}

// A helper class to read values out of a byte buffer.
fileprivate class Reader {
    let data: Data
    var offset: Data.Index

    init(data: Data) {
        self.data = data
        self.offset = 0
    }

    // Reads an integer at the current offset, in big-endian order, and advances
    // the offset on success. Throws if reading the integer would move the
    // offset past the end of the buffer.
    func readInt<T: FixedWidthInteger>() throws -> T {
        let range = offset..<offset + MemoryLayout<T>.size
        guard data.count >= range.upperBound else {
            throw UniffiInternalError.bufferOverflow
        }
        if T.self == UInt8.self {
            let value = data[offset]
            offset += 1
            return value as! T
        }
        var value: T = 0
        let _ = withUnsafeMutableBytes(of: &value, { data.copyBytes(to: $0, from: range)})
        offset = range.upperBound
        return value.bigEndian
    }

    // Reads an arbitrary number of bytes, to be used to read
    // raw bytes, this is useful when lifting strings
    func readBytes(count: Int) throws -> Array<UInt8> {
        let range = offset..<(offset+count)
        guard data.count >= range.upperBound else {
            throw UniffiInternalError.bufferOverflow
        }
        var value = [UInt8](repeating: 0, count: count)
        value.withUnsafeMutableBufferPointer({ buffer in
            data.copyBytes(to: buffer, from: range)
        })
        offset = range.upperBound
        return value
    }

    // Reads a float at the current offset.
    @inlinable
    func readFloat() throws -> Float {
        return Float(bitPattern: try readInt())
    }

    // Reads a float at the current offset.
    @inlinable
    func readDouble() throws -> Double {
        return Double(bitPattern: try readInt())
    }

    // Indicates if the offset has reached the end of the buffer.
    @inlinable
    func hasRemaining() -> Bool {
        return offset < data.count
    }
}

// A helper class to write values into a byte buffer.
fileprivate class Writer {
    var bytes: [UInt8]
    var offset: Array<UInt8>.Index

    init() {
        self.bytes = []
        self.offset = 0
    }

    func writeBytes<S>(_ byteArr: S) where S: Sequence, S.Element == UInt8 {
        bytes.append(contentsOf: byteArr)
    }

    // Writes an integer in big-endian order.
    //
    // Warning: make sure what you are trying to write
    // is in the correct type!
    func writeInt<T: FixedWidthInteger>(_ value: T) {
        var value = value.bigEndian
        withUnsafeBytes(of: &value) { bytes.append(contentsOf: $0) }
    }

    @inlinable
    func writeFloat(_ value: Float) {
        writeInt(value.bitPattern)
    }

    @inlinable
    func writeDouble(_ value: Double) {
        writeInt(value.bitPattern)
    }
}


// Types conforming to `Serializable` can be read and written in a bytebuffer.
fileprivate protocol Serializable {
    func write(into: Writer)
    static func read(from: Reader) throws -> Self
}

// Types confirming to `ViaFfi` can be transferred back-and-for over the FFI.
// This is analogous to the Rust trait of the same name.
fileprivate protocol ViaFfi: Serializable {
    associatedtype FfiType
    static func lift(_ v: FfiType) throws -> Self
    func lower() -> FfiType
}

// Types conforming to `Primitive` pass themselves directly over the FFI.
fileprivate protocol Primitive {}

extension Primitive {
    fileprivate typealias FfiType = Self

    fileprivate static func lift(_ v: Self) throws -> Self {
        return v
    }

    fileprivate func lower() -> Self {
        return self
    }
}

// Types conforming to `ViaFfiUsingByteBuffer` lift and lower into a bytebuffer.
// Use this for complex types where it's hard to write a custom lift/lower.
fileprivate protocol ViaFfiUsingByteBuffer: Serializable {}

extension ViaFfiUsingByteBuffer {
    fileprivate typealias FfiType = RustBuffer

    fileprivate static func lift(_ buf: RustBuffer) throws -> Self {
      let reader = Reader(data: Data(rustBuffer: buf))
      let value = try Self.read(from: reader)
      if reader.hasRemaining() {
          throw UniffiInternalError.incompleteData
      }
      buf.deallocate()
      return value
    }

    fileprivate func lower() -> RustBuffer {
      let writer = Writer()
      self.write(into: writer)
      return RustBuffer(bytes: writer.bytes)
    }
}

// Implement our protocols for the built-in types that we use.




extension Array: ViaFfiUsingByteBuffer, ViaFfi, Serializable where Element: Serializable {
    fileprivate static func read(from buf: Reader) throws -> Self {
        let len: Int32 = try buf.readInt()
        var seq = [Element]()
        seq.reserveCapacity(Int(len))
        for _ in 0..<len {
            seq.append(try Element.read(from: buf))
        }
        return seq
    }

    fileprivate func write(into buf: Writer) {
        let len = Int32(self.count)
        buf.writeInt(len)
        for item in self {
            item.write(into: buf)
        }
    }
}








extension UInt8: Primitive, ViaFfi {
    fileprivate static func read(from buf: Reader) throws -> UInt8 {
        return try self.lift(buf.readInt())
    }

    fileprivate func write(into buf: Writer) {
        buf.writeInt(self.lower())
    }
}





extension UInt32: Primitive, ViaFfi {
    fileprivate static func read(from buf: Reader) throws -> UInt32 {
        return try self.lift(buf.readInt())
    }

    fileprivate func write(into buf: Writer) {
        buf.writeInt(self.lower())
    }
}





extension UInt64: Primitive, ViaFfi {
    fileprivate static func read(from buf: Reader) throws -> UInt64 {
        return try self.lift(buf.readInt())
    }

    fileprivate func write(into buf: Writer) {
        buf.writeInt(self.lower())
    }
}





extension String: ViaFfi {
    fileprivate typealias FfiType = RustBuffer

    fileprivate static func lift(_ v: FfiType) throws -> Self {
        defer {
            try! rustCall { ffi_jormungandr_wallet_cc83_rustbuffer_free(v, $0) }
        }
        if v.data == nil {
            return String()
        }
        let bytes = UnsafeBufferPointer<UInt8>(start: v.data!, count: Int(v.len))
        return String(bytes: bytes, encoding: String.Encoding.utf8)!
    }

    fileprivate func lower() -> FfiType {
        return self.utf8CString.withUnsafeBufferPointer { ptr in
            // The swift string gives us int8_t, we want uint8_t.
            ptr.withMemoryRebound(to: UInt8.self) { ptr in
                // The swift string gives us a trailing null byte, we don't want it.
                let buf = UnsafeBufferPointer(rebasing: ptr.prefix(upTo: ptr.count - 1))
                let bytes = ForeignBytes(bufferPointer: buf)
                return try! rustCall { ffi_jormungandr_wallet_cc83_rustbuffer_from_bytes(bytes, $0) }
            }
        }
    }

    fileprivate static func read(from buf: Reader) throws -> Self {
        let len: Int32 = try buf.readInt()
        return String(bytes: try buf.readBytes(count: Int(len)), encoding: String.Encoding.utf8)!
    }

    fileprivate func write(into buf: Writer) {
        let len = Int32(self.utf8.count)
        buf.writeInt(len)
        buf.writeBytes(self.utf8)
    }
}































































































// Public interface members begin here.



// Note that we don't yet support `indirect` for enums.
// See https://github.com/mozilla/uniffi-rs/issues/396 for further discussion.
public enum Discrimination {
    
    case production
    case test
}

extension Discrimination: ViaFfiUsingByteBuffer, ViaFfi {
    fileprivate static func read(from buf: Reader) throws -> Discrimination {
        let variant: Int32 = try buf.readInt()
        switch variant {
        
        case 1: return .production
        case 2: return .test
        default: throw UniffiInternalError.unexpectedEnumCase
        }
    }

    fileprivate func write(into buf: Writer) {
        switch self {
        
        
        case .production:
            buf.writeInt(Int32(1))
        
        
        case .test:
            buf.writeInt(Int32(2))
        
        }
    }
}


extension Discrimination: Equatable, Hashable {}


// Note that we don't yet support `indirect` for enums.
// See https://github.com/mozilla/uniffi-rs/issues/396 for further discussion.
public enum PayloadTypeConfig {
    
    case public
    case private(encryptionKey: String )
}

extension PayloadTypeConfig: ViaFfiUsingByteBuffer, ViaFfi {
    fileprivate static func read(from buf: Reader) throws -> PayloadTypeConfig {
        let variant: Int32 = try buf.readInt()
        switch variant {
        
        case 1: return .public
        case 2: return .private(
            encryptionKey: try String.read(from: buf)
            )
        default: throw UniffiInternalError.unexpectedEnumCase
        }
    }

    fileprivate func write(into buf: Writer) {
        switch self {
        
        
        case .public:
            buf.writeInt(Int32(1))
        
        
        case let .private(encryptionKey):
            buf.writeInt(Int32(2))
            encryptionKey.write(into: buf)
            
        
        }
    }
}


extension PayloadTypeConfig: Equatable, Hashable {}
// An error type for FFI errors. These errors occur at the UniFFI level, not
// the library level.
fileprivate enum UniffiInternalError: LocalizedError {
    case bufferOverflow
    case incompleteData
    case unexpectedOptionalTag
    case unexpectedEnumCase
    case unexpectedNullPointer
    case unexpectedRustCallStatusCode
    case unexpectedRustCallError
    case rustPanic(_ message: String)

    public var errorDescription: String? {
        switch self {
        case .bufferOverflow: return "Reading the requested value would read past the end of the buffer"
        case .incompleteData: return "The buffer still has data after lifting its containing value"
        case .unexpectedOptionalTag: return "Unexpected optional tag; should be 0 or 1"
        case .unexpectedEnumCase: return "Raw enum value doesn't match any cases"
        case .unexpectedNullPointer: return "Raw pointer value was null"
        case .unexpectedRustCallStatusCode: return "Unexpected RustCallStatus code"
        case .unexpectedRustCallError: return "CALL_ERROR but no errorClass specified"
        case let .rustPanic(message): return message
        }
    }
}

fileprivate let CALL_SUCCESS: Int8 = 0
fileprivate let CALL_ERROR: Int8 = 1
fileprivate let CALL_PANIC: Int8 = 2

fileprivate extension RustCallStatus {
    init() {
        self.init(
            code: CALL_SUCCESS,
            errorBuf: RustBuffer.init(
                capacity: 0,
                len: 0,
                data: nil
            )
        )
    }
}



public enum Error {

    
    
    // Simple error enums only carry a message
    case InvalidEncryptionKey(message: String)
    
    // Simple error enums only carry a message
    case MalformedVotePlanId(message: String)
    
    // Simple error enums only carry a message
    case CoreError(message: String)
    
    // Simple error enums only carry a message
    case MalformedBlock0Hash(message: String)
    
}

extension Error: ViaFfiUsingByteBuffer, ViaFfi {
    fileprivate static func read(from buf: Reader) throws -> Error {
        let variant: Int32 = try buf.readInt()
        switch variant {

        

        
        case 1: return .InvalidEncryptionKey(
            message: try String.read(from: buf)
        )
        
        case 2: return .MalformedVotePlanId(
            message: try String.read(from: buf)
        )
        
        case 3: return .CoreError(
            message: try String.read(from: buf)
        )
        
        case 4: return .MalformedBlock0Hash(
            message: try String.read(from: buf)
        )
        

         default: throw UniffiInternalError.unexpectedEnumCase
        }
    }

    fileprivate func write(into buf: Writer) {
        switch self {

        

        
        case let .InvalidEncryptionKey(message):
            buf.writeInt(Int32(1))
            message.write(into: buf)
        case let .MalformedVotePlanId(message):
            buf.writeInt(Int32(2))
            message.write(into: buf)
        case let .CoreError(message):
            buf.writeInt(Int32(3))
            message.write(into: buf)
        case let .MalformedBlock0Hash(message):
            buf.writeInt(Int32(4))
            message.write(into: buf)
        }
    }
}


extension Error: Equatable, Hashable {}

extension Error: Error { }


private func rustCall<T>(_ callback: (UnsafeMutablePointer<RustCallStatus>) -> T) throws -> T {
    try makeRustCall(callback, errorHandler: {
        $0.deallocate()
        return UniffiInternalError.unexpectedRustCallError
    })
}

private func rustCallWithError<T, E: ViaFfiUsingByteBuffer & Error>(_ errorClass: E.Type, _ callback: (UnsafeMutablePointer<RustCallStatus>) -> T) throws -> T {
    try makeRustCall(callback, errorHandler: { return try E.lift($0) })
}

private func makeRustCall<T>(_ callback: (UnsafeMutablePointer<RustCallStatus>) -> T, errorHandler: (RustBuffer) throws -> Error) throws -> T {
    var callStatus = RustCallStatus.init()
    let returnedVal = callback(&callStatus)
    switch callStatus.code {
        case CALL_SUCCESS:
            return returnedVal

        case CALL_ERROR:
            throw try errorHandler(callStatus.errorBuf)

        case CALL_PANIC:
            // When the rust code sees a panic, it tries to construct a RustBuffer
            // with the message.  But if that code panics, then it just sends back
            // an empty buffer.
            if callStatus.errorBuf.len > 0 {
                throw UniffiInternalError.rustPanic(try String.lift(callStatus.errorBuf))
            } else {
                callStatus.errorBuf.deallocate()
                throw UniffiInternalError.rustPanic("Rust panic")
            }

        default:
            throw UniffiInternalError.unexpectedRustCallStatusCode
    }
}
public struct LinearFee {
    public var constant: UInt64
    public var coefficient: UInt64
    public var certificate: UInt64
    public var perCertificateFees: PerCertificateFee
    public var perVoteCertificateFees: PerVoteCertificateFee

    // Default memberwise initializers are never public by default, so we
    // declare one manually.
    public init(constant: UInt64, coefficient: UInt64, certificate: UInt64, perCertificateFees: PerCertificateFee, perVoteCertificateFees: PerVoteCertificateFee ) {
        self.constant = constant
        self.coefficient = coefficient
        self.certificate = certificate
        self.perCertificateFees = perCertificateFees
        self.perVoteCertificateFees = perVoteCertificateFees
    }
}


extension LinearFee: Equatable, Hashable {
    public static func ==(lhs: LinearFee, rhs: LinearFee) -> Bool {
        if lhs.constant != rhs.constant {
            return false
        }
        if lhs.coefficient != rhs.coefficient {
            return false
        }
        if lhs.certificate != rhs.certificate {
            return false
        }
        if lhs.perCertificateFees != rhs.perCertificateFees {
            return false
        }
        if lhs.perVoteCertificateFees != rhs.perVoteCertificateFees {
            return false
        }
        return true
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(constant)
        hasher.combine(coefficient)
        hasher.combine(certificate)
        hasher.combine(perCertificateFees)
        hasher.combine(perVoteCertificateFees)
    }
}


fileprivate extension LinearFee {
    static func read(from buf: Reader) throws -> LinearFee {
        return try LinearFee(
            constant: UInt64.read(from: buf),
            coefficient: UInt64.read(from: buf),
            certificate: UInt64.read(from: buf),
            perCertificateFees: PerCertificateFee.read(from: buf),
            perVoteCertificateFees: PerVoteCertificateFee.read(from: buf)
        )
    }

    func write(into buf: Writer) {
        constant.write(into: buf)
        coefficient.write(into: buf)
        certificate.write(into: buf)
        perCertificateFees.write(into: buf)
        perVoteCertificateFees.write(into: buf)
    }
}

extension LinearFee: ViaFfiUsingByteBuffer, ViaFfi {}

public struct PerCertificateFee {
    public var certificatePoolRegistration: UInt64
    public var certificateStakeDelegation: UInt64
    public var certificateOwnerStakeDelegation: UInt64

    // Default memberwise initializers are never public by default, so we
    // declare one manually.
    public init(certificatePoolRegistration: UInt64, certificateStakeDelegation: UInt64, certificateOwnerStakeDelegation: UInt64 ) {
        self.certificatePoolRegistration = certificatePoolRegistration
        self.certificateStakeDelegation = certificateStakeDelegation
        self.certificateOwnerStakeDelegation = certificateOwnerStakeDelegation
    }
}


extension PerCertificateFee: Equatable, Hashable {
    public static func ==(lhs: PerCertificateFee, rhs: PerCertificateFee) -> Bool {
        if lhs.certificatePoolRegistration != rhs.certificatePoolRegistration {
            return false
        }
        if lhs.certificateStakeDelegation != rhs.certificateStakeDelegation {
            return false
        }
        if lhs.certificateOwnerStakeDelegation != rhs.certificateOwnerStakeDelegation {
            return false
        }
        return true
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(certificatePoolRegistration)
        hasher.combine(certificateStakeDelegation)
        hasher.combine(certificateOwnerStakeDelegation)
    }
}


fileprivate extension PerCertificateFee {
    static func read(from buf: Reader) throws -> PerCertificateFee {
        return try PerCertificateFee(
            certificatePoolRegistration: UInt64.read(from: buf),
            certificateStakeDelegation: UInt64.read(from: buf),
            certificateOwnerStakeDelegation: UInt64.read(from: buf)
        )
    }

    func write(into buf: Writer) {
        certificatePoolRegistration.write(into: buf)
        certificateStakeDelegation.write(into: buf)
        certificateOwnerStakeDelegation.write(into: buf)
    }
}

extension PerCertificateFee: ViaFfiUsingByteBuffer, ViaFfi {}

public struct PerVoteCertificateFee {
    public var certificateVotePlan: UInt64
    public var certificateVoteCast: UInt64

    // Default memberwise initializers are never public by default, so we
    // declare one manually.
    public init(certificateVotePlan: UInt64, certificateVoteCast: UInt64 ) {
        self.certificateVotePlan = certificateVotePlan
        self.certificateVoteCast = certificateVoteCast
    }
}


extension PerVoteCertificateFee: Equatable, Hashable {
    public static func ==(lhs: PerVoteCertificateFee, rhs: PerVoteCertificateFee) -> Bool {
        if lhs.certificateVotePlan != rhs.certificateVotePlan {
            return false
        }
        if lhs.certificateVoteCast != rhs.certificateVoteCast {
            return false
        }
        return true
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(certificateVotePlan)
        hasher.combine(certificateVoteCast)
    }
}


fileprivate extension PerVoteCertificateFee {
    static func read(from buf: Reader) throws -> PerVoteCertificateFee {
        return try PerVoteCertificateFee(
            certificateVotePlan: UInt64.read(from: buf),
            certificateVoteCast: UInt64.read(from: buf)
        )
    }

    func write(into buf: Writer) {
        certificateVotePlan.write(into: buf)
        certificateVoteCast.write(into: buf)
    }
}

extension PerVoteCertificateFee: ViaFfiUsingByteBuffer, ViaFfi {}

public struct TimeEra {
    public var epochStart: UInt32
    public var slotStart: UInt64
    public var slotsPerEpoch: UInt32

    // Default memberwise initializers are never public by default, so we
    // declare one manually.
    public init(epochStart: UInt32, slotStart: UInt64, slotsPerEpoch: UInt32 ) {
        self.epochStart = epochStart
        self.slotStart = slotStart
        self.slotsPerEpoch = slotsPerEpoch
    }
}


extension TimeEra: Equatable, Hashable {
    public static func ==(lhs: TimeEra, rhs: TimeEra) -> Bool {
        if lhs.epochStart != rhs.epochStart {
            return false
        }
        if lhs.slotStart != rhs.slotStart {
            return false
        }
        if lhs.slotsPerEpoch != rhs.slotsPerEpoch {
            return false
        }
        return true
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(epochStart)
        hasher.combine(slotStart)
        hasher.combine(slotsPerEpoch)
    }
}


fileprivate extension TimeEra {
    static func read(from buf: Reader) throws -> TimeEra {
        return try TimeEra(
            epochStart: UInt32.read(from: buf),
            slotStart: UInt64.read(from: buf),
            slotsPerEpoch: UInt32.read(from: buf)
        )
    }

    func write(into buf: Writer) {
        epochStart.write(into: buf)
        slotStart.write(into: buf)
        slotsPerEpoch.write(into: buf)
    }
}

extension TimeEra: ViaFfiUsingByteBuffer, ViaFfi {}

public struct SettingsInit {
    public var fees: LinearFee
    public var discrimination: Discrimination
    public var block0Hash: [UInt8]
    public var block0Date: UInt64
    public var slotDuration: UInt8
    public var timeEra: TimeEra
    public var transactionMaxExpiryEpochs: UInt8

    // Default memberwise initializers are never public by default, so we
    // declare one manually.
    public init(fees: LinearFee, discrimination: Discrimination, block0Hash: [UInt8], block0Date: UInt64, slotDuration: UInt8, timeEra: TimeEra, transactionMaxExpiryEpochs: UInt8 ) {
        self.fees = fees
        self.discrimination = discrimination
        self.block0Hash = block0Hash
        self.block0Date = block0Date
        self.slotDuration = slotDuration
        self.timeEra = timeEra
        self.transactionMaxExpiryEpochs = transactionMaxExpiryEpochs
    }
}


extension SettingsInit: Equatable, Hashable {
    public static func ==(lhs: SettingsInit, rhs: SettingsInit) -> Bool {
        if lhs.fees != rhs.fees {
            return false
        }
        if lhs.discrimination != rhs.discrimination {
            return false
        }
        if lhs.block0Hash != rhs.block0Hash {
            return false
        }
        if lhs.block0Date != rhs.block0Date {
            return false
        }
        if lhs.slotDuration != rhs.slotDuration {
            return false
        }
        if lhs.timeEra != rhs.timeEra {
            return false
        }
        if lhs.transactionMaxExpiryEpochs != rhs.transactionMaxExpiryEpochs {
            return false
        }
        return true
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(fees)
        hasher.combine(discrimination)
        hasher.combine(block0Hash)
        hasher.combine(block0Date)
        hasher.combine(slotDuration)
        hasher.combine(timeEra)
        hasher.combine(transactionMaxExpiryEpochs)
    }
}


fileprivate extension SettingsInit {
    static func read(from buf: Reader) throws -> SettingsInit {
        return try SettingsInit(
            fees: LinearFee.read(from: buf),
            discrimination: Discrimination.read(from: buf),
            block0Hash: [UInt8].read(from: buf),
            block0Date: UInt64.read(from: buf),
            slotDuration: UInt8.read(from: buf),
            timeEra: TimeEra.read(from: buf),
            transactionMaxExpiryEpochs: UInt8.read(from: buf)
        )
    }

    func write(into buf: Writer) {
        fees.write(into: buf)
        discrimination.write(into: buf)
        block0Hash.write(into: buf)
        block0Date.write(into: buf)
        slotDuration.write(into: buf)
        timeEra.write(into: buf)
        transactionMaxExpiryEpochs.write(into: buf)
    }
}

extension SettingsInit: ViaFfiUsingByteBuffer, ViaFfi {}

public struct Proposal {
    public var votePlanId: [UInt8]
    public var index: UInt8
    public var options: UInt8
    public var payloadType: PayloadTypeConfig

    // Default memberwise initializers are never public by default, so we
    // declare one manually.
    public init(votePlanId: [UInt8], index: UInt8, options: UInt8, payloadType: PayloadTypeConfig ) {
        self.votePlanId = votePlanId
        self.index = index
        self.options = options
        self.payloadType = payloadType
    }
}


extension Proposal: Equatable, Hashable {
    public static func ==(lhs: Proposal, rhs: Proposal) -> Bool {
        if lhs.votePlanId != rhs.votePlanId {
            return false
        }
        if lhs.index != rhs.index {
            return false
        }
        if lhs.options != rhs.options {
            return false
        }
        if lhs.payloadType != rhs.payloadType {
            return false
        }
        return true
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(votePlanId)
        hasher.combine(index)
        hasher.combine(options)
        hasher.combine(payloadType)
    }
}


fileprivate extension Proposal {
    static func read(from buf: Reader) throws -> Proposal {
        return try Proposal(
            votePlanId: [UInt8].read(from: buf),
            index: UInt8.read(from: buf),
            options: UInt8.read(from: buf),
            payloadType: PayloadTypeConfig.read(from: buf)
        )
    }

    func write(into buf: Writer) {
        votePlanId.write(into: buf)
        index.write(into: buf)
        options.write(into: buf)
        payloadType.write(into: buf)
    }
}

extension Proposal: ViaFfiUsingByteBuffer, ViaFfi {}

public struct BlockDate {
    public var epoch: UInt32
    public var slot: UInt32

    // Default memberwise initializers are never public by default, so we
    // declare one manually.
    public init(epoch: UInt32, slot: UInt32 ) {
        self.epoch = epoch
        self.slot = slot
    }
}


extension BlockDate: Equatable, Hashable {
    public static func ==(lhs: BlockDate, rhs: BlockDate) -> Bool {
        if lhs.epoch != rhs.epoch {
            return false
        }
        if lhs.slot != rhs.slot {
            return false
        }
        return true
    }

    public func hash(into hasher: inout Hasher) {
        hasher.combine(epoch)
        hasher.combine(slot)
    }
}


fileprivate extension BlockDate {
    static func read(from buf: Reader) throws -> BlockDate {
        return try BlockDate(
            epoch: UInt32.read(from: buf),
            slot: UInt32.read(from: buf)
        )
    }

    func write(into buf: Writer) {
        epoch.write(into: buf)
        slot.write(into: buf)
    }
}

extension BlockDate: ViaFfiUsingByteBuffer, ViaFfi {}






public protocol WalletProtocol {
    func setState(value: UInt64, counter: UInt32 ) 
    func vote(settings: Settings, proposal: Proposal, choice: UInt8, validUntil: BlockDate ) throws -> [UInt8]
    
}

public class Wallet: WalletProtocol {
    fileprivate let pointer: UnsafeMutableRawPointer

    // TODO: We'd like this to be `private` but for Swifty reasons,
    // we can't implement `ViaFfi` without making this `required` and we can't
    // make it `required` without making it `public`.
    required init(unsafeFromRawPointer pointer: UnsafeMutableRawPointer) {
        self.pointer = pointer
    }
    public convenience init(accountKey: [UInt8] )  {
        self.init(unsafeFromRawPointer: try!
    
    
    rustCall() {
    
    jormungandr_wallet_cc83_Wallet_new(accountKey.lower() , $0)
})
    }

    deinit {
        try! rustCall { ffi_jormungandr_wallet_cc83_Wallet_object_free(pointer, $0) }
    }

    

    
    public func setState(value: UInt64, counter: UInt32 )  {
        try!
    rustCall() {
    
    jormungandr_wallet_cc83_Wallet_set_state(self.pointer, value.lower(), counter.lower() , $0
    )
}
    }
    public func vote(settings: Settings, proposal: Proposal, choice: UInt8, validUntil: BlockDate ) throws -> [UInt8] {
        let _retval = try
    rustCallWithError(Error.self) {
    
    jormungandr_wallet_cc83_Wallet_vote(self.pointer, settings.lower(), proposal.lower(), choice.lower(), validUntil.lower() , $0
    )
}
        return try [UInt8].lift(_retval)
    }
    
}


fileprivate extension Wallet {
    fileprivate typealias FfiType = UnsafeMutableRawPointer

    fileprivate static func read(from buf: Reader) throws -> Self {
        let v: UInt64 = try buf.readInt()
        // The Rust code won't compile if a pointer won't fit in a UInt64.
        // We have to go via `UInt` because that's the thing that's the size of a pointer.
        let ptr = UnsafeMutableRawPointer(bitPattern: UInt(truncatingIfNeeded: v))
        if (ptr == nil) {
            throw UniffiInternalError.unexpectedNullPointer
        }
        return try self.lift(ptr!)
    }

    fileprivate func write(into buf: Writer) {
        // This fiddling is because `Int` is the thing that's the same size as a pointer.
        // The Rust code won't compile if a pointer won't fit in a `UInt64`.
        buf.writeInt(UInt64(bitPattern: Int64(Int(bitPattern: self.lower()))))
    }

    fileprivate static func lift(_ pointer: UnsafeMutableRawPointer) throws -> Self {
        return Self(unsafeFromRawPointer: pointer)
    }

    fileprivate func lower() -> UnsafeMutableRawPointer {
        return self.pointer
    }
}

// Ideally this would be `fileprivate`, but Swift says:
// """
// 'private' modifier cannot be used with extensions that declare protocol conformances
// """
extension Wallet : ViaFfi, Serializable {}


public protocol SettingsProtocol {
    
}

public class Settings: SettingsProtocol {
    fileprivate let pointer: UnsafeMutableRawPointer

    // TODO: We'd like this to be `private` but for Swifty reasons,
    // we can't implement `ViaFfi` without making this `required` and we can't
    // make it `required` without making it `public`.
    required init(unsafeFromRawPointer pointer: UnsafeMutableRawPointer) {
        self.pointer = pointer
    }
    public convenience init(settings: SettingsInit ) throws {
        self.init(unsafeFromRawPointer: try
    
    
    rustCallWithError(Error.self) {
    
    jormungandr_wallet_cc83_Settings_new(settings.lower() , $0)
})
    }

    deinit {
        try! rustCall { ffi_jormungandr_wallet_cc83_Settings_object_free(pointer, $0) }
    }

    

    
    
}


fileprivate extension Settings {
    fileprivate typealias FfiType = UnsafeMutableRawPointer

    fileprivate static func read(from buf: Reader) throws -> Self {
        let v: UInt64 = try buf.readInt()
        // The Rust code won't compile if a pointer won't fit in a UInt64.
        // We have to go via `UInt` because that's the thing that's the size of a pointer.
        let ptr = UnsafeMutableRawPointer(bitPattern: UInt(truncatingIfNeeded: v))
        if (ptr == nil) {
            throw UniffiInternalError.unexpectedNullPointer
        }
        return try self.lift(ptr!)
    }

    fileprivate func write(into buf: Writer) {
        // This fiddling is because `Int` is the thing that's the same size as a pointer.
        // The Rust code won't compile if a pointer won't fit in a `UInt64`.
        buf.writeInt(UInt64(bitPattern: Int64(Int(bitPattern: self.lower()))))
    }

    fileprivate static func lift(_ pointer: UnsafeMutableRawPointer) throws -> Self {
        return Self(unsafeFromRawPointer: pointer)
    }

    fileprivate func lower() -> UnsafeMutableRawPointer {
        return self.pointer
    }
}

// Ideally this would be `fileprivate`, but Swift says:
// """
// 'private' modifier cannot be used with extensions that declare protocol conformances
// """
extension Settings : ViaFfi, Serializable {}


