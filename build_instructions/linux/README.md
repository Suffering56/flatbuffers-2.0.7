# Инструкция по сборке flatc под linux

Возможно можно сделать билд и под виндой, но я не силен в cmake и поэтому собирал всё в debian докер контейнере
(основная инфа взята отсюда: https://stackoverflow.com/questions/55394537/how-to-install-flatc-and-flatbuffers-on-linux-ubuntu)

1) Собираем подготовленный debian образ, запускаем его:

`cmd -> cd ../build_instructions/linux`

`docker build`

`docker run -it --name cmake -d debian:latest`

2) Далее нужно прокинуть в контейнер исходники flatbuffers: 
например
`docker exec cmake git clone ...`
или
`docker cp local_path_to_flatbuffers_sources cmake:/home/`


3) Далее собираем flatc:
* если вы еще не в контейнере:`docker attach cmake`
* переходим в папку с исходниками FB: `cd /home/flatbuffers`
* `cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release`
* `make`
* `make install`

4) Готово. В результате у нас должен появиться файл по адресу cmake:/home/flatbuffers/flatc

---
## Если у вас Ubuntu, то можно попробовать альтернативный вариант с использованием flatbuffers-compiler:

* `sudo apt-add-repository ppa:hnakamur/flatbuffers`
* `sudo apt update`
* `sudo apt install -y flatbuffers-compiler`
* Дальнейшие шаги мне неизвестны, т.к. сам не пользовался этой штукой.