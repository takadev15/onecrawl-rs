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
Scientific writing is a crucial part of the research process, allowing researchers to share their findings with the wider scientific community. However, the process of typesetting scientific documents can often be a frustrating and time-consuming affair, particularly when using outdated tools such as LaTeX. Despite being over 30 years old, it remains a popular choice for scientific writing due to its power and flexibility. However, it also comes with a steep learning curve, complex syntax, and long compile times, leading to frustration and despair for many researchers. @netwok2020

== Paper overview
In this paper we introduce Typst, a new typesetting system designed to streamline the scientific writing process and provide researchers with a fast, efficient, and easy-to-use alternative to existing systems. Our goal is to shake up the status quo and offer researchers a better way to approach scientific writing.

By leveraging advanced algorithms and a user-friendly interface, Typst offers several advantages over existing typesetting systems, including faster document creation, simplified syntax, and increased ease-of-use.

To demonstrate the potential of Typst, we conducted a series of experiments comparing it to other popular typesetting systems, including LaTeX. Our findings suggest that Typst offers several benefits for scientific writing, particularly for novice users who may struggle with the complexities of LaTeX. Additionally, we demonstrate that Typst offers advanced features for experienced users, allowing for greater customization and flexibility in document creation.

Overall, we believe that Typst represents a significant step forward in the field of scientific writing and typesetting, providing researchers with a valuable tool to streamline their workflow and focus on what really matters: their research. In the following sections, we will introduce Typst in more detail and provide evidence for its superiority over other typesetting systems in a variety of scenarios.

= Kajian Pustaka
#lorem(90)

$ a + b = gamma $

#lorem(200)

= Desain Model

#lorem(200)

= Hasil dan Pembahasan

#lorem(200)
