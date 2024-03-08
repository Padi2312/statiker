pub fn get_upload_html() -> String {
    r##"<!DOCTYPE html>
<html>

<head>
    <title>Upload Page</title>
    <style>
        :root {
            --primary-color: #8657de;
            --primary-light-color: #b38dfa;
            --background-color: #121212;
            --background-start-color: #484848;
            --background-end-color: #1e1e1e;

            --container-bg: #3f3f3f;
            --text-color: #ffffff;
            --input-bg: #333333;
            --input-border-color: #444444;
            --button-bg: linear-gradient(145deg, var(--primary-color), var(--primary-color));
            --button-hover-bg: linear-gradient(145deg, var(--primary-light-color), var(--primary-light-color));
        }

        body,
        html {
            height: 100%;
            margin: 0;
            background: linear-gradient(360deg, var(--background-start-color), var(--background-end-color));
        }

        body {
            display: flex;
            justify-content: center;
            align-items: center;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            color: var(--text-color);
        }

        .container {
            width: 400px;
            background-color: var(--container-bg);
            padding: 40px;
            border-radius: 8px;
            box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
        }

        h1 {
            font-size: 24px;
            text-align: center;
            color: var(--primary-color);
            margin-bottom: 30px;
        }

        .form-group {
            margin-bottom: 20px;
        }

        .form-group label {
            display: block;
            font-weight: 600;
            margin-bottom: 8px;
        }

        .form-group input[type="file"],
        .form-group input[type="text"] {
            width: 100%;
            padding: 12px 20px;
            margin: 8px 0;
            box-sizing: border-box;
            border: 1px solid #696969;
            border-radius: 4px;
            background-color: #696969;
            color: white;
            font-size: 16px;
            transition: border-color 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
        }

        .form-group input[type="file"]:focus,
        .form-group input[type="text"]:focus {
            border-color: var(--primary-color);
            outline: none;
            box-shadow: 0 0 0 2px var(--primary-color) inset;
        }

        button {
            width: 100%;
            padding: 12px;
            background: var(--button-bg);
            color: var(--text-color);
            border: none;
            border-radius: 5px;
            cursor: pointer;
            transition: background 0.3s ease;
            font-weight: 600;
        }

        button:hover {
            background: var(--button-hover-bg);
        }

        #fileLink {
            margin-top: 20px;
            display: none;
            color: var(--text-color);
        }

        #fileLink a {
            color: var(--primary-light-color);
            text-decoration: none;
            transition: color 0.3s ease;
        }

        #fileLink a:hover {
            color: var(--button-hover-bg);
        }
    </style>
</head>

<body>
    <div class="container">
        <h1>Upload File</h1>
        <form id="uploadForm" enctype="multipart/form-data">
            <div class="form-group">
                <label for="file">Select File:</label>
                <input type="file" id="file" name="file">
            </div>
            <div class="form-group">
                <label for="fileName">File Name:</label>
                <input type="text" id="fileName" name="fileName">
            </div>
            <button type="submit">Upload</button>
            <div id="fileLink"></div>
        </form>
    </div>

    <script>
        const form = document.getElementById('uploadForm');

        form.addEventListener('submit', (e) => {
            e.preventDefault();

            const fileInput = document.getElementById('file');
            const fileNameInput = document.getElementById('fileName');
            let fileName = fileNameInput.value;

            let file;
            if (fileName.length === 0) {
                file = fileInput.files[0];
                fileName = file.name;
            } else {
                file = new File([fileInput.files[0]], fileName);
            }
            const formData = new FormData();
            formData.append('file', file);

            fetch('/upload', {
                method: 'POST',
                body: formData,
            })
                .then(response => response.json())
                .then(data => {
                    const link = window.location.origin + "/" + fileName;
                    const fileLink = document.getElementById('fileLink');
                    fileLink.innerHTML = `File uploaded successfully. Link: <a href="${link}" target="_blank">${link}</a>`;
                    fileLink.style.display = 'block';
                })
                .catch(error => {
                    console.error(error);
                });
        });
    </script>
</body>

</html>
    "##
        .to_string()
}
