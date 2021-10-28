package com.iohk.jormungandr_wallet

import android.util.Base64
import android.util.Log
import org.apache.cordova.*
import org.json.JSONException
import org.json.JSONObject


class WalletPlugin
/**
 * Constructor.
 */
    : CordovaPlugin() {
    @ExperimentalUnsignedTypes
    private val wallets: MutableMap<Int, Wallet> = mutableMapOf()
    private var nextWalletId = 0

    @ExperimentalUnsignedTypes
    private val settingsPool: MutableMap<Int, Settings> = mutableMapOf()
    private var nextSettingsId = 0

    /**
     * Sets the context of the Command. This can then be used to do things like get
     * file paths associated with the Activity.
     *
     * @param cordova The context of the main Activity.
     * @param webView The CordovaWebView Cordova is running in.
     */
    override fun initialize(cordova: CordovaInterface?, webView: CordovaWebView?) {
        super.initialize(cordova, webView)
        Log.d(TAG, "Initializing wallet plugin")
    }

    /**
     * Executes the request and returns PluginResult.
     *
     * @param action          The action to execute.
     * @param args            JSONArry of arguments for the plugin.
     * @param callbackContext The callback id used when calling back into
     * JavaScript.
     * @return True if the action was valid, false if not.
     */
    @ExperimentalUnsignedTypes
    @Throws(JSONException::class)
    override fun execute(action: String, args: CordovaArgs, callbackContext: CallbackContext): Boolean {
        Log.d(TAG, "action: $action")
        when (action) {
            "WALLET_IMPORT_KEYS" -> walletImportKeys(args, callbackContext)
            "SETTINGS_NEW" -> settingsNew(args, callbackContext)
            "SETTINGS_GET" -> settingsGet(args, callbackContext)
            "WALLET_DELETE" -> walletDelete(args, callbackContext)
            "SETTINGS_DELETE" -> settingsDelete(args, callbackContext)
            else -> return false
        }
        return true
    }

    @ExperimentalUnsignedTypes
    @Throws(JSONException::class)
    private fun walletImportKeys(args: CordovaArgs, callbackContext: CallbackContext) {
        val accountKey: UByteArray = args.getArrayBuffer(0).toUByteArray()
        try {
            wallets[nextWalletId] = Wallet(accountKey.toList())
            callbackContext.success(nextWalletId)
            nextWalletId += 1
        } catch (e: Exception) {
            callbackContext.error(e.message)
        }
    }

    @ExperimentalUnsignedTypes
    @Throws(JSONException::class)
    private fun settingsNew(args: CordovaArgs, callbackContext: CallbackContext) {
        val block0Hash = args.getArrayBuffer(0).toUByteArray().toList()
        val discriminationInput = args.getInt(1)
        val fees = args[2] as JSONObject
        val block0Date = args.getString(3).toULong()
        val slotDuration = args.getString(4).toUByte()
        val era = args[5] as JSONObject
        val transactionMaxExpiryEpochs = args.getString(6).toUByte()
        try {
            val constant = fees.getString("constant").toULong()
            val coefficient = fees.getString("coefficient").toULong()
            val certificate = fees.getString("certificate").toULong()
            val certificatePoolRegistration = fees.getString("certificatePoolRegistration").toULong()
            val certificateStakeDelegation = fees.getString("certificateStakeDelegation").toULong()
            val certificateOwnerStakeDelegation = fees.getString("certificateOwnerStakeDelegation").toULong()
            val certificateVotePlan = fees.getString("certificateVotePlan").toULong()
            val certificateVoteCast = fees.getString("certificateVoteCast").toULong()
            val linearFees: LinearFee = LinearFee(constant, coefficient, certificate,
                    PerCertificateFee(certificatePoolRegistration.toULong(), certificateStakeDelegation.toULong(),
                            certificateOwnerStakeDelegation.toULong()),
                    PerVoteCertificateFee(certificateVotePlan.toULong(), certificateVoteCast.toULong()))
            val discrimination: Discrimination = if (discriminationInput == 0) Discrimination.PRODUCTION else Discrimination.TEST
            val timeEra = TimeEra(era.getString("epochStart").toUInt(),
                    era.getString("slotStart").toULong(),
                    era.getString("slotsPerEpoch").toUInt())
            val settingsInit = SettingsInit(linearFees, discrimination, block0Hash, block0Date, slotDuration,
                    timeEra, transactionMaxExpiryEpochs)

            val settingsId = nextSettingsId
            nextSettingsId += 1
            settingsPool[settingsId] = Settings(settingsInit)

            callbackContext.success(settingsId.toString())
        } catch (e: java.lang.Exception) {
            callbackContext.error(e.message)
        }
    }

    @ExperimentalUnsignedTypes
    @Throws(JSONException::class)
    private fun settingsGet(args: CordovaArgs, callbackContext: CallbackContext) {
        val settingsId = args.getInt(0)
        try {
            val settings = settingsPool[settingsId]

            val settingsRaw = settings?.settingsRaw()

            val fees = settingsRaw?.fees
            val discrimination: Discrimination? = settingsRaw?.discrimination
            val block0Hash = settingsRaw?.block0Hash?.toUByteArray()
            val feesJson = JSONObject().put("constant", fees?.constant.toString())
                    .put("coefficient", fees?.coefficient.toString())
                    .put("certificate", fees?.certificate.toString())
                    .put("certificatePoolRegistration",
                            fees?.perCertificateFees?.certificatePoolRegistration.toString())
                    .put("certificateStakeDelegation",
                            fees?.perCertificateFees?.certificateStakeDelegation.toString())
                    .put("certificateOwnerStakeDelegation",
                            fees?.perCertificateFees?.certificateOwnerStakeDelegation.toString())
                    .put("certificateVotePlan", fees?.perVoteCertificateFees?.certificateVotePlan.toString())
                    .put("certificateVoteCast", fees?.perVoteCertificateFees?.certificateVoteCast.toString())
            val result: JSONObject = JSONObject().put("fees", feesJson)
                    .put("discrimination", if (discrimination === Discrimination.PRODUCTION) 0 else 1)
                    .put("block0Hash", Base64.encodeToString(block0Hash?.asByteArray(), Base64.NO_WRAP))

            callbackContext.success(result)
        } catch (e: java.lang.Exception) {
            callbackContext.error(e.message)
        }
    }

    @ExperimentalUnsignedTypes
    @Throws(JSONException::class)
    private fun walletDelete(args: CordovaArgs, callbackContext: CallbackContext) {
        val walletId: Int = args.getInt(0)
        try {
            wallets[walletId]?.destroy()
            callbackContext.success()
        } catch (e: Exception) {
            callbackContext.error(e.message)
        }
    }

    @ExperimentalUnsignedTypes
    @Throws(JSONException::class)
    private fun settingsDelete(args: CordovaArgs, callbackContext: CallbackContext) {
        val settingsId: Int = args.getInt(0)
        try {
            settingsPool[settingsId]?.destroy()
            callbackContext.success()
        } catch (e: Exception) {
            callbackContext.error(e.message)
        }
    }

    companion object {
        const val TAG = "WALLET"
    }
}
