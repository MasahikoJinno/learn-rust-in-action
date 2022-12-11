## memo

このコードは10行目で削除した `grains` を参照しようとしてコンパイルエラーとなっている

```bash
error[E0382]: borrow of moved value: `grains`
  --> src/main.rs:11:22
   |
8  |     let mut grains: Vec<Cereal> = vec![];
   |         ---------- move occurs because `grains` has type `Vec<Cereal>`, which does not implement the `Copy` trait
9  |     grains.push(Cereal::Barley);
10 |     drop(grains);
   |          ------ value moved here
11 |     println!("{:?}", grains);
   |                      ^^^^^^ value borrowed here after move
```

`value borrowed here after move` -> 移動(move)された値を借用(borrow)しようとした

