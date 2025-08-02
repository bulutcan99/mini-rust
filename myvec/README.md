# 🦀 MyVec – Rust’ta Sıfırdan `Vec` Implementasyonu

Bu proje, Rust programlama dilinin standart kütüphanesinde bulunan `Vec<T>` yapısının basitleştirilmiş bir versiyonunu sıfırdan implemente etmeyi amaçlamaktadır. Projenin temel hedefi, **`unsafe` kod**, **manuel bellek yönetimi** ve Rust'taki **container** veri yapılarının iç işleyişi üzerine pratik yapmaktır.

## 🎯 Hedefler

*   **Heap üzerinde bellek yönetimi:** `std::alloc` crate'i ile doğrudan bellek ayırma ve serbest bırakma.
*   **Raw pointer (`*mut T`) kullanımı:** Ham pointer'lar ile bellek üzerinde doğrudan işlem yapma.
*   **`MaybeUninit<T>` ile başlatılmamış bellek yönetimi:** Bellek alanlarının henüz başlatılmamış olabileceği durumları güvenli bir şekilde yönetme.
*   **Custom destructor (`Drop` trait):** Oluşturulan veri yapısı kapsam dışına çıktığında belleği temizlemek için `Drop` trait'ini uygulama.
*   **`Vec` benzeri bir API tasarlama:** `push`, `pop`, `get` gibi standart `Vec` metotlarına benzer bir arayüz oluşturma.

---

## 📋 Yapılacaklar Listesi

### 1️⃣ Proje Hazırlığı

-   [ ] Yeni bir Cargo projesi oluşturun.
    ```bash
    cargo new myvec --lib
    ```
-   [ ] `std::alloc`, `NonNull<T>`, ve `MaybeUninit<T>` gibi yapıları araştırın.
-   [ ] `unsafe` blokların nasıl güvenli bir arayüz arkasında kullanılabileceğini öğrenin (encapsulation).

### 2️⃣ Temel `struct` Tasarımı

-   [ ] `MyVec<T>` struct'ını tanımlayın.
    ```rust
    pub struct MyVec<T> {
        ptr: NonNull<T>, // Heap üzerindeki veri pointer'ı
        len: usize,      // Mevcut eleman sayısı
        cap: usize,      // Ayrılmış toplam kapasite
    }
    ```
-   [ ] `new()` metodu ile boş bir `MyVec` oluşturun.
-   [ ] Boş durumda bellek ayırmamak için `NonNull::dangling()` kullanın.

### 3️⃣ Bellek Yönetimi

-   [ ] `grow()` metodu ile kapasiteyi artırın.
    -   [ ] İlk eleman eklendiğinde kapasiteyi `1` olarak ayarlayın.
    -   [ ] Sonraki büyütmelerde kapasiteyi iki katına çıkarın (`cap * 2`).
    -   [ ] Bellek ayırmak için `std::alloc::alloc` ve `std::alloc::realloc` fonksiyonlarını kullanın.
    -   [ ] Pointer'ın null olmamasını garantilemek için `NonNull<T>` ile sarmalayın.

### 4️⃣ Veri Ekleme ve Okuma

-   [ ] `push(&mut self, value: T)` metodu yazın.
    -   [ ] Eğer `len == cap` ise `grow()` metodunu çağırın.
    -   [ ] `unsafe` blok içinde `ptr.add(len).write(value)` kullanarak veriyi belleğe yazın.
    -   [ ] `len` değerini bir artırın.
-   [ ] `get(&self, index: usize) -> Option<&T>` metodu ekleyin.
    -   [ ] `index`'in sınırlar içinde olup olmadığını (`index < len`) kontrol edin.
    -   [ ] `unsafe` blok içinde pointer'dan güvenli bir referans (`&T`) döndürün.

### 5️⃣ Belleği Serbest Bırakma

-   [ ] `Drop` trait'ini `MyVec<T>` için implemente edin.
    -   [ ] Eğer `len > 0` ise, `for` döngüsü veya `slice::from_raw_parts_mut` ile tüm elemanları `std::ptr::drop_in_place` ile temizleyin. Bu, `T` tipinin kendi `Drop` implementasyonunu çağırır.
    -   [ ] `std::alloc::dealloc` ile `MyVec`'e ait olan bellek alanını serbest bırakın.

### 6️⃣ Ek Özellikler (Opsiyonel)

-   [ ] `pop(&mut self) -> Option<T>` metodu.
-   [ ] `insert(&mut self, index: usize, element: T)` metodu.
-   [ ] `remove(&mut self, index: usize) -> T` metodu.
-   [ ] `Index` ve `IndexMut` trait'lerini implemente ederek `vec[i]` erişimi sağlayın.
-   [ ] `IntoIterator` trait'ini implemente ederek `for` döngüsü desteği ekleyin.
-   [ ] `FromIterator<T>` implementasyonu ile koleksiyonlardan `MyVec` oluşturmayı sağlayın.

---

### 🛠️ Derleme ve Test

-   [ ] Projenizin davranışını doğrulamak için testler ekleyin.
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn push_and_get() {
            let mut v = MyVec::new();
            v.push(42);
            v.push(1337);
            assert_eq!(v.len(), 2);
            assert_eq!(v.capacity(), 2);
            assert_eq!(v.get(0), Some(&42));
            assert_eq!(v.get(1), Some(&1337));
            assert_eq!(v.get(2), None);
        }

        #[test]
        fn drop_test() {
            // Drop testleri için `std::rc::Rc` gibi tiplerle
            // bellek sızıntısı olup olmadığını kontrol edebilirsiniz.
            // Bu kısım daha ileri seviye bir testtir.
        }
    }
    ```
-   [ ] `cargo test` komutu ile testlerinizi çalıştırın.

---

### 📚 Kaynaklar

*   **The Rustonomicon - "Implementing Vec":** [https://doc.rust-lang.org/nomicon/vec/vec.html](https://doc.rust-lang.org/nomicon/vec/vec.html)
*   **`std::alloc` modülü belgeleri:** [https://doc.rust-lang.org/std/alloc/index.html](https://doc.rust-lang.org/std/alloc/index.html)
*   **`MaybeUninit<T>` belgeleri:** [https://doc.rust-lang.org/std/mem/union.MaybeUninit.html](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html)
