#[macro_use] extern crate rocket;

use rocket::response::content::Html;

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Language Level Selection</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            color: blue;
            font-size: 18px;
            font-weight: bold;
            text-align: center;
        }
        label {
            display: block;
            margin-bottom: 20px;
            font-weight: bold;
            color: black;
            font-size: 24px;
        }
        select {
            font-weight: bold;
            color: red;
            font-size: 18px;
        }
        button {
            font-weight: bold;
            color: white;
            background-color: black;
            font-size: 18px;
            padding: 10px 20px;
            border: none;
            cursor: pointer;
        }
        #message {
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <label for="language-level">Choose your language level:</label>
    <select id="language-level" name="language-level">
        <option value="beginner">Beginner</option>
        <option value="intermediate">Intermediate</option>
        <option value="advanced">Advanced</option>
    </select>
    <br><br>
    <button onclick="showMessage()">Next</button>
    
    <div id="message"></div>

    <script>
        const messages = {
            beginner: [
                "Start talking with other native speakers for at least 30 minutes a day",
                "Start watching your favorite films and TV series in this language",
                "Start learning the language using different apps",
                "Start singing along with subtitled songs",
                "Start storytelling a picture"
            ],
            intermediate: [
                "Start describing your day",
                "Start translating famous phrases in this language",
                "Start joining forums in this language",
                "Start writing short posts or comments daily in the language",
                "Start listening to short podcast episodes"
            ],
            advanced: [
                "Start reading new books in this language",
                "Start finding new friends by speaking this language",
                "Start learning new disciplines in a new language",
                "Start creative writing",
                "Start recording your speech"
            ]
        };

        function showMessage() {
            const level = document.getElementById('language-level').value;
            const messageArray = messages[level];
            const randomMessage = messageArray[Math.floor(Math.random() * messageArray.length)];
            document.getElementById('message').innerText = randomMessage;
        }
    </script>
</body>
</html>
    "#)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
