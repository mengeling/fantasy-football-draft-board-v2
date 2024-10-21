# Fantasy Football Draft Board

[Click here to use the draft board!](http://54.162.53.255/) Type "test" when prompted for your name if you want to play with the draft board without waiting for the data to be scraped.
<br>

This web application provides the same interactive fantasy football drafting experience as the official draft boards on ESPN, Yahoo, NFL.com, etc., but it uses consensus player rankings consolidated from 100+ experts.

### App Demo

![Demo](frontend/src/static/img/fantasy_football_recording.gif)
<br>
<br>

### App Screenshot

![App Screenshot](frontend/src/static/img/app_pic.png)

### App Setup

1. Clone the repository
2. Run `sudo apt-get install postgresql`
3. Run `sudo systemctl start postgresql`
4. Run `psql -U postgres -c "CREATE USER ffball WITH SUPERUSER CREATEDB CREATEROLE LOGIN PASSWORD 'ffball';"`
5. Run `createdb -U ffball ffball`
6. Run `\q` to quit
7. Run `cargo run` from the `backend` directory
8. Run `npm run dev` from the `frontend` directory
9. Open your browser and navigate to `http://localhost:3000`
