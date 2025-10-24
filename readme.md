Click “New → Web Service” (⚠️ not “Static Site”).
So if you only see “Publish Directory”, you accidentally chose Static Site.
That’s not what we want for Actix that serves html

`cargo run --release` it would rebuild every time it starts your service — slow and wasteful.

[demo url](https://actix-serve-html.onrender.com/)