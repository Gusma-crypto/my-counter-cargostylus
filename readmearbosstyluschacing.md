
# 🤖 Gemini Berkata: Mengatasi Koneksi RPC pada Caching Stylus
### 💡 Analogi Sederhana
---

### 🧠 Apa itu "Caching" di Stylus?

*(Penting untuk pengetahuan Web3 kamu)*

Di Arbitrum Stylus, terdapat fitur unik yang disebut **ArbOS Stylus Cache**. Ini adalah cara jaringan mengoptimalkan kontrak pintar agar berjalan secepat mungkin.

* **Tanpa Cache:** Setiap kali seseorang memanggil kontrakmu, jaringan harus memuat kode WASM tersebut dari penyimpanan permanen (disk), yang memakan waktu sedikit lebih lama dan biaya gas yang lebih mahal.
* **Dengan Cache:** Kontrakmu "didaftarkan" ke dalam **memori cepat** (seperti RAM di komputer). Ini membuat eksekusi fungsi menjadi instan.
* **Bid 0:** Angka `0` di akhir perintah berarti kamu melakukan penawaran (*bid*) harga untuk masuk ke cache. Biasanya, di jaringan *testnet*, bid `0` sudah cukup untuk memasukkan kontrakmu ke dalam daftar prioritas cache.

---
