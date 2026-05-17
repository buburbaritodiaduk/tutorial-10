# Tutorial 1

# Penjelasan 1.2:
Mengapa tulisan "hey hey" muncul lebih dulu padahal dideklarasikan setelah blok spawner.spawn?
Hal ini terjadi karena Future di Rust bersifat lazy (tidak akan dieksekusi sampai secara aktif dijalankan). Saat program memanggil spawner.spawn(), program hanya mendaftarkan task asinkronus tersebut ke dalam antrean (channel) eksekutor tanpa langsung menjalankannya.  Karena belum dieksekusi, thread utama (main thread) terus berjalan ke baris berikutnya secara sinkronus dan langsung mencetak "hey hey". Task yang berisi "howdy!" dan "done!" baru benar-benar mulai dikerjakan ketika eksekutor mulai menarik data dari antrean melalui pemanggilan fungsi executor.run() di baris paling akhir.

# Penjelasan Eksperimen 1.3:

Efek Multiple Spawn: Walaupun ada tiga task asinkronus yang disubmit, waktu tunggu totalnya tidak menjadi 6 detik (2+2+2). Hal ini menunjukkan bahwa executor menjalankan task tersebut secara konkuren (bersamaan). Saat task 1 sedang "tidur/menunggu" (sleep 2 detik), CPU tidak menganggur, melainkan langsung mengeksekusi task 2 dan task 3.

Efek Menghapus drop(spawner): Program menjadi tidak bisa berhenti/berakhir dengan sendirinya. Hal ini terjadi karena executor diimplementasikan menggunakan sync_channel (sistem antrean pesan). Fungsi executor.run() akan terus memanggil ready_queue.recv() yang sifatnya blocking (menunggu data masuk). Saluran (channel) ini hanya akan ditutup jika semua sender (yaitu spawner) di-drop (dihancurkan). Karena drop(spawner) dihapus, executor mengira masih akan ada task baru yang dikirim, sehingga ia terus menunggu dan program tidak mau berhenti.

# Tutorial 2: Broadcast Chat

## Eksperimen 2.1: Original Code and How it Run

Berikut adalah hasil tangkapan layar dari interaksi antara 1 server dan 3 client yang saling terhubung secara asinkronus menggunakan WebSocket:

[Bukti Interaksi Server dan 3 Client]
![Screenshot 2026-05-16 at 22.37.00.png](img/Screenshot%202026-05-16%20at%2022.37.00.png)
![Screenshot 2026-05-16 at 22.38.10.png](img/Screenshot%202026-05-16%20at%2022.38.10.png)
![Screenshot 2026-05-16 at 22.39.03.png](img/Screenshot%202026-05-16%20at%2022.39.03.png)
![Screenshot 2026-05-16 at 22.39.25.png](img/Screenshot%202026-05-16%20at%2022.39.25.png)
        

### Penjelasan:
Ketika pesan diketikkan pada salah satu terminal client, pesan tersebut akan dikirim ke server. Server kemudian menyebarkan (broadcast) pesan tersebut secara real-time ke seluruh client lain yang terhubung tanpa memblokir jalannya program.

## Eksperimen 2.2: Modifying Port

Pada eksperimen ini, port komunikasi diubah dari `2000` menjadi `8080`.

**Penjelasan:**
Karena WebSocket adalah protokol komunikasi dua arah, perubahan port tidak bisa dilakukan di satu sisi saja. Saya harus mengubah port pada file `server.rs` (di bagian `TcpListener::bind`) agar server mendengarkan di port 8080, dan juga pada file `client.rs` (di bagian `Uri::from_static`) agar client menembak ke alamat dan port yang tepat. Kedua sisi juga dipastikan menggunakan protokol websocket yang sama, yang didefinisikan dengan skema `ws://` pada URI client.

## Eksperimen 2.3: Small changes, add IP and Port

Berikut adalah modifikasi di mana informasi pengirim (IP dan Port) serta nama pengguna ditampilkan pada setiap pesan yang diterima client:

[Screenshot Eksperimen 2.3]
![Screenshot 2026-05-16 at 22.45.53.png](img/Screenshot%202026-05-16%20at%2022.45.53.png)

**Penjelasan Modifikasi:**
Untuk mendapatkan hasil ini, server (`server.rs`) memformat pesan yang masuk dengan menggabungkan alamat pengirim (`addr`) dan teks asli sebelum melakukan broadcast (`bcast_tx.send(format!("{addr}: {text}"))`). Kemudian pada sisi client (`client.rs`), saya menambahkan string "Aryandana's Computer -" pada fungsi `println!` saat menerima pesan dari server, sehingga pesannya menjadi lebih informatif dan mudah diidentifikasi siapa pengirimnya.

## Tutorial 3: WebChat using Yew

### Eksperimen 3.1: Original Code
Berikut adalah hasil menjalankan aplikasi WebChat (Yew framework) di browser. Aplikasi berhasil dijalankan dan terkoneksi ke websocket server Rust dari tutorial sebelumnya:


![Screenshot 2026-05-17 at 13.13.55.png](img/Screenshot%202026-05-17%20at%2013.13.55.png)
![Screenshot 2026-05-17 at 13.14.20.png](img/Screenshot%202026-05-17%20at%2013.14.20.png)

**Penjelasan:**
Awalnya terjadi beberapa kendala kompabilitas library (versi wasm-bindgen usang dan error parser Webpack 5), serta branch repository default yang kosong. Setelah memperbarui `wasm-bindgen`, mengubah konfigurasi `webpack`, dan menggunakan branch `websockets-part2`, frontend sukses di-compile menjadi WebAssembly.

Aplikasi frontend berhasil terkoneksi ke Server WebSocket Rust yang berjalan di port 8080 dan sukses mengirimkan format JSON murni. Namun, pesan chat tidak otomatis dirender ke layar karena YewChat secara spesifik mengharapkan server untuk mengelola status "Users" dan mengirimkan balik daftar user aktif (`messageType: "users"`), sementara server Rust kita saat ini hanya berfungsi sebagai simple echo-broadcaster tanpa pengelolaan status state pengguna.

### Eksperimen 3.2: Add some creativities to the webclient
Berikut adalah hasil modifikasi UI pada aplikasi YewChat:

![Screenshot 2026-05-17 at 13.24.46.png](img/Screenshot%202026-05-17%20at%2013.24.46.png)
![Screenshot 2026-05-17 at 13.25.08.png](img/Screenshot%202026-05-17%20at%2013.25.08.png)

**Penjelasan:**
Pada tahap ini, saya menambahkan sentuhan kreativitas dengan memodifikasi komponen UI yang ada di dalam framework Yew. Saya mengubah file `login.rs` dan `chat.rs` pada direktori komponen untuk menyesuaikan warna tombol menggunakan class Tailwind CSS (contohnya mengubah dari warna indigo menjadi red/emerald) dan memodifikasi teks header agar terlihat lebih personal. Proses kompilasi WebAssembly otomatis menyesuaikan perubahan UI tersebut ke dalam browser.

