import Foundation
import JormungandrWalletC

enum WalletError: Error {
    case walletCError(reason: String)
}

enum VotePayloadType: UInt8 {
    case `public` = 1
}

private func processError(_ error: ErrorPtr?) throws {
    if error == nil {
        return
    }

    let cStringReason = iohk_jormungandr_wallet_error_to_string(error)
    let reason = String(cString: cStringReason!)
    iohk_jormungandr_wallet_delete_string(cStringReason)
    iohk_jormungandr_wallet_delete_error(error)
    throw WalletError.walletCError(reason: reason)
}

struct WalletC {
    private init() {}

    struct Wallet {
        private init() {}

        static func recover(mnemonics: String) throws -> WalletPtr {
            var wallet: WalletPtr?
            let error = mnemonics.withCString { cs in
                iohk_jormungandr_wallet_recover(cs, nil, 0, &wallet)
            }
            try processError(error)
            return wallet!
        }

        static func totalValue(wallet: WalletPtr) throws -> UInt64 {
            var total: UInt64 = 0
            let error = iohk_jormungandr_wallet_total_value(wallet, &total)
            try processError(error)
            return total
        }

        static func settings(wallet: WalletPtr, block0: Data) throws -> SettingsPtr {
            let block0 = Array(block0)
            var settings: SettingsPtr?
            let error = iohk_jormungandr_wallet_retrieve_funds(
                wallet,
                block0,
                UInt(block0.count),
                &settings
            )
            try processError(error)
            return settings!
        }

        static func id(wallet: WalletPtr) throws -> Data {
            var id: [UInt8] = Array(repeating: 0, count: 32)
            let error = iohk_jormungandr_wallet_id(wallet, &id)
            try processError(error)
            return Data(id)
        }

        static func setState(wallet: WalletPtr, value: UInt64, counter: UInt32) throws {
            let error = iohk_jormungandr_wallet_set_state(wallet, value, counter)
            try processError(error)
        }

        static func castVote(
            wallet: WalletPtr,
            settings: SettingsPtr,
            proposal: ProposalPtr,
            choice: UInt8
        )
            throws -> Data
        {
            var result: UnsafePointer<UInt8>?
            var length: UInt = 0
            let error = iohk_jormungandr_wallet_vote_cast(
                wallet,
                settings,
                proposal,
                choice,
                &result,
                &length
            )
            try processError(error)
            let buffer = UnsafeBufferPointer(start: result, count: Int(length))
            iohk_jormungandr_wallet_delete_buffer(UnsafeMutablePointer(mutating: result), length)
            return Data(buffer)
        }

        static func convert(wallet: WalletPtr, settings: SettingsPtr) throws -> ConversionPtr {
            var conversion: ConversionPtr?
            let error = iohk_jormungandr_wallet_convert(wallet, settings, &conversion)
            try processError(error)
            return conversion!
        }

        static func delete(wallet: WalletPtr) {
            iohk_jormungandr_wallet_delete_wallet(wallet)
        }

    }

    struct Conversion {
        private init() {}

        static func transactionsSize(conversion: ConversionPtr) -> UInt {
            return iohk_jormungandr_wallet_convert_transactions_size(conversion)
        }

        static func transactionsGet(conversion: ConversionPtr, index: UInt) throws -> Data {
            var result: UnsafePointer<UInt8>?
            var length: UInt = 0
            let error = iohk_jormungandr_wallet_convert_transactions_get(
                conversion,
                index,
                &result,
                &length
            )
            try processError(error)
            return Data(UnsafeBufferPointer(start: result, count: Int(length)))
        }

        static func ignored(conversion: ConversionPtr) throws -> (
            value: UInt64, ignored: UInt
        ) {
            var value: UInt64 = 0
            var ignored: UInt = 0
            let error = iohk_jormungandr_wallet_convert_ignored(conversion, &value, &ignored)
            try processError(error)
            return (value, ignored)
        }

        static func delete(conversion: ConversionPtr) {
            iohk_jormungandr_wallet_delete_conversion(conversion)
        }
    }

    struct Proposal {
        private init() {}

        static func new(
            votePlanId: Data,
            payloadType: VotePayloadType,
            index: UInt8,
            numChoices: UInt8
        )
            throws -> ProposalPtr
        {
            let votePlanId = Array(votePlanId)
            var proposal: ProposalPtr?
            let error = iohk_jormungandr_wallet_vote_proposal(
                votePlanId,
                payloadType.rawValue,
                index,
                numChoices,
                &proposal
            )
            try processError(error)
            return proposal!
        }

        static func delete(proposal: ProposalPtr) {
            iohk_jormungandr_wallet_delete_proposal(proposal)
        }
    }

    struct Settings {
        private init() {}

        static func delete(settings: SettingsPtr) {
            iohk_jormungandr_wallet_delete_settings(settings)
        }
    }
}
