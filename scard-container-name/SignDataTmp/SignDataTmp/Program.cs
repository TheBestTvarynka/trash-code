using System.Diagnostics;
using System.Security;
using System.Security.Cryptography;

namespace TpmSmartcardSigning
{
    class Program
    {
        static unsafe void sign_data()
        {
            Process currentProcess = Process.GetCurrentProcess();
            Console.WriteLine("Current Process ID: " + currentProcess.Id);

            // Container name of the emulated smart card.
            string containerName = "1b22c362-46ba-4889-ad5c-01f7f25fc10awww";

            SecureString pwd = new SecureString();
            foreach (char c in "214653")
            {
                pwd.AppendChar(c);
            }

            CspParameters csp = new CspParameters(1,
                "Microsoft Base Smart Card Crypto Provider",
                containerName
            );
            csp.KeyPassword = pwd;

            // Just some data we need to sign.
            byte[] dataToSign = new byte[] { 49, 61, 48, 22, 6, 9, 42, 134, 72, 134, 247, 13, 1, 9, 3, 49, 9, 6, 7, 43, 6, 1, 5, 2, 3, 1, 48, 35, 6, 9, 42, 134, 72, 134, 247, 13, 1, 9, 4, 49, 22, 4, 20, 104, 221, 52, 141, 24, 155, 142, 191, 87, 137, 42, 95, 159, 174, 189, 43, 85, 59, 48, 253 };

            Console.WriteLine("Data to sign : " + BitConverter.ToString(dataToSign));

            RSACryptoServiceProvider rsaCsp = new RSACryptoServiceProvider(csp);
            byte[] signature = rsaCsp.SignData(dataToSign, HashAlgorithmName.SHA1, RSASignaturePadding.Pkcs1);

            Console.WriteLine();
            Console.WriteLine("Signature: " + BitConverter.ToString(signature));
        }

        static unsafe void Main(string[] args)
        {
            sign_data();
        }
    }
}
