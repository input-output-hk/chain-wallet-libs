@objc(WalletPlugin)
public class WalletPlugin : CDVPlugin {
    var wallets = [Int : Wallet]()
    var nextWalletId: Int = 0

    @objc(WALLET_RESTORE:)
    func walletRestore(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_IMPORT_KEYS:)
    func walletImportKeys(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(SYMMETRIC_CIPHER_DECRYPT:)
    func symmetricCipherDecrypt(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_RETRIEVE_FUNDS:)
    func walletRetrieveFunds(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_SPENDING_COUNTER:)
    func walletSpendingCounter(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_TOTAL_FUNDS:)
    func walletTotalFunds(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_ID:)
    func walletId(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_SET_STATE:)
    func walletSetState(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_VOTE:)
    func walletVote(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_CONVERT:)
    func walletConvert(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(CONVERSION_TRANSACTIONS_SIZE:)
    func conversionTransactionsSize(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(CONVERSION_TRANSACTIONS_GET:)
    func conversionTransactionsGet(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(CONVERSION_IGNORED:)
    func conversionIgnored(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(PROPOSAL_NEW_PUBLIC:)
    func proposalNewPublic(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(PROPOSAL_NEW_PRIVATE:)
    func proposalNewPrivate(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(SETTINGS_NEW:)
    func settingsNew(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(SETTINGS_GET:)
    func settingsGet(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(FRAGMENT_ID:)
    func fragmentId(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(BLOCK_DATE_FROM_SYSTEM_TIME:)
    func blockDateFromSystemTime(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(MAX_EXPIRATION_DATE:)
    func maxExpirationDate(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(WALLET_DELETE:)
    func walletDelete(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(SETTINGS_DELETE:)
    func settingsDelete(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(CONVERSION_DELETE:)
    func conversionDelete(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }

    @objc(PROPOSAL_DELETE:)
    func proposalDelete(command: CDVInvokedUrlCommand) {
        let pluginResult = CDVPluginResult(status: CDVCommandStatus_ERROR, messageAs: "unimplemented")
        self.commandDelegate.send(pluginResult, callbackId: command.callbackId)
    }
}
