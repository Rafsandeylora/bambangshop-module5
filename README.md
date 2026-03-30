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