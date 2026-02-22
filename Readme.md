# DEPLOY
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


