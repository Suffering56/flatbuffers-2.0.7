# Инструкция по сборке flatc под linux

Возможно можно сделать билд и под виндой, но я не силен в cmake и поэтому собирал всё в debian докер контейнере
(основная инфа взята отсюда: https://stackoverflow.com/questions/55394537/how-to-install-flatc-and-flatbuffers-on-linux-ubuntu)

Раньше тут была длинная инструкция, 
но теперь нужно просто даблкликнуть на файл `./build_instructions/linux/build_flatc.bat`
При клике активной директорией выбирается `..path_to_repo/build_instructions/linux`, 
поэтому если запускаете скрипт из консоли - сделайте эту директорию активной.

Скрипт выполняется не быстро, но в результате его выполнения собранный *flatc* будет нас ждать в `/build_instructions/linux`

---

PS: скрипт можно ускорить и оптимизировать миллионами способов, но я и так уже тут засиделся с этой таской, так что в другой раз