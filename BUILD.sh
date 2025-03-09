trunk build --release;
sudo docker run -d -v /home/pipi/mtvleptos/dist:/usr/share/nginx/html:ro -p 9000:80 nginx:bookworm;