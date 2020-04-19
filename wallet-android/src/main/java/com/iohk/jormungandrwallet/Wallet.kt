package com.iohk.jormungandrwallet

class Wallet {
    private var walletPtr: Long = 0


    private external fun recover(mnemonics: String) : Long
    private external fun delete(wallet: Long)
    private external fun totalValue(wallet: Long) : Int
    private external fun initialFunds(wallet: Long, block0: ByteArray) : Long
    companion object {
        init {
            System.loadLibrary("wallet_jni")
        }
    }

    constructor(mnemonics: String) {
        this.walletPtr = recover(mnemonics)
        assert(this.walletPtr != 0.toLong())
    }

    fun initialFunds(block0: ByteArray) : Settings {
        val settings_ptr : Long = initialFunds(this.walletPtr, block0)
        return Settings(settings_ptr)
    }

    fun value() : Int {
        return totalValue(this.walletPtr)
    }
}