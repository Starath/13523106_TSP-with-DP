# 13523106_TSP-with-DP
#### Tugas Kecil^2 dari Bapak Dr. Ir. Rinaldi Munir, M.T.

## Cara Kerja ⚙️
1.  **Representasi State**:
    State dalam tabel DP didefinisikan sebagai `dp[mask][i]`.
    * `dp[mask][i]`: Menyimpan biaya (jarak) minimum untuk mengunjungi semua kota dalam himpunan `mask`, dimulai dari `start_node` yang ditentukan.

2.  **Kasus Dasar**:
    Jalur yang hanya mengunjungi kota awal (`start_node`) dan berakhir di `start_node` memiliki biaya 0.
    `dp[1 << start_node][start_node] = 0`
    (Di sini, `1 << start_node` adalah mask dengan hanya bit `start_node` yang diset).

3.  **Transisi**:
    Untuk menghitung `dp[mask][i]`, kita mempertimbangkan semua kemungkinan kota `j` yang dikunjungi tepat sebelum kota `i`. Kota `j` harus ada dalam `mask` dan `j != i`.
    `dp[mask][i] = min( dp[mask_sebelumnya][j] + biaya[j][i] )`
    di mana:
    * `mask_sebelumnya` adalah `mask` tanpa kota `i` (yaitu, `mask ^ (1 << i)`).
    * `biaya[j][i]` adalah biaya perjalanan langsung dari kota `j` ke kota `i`.
    Iterasi dilakukan untuk semua `j` yang valid dalam `mask_sebelumnya`.

4.  **Solusi Akhir**:
    Setelah semua state `dp` dihitung, solusi akhir TSP adalah jalur minimum yang mengunjungi semua kota dan kembali ke `start_node`.
    Biaya total = `min( dp[(1 << N) - 1][i] + biaya[i][start_node] )`
    di mana:
    * `(1 << N) - 1` adalah mask di mana semua `N` bit diset (semua kota dikunjungi).
    * `i` adalah kota terakhir yang dikunjungi sebelum kembali ke `start_node` (`i != start_node`).
    * `biaya[i][start_node]` adalah biaya untuk kembali dari kota `i` ke `start_node`.

5.  **Rekonstruksi Jalur**:
    Untuk mendapatkan urutan kota yang sebenarnya, kita perlu menyimpan "parent" atau kota pendahulu (`parent[mask][i] = j`) yang menghasilkan biaya minimum untuk state `dp[mask][i]`. Jalur direkonstruksi secara _reverse_ dari state akhir.

### Kompleksitas

* **Kompleksitas Waktu**: `O(2^N * N^2)`
    * Ada `2^N` kemungkinan subset (mask).
    * Untuk setiap mask, ada `N` kemungkinan kota akhir `i`.
    * Untuk setiap `(mask, i)`, ada `N` kemungkinan kota sebelumnya `j`.
* **Kompleksitas Ruang**: `O(2^N * N)`
    * Untuk menyimpan tabel `dp` dan `parent`.

Karena kompleksitasnya yang eksponensial, algoritma ini praktis untuk jumlah kota (`N`) yang relatif kecil.

### Prasyarat

Rust: [rustup](https://rustup.rs/).

### Kompilasi dan Menjalankan Program
1.  **Kompilasi**:
    ```bash
    cargo build
    ```

2.  **Menjalankan**:
    ```bash
    cd tsp_rust_dp
    ```
    ```bash
    cargo run
    ```

### Input Graf

Graf didefinisikan dalam fungsi `main()` di `tsp_rust_dp/src/main.rs` sebagai sebuah matriks adjacency