/*
   Copyright 2024 Ibrahim Mbaziira

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
const loginLink = document.getElementById("login-link");
const logoutLink = document.getElementById("logout-link");
const protectImg = document.getElementById("protected-img");

logoutLink.addEventListener("click", (e) => {
    e.preventDefault();

    let url = logoutLink.href;

    fetch(url, {
        method: 'POST',
        credentials: 'include', // This will include cookies in the request
    }).then(response => {
        if (response.ok) {
            loginLink.style.display = "block";
            logoutLink.style.display = "none";
            protectImg.src = "/assets/default.jpg";
        } else {
            alert("Failed to logout");
        }
    });
});

(() => {
    fetch('/protected').then(response => {
        if (response.ok) {
            loginLink.style.display = "none";
            logoutLink.style.display = "block";

            response.json().then(data => {
                let img_url = data.img_url;
                if (img_url !== undefined && img_url !== null && img_url !== "") {
                    protectImg.src = img_url;
                } else {
                    protectImg.src = "/assets/default.jpg";
                }
            });
        } else {
            loginLink.style.display = "block";
            logoutLink.style.display = "none";
            protectImg.src = "/assets/default.jpg";
        }
    });
})();