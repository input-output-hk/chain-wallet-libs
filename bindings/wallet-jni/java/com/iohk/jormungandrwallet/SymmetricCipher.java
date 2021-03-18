package com.iohk.jormungandrwallet;

import com.iohk.jormungandrwallet.*;

public class SymmetricCipher {
    static {
        System.loadLibrary("wallet_jni");
    }

    public native static byte[] decrypt(byte[] password, byte[] ciphertext) throws JormungandrWalletException;
}