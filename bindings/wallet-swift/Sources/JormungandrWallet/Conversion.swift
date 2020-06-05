import Foundation
import JormungandrWalletC

class Conversion {
    private var pointer: ConversionPtr

    internal init(withRawPointer pointer: ConversionPtr) {
        self.pointer = pointer
    }

    func size() -> UInt {
        return conversionTransactionsSize(conversion: self.pointer)
    }

    func get(index: UInt) throws -> Data {
        return try conversionTransactionsGet(conversion: self.pointer, index: index)
    }

    func ignored() throws -> (value: UInt64, ignored: UInt) {
        return try conversionIgnored(conversion: self.pointer)
    }

    deinit {
        conversionDelete(conversion: self.pointer)
    }
}
