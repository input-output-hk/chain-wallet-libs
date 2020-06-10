import Foundation
import JormungandrWalletC

class Conversion {
    private var pointer: ConversionPtr

    internal init(withRawPointer pointer: ConversionPtr) {
        self.pointer = pointer
    }

    func size() -> UInt {
        return WalletC.Conversion.transactionsSize(conversion: self.pointer)
    }

    func get(index: UInt) throws -> Data {
        return try WalletC.Conversion.transactionsGet(conversion: self.pointer, index: index)
    }

    func ignored() throws -> (value: UInt64, ignored: UInt) {
        return try WalletC.Conversion.ignored(conversion: self.pointer)
    }

    deinit {
        WalletC.Conversion.delete(conversion: self.pointer)
    }
}
