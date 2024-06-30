# may have to run this script as root user
Xephyr :1 -ac -br -noreset -screen 800x600 &
DISPLAY=:1 ./target/debug/sandy
DISPLAY=:1 $TERMINAL
