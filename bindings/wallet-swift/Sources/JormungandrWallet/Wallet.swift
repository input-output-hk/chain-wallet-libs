import Foundation
import JormungandrWalletC

class Wallet {
    private var pointer: WalletPtr

    init(withMnemonics mnemonics: String) throws {
        self.pointer = try WalletC.Wallet.recover(mnemonics: mnemonics)
    }

    func totalValue() throws -> UInt64 {
        return try WalletC.Wallet.totalValue(wallet: self.pointer)
    }

    func settings(block0: Data) throws -> Settings {
        return try Settings(
            withRawPointer: WalletC.Wallet.settings(wallet: self.pointer, block0: block0)
        )
    }

    func id() throws -> Data {
        return try WalletC.Wallet.id(wallet: self.pointer)
    }

    func setState(value: UInt64, counter: UInt32) throws {
        try WalletC.Wallet.setState(wallet: self.pointer, value: value, counter: counter)
    }

    func vote(settings: Settings, proposal: Proposal, choice: UInt8) throws -> Data {
        return try WalletC.Wallet.castVote(
            wallet: self.pointer,
            settings: settings.pointer,
            proposal: proposal.pointer,
            choice: choice
        )
    }

    func convert(settings: Settings) throws -> Conversion {
        return try Conversion(
            withRawPointer: WalletC.Wallet.convert(wallet: self.pointer, settings: settings.pointer)
        )
    }

    deinit {
        WalletC.Wallet.delete(wallet: self.pointer)
    }
}
