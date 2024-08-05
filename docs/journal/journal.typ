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

== Definisi #emph[Search Engine]

Mesin Pencari atau #emph[Search Engine] merupakan software yang digunakan untuk pencarian terhadap banyak situs web di internet berdasarkan input kata yang ditanyakan. #emph[Search Engine] memungkinkan pengguna untuk mencari situs web yang berkaitan dengan kata kunci ataupun pertanyaan yang diajukan oleh pengguna @seymour2011history. Dalam penggunaannya, search engine hanyalah sebuah halaman situs website yang dapat diakses oleh pengguna yang perannya adalah mengumpulkan dan menampilkan hasil pencarian tersebut kepada user dengan tampilan yang menarik dan informatif @seymour2011history. 


== Arsitektur #emph[Search Engine]

Secara sederhana #emph[Search Engine] bekerja dengan menyimpan dan melakukan pengindeksan informasi-informasi dari situs web dan menyajikannya dalam bentuk yang dapat di mengerti oleh pengguna. Informasi dari halaman situs web didapatkan menggunakan program bernama #emph[Web crawler] yang mengunduh dan menyimpan informasi dari halaman situs web kedalam #emph[Database]. Setelah di simpan, infomasi akan dianalisis dan dipilih oleh program #emph[Indexer] @lazuardithesis.

Proses #emph[crawling] dalam arsitektur #emph[Search engine] milik lazuard memiliki beberapa tahap, Tahap awal adalah #emph[crawler] akan mengakses #emph[origin url] yang disediakan dalam #emph[environment variable]. Untuk melakukan proses #emph[crawling], perlu untuk menginisiasikan beberapa data yang akan digunakan dalam proses crawling seperti #emph[origin url] yang akan di akses, maksimum #emph[os threads] yang akan digunakan oleh crawler, dan durasi proses #emph[crawling]. Proses pertama yang dilakukan oleh #emph[crawler] setelah di inisiasi adalah dengan melakukan pengecekan ke #emph[database] apakah terdapat page yang sudah di #emph[crawl] atau belum, bila sudah maka crawler akan memulai proses #emph[crawling] dari page terakhir yang sebelumnya telah di #emph[parse]. Bila tidak, maka proses #emph[crawling] akan dimulai dari #emph[origin url]. List dari origin url akan dimasukkan kedalam #emph[queue] yang nantinya akan digunakan oleh proses #emph[Breadth First Search] dalam #emph[crawler]. Sebelum proses parse dilakukan data-data yang berhubungan dengan #emph[page] yang akan di #emph[parse] akan di insert kedalam database, seperti #emph[string url] dan duration #emph[parse]. Proses parsing page dilakukan menggunakan algoritma #emph[Breadth First Search]. 

Dalam menjalankan #emph[Breadth First Search], setiap #emph[instance] #emph[page scrapper] dijalankan secara paralel didalam #emph[thread process]. #emph[Page scrapper] akan melakukan parsing tiap #emph[page] yang diakses dan mengambil beberapa bagian data dari #emph[page] tersebut. Data yang parse dari #emph[page] merupakan data penting yang berisi inti sari dari #emph[page] tersebut dan data lain yang akan mendukung proses #emph[crawling] dan proses-proses selanjutnya dalam arsitektur #emph[search engine], beberapa data yang diambil oleh #emph[scrapper] adalah #emph[article body] dari #emph[html page], #emph[meta description] dari #emph[page], #emph[meta keyword], #emph[css page style] dari page, #emph[script] yang di #emph[embedded] dalam page, #emph[list], #emph[form], #emph[table], #emph[image] dalam #emph[page] dan #emph[hyperlink] yang ada di #emph[page] tersebut. Data-data yang telah dikumpulkan tersebut akan di masukkan ke dalam #emph[database] @lazuardithesis. Dalam proses #emph[crawling] setiap kali #emph[page scrapper] selesai dalam menjelajahi dan melakukan #emph[parsing] dalam satu page, #emph[page scrapper] akan memasukkan url list yang di dapat dari dalam page kedalam #emph[queue]. Agar proses penambahan link kedalam queue tidak terganggu, penambahan #emph[queue] dilakukan secara #emph[syncronous] menggunakan lock. #emph[Url list] yang disimpan di dalam #emph[queue] ini nantinya akan di akses oleh #emph[page scrapper] lain Proses ini akan berlanjut terus secara paralel dan pengaksesan tiap-tiap url dilakukan menggunakan algoritma #emph[breadth first search] @lazuardithesis.

== Algoritma #emph[Breadth First Search]

Untuk menjelajahi #emph[url list] yang ada di dalam #emph[queue] #emph[crawler] menggunakan algoritma #emph[breadth first search] algoritma ini pada dasarnya merupakan algoritma untuk menjelajahi #emph[graph] dalam suatu #emph[tree]. Dalam penerapannya di dalam arsitektur #emph[search engine] milik lazuardi #emph[breadth first search] dimanfaatkan dalam proses pemilihan url yang akan diakses dalam setiap iterasi proses #emph[page scrapping] @lazuardithesis. Dalam menjelajahi #emph[tree], algoritma ini menggunakan struktur data #emph[queue] untuk menyimpan informasi tentang #emph[node] yang akan dijelajahi selanjutnya dan #emph[stack] untuk menyimpan informasi mengenai #emph[node] yang telah di jelajahi. Metode #emph[breadth first search] memungkinkan untuk #emph[crawler] memprioritaskan penjelajahan #emph[url] yang telah dimasukkan ke dalam #emph[queue] terlebih dahulu hal ini menjamin agar setiap tingkatan #emph[node] sudah dijelajahi sebelum lanjut ke tingkat #emph[node] selanjutnya @cormen2009introduction.

#figure(
  image("gambar/operation_bfs.png"),
  caption: [
    Diagram alur algoritma #emph[breadth first search] @cormen2009introduction
  ],
)

== Definisi #emph[Processes] dalam #emph[Operating Systems]

#emph[Process] pada dasarnya adalah #emph[program] yang sedang dijalankan oleh komputer. #emph[Program] pada sendirinya hanya file pasif yang berisi instruksi - instruksi yang perlu dijalankan oleh komputer. Instruksi tersebut yang menjadi satu #emph[instance] dari #emph[process]. #emph[Process] mengakses data yang diperlukan untuk proses komputasi dari #emph[virtual memory], data di dalam memory ini bersifat sementara dan disimpan sebagai #emph[cache]. #emph[Memory] yang dapat diakses oleh #emph[process] memiliki susunan tertentu yang terbagi menjadi beberapa bagian.

#figure(
  image("gambar/memory-layout.jpeg"),
  caption: [
    Susunan bagian dari #emph[virtual memory] @operatingsystemconcept
  ],
) <memory-layout>
Dari gambar @memory-layout dapat dilihat bahwa susunan #emph[virtual memory] yang dapat diakses oleh #emph[process] yang berjalan terbagi menjadi beberapa bagian yang dibagi berdasarkan jenis data yang disimpan dan tingkat alamat dari data tersebut dalam #emph[memory] @operatingsystemconcept. Bagian-bagian dalam #emph[virtual memory] adalah,
+ #emph[Text]. Data yang berisi kode yang dijalankan
+ #emph[Data]. Data yang berisi variabel global dalam kode
+ #emph[Heap]. Sejumlah ukuran #emph[memory] yang dialokasikan secara dinamis oleh program saat program sedang berjalan atau #emph[runtime]
+ #emph[Stack]. Penyimpanan data sementara yang disediakan saat pemanggilan fungsi dalam kode.

= Desain Model

== Modifikasi Arsitektur #emph[Crawler]

Dari arsitektur #emph[crawler] yang sudah ada saat ini terdapat beberapa modifikasi yang perlu dilakukan untuk memperbaiki performa proses #emph[crawling].
- Arsitektur ini akan dibagi menjadi 2 #emph[service] yaitu, #emph[crawler] dan #emph[indexer].
- #emph[Service] #emph[Crawler] akan dibagi menjadi dua #emph[process], #emph[Scouter] dan #emph[Parser]. #emph[Scouter] bertugas sebagai pengunduh halaman #emph[website] dan #emph[parser] bertugas sebagai pembangun #emph[language tree] dan melakukan #emph[input] kedalam #emph[database].
- Jalannya #emph[crawler] dan #emph[indexer] akan secara bersamaan dan otomatis. Untuk mengontrol jalannya #emph[indexer] agar konsistensi data dapat terjaga,

#figure(
  image("gambar/crawler-multiprocess-architecture.png"),
  caption: [
    Diagram Arsitektur Crawler Termodifikasi
    ],
) <crawler-services-arch>

== Modifikasi #emph[Breadth-first Search] dengan #emph[Domain Constraint]

Untuk menyeragamkan jumlah halaman #emph[web] yang diakses oleh tiap #emph[thread], algoritma #emph[breadth-first search] yang digunakan untuk mengunjungi tiap-tiap halaman perlu dimodifikasi. Modifikasi yang dilakukan adalah dengan menugaskan jalannya #emph[crawler] di tiap #emph[thread] sebuah domain #emph[url] tertentu, dan membatasi #emph[url] yang dapat diakses oleh #emph[thread] tersebut sesuai dengan #emph[url] yang telah ditugaskan. Setiap #emph[thread] akan mengambil dan menyimpan #emph[url] di dalam #emph[queue] global #emph[Queue] ini merupakan #emph[multi-lock queue] dengan format yang lebih kompleks dari #emph[queue] normal, ini dilakukan agar tidak terjadi #emph[race condition] antar #emph[thread] ketika mengambil ataupun menyimpan data kedalam #emph[queue] tersebut. Gambar @diagram_modified_bfs merupakan ilustrasi dari jalannya algoritma #emph[breadth-first search] termodifikasi.

#figure(
  image("gambar/modified-bfs-diagram.png", height: 30%),
  caption: [
    Diagram cara kerja algoritma #emph[breadth-first search] termodifikasi
    ],
) <diagram_modified_bfs>

= Hasil dan Pembahasan

#lorem(200)

= Kesimpulan dan Saran

== Kesimpulan 

Berdasarkan hasil implementasi dan pengujian fitur sistem informasi yang telah dirancang, maka diperoleh kesimpulan sebagai berikut:
+ #emph[Crawler] berhasil mengumpulkan data dari halaman web yang #emph[domain]-nya telah di definiskan dalam #emph[origin url].
+ Dari perbandingan hasil proses #emph[crawling], bahasa pemograman berabstraksi rendah lebih cocok untuk digunakan dalam #emph[high-intensity application] seperti #emph[web crawler] ini.
+ Migrasi #emph[database] dari berbasis #emph[SQL] menuju berbasis #emph[MonggoDB] berhasil dan data yang tersimpan konsisten.
+ #emph[Crawler] yang dirancang menggunakan metode #emph[multi-threading] berhasil mengumpulkan jumlah halaman web lebih banyak daripada #emph[crawler] sebelumnya.
+ Algoritma #emph[breadth-first search] termodifikasi dalam skripsi ini belum cukup untuk meningkatkan akurasi proses #emph[crawling] terhadap halaman web yang didefiniskan oleh #emph[top-level domain].
+ Penggunaan #emph[resource] lebih banyak berada di #emph[scouter service] bila dibandingkan dengan #emph[parser service].

== Saran
Adapun saran untuk penelitian selanjutnya adalah:
+ Melanjutkan penelitian dalam eksplorasi algoritma #emph[crawler] lain untuk meningkatkan akurasi pengumpulan halaman web yang telah didefinisikan oleh #emph[top-level domain].
+ Melanjutkan penelitian dalam eksplorasi algoritma #emph[information retrieval] yang mengakomodasi lebih banyak jenis #emph[website].
+ Eksplorasi penggunaan #emph[filesystem] sebagai platform untuk menyimpan data hasil #emph[crawling] untuk mengakomodasi struktur halaman web yang berbeda-beda.
+ Eksplorasi implementasi #emph[distributed crawler] dengan menggunakan skema #emph[multi-threading] dengan bahasa pemograman berabstraksi rendah seperti #emph[Rust], #emph[C/C++], atau #emph[Zig].

