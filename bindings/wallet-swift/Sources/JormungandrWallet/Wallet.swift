import Foundation

import JormungandrWalletC

class Wallet {
    private var pointer: WalletPtr

    init(withMnemonics mnemonics: String) throws {
        self.pointer = try walletRecover(mnemonics: mnemonics)
    }

    func totalValue() throws -> UInt64 {
        return try walletTotalValue(wallet: self.pointer)
    }

    func settings(block0: Data) throws -> Settings {
        return try Settings(withRawPointer: walletSettings(wallet: self.pointer, block0: block0))
    }

    func id() throws -> Data {
        return try walletId(wallet: self.pointer)
    }

    func setState(value: UInt64, counter: UInt32) throws {
        try walletSetState(wallet: self.pointer, value: value, counter: counter)
    }

    func vote(settings: Settings, proposal: Proposal, choice: UInt8) throws -> Data {
        return try walletCastVote(
            wallet: self.pointer,
            settings: settings.pointer,
            proposal: proposal.pointer,
            choice: choice
        )
    }

    func convert(settings: Settings) throws -> Conversion {
        return try Conversion(
            withRawPointer: walletConvert(wallet: self.pointer, settings: settings.pointer)
        )
    }

    deinit {
        walletDelete(wallet: self.pointer)
    }
}
