# DEPLOY
---
* **Noted:** gunakan versi cargo stylus cli 0.6.3
* **cara cek:** 
```bash 
cargo stylus --version
```
* **cara upgrade biar g gagal:**
```bash
cargo install cargo-stylus --version 0.6.3 --locked --force
```
---
## deploy dengan .env
-- deploy menggunakan env
```bash
cargo stylus deploy \
  --private-key-path=.env \
  --endpoint=https://sepolia-rollup.arbitrum.io/rpc
```
---
## jika deploy dengan .env tidak berhasil maka dengan txt
-- lakukan pengecekan endpoint rpc
```bash
cargo stylus check --rpc-url https://sepolia-rollup.arbitrum.io/rpc
```
---
-- cara deploy jika tidak berhasil menggunakan .env buat file pk.txt tapi buat abaikan di .gitignore jangan sampai ke upload

```bash
cargo stylus deploy \
  --private-key-path=pk.txt \
  --endpoint=https://sepolia-rollup.arbitrum.io/rpc
```
---
## jika di suruh Arbos Stylus Chace. apa Arbos stylus Klik di sini untuk membaca [arbOS Stylus](readmearbosstyluschacing.md)
```bash
cargo stylus cache bid 6005b705a7d459f58477231ff704adedc36ac479 0 \
  --endpoint=https://sepolia-rollup.arbitrum.io/rpc \
  --private-key-path=pk.txt
```
---

## contract 
* **mycontract :** 0x6005b705a7d459f58477231ff704adedc36ac479
[scan contract:](https://sepolia.arbiscan.io/address/0x6005b705a7d459f58477231ff704adedc36ac479#)


## Verifikasi Deploy
-- * **upload on repositori github**

```bash
MY_API_KEY=$(cat api_key.txt)
CONTRACT="0x6005b705a7d459f58477231ff704adedc36ac479"
CHAIN_ID="421614" # Arbitrum Sepolia
BASE_URL="https://api.etherscan.io/v2/api"

curl -X POST "https://api.etherscan.io/v2/$MY_API_KEY" \
  -d "chainid=421614" \
  -d "module=contract" \
  -d "action=verifysourcecode" \
  -d "apikey=$API_KEY" \
  -d "codeformat=stylus" \
  -d "sourceCode=$GITHUB_REPO" \
  -d "contractaddress=$CONTRACT" \
  -d "contractname=my_counter" \
  -d "compilerversion=stylus:0.6.3" \
  -d "licenseType=3"
```
