echo " Hello deploying locally to 4 servers"

cargo build
set @src=C:\Users\david\Documents\projects\Rust Projects\vhenn_coin\target\debug\Vhenn_coin.exe
set @dst=C:\Users\david\Documents\projects\Rust Projects\vhenn_coin\test_servers\

echo %@dst%
xcopy /y "%@src%" "%@dst%server1"
xcopy /y "%@src%" "%@dst%server2"
xcopy /y "%@src%" "%@dst%server3"
xcopy /y "%@src%" "%@dst%server4"

start cmd.exe /k "title server1 & cd %@dst%server1 & Vhenn_coin.exe"
start cmd.exe /k "title server2 & cd %@dst%server2 & Vhenn_coin.exe"
start cmd.exe /k "title server3 & cd %@dst%server3 & Vhenn_coin.exe"
start cmd.exe /k "title server4 & cd %@dst%server4 & Vhenn_coin.exe"