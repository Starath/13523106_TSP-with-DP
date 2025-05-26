# 13523106_TSP-with-DP
#### Tugas Kecil^2 dari Bapak Dr. Ir. Rinaldi Munir, M.T.

## Cara Kerja ⚙️

Traveling Salesman Problem (TSP) adalah masalah optimasi klasik di mana seorang salesman harus mengunjungi sejumlah kota (masing-masing tepat satu kali) dan kembali ke kota awal dengan total jarak tempuh seminimal mungkin.

Algoritma yang digunakan di sini adalah **algoritma Held-Karp**, sebuah algoritma berbasis pemrograman dinamis. Cara kerjanya adalah sebagai berikut:

1.  **Representasi State**:
    State dalam tabel DP didefinisikan sebagai `dp[mask][i]`.
    * `mask`: Sebuah *bitmask* (integer) yang merepresentasikan himpunan kota-kota yang telah dikunjungi. Jika bit ke-`k` pada `mask` diset (bernilai 1), maka kota `k` telah dikunjungi.
    * `i`: Kota terakhir yang dikunjungi dalam himpunan `mask`.
    * `dp[mask][i]`: Menyimpan biaya (jarak) minimum untuk mengunjungi semua kota dalam himpunan `mask`, dimulai dari `start_node` yang ditentukan, dan berakhir di kota `i`.

2.  **Kasus Dasar (Base Case)**:
    Jalur yang hanya mengunjungi kota awal (`start_node`) dan berakhir di `start_node` memiliki biaya 0.
    `dp[1 << start_node][start_node] = 0`
    (Di sini, `1 << start_node` adalah mask dengan hanya bit `start_node` yang diset).

3.  **Transisi (Recursive Relation)**:
    Untuk menghitung `dp[mask][i]`, kita mempertimbangkan semua kemungkinan kota `j` yang dikunjungi tepat sebelum kota `i`. Kota `j` harus ada dalam `mask` dan `j != i`.
    `dp[mask][i] = min( dp[mask_sebelumnya][j] + biaya[j][i] )`
    di mana:
    * `mask_sebelumnya` adalah `mask` tanpa kota `i` (yaitu, `mask ^ (1 << i)`).
    * `biaya[j][i]` adalah biaya perjalanan langsung dari kota `j` ke kota `i`.
    Iterasi dilakukan untuk semua `j` yang valid dalam `mask_sebelumnya`.

4.  **Solusi Akhir**:
    Setelah semua state `dp` dihitung, solusi akhir TSP adalah jalur minimum yang mengunjungi *semua* kota dan kembali ke `start_node`.
    Biaya total = `min( dp[(1 << N) - 1][i] + biaya[i][start_node] )`
    di mana:
    * `(1 << N) - 1` adalah mask di mana semua `N` bit diset (semua kota dikunjungi).
    * `i` adalah kota terakhir yang dikunjungi sebelum kembali ke `start_node` (`i != start_node`).
    * `biaya[i][start_node]` adalah biaya untuk kembali dari kota `i` ke `start_node`.

5.  **Rekonstruksi Jalur**:
    Untuk mendapatkan urutan kota yang sebenarnya, kita perlu menyimpan "parent" atau kota pendahulu (`parent[mask][i] = j`) yang menghasilkan biaya minimum untuk state `dp[mask][i]`. Jalur direkonstruksi secara mundur dari state akhir.

### Kompleksitas

* **Kompleksitas Waktu**: `O(2^N * N^2)`
    * Ada `2^N` kemungkinan subset (mask).
    * Untuk setiap mask, ada `N` kemungkinan kota akhir `i`.
    * Untuk setiap `(mask, i)`, ada `N` kemungkinan kota sebelumnya `j`.
* **Kompleksitas Ruang**: `O(2^N * N)`
    * Untuk menyimpan tabel `dp` dan `parent`.

Karena kompleksitas eksponensialnya, algoritma ini praktis untuk jumlah kota (`N`) yang relatif kecil (misalnya, hingga N sekitar 20-25). Untuk `N` yang lebih besar, algoritma heuristik atau aproksimasi biasanya digunakan.

## Penggunaan Program 🚀

### Prasyarat

Pastikan Anda telah menginstal Rust di sistem Anda. Anda bisa menginstalnya melalui [rustup](https://rustup.rs/).

### Kompilasi dan Menjalankan Program

1.  **Simpan Kode**:
    Simpan kode di atas sebagai `src/main.rs` dalam sebuah proyek Cargo baru.
    Jika belum ada proyek, buat dengan:
    ```bash
    cargo new tsp_rust_dp
    cd tsp_rust_dp
    ```
    Kemudian letakkan kode di `tsp_rust_dp/src/main.rs`.

2.  **Kompilasi**:
    ```bash
    cargo build
    ```
    Atau untuk kompilasi rilis (lebih optimal):
    ```bash
    cargo build --release
    ```

3.  **Menjalankan**:
    ```bash
    cargo run
    ```
    Atau jika menggunakan versi rilis:
    ```bash
    target/release/tsp_rust_dp
    ```

### Input Graf

Graf didefinisikan dalam fungsi `main()` di `src/main.rs` sebagai sebuah matriks adjacency (jarak antar kota):

```rust
let example_graph = vec![
    vec![0, 10, 15, 20], // Biaya dari kota 0 ke 0, 1, 2, 3
    vec![10, 0, 35, 25], // Biaya dari kota 1 ke 0, 1, 2, 3
    vec![15, 35, 0, 30], // Biaya dari kota 2 ke 0, 1, 2, 3
    vec![20, 25, 30, 0], // Biaya dari kota 3 ke 0, 1, 2, 3
];
let start_node_example = 0; // Memulai dari kota 0