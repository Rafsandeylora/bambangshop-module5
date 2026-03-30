# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

## Reflection Publisher-1

### 1. Pada diagram Observer Pattern yang dijelaskan di buku *Head First Design Patterns*, Subscriber didefinisikan sebagai interface. Berdasarkan pemahamanmu tentang Observer pattern, apakah pada kasus BambangShop ini kita masih memerlukan interface (atau trait di Rust), atau cukup satu Model struct saja?

Menurut saya, untuk kasus BambangShop saat ini, satu struct `Subscriber` saja sudah cukup.  
Alasannya adalah karena pada tutorial ini semua subscriber masih memiliki tanggung jawab yang sama, yaitu menyimpan data subscriber seperti URL/endpoint dan menerima update dari publisher dengan cara yang seragam. Dengan kata lain, saat ini kita baru punya satu bentuk perilaku subscriber yang konkret.

Namun, jika dilihat dari sudut pandang design pattern, penggunaan interface atau trait tetap merupakan desain yang lebih extensible. Dalam Observer pattern, publisher idealnya bergantung pada abstraksi, bukan implementasi konkret, sehingga jika nanti ada jenis subscriber baru, publisher tidak perlu diubah. Misalnya, di masa depan BambangShop ingin mendukung subscriber berbasis HTTP, logging, message queue, atau in-memory subscriber, maka trait akan menjadi pilihan yang lebih tepat.

Jadi, untuk kebutuhan tutorial saat ini, satu struct sudah cukup dan lebih sederhana. Tetapi untuk desain jangka panjang yang lebih fleksibel dan lebih sesuai dengan bentuk klasik Observer pattern, penggunaan trait akan lebih baik.

### 2. `id` pada Program dan `url` pada Subscriber dimaksudkan untuk bersifat unik. Berdasarkan pemahamanmu, apakah penggunaan `Vec` (list) sudah cukup atau penggunaan `DashMap` (map/dictionary) seperti yang dipakai sekarang memang diperlukan?

`Vec` sebenarnya cukup untuk implementasi yang kecil dan sederhana, karena kita masih bisa menyimpan semua subscriber dalam list lalu melakukan pengecekan manual agar tidak ada `id` atau `url` yang duplikat sebelum data baru dimasukkan. Untuk skala tutorial, pendekatan ini masih bisa bekerja dengan benar.

Namun, `DashMap` lebih cocok untuk kasus ini karena keunikan data merupakan kebutuhan inti. Struktur map secara alami memang dirancang untuk merepresentasikan key yang unik, sehingga proses pengecekan, penambahan, pembaruan, dan penghapusan data menjadi lebih langsung dan lebih efisien dibandingkan `Vec`. Jika memakai `Vec`, operasi seperti mencari subscriber tertentu atau menghapus data akan membutuhkan iterasi terhadap seluruh isi list, dan hal ini kurang efisien ketika jumlah subscriber bertambah banyak.

Selain itu, project ini berjalan dalam konteks web app dengan shared state yang bisa diakses oleh beberapa request. `DashMap` dirancang untuk akses konkuren yang aman, sehingga lebih sesuai dibandingkan `Vec` biasa. Jadi, walaupun `Vec` memungkinkan, `DashMap` adalah pilihan yang lebih baik dari sisi keunikan data, efisiensi, dan thread safety.

### 3. Saat menggunakan Rust, kita dipaksa oleh compiler constraint yang ketat untuk membuat program yang thread-safe. Dalam kasus static variable `SUBSCRIBERS`, kita menggunakan library eksternal `DashMap` untuk membuat `HashMap` yang thread-safe. Berdasarkan pemahamanmu tentang design pattern, apakah kita masih memerlukan `DashMap`, atau kita bisa menggantinya dengan Singleton pattern?

Menurut saya, `DashMap` dan Singleton pattern menyelesaikan dua masalah yang berbeda, jadi Singleton tidak bisa langsung menggantikan `DashMap`.

Singleton pattern bertujuan untuk memastikan bahwa hanya ada satu instance objek yang digunakan secara bersama di seluruh aplikasi. Ini berguna jika kita ingin mempunyai satu titik akses global terhadap shared data. Namun, Singleton sendiri tidak otomatis membuat data di dalamnya menjadi thread-safe. Jika ada banyak thread yang mengakses dan mengubah state yang sama, kita tetap membutuhkan mekanisme sinkronisasi atau struktur data yang memang aman untuk concurrency.

Di sisi lain, `DashMap` memang secara khusus menyelesaikan masalah concurrency dengan menyediakan implementasi map yang thread-safe. Dengan begitu, data dapat diakses dan dimodifikasi bersama dengan cara yang lebih aman dan lebih praktis dalam program konkuren.

Karena itu, kalaupun kita ingin menerapkan Singleton untuk repository, data di dalam Singleton tersebut tetap sebaiknya menggunakan struktur data yang thread-safe seperti `DashMap` (atau alternatif lain seperti `Mutex<HashMap<...>>` atau `RwLock<HashMap<...>>`). Jadi kesimpulannya: Singleton bisa membantu memastikan hanya ada satu instance repository, tetapi Singleton tidak menghilangkan kebutuhan terhadap `DashMap` atau struktur data thread-safe lainnya.

## Reflection Publisher-2

### 1. Dalam compound pattern Model-View-Controller (MVC), tidak ada “Service” dan “Repository”. Model pada MVC mencakup penyimpanan data dan business logic. Berdasarkan pemahamanmu tentang design principles, mengapa kita perlu memisahkan “Service” dan “Repository” dari Model?

Menurut saya, pemisahan `Service` dan `Repository` dari `Model` diperlukan agar setiap komponen memiliki tanggung jawab yang lebih jelas dan spesifik. Jika semua hal dimasukkan ke dalam `Model`, maka satu class atau struct akan menangani terlalu banyak urusan sekaligus, seperti representasi data, validasi, business logic, dan akses data. Hal ini bertentangan dengan **Single Responsibility Principle**, karena satu komponen seharusnya hanya punya satu alasan untuk berubah.

`Model` sebaiknya fokus pada representasi data dan aturan dasar yang melekat pada data tersebut.  
`Repository` bertugas mengatur penyimpanan, pengambilan, penambahan, dan penghapusan data.  
`Service` bertugas mengatur alur business logic dan koordinasi antar model/repository.

Dengan pemisahan ini, kode menjadi lebih mudah dipahami, diuji, dan dikembangkan. Jika suatu saat cara penyimpanan data berubah, kita cukup mengubah `Repository` tanpa harus mengubah `Model` atau `Controller`. Jika business logic berubah, kita cukup menyesuaikan `Service`. Jadi, pemisahan ini membantu menjaga modularitas, maintainability, dan testability dari aplikasi.

### 2. Apa yang terjadi jika kita hanya menggunakan Model? Jelaskan bayanganmu tentang bagaimana interaksi antar model (Program, Subscriber, Notification) memengaruhi kompleksitas kode pada masing-masing model.

Kalau kita hanya menggunakan `Model`, menurut saya kompleksitas kode akan meningkat cukup signifikan. Setiap model tidak hanya menyimpan data, tetapi juga harus menangani logika proses dan interaksi dengan model lain. Akibatnya, tiap model akan menjadi terlalu “gemuk” dan sulit dipelihara.

Sebagai contoh:
- `Program` tidak hanya perlu menyimpan data program, tetapi juga harus tahu bagaimana cara mencari subscriber yang relevan, membuat notification, dan memicu pengiriman notifikasi.
- `Subscriber` tidak hanya menyimpan data subscriber, tetapi mungkin juga harus tahu bagaimana cara subscribe/unsubscribe dirinya ke daftar tertentu.
- `Notification` tidak hanya merepresentasikan notifikasi, tetapi bisa ikut menanggung proses pembuatan format pesan atau alur pengirimannya.

Kalau semua tanggung jawab ini ditaruh di model, maka akan terjadi coupling yang tinggi antar model. `Program` akan terlalu bergantung pada `Subscriber` dan `Notification`, begitu juga sebaliknya. Akibatnya:
1. kode menjadi lebih panjang dan kompleks,
2. sulit mengetahui batas tanggung jawab tiap model,
3. perubahan kecil pada satu model bisa memengaruhi model lain,
4. pengujian menjadi lebih sulit karena setiap model saling terkait erat.

Jadi, kalau hanya menggunakan `Model`, interaksi antar `Program`, `Subscriber`, dan `Notification` akan membuat masing-masing model menjadi lebih rumit, lebih sulit diuji, dan lebih sulit dikembangkan di masa depan.

### 3. Apakah kamu sudah mengeksplor lebih jauh tentang Postman? Ceritakan bagaimana tool ini membantu kamu menguji pekerjaanmu saat ini. Kamu juga boleh menyebutkan fitur Postman apa yang menurutmu menarik atau membantu untuk Group Project atau project software engineering lain di masa depan.

Ya, saya sudah mulai mengeksplor Postman, dan menurut saya tool ini sangat membantu untuk menguji API dengan cepat tanpa harus membuat frontend terlebih dahulu. Dalam pekerjaan saat ini, Postman membantu saya mengirim HTTP request ke endpoint yang saya buat, seperti endpoint untuk subscribe dan unsubscribe notification. Dengan Postman, saya bisa langsung mengecek apakah request berhasil, bagaimana response yang dikembalikan, dan apakah format JSON yang saya kirim sudah sesuai.

Postman membantu saya dalam beberapa hal:
- menguji endpoint secara manual dengan cepat,
- melihat status response seperti `200 OK`, `201 Created`, atau `404 Not Found`,
- mencoba berbagai payload JSON tanpa perlu mengubah kode aplikasi,
- mempermudah debugging karena saya bisa membandingkan request dan response secara langsung.

Fitur Postman yang menurut saya menarik dan berguna:
1. **Collections**  
   Sangat membantu untuk menyimpan sekumpulan request agar pengujian bisa dilakukan berulang kali dengan lebih rapi.
2. **Environment Variables**  
   Memudahkan jika nanti project memiliki banyak base URL atau konfigurasi yang berbeda antara local, staging, dan production.
3. **Request History**  
   Berguna untuk melacak request yang pernah diuji sebelumnya.
4. **Automated Testing / Scripts**  
   Menarik untuk dipelajari lebih lanjut karena bisa membantu membuat pengujian API yang lebih konsisten.
5. **Sharing Collection**  
   Sangat berguna untuk Group Project karena anggota tim bisa memakai request yang sama tanpa harus membuat ulang satu per satu.

Menurut saya, Postman adalah tool yang sangat praktis untuk software engineering project karena mempercepat proses testing, debugging, dan kolaborasi dalam pengembangan API.

## Reflection Publisher-3

### 1. Observer Pattern memiliki dua variasi: Push model (publisher mendorong data ke subscriber) dan Pull model (subscriber mengambil data dari publisher). Pada kasus tutorial ini, variasi mana yang kita gunakan?

Pada tutorial ini, kita menggunakan **Push model**.

Menurut saya, hal ini terlihat dari cara kerja sistem notifikasi yang sudah kita buat. Saat ada event pada product, misalnya product dibuat, dipromosikan, atau dihapus, pihak publisher langsung menyiapkan payload notifikasi lalu mengirimkannya ke subscriber. Jadi subscriber tidak perlu meminta data sendiri ke publisher. Publisher yang secara aktif “mendorong” data ke subscriber.

Hal ini juga sesuai dengan penjelasan di modul bahwa pada **Push model**, publisher bertanggung jawab membagikan data ke semua observer/subscriber yang terdaftar, biasanya dengan memanggil method `update()`. Pada kasus lintas aplikasi, subscriber bahkan bisa direpresentasikan sebagai objek yang menyimpan URL, lalu `update()` akan mengirim request HTTP ke client app lain. Itu persis seperti yang terjadi di tutorial ini.

### 2. Apa kelebihan dan kekurangan jika kita menggunakan variasi Observer Pattern yang lain? (Misalnya jika pada nomor 1 kita menjawab Push, maka bayangkan jika kita memakai Pull)

Kalau pada tutorial ini kita memakai **Pull model**, maka ada beberapa kemungkinan kelebihan dan kekurangannya.

**Kelebihan Pull model:**
1. Subscriber memiliki kontrol lebih besar terhadap kapan data diambil.  
   Jadi subscriber bisa menentukan sendiri kapan ingin meminta update dari publisher.
2. Payload dari publisher bisa lebih sederhana.  
   Publisher tidak perlu langsung mengirim seluruh detail notifikasi, karena subscriber bisa meminta data tambahan jika diperlukan.
3. Bisa lebih fleksibel jika kebutuhan tiap subscriber berbeda.  
   Misalnya ada subscriber yang hanya butuh sebagian data, dan ada yang butuh data lengkap.

**Kekurangan Pull model:**
1. Subscriber menjadi lebih aktif dan lebih kompleks.  
   Subscriber harus tahu bagaimana cara menghubungi publisher dan bagaimana cara mengambil data yang diperlukan.
2. Tidak cocok untuk kebutuhan notifikasi real-time seperti tutorial ini.  
   Kalau subscriber hanya menarik data secara berkala, maka update bisa terlambat diterima.
3. Publisher akan terekspos ke subscriber.  
   Modul juga menjelaskan bahwa pada Pull model, observer akan terekspos ke subject/publisher karena observer harus aktif meminta data.
4. Akan menambah jumlah request.  
   Jika banyak subscriber terus-menerus melakukan polling atau pengecekan ke publisher, maka trafik jaringan bisa bertambah dan sistem menjadi kurang efisien.

Jadi, menurut saya, untuk kasus tutorial BambangShop yang ingin mengirim notifikasi segera ketika ada perubahan product, **Push model lebih cocok** dibanding Pull model.

### 3. Jelaskan apa yang akan terjadi pada program jika kita memutuskan untuk tidak menggunakan multi-threading dalam proses notifikasi

Kalau kita tidak menggunakan multi-threading dalam proses notifikasi, maka pengiriman notifikasi ke subscriber akan berjalan secara **sekuensial** atau satu per satu. Artinya, publisher harus menunggu subscriber pertama selesai diproses dulu, baru lanjut ke subscriber kedua, dan seterusnya.

Dampaknya menurut saya adalah:
1. **Response program menjadi lebih lambat**  
   Jika jumlah subscriber banyak, atau ada subscriber yang lambat merespons, maka proses create, publish, atau delete product juga akan ikut melambat karena harus menunggu semua notifikasi selesai dikirim.
2. **Satu subscriber yang lambat bisa menghambat subscriber lain**  
   Misalnya ada satu endpoint subscriber yang lambat atau bahkan bermasalah, maka seluruh alur notifikasi akan tertahan.
3. **User experience menjadi lebih buruk**  
   Dari sisi pengguna atau client, request ke server bisa terasa lama padahal inti proses bisnisnya sebenarnya sudah selesai lebih cepat.
4. **Scalability sistem menurun**  
   Semakin banyak subscriber, semakin besar dampak perlambatan jika semua notifikasi diproses satu per satu.
5. **Risiko blocking lebih tinggi**  
   Karena proses network request ke subscriber adalah operasi yang bisa memakan waktu, menjalankannya tanpa multi-threading membuat alur utama aplikasi lebih mudah terblokir.

Karena itu, penggunaan multi-threading pada proses notifikasi di tutorial ini menurut saya sangat membantu agar pengiriman notifikasi ke banyak subscriber bisa berjalan lebih cepat, lebih efisien, dan tidak terlalu menghambat request utama pada publisher.