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

![Bukti Interaksi Server dan 3 Client]
![Screenshot 2026-05-16 at 22.37.00.png](img/Screenshot%202026-05-16%20at%2022.37.00.png)
![Screenshot 2026-05-16 at 22.38.10.png](img/Screenshot%202026-05-16%20at%2022.38.10.png)
![Screenshot 2026-05-16 at 22.39.03.png](img/Screenshot%202026-05-16%20at%2022.39.03.png)
![Screenshot 2026-05-16 at 22.39.25.png](img/Screenshot%202026-05-16%20at%2022.39.25.png)
        

### Penjelasan:
Ketika pesan diketikkan pada salah satu terminal client, pesan tersebut akan dikirim ke server. Server kemudian menyebarkan (broadcast) pesan tersebut secara real-time ke seluruh client lain yang terhubung tanpa memblokir jalannya program.


