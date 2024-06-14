ECHO OFF
cls
rm day07.out
rustc -Anon_snake_case day07.rs
day07.exe >> day07.out
nvim day07.out
