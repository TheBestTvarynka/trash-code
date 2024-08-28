./xfreerdp /v:DESKTOP-8F33RFH.tbt.com /u:t2@tbt.com /p:214653 /log-level:TRACE /smartcard-logon /sec:nla /kerberos:pkcs11-module:/usr/local/lib/libykcs11.so.2.5.2

cmake -GNinja -B freerdp-build-pkcs11-krb -S ./ -DCMAKE_BUILD_TYPE=Debug -DCMAKE_SKIP_INSTALL_ALL_DEPENDENCY=ON -DWITH_SERVER=OFF -DWITH_SAMPLE=OFF -DWITH_PLATFORM_SERVER=OFF -DUSE_UNWIND=OFF -DWITH_SWSCALE=OFF -DWITH_FFMPEG=OFF -DWITH_WEBVIEW=OFF -WITH_PKCS11=ON -WITH_KRB5=ON -WITH_KRB5_NO_NTLM_FALLBACK=ON -DCMAKE_INSTALL_PREFIX=/home/pavlo-myroniuk/apriorit/FreeRDP/freerdp-build-pkcs11-krb/debug
cmake --build freerdp-build-3
sudo cmake --install freerdp-build-3

https://developers.yubico.com/yubico-piv-tool/Actions/read_write_objects.html
reader: `Yubico YubiKey CCID 00 00`

yubico-piv-tool -v 1 -a read-object --id 0x5fc107

management key: c99070c70831160e12ea1dcd2e68ed58f940212114db7767
PIN code: 214653
PUK code: 896754

pkcs11-tool --init-token --label "tbt-test-token" --module "/usr/local/lib/libykcs11.so.2.5.2" --so-pin "010203040506070801020304050607080102030405060708" --pin "123456"
yubico-piv-tool -a import-key -a import-certificate -s 9c -k -i ~/Downloads/Telegram\ Desktop/t2/t2@tbt.com.pfx -K PKCS12
pkcs11-tool --module "/usr/local/lib/libykcs11.so.2.5.2" --show-info -v
pkcs11-tool --module "/usr/local/lib/libykcs11.so.2.5.2" -L -l -p 123456 -O
cat data | pkcs11-tool --module "/usr/local/lib/libykcs11.so.2.5.2" -m RSA-PKCS -p 123456 -s --id 02
cat delete_it | LD_LIBRARY_PATH=/usr/local/lib pkcs11-tool --module "/usr/local/lib/libykcs11.so.2.5.2" -m RSA-PKCS -p 123456 -s --id 02 | base64
LD_LIBRARY_PATH=/usr/local/lib 
gcc main.c -I/usr/include/pkcs11-helper-1.0 && LD_LIBRARY_PATH=/usr/local/lib ./a.out
LD_LIBRARY_PATH=/usr/local/lib pkcs11-tool --module "/usr/local/lib/libykcs11.so.2.5.2" --list-token-slots

/usr/local/lib/libwinpr3.so
LD_PRELOAD=/usr/lib/libpcsclite.so.1 LD_LIBRARY_PATH=/usr/local/lib xfreerdp /list:smartcard:pkcs11-module:/usr/local/lib/libykcs11.so.2.5.2
cat delete_it | pkcs11-tool --module "libykcs11.so.2.5.2" -m RSA-PKCS -p 123456 -s --id 02 | base64

LD_PRELOAD=/usr/lib/libpcsclite.so.1 xfreerdp /v:DESKTOP-8F33RFH.tbt.com /u:t2@tbt.com /p:123456 /cert:ignore /log-level:TRACE /smartcard-logon /sec:nla
LD_PRELOAD=/usr/lib/libpcsclite.so.1 xfreerdp /v:DESKTOP-8F33RFH.tbt.com /u:t2@tbt.com /p:123456 /cert:ignore /log-level:TRACE /smartcard-logon /sec:nla /kerberos:pkcs11-module:libykcs11.so.2.5.2 /auth-pkg-list:\!ntlm,kerberos

WITH_KRB5_MIT

export KRB5_TRACE=/dev/stdout
export KRB5_CONFIG=/home/pavlo-myroniuk/apriorit/reverse_freerdp_scard_auth/krb5.conf
export WLOG_LEVEL=trace

LD_PRELOAD=/usr/lib/libpcsclite.so.1 gdb --args xfreerdp /v:DESKTOP-8F33RFH.tbt.com /u:t2@tbt.com /p:123456 /cert:ignore /log-level:TRACE /smartcard-logon /sec:nla /kerberos:pkcs11-module:libykcs11.so.2.5.2 /auth-pkg-list:\!ntlm,kerberos /winscard-module:libsspi.so
