#import "template.typ": *

#show: ieee.with(
  title: [Perancangan Improvisasi Arsitektur #emph[Web Crawler] Berbasis #emph[Multi-Threading] dan #emph[Multi-Processing] Dengan Menggunakan Bahasa Pemrograman #emph[Rust]],
  abstract: [
    Mesin pencari atau #emph[search engine] merupkan #emph[software] yang digunakan untuk melakukan pencarian terhadap informasi tertentu. Untuk menjalankan proses pencarian diperlukan jumlah data yang banyak yang terkumpul dan dapat diakses dengan mudah, proses pengumpulan data ini lah yang disebut #emph[crawling]. Penelitian ini mencoba untuk memperbaiki kekurangan-kekurangan dari #emph[crawler] versi lazuardy dengan penekanan dalam efisiensi peforma dan penggunaan #emph[computing resource]. Penelitian ini menggunakan metode #emph[multi-threading] dan #emph[multi-processing] untuk membagi beban tugas kerja dari #emph[crawler] menjadi dua modul yaitu, #emph[scouter] dan #emph[parser], selain itu algoritma #emph[breadth-first search] yang digunakan dalam #emph[crawler] dimodifikasi untuk membatasi halaman web apa yang dapat di jelajahi oleh #emph[crawler]. Hasil akhir dari penelitian ini menunjukkan bahwa terdapat improvisasi dengan metode baru ini sebesar 17x dibandingkan dengan #emph[crawler] orisinil, dengan catatan penyeratan halaman terunduh antar #emph[domain] belum berhasil.
  ],
  authors: (
    (
      name: "Muhammad Daffa Haryadi Putra, Muhammad Eka Suryana, Med Irzal",
      department: [Program Studi Ilmu Komputer, Fakultas Matematika dan Ilmu Pengetahuan Alam],
      organization: [Universitas Negeri Jakarta],
      location: [Jakarta Timur, Indonesia],
      email: "daffahr15@protonmail.com, eka-suryana@unj.ac.id, medirzal@unj.ac.id"
    ),
  ),
  index-terms: ("Search Engine", "Web Crawler", "Rust Programming Language", "Multi-Thread", "Multi-Process"),
  bibliography: bibliography("refs.bib"),
)

= Pendahuluan
#emph[Search engine] merupakan sebuah program yang digunakan untuk menjelajahi dan mencari informasi dari web @seymour2011history. Terdapat beberapa komponen yang membangun arsitektur #emph[Search engine] seperti #emph[Web crawler], #emph[Page rank], dan #emph[indexer] @brin1998anatomy. Dalam proses pencarian web yang dilakukan #emph[Search engine] tahap pertama yang di lakukan adalah #emph[Web crawler] menjelajahi dan mengekstraksi data-data dari list #emph[url] lalu menyimpan data tersebut dan data lain yang terkait ke dalam database @brin1998anatomy. Data yang disimpan akan di-index, diberikan skor dan di urutkan melalui algoritma #emph[pagerank] @brin1998anatomy
#emph[Web Crawler] merupakan komponen penting dalam pembuatan arsitektur #emph[Search engine] secara keseluruhan. Penelitian sebelumnya yang telah dilakukan oleh #emph[Lazuardy Khatulisitwa] telah berhasil mengimplementasikan #emph[Web crawler] kedalam arsitektur #emph[Search engine] yang berjalan @lazuardithesis. #emph[Web crawler] tersebut mengimplentasikan algoritma #emph[Breadth First Search] dengan modifikasi algoritma #emph[similiarity based] untuk meningkatkan akurasi dari proses #emph[crawling] dan pengambilan data dari  suatu halaman @lazuardithesis. Algoritma #emph[Modified Similarity-Based] yang digunakan oleh #emph[Fathan] untuk memperbaiki akurasi dari #emph[Breadth First Search] memanfaatkan konsep penyimpanan #emph[queue] dalam melakukan proses #emph[crawling] @fathanthesis. Dalam proses tersebut #emph[crawler] akan menyimpan 2 jenis #emph[queue] yaitu, #emph[hot queue] untuk menyimpan #emph[url] yang mengandung kata #emph[anchor] sedangkan #emph[url queue] digunakan untuk menyimpan #emph[url] lain @cho1998efficient. Proses ini dapat membantu #emph[crawler] untuk mengunjungi dan melakukan #emph[crawling] ke dalam #emph[page] yang terdapat di #emph[hot queue] terlebih dahulu bila #emph[page] yang berkaitan dengan kata #emph[anchor] di kunjungi terlebih dahulu maka #emph[child page]-nya kemungkinan besar akan memiliki konten yang berkaitan dengan kata #emph[anchor] tersebut @cho1998efficient.

Arsitektur dari #emph[crawler] yang di kembangkan oleh Lazuardi, menggunakan #emph[python] sebagai bahasa pemograman dan #emph[library] pendukung yang digunakan adalah #emph[beautifulsoup4] untuk melakukan #emph[parsing] dari halaman #emph[website], #emph[request] untuk mengirimkan request kepada halaman #emph[website] yang ingin di ambil data-nya, dan #emph[regex] untuk melakukan pencocokan kata - kata yang telah di dapat dengan #emph[keyword] yang sudah di tentukan @lazuardithesis. Dari hasil penelitian #emph[lazuardi] terdapat beberapa saran peningkatan yang tercatat, dimana salah satunya terkait dengan meningkatkan kinerja dan peforma dari #emph[web crawler] agar memiliki penggunaan #emph[RAM] yang lebih kecil dan mencapai kinerja yang maksimal @lazuardithesis.

Salah satu metode untuk mempercepat jalannya #emph[search engine] adalah #emph[Multi-threading] @multithreadedtextsearch. Metode ini sudah pernah digunakan dalam #emph[search engine] sebelumnya, tetapi #emph[search engine] ini mencari data bukan ke #emph[web] tetapi pada kumpulan data teks atau dapat disebut dengan nama #emph[text search] @multithreadedtextsearch. Dari hasil penelitian tersebut ditemukan metode #emph[multi-threading] yang digunakan berhasil mencapai improvisasi yang sebelumnya membutuhkan waktu 16 menit dalam menjelajahi seluruh data teks menjadi 4 menit, yang berarti berhasil mencapai improviasi waktu eksekusi program sebesar 4x @multithreadedtextsearch. Dalam penelitian tersebut metode #emph[multi-threading] digunakan untuk memecah proses pengambilan data dari sumber data dan proses parsing dari data teks yang sudah di ambil @multithreadedtextsearch.

Dalam konteks #emph[search engine] untuk pencarian web penelitian yang dilakukan oleh #emph[Pramudita, Y.D et all] telah menunjukkan bahwa mekanisme #emph[multi-threading] dapat diimplementasi dengan benar @Pramudita_2020. Dalam penelitian tersebut tiap-tiap #emph[thread] menjalankan satu #emph[instance] dari #emph[crawler] nya itu sendiri, dan penelitian tersebut berhasil mencapai percepatan waktu #emph[crawling] selama 123 detik @Pramudita_2020.

Selanjutnya penelitian hanya akan melakukan improvisasi terhadap komponen #emph[web crawler] saja untuk membatasi area penelitian. Penelitian ini akan berusaha untuk meningkatkan performa, yang dimana merupakan jumlah halaman yang terkumpul pada waktu yang sudah definisikan. Berdasarkan hasil penelitian #emph[Pramudita, Y.D et all], yang dimana menjalankan keseluruhan proses #emph[crawler] dalam satu thread @Pramudita_2020, maka penelitian ini akan berusaha untuk meningkatkan performa dengan memisahkan proses #emph[parsing] dalam #emph[crawler] dalam proses yang berbeda atau yang dapat disebut dengan metode #emph[multi-processing]. Selain itu penelitian ini juga akan berusaha untuk meningkatkan akurasi hasil proses #emph[crawling] dengan menggunakan algoritma #emph[breadht-first search] yang dimodifikasi dengan tujuan agar #emph[crawler] hanya menjelajahi domain yang telah ditentukan saja, sehingga diharapkan hasil proses #emph[crawling] hanya akan berisi halaman web yang diinginkan. Perbaikan lain yang akan dilakukan adalah dengan menggunakan bahasa pemograman dengan waktu eksekusi yang lebih cepat, yaitu #emph[rust] @RustPerformance. Keputusan ini didasari dari hasil pengujian bahasa pemograman #emph[rust] dalam proses dengan intensitas tinggi dan konteks #emph[low-level] @RustPerformance.

= Kajian Pustaka
#lorem(90)

$ a + b = gamma $

#lorem(200)

= Desain Model

#lorem(200)

= Hasil dan Pembahasan

#lorem(200)
