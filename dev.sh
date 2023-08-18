#!/usr/bin/env bash

./tailwindcss --input static/styles/tailwind.css --output static/styles/site.css --watch &
systemfd --no-pid --socket http::3000 -- cargo watch -x run