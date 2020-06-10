import Foundation

import JormungandrWallet

@objc class WalletPlugin: CDVPlugin {
    func WALLET_RESTORE(command: CDVInvokedUrlCommand) {
        let mnemonics = command.arguments.objectAtIndex(0)

        self.commandDelegate.runInBackground {
            var pluginResult: CDVPluginResult?
            do {
                let walletPtr = try WalletC.Wallet.recover(mnemonics: mnemonics)
                pluginResult = CDVPluginResult.resultWithStatus(
                    CDVCommandStatus_OK,
                    messageAsString: String(walletPtr)
                )
            } catch WalletError.walletCError(let reason) {
                pluginResult = CDVPluginResult.resultWithStatus(
                    CDVCommandStatus_ERROR,
                    messageAsString: reason
                )
            }
            self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
        }
    }

    func WALLET_RETRIEVE_FUNDS(command: CDVInvokedUrlCommand) {
        let walletPtr = WalletPtr(UInt(command.arguments.objectAtIndex(0)))
        let block0 = command.arguments.objectAtIndex(1)

        self.commandDelegate.runInBackground {
            var pluginResult: CDVPluginResult?
            do {
                let settingsPtr = try WalletC.Wallet.settings(wallet: walletPtr, block0: block0)
                pluginResult = CDVPluginResult.resultWithStatus(
                    CDVCommandStatus_OK,
                    messageAsString: String(settingsPtr)
                )
            } catch WalletError.walletCError(let reason) {
                pluginResult = CDVPluginResult.resultWithStatus(
                    CDVCommandStatus_ERROR,
                    messageAsString: reason
                )
            }
            self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
        }
    }

    func WALLET_TOTAL_FUNDS(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let walletPtr = WalletPtr(UInt(command.arguments.objectAtIndex(0)))
        do {
            let totalValue = try WalletC.Wallet.totalValue(wallet: walletPtr)
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_OK,
                messageAsString: String(totalValue)
            )
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func WALLET_ID(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let walletPtr = WalletPtr(UInt(command.arguments.objectAtIndex(0)))
        do {
            let id = try WalletC.Wallet.id(wallet: walletPtr)
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_OK,
                messageAsArrayBuffer: id
            )
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func WALLET_SET_STATE(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let walletPtr = WalletPtr(UInt(command.arguments.objectAtIndex(0)))
        let value = UInt64(command.arguments.objectAtIndex(1))
        let counter = UInt32(command.arguments.objectAtIndex(2))
        do {
            try WalletC.Wallet.setState(wallet: walletPtr, value: value, counter: counter)
            pluginResult = CDVPluginResult.resultWithStatus(CDVCommandStatus_OK)
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func WALLET_VOTE(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let walletPtr = WalletPtr(UInt(command.arguments.objectAtIndex(0)))
        let settingsPtr = SettingsPtr(UInt(command.arguments.objectAtIndex(1)))
        let proposalPtr = ProposalPtr(UInt(command.arguments.objectAtIndex(2)))
        let choice = Int(command.arguments.objectAtIndex(3))
        do {
            let voteCastTx = try WalletC.Wallet.castVote(
                wallet: walletPtr,
                settings: settingsPtr,
                proposal: proposalPtr,
                choice: choice
            )
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_OK,
                messageAsArrayBuffer: voteCastTx
            )
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func WALLET_CONVERT(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let walletPtr = WalletPtr(UInt(command.arguments.objectAtIndex(0)))
        let settingsPtr = SettingsPtr(UInt(command.arguments.objectAtIndex(1)))
        do {
            let conversionPtr = try WalletC.Wallet.convert(wallet: walletPtr, settings: settingsPtr)
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_OK,
                messageAsString: String(conversionPtr)
            )
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func CONVERSION_TRANSACTIONS_SIZE(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let conversionPtr = ConversiontPtr(UInt(command.arguments.objectAtIndex(0)))
        do {
            let txSize = try WalletC.Convert.transactionsSize(conversion: conversionPtr)
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_OK,
                messageAsString: String(txSize)
            )
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func CONVERSION_TRANSACTIONS_GET(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let conversionPtr = ConversiontPtr(UInt(command.arguments.objectAtIndex(0)))
        let index = UInt(command.arguments.objectAtIndex(1))
        do {
            let transaction = try WalletC.Convert.transactionsGet(
                conversion: conversionPtr,
                index: index
            )
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_OK,
                messageAsArrayBuffer: transaction
            )
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func CONVERSION_IGNORED(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let conversionPtr = ConversiontPtr(UInt(command.arguments.objectAtIndex(0)))
        do {
            let (value, ignored) = try WalletC.Convert.ignored(conversion: conversionPtr)
            let returnValue = ["value": String(value), "ignored": String(ignored)]
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_OK,
                messageAsDictionary: returnValue
            )
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func PROPOSAL_NEW(command: CDVInvokedUrlCommand) {
        var pluginResult: CDVPluginResult?
        let votePlanId = command.arguments.objectAtIndex(0)
        let payloadType = VotePayloadType(Int(command.arguments.objectAtIndex(1)))
        let index = UInt8(command.arguments.objectAtIndex(2))
        let numChoices = UInt8(command.arguments.objectAtIndex(3))
        do {
            let proposalPtr = try WalletC.Proposal.new(
                votePlanId: votePlanId,
                payloadType: payloadType,
                index: index,
                numChoices: numChoices
            )
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_OK,
                messageAsString: String(proposalPtr)
            )
        } catch WalletError.walletCError(let reason) {
            pluginResult = CDVPluginResult.resultWithStatus(
                CDVCommandStatus_ERROR,
                messageAsString: reason
            )
        }
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func WALLET_DELETE(command: CDVInvokedUrlCommand) {
        let walletPtr = WalletPtr(UInt(command.arguments.objectAtIndex(0)))
        WalletC.Wallet.delete(wallet: walletPtr)
        let CDVPluginResult
        .resultWithStatus(CDVCommandStatus_OK)
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func SETTINGS_DELETE(command: CDVInvokedUrlCommand) {
        let settingsPtr = SettingsPtr(UInt(command.arguments.objectAtIndex(0)))
        WalletC.Settings.delete(settings: settingsPtr)
        let CDVPluginResult
        .resultWithStatus(CDVCommandStatus_OK)
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func CONVERSION_DELETE(command: CDVInvokedUrlCommand) {
        let conversionPtr = ConversionPtr(UInt(command.arguments.objectAtIndex(0)))
        WalletC.Conversion.delete(conversion: conversionPtr)
        let CDVPluginResult
        .resultWithStatus(CDVCommandStatus_OK)
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }

    func PROPOSAL_DELETE(command: CDVInvokedUrlCommand) {
        let proposalPtr = ProposalPtr(UInt(command.arguments.objectAtIndex(0)))
        WalletC.Proposal.delete(proposal: proposalPtr)
        let CDVPluginResult
        .resultWithStatus(CDVCommandStatus_OK)
        self.commandDelegate.sendPluginResult(pluginResult, callbackId: command.callbackId)
    }
}
