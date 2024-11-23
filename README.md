# Fantasy Football Draft Board

[Click here to use the draft board!](http://54.162.53.255/) Type "test" when prompted for your name if you want to play with the draft board without waiting for the data to be scraped.

This web application provides the same interactive fantasy football drafting experience as the official draft boards on ESPN, Yahoo, NFL.com, etc., but it uses consensus player rankings consolidated from 100+ experts.

## App Demo

![Demo](frontend/src/static/img/fantasy_football_recording.gif)

## App Screenshot

![App Screenshot](frontend/src/static/img/app_pic.png)

## App Setup

1. Clone the repository
2. Run `crontab -e` and paste this line with updated paths: `0 0 * * * RUST_LOG=info ./target/release/your_binary_name >> /path/to/logfile.log 2>&1`
3. Run `sudo apt-get install postgresql`
4. Run `sudo systemctl start postgresql`
5. Run `sudo -u postgres psql -c "CREATE USER ffball WITH SUPERUSER CREATEDB CREATEROLE LOGIN PASSWORD 'ffball';"`
6. Run `sudo -u postgres createdb -O ffball ffball`
7. Run `sudo nano /etc/postgresql/17/main/pg_hba.conf`
8. Update the following lines:

   ```bash
   # Database administrative login by Unix domain socket
   local   all             postgres                                peer

   # TYPE  DATABASE        USER            ADDRESS                 METHOD

   # "local" is for Unix domain socket connections only
   local   all             all                                     md5
   # IPv4 local connections:
   host    all             all             127.0.0.1/32            md5
   # IPv6 local connections:
   host    all             all             ::1/128                 md5
   # Allow replication connections from localhost, by a user with the
   # replication privilege.
   local   replication     all                                     peer
   host    replication     all             127.0.0.1/32            scram-sha-256
   host    replication     all             ::1/128                 scram-sha-256
   ```

9. Run `sudo systemctl restart postgresql`
   i. Run `PGPASSWORD=ffball psql -U ffball -d ffball` to access database
10. Run `./src/scripts/setup_db.sql` from the `backend` directory to perform initial DB setup
11. Run `cargo run` from the `backend` directory
12. Run `npm run dev` from the `frontend` directory
13. Open your browser and navigate to `http://localhost:3000`
