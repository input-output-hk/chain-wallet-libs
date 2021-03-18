import com.iohk.jormungandrwallet.Wallet;
import com.iohk.jormungandrwallet.Settings;
import com.iohk.jormungandrwallet.Conversion;
import com.iohk.jormungandrwallet.Proposal;
import com.iohk.jormungandrwallet.PendingTransactions;
import com.iohk.jormungandrwallet.SymmetricCipher;
import com.iohk.jormungandrwallet.JormungandrWalletException;
import com.iohk.jormungandrwallet.ErrorCode;

import java.util.Properties;
import java.util.Enumeration;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

import org.junit.Test;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;

public class WalletTest {
    @Test
    public void recoverWallet() throws IOException {
        final long walletPtr = Wallet.recover(
                "neck bulb teach illegal soul cry monitor claw amount boring provide village rival draft stone");

        final byte[] block0 = Files.readAllBytes(Paths.get("../../../test-vectors/block0"));

        final long settingsPtr = Wallet.initialFunds(walletPtr, block0);

        final int totalValue = Wallet.totalValue(walletPtr);

        assertEquals(1000000 + 10000 + 10000 + 1 + 100, totalValue);

        Settings.delete(settingsPtr);
        Wallet.delete(walletPtr);
    }

    @Test
    public void importKeys() throws IOException {
        final byte[] accountKey = { -56, 101, -106, -62, -47, 32, -120, -123, -37, 31, -29, 101, -124, 6, -86, 15, 124,
                -57, -72, -31, 60, 54, 47, -28, 106, 109, -78, 119, -4, 80, 100, 88, 62, 72, 117, -120, -55, -118, 108,
                54, -30, -25, 68, 92, 10, -35, 54, -8, 63, 23, 28, -75, -52, -3, -127, 85, 9, -47, -100, -45, -114, -53,
                10, -13, };

        final byte[] utxoKeys = { 48, 21, 89, -52, -78, -44, -52, 126, -98, 84, -90, -11, 90, -128, -106, 11, -74, -111,
                -73, -79, 64, -107, 73, -17, -122, -107, -87, 46, -92, 26, 111, 79, 64, 82, 49, -88, 6, -62, -25, -71,
                -48, -37, 48, -31, 94, -32, -52, 31, 38, 28, 27, -97, -106, 21, 99, 107, 72, -67, -119, -2, 123, -26,
                -22, 31, -88, -74, -67, -16, -128, -57, 79, -68, 49, 51, 126, -34, 75, 102, -110, -62, -21, -19, 126,
                52, -81, 109, -104, -73, -69, -51, 71, -116, -16, 123, 13, 94, -39, 63, 126, -99, 74, -93, -81, -34, 50,
                26, -31, -85, -74, 27, -125, 68, -62, 67, -55, -48, -76, 7, -53, -8, -111, 125, -74, -33, 44, 101, 61,
                -22, };

        final long walletPtr = Wallet.importKeys(accountKey, utxoKeys);

        final byte[] block0 = Files.readAllBytes(Paths.get("../../../test-vectors/block0"));

        final long settingsPtr = Wallet.initialFunds(walletPtr, block0);

        final int totalValue = Wallet.totalValue(walletPtr);

        assertEquals(10000 + 1000, totalValue);

        Settings.delete(settingsPtr);
        Wallet.delete(walletPtr);
    }

    @Test
    public void convertWallet() throws IOException {
        final long walletPtr = Wallet.recover(
                "neck bulb teach illegal soul cry monitor claw amount boring provide village rival draft stone");

        final byte[] block0 = Files.readAllBytes(Paths.get("../../../test-vectors/block0"));

        final long settingsPtr = Wallet.initialFunds(walletPtr, block0);

        final long conversionPtr = Wallet.convert(walletPtr, settingsPtr);

        final int transactionsSize = Conversion.transactionsSize(conversionPtr);

        assertEquals(1, transactionsSize);

        final byte[] transaction = Conversion.transactionsGet(conversionPtr, 0);

        Conversion.ignored(conversionPtr, new Conversion.IgnoredCallback() {
            @Override
            public void call(long value, long ignored) {
                assertEquals(1, value);
                assertEquals(1, ignored);
            }
        });

        Conversion.delete(conversionPtr);
        Settings.delete(settingsPtr);
        Wallet.delete(walletPtr);
    }

    @Test(expected = IndexOutOfBoundsException.class)
    public void negativeIndexConversionTransaction() throws IOException {
        final long walletPtr = Wallet.recover(
                "neck bulb teach illegal soul cry monitor claw amount boring provide village rival draft stone");

        final byte[] block0 = Files.readAllBytes(Paths.get("../../../test-vectors/block0"));

        final long settingsPtr = Wallet.initialFunds(walletPtr, block0);

        final long conversionPtr = Wallet.convert(walletPtr, settingsPtr);

        final int transactionsSize = Conversion.transactionsSize(conversionPtr);

        try {
            final byte[] transaction = Conversion.transactionsGet(conversionPtr, -1);
        } catch (final Exception e) {
            Conversion.delete(conversionPtr);
            Settings.delete(settingsPtr);
            Wallet.delete(walletPtr);
            throw e;
        }
    }

    @Test
    public void voteCast() throws IOException {
        final long walletPtr = Wallet.recover(
                "neck bulb teach illegal soul cry monitor claw amount boring provide village rival draft stone");

        final byte[] block0 = Files.readAllBytes(Paths.get("../../../test-vectors/block0"));

        final long settingsPtr = Wallet.initialFunds(walletPtr, block0);

        final byte[] id = new byte[Proposal.ID_SIZE];
        final long proposalPtr = Proposal.withPublicPayload(id, 0, 3);

        Wallet.setState(walletPtr, 10000000, 0);
        try {
            final byte[] transaction = Wallet.voteCast(walletPtr, settingsPtr, proposalPtr, 1);
        } catch (final Exception e) {
            Proposal.delete(proposalPtr);
            Settings.delete(settingsPtr);
            Wallet.delete(walletPtr);
            System.out.println(e.getMessage());
            throw e;
        }
    }

    @Test
    public void confirmConversionTransaction() throws IOException {
        final long walletPtr = Wallet.recover(
                "neck bulb teach illegal soul cry monitor claw amount boring provide village rival draft stone");

        final byte[] block0 = Files.readAllBytes(Paths.get("../../../test-vectors/block0"));

        final long settingsPtr = Wallet.initialFunds(walletPtr, block0);

        final long conversionPtr = Wallet.convert(walletPtr, settingsPtr);

        final int transactionsSize = Conversion.transactionsSize(conversionPtr);

        final long before = Wallet.pendingTransactions(walletPtr);

        final int sizeBefore = PendingTransactions.len(before);

        assertEquals(sizeBefore, transactionsSize);

        final byte[] fragmentId = PendingTransactions.get(before, 0);

        PendingTransactions.delete(before);

        Wallet.confirmTransaction(walletPtr, fragmentId);

        final long after = Wallet.pendingTransactions(walletPtr);

        final int sizeAfter = PendingTransactions.len(after);

        assertEquals(sizeAfter, 0);

        PendingTransactions.delete(after);
        Conversion.delete(conversionPtr);
        Settings.delete(settingsPtr);
        Wallet.delete(walletPtr);
    }

    @Test
    public void confirmVoteCast() throws IOException {
        final long walletPtr = Wallet.recover(
                "neck bulb teach illegal soul cry monitor claw amount boring provide village rival draft stone");

        final byte[] block0 = Files.readAllBytes(Paths.get("../../../test-vectors/block0"));

        final long settingsPtr = Wallet.initialFunds(walletPtr, block0);

        final byte[] id = new byte[Proposal.ID_SIZE];
        final long proposalPtr = Proposal.withPublicPayload(id, 0, 3);

        Wallet.setState(walletPtr, 10000000, 0);
        try {
            final byte[] transaction = Wallet.voteCast(walletPtr, settingsPtr, proposalPtr, 1);

            final long before = Wallet.pendingTransactions(walletPtr);

            final int sizeBefore = PendingTransactions.len(before);

            assertEquals(sizeBefore, 1);

            final byte[] fragmentId = PendingTransactions.get(before, 0);

            PendingTransactions.delete(before);

            Wallet.confirmTransaction(walletPtr, fragmentId);

            final long after = Wallet.pendingTransactions(walletPtr);

            final int sizeAfter = PendingTransactions.len(after);

            assertEquals(sizeAfter, 0);

            PendingTransactions.delete(after);
        } catch (final Exception e) {
            Proposal.delete(proposalPtr);
            Settings.delete(settingsPtr);
            Wallet.delete(walletPtr);
            System.out.println(e.getMessage());
            throw e;
        }
    }

    @Test
    public void privateVoteCast() throws IOException {
        final long walletPtr = Wallet.recover(
                "neck bulb teach illegal soul cry monitor claw amount boring provide village rival draft stone");

        final byte[] block0 = Files.readAllBytes(Paths.get("../../../test-vectors/block0"));

        final String encryptingKey = "p256k1_votepk1q30fm0td4pavzzellykgce2a6qv2e3gvexy7q0vj9aalumuc9ankvvpcxm5vmy09n650h73srhq30568023lsr9vcfnvnyatttsxwppept8fnt";

        final long settingsPtr = Wallet.initialFunds(walletPtr, block0);

        final byte[] id = new byte[Proposal.ID_SIZE];
        final long proposalPtr = Proposal.withPrivatePayload(id, 0, 3, encryptingKey);

        Wallet.setState(walletPtr, 10000000, 0);
        try {
            final byte[] transaction = Wallet.voteCast(walletPtr, settingsPtr, proposalPtr, 1);

            final long before = Wallet.pendingTransactions(walletPtr);

            final int sizeBefore = PendingTransactions.len(before);

            assertEquals(sizeBefore, 1);

            final byte[] fragmentId = PendingTransactions.get(before, 0);

            PendingTransactions.delete(before);

            Wallet.confirmTransaction(walletPtr, fragmentId);

            final long after = Wallet.pendingTransactions(walletPtr);

            final int sizeAfter = PendingTransactions.len(after);

            assertEquals(sizeAfter, 0);

            PendingTransactions.delete(after);
        } catch (final Exception e) {
            Proposal.delete(proposalPtr);
            Settings.delete(settingsPtr);
            Wallet.delete(walletPtr);
            System.out.println(e.getMessage());
            throw e;
        }
    }

    @Test
    public void testDecrypt() throws IOException {
        final String hex = "017b938f189c7d1d9e4c75b02710a9c9a6b287b6ca55d624001828cba8aeb3a9d4c2a86261016693c7e05fb281f012fb2d7af44484da09c4d7b2dea6585965a4cc208d2b2fb1aa5ba6338520b3aa9c4f908fdd62816ebe01f496f8b4fc0344892fe245db072d054c3dedff926320589231298e216506c1f6858c5dba915c959a98ba0d0e3995aef91d4216b5172dedf2736b451d452916b81532eb7f8487e9f88a2de4f9261d0a0ddf11698796ad8b6894908024ebc4be9bba985ef9c0f2f71afce0b37520c66938313f6bf81b3fc24f5c93d216cd2528dabc716b8093359fda84db4e58d876d215713f2db000";

        final byte[] encrypted = hexStringToByteArray(hex);

        final byte[] password = { 1, 2, 3, 4 };

        try {
            final byte[] decrypted = SymmetricCipher.decrypt(password, encrypted);

            final byte[] account = { -56, 101, -106, -62, -47, 32, -120, -123, -37, 31, -29, 101, -124, 6, -86, 15, 124,
                    -57, -72, -31, 60, 54, 47, -28, 106, 109, -78, 119, -4, 80, 100, 88, 62, 72, 117, -120, -55, -118,
                    108, 54, -30, -25, 68, 92, 10, -35, 54, -8, 63, 23, 28, -75, -52, -3, -127, 85, 9, -47, -100, -45,
                    -114, -53, 10, -13, };

            final byte[] key1 = { 48, 21, 89, -52, -78, -44, -52, 126, -98, 84, -90, -11, 90, -128, -106, 11, -74, -111,
                    -73, -79, 64, -107, 73, -17, -122, -107, -87, 46, -92, 26, 111, 79, 64, 82, 49, -88, 6, -62, -25,
                    -71, -48, -37, 48, -31, 94, -32, -52, 31, 38, 28, 27, -97, -106, 21, 99, 107, 72, -67, -119, -2,
                    123, -26, -22, 31, };

            final byte[] key2 = { -88, -74, -67, -16, -128, -57, 79, -68, 49, 51, 126, -34, 75, 102, -110, -62, -21,
                    -19, 126, 52, -81, 109, -104, -73, -69, -51, 71, -116, -16, 123, 13, 94, -39, 63, 126, -99, 74, -93,
                    -81, -34, 50, 26, -31, -85, -74, 27, -125, 68, -62, 67, -55, -48, -76, 7, -53, -8, -111, 125, -74,
                    -33, 44, 101, 61, -22, };

            for (int i = 0; i < 64; i++) {
                assertEquals(decrypted[0 * 64 + i], account[i]);
            }

            for (int i = 0; i < 64; i++) {
                assertEquals(decrypted[1 * 64 + i], key1[i]);
            }

            for (int i = 0; i < 64; i++) {
                assertEquals(decrypted[2 * 64 + i], key2[i]);
            }
        } catch (JormungandrWalletException e) {
            assert (false);
        }
    }

    @Test(expected = JormungandrWalletException.class)
    public void testDecryptWrongPassword() throws JormungandrWalletException {
        final String hex = "017b938f189c7d1d9e4c75b02710a9c9a6b287b6ca55d624001828cba8aeb3a9d4c2a86261016693c7e05fb281f012fb2d7af44484da09c4d7b2dea6585965a4cc208d2b2fb1aa5ba6338520b3aa9c4f908fdd62816ebe01f496f8b4fc0344892fe245db072d054c3dedff926320589231298e216506c1f6858c5dba915c959a98ba0d0e3995aef91d4216b5172dedf2736b451d452916b81532eb7f8487e9f88a2de4f9261d0a0ddf11698796ad8b6894908024ebc4be9bba985ef9c0f2f71afce0b37520c66938313f6bf81b3fc24f5c93d216cd2528dabc716b8093359fda84db4e58d876d215713f2db000";

        final byte[] encrypted = hexStringToByteArray(hex);
        final byte[] password = { 127, 127, 127, 127 };
        try {
            SymmetricCipher.decrypt(password, encrypted);
        } catch (JormungandrWalletException e) {
            assertEquals(e.getCode(), ErrorCode.SYMMETRIC_CIPHER_INVALID_PASSWORD);
            throw e;
        }
    }

    @Test(expected = JormungandrWalletException.class)
    public void testDecryptMalformedInput() throws JormungandrWalletException {
        // valid hex, but obviously not encrypted
        final String hex = "0000000000";

        final byte[] encrypted = hexStringToByteArray(hex);
        final byte[] password = { 127, 127, 127, 127 };

        try {
            SymmetricCipher.decrypt(password, encrypted);
        } catch (JormungandrWalletException e) {
            assertEquals(e.getCode(), ErrorCode.SYMMETRIC_CIPHER_ERROR);
            throw e;
        }
    }

    public static byte[] hexStringToByteArray(String s) {
        int len = s.length();
        byte[] data = new byte[len / 2];
        for (int i = 0; i < len; i += 2) {
            data[i / 2] = (byte) ((Character.digit(s.charAt(i), 16) << 4) + Character.digit(s.charAt(i + 1), 16));
        }
        return data;
    }
}
