# wave
This is my custom broswer engine.

# What is a browser engine?
A browser engine is the part of a browser responsible for the rendering of HTML & CSS. This will give a much better understanding of how these technologies work, and because we use rust for this project, we can directly compete with Firefox's new Servo engine. (The (devs)[[https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html]] also gave a great starting place, and I am going to stick to the rust standard library as much as possible for the entire browser).

# Why make a seperate project for the engine & the browser?
If we look at the now defunct Gecko engine; we can see how big of an undertaking this type of project actually is. Naturally since we are starting out as a "toy" engine, we would want to eventually branch it out into it's own thing entirely. This is where the seperate repo comes in. This way as the engine grows, the functionality of the browser only gets better.

# Parts To Consider for an Engine:
HTML/CSS rendering
JavaScript Interpreter (sometimes also seperate; see (v8)[[https://v8.dev/docs]])

# Parts Needed for a Browser:
    HTML/CSS Rendering
    UI
    Networking
    Data Storage
    JS Interpreter

Combining the interpreter & renderer together should give our unique stack a bit of an edge (will find out later ig...) if I need to I can always sperate them in the future.

I will most likely keep both in a seperate file, and have them share functionality in our `main.rs` file. The networking, etc. will all be handled with the `std::net` library, and won't be using advanced features from Tokiyo no matter how much I'd probably prefer in order to keep dependicies low and the application runnable on the most platforms (eventhough you only need to change the build file).

# GPT build process:
Creating a full-fledged browser engine from scratch is a complex task that requires deep understanding of web standards, rendering, networking, and more. However, I can outline the basic steps involved in building a simple browser engine using Rust.

Step 1: Parsing the HTML Markup

Begin by parsing the HTML markup of a web page. Rust provides libraries like html5ever or scraper that can help with HTML parsing.
Parse the HTML tags, attributes, and their values to build a Document Object Model (DOM) tree, representing the webpage's structure.
Step 2: Rendering the DOM Tree

Implement a rendering engine to traverse the DOM tree and generate the layout of each element. This involves calculating positions, styles, and sizes.
Rust's webrender library can be used to efficiently render the DOM tree into pixels on the screen.
Step 3: Handling CSS Styling

Parse the CSS stylesheets associated with the webpage. Rust provides libraries like cssparser or stylish for CSS parsing.
Apply the computed styles to the DOM tree, modifying the layout and appearance of each element.
Step 4: Implementing JavaScript Execution

Integrate a JavaScript engine, such as rusty_v8 or quickjs, to execute the Javascript code embedded within the webpage.
Implement the necessary APIs to interact with the DOM tree and handle events.
Step 5: Networking and Resource Loading

Implement a networking layer to handle HTTP requests and responses. Rust provides libraries like reqwest or hyper for networking.
Load external resources, such as images, CSS files, scripts, and handle their integration with the DOM tree.
Step 6: Handling User Input and Interactivity

Capture user input events, such as clicks or keyboard input, and respond accordingly.
Implement event handling mechanisms, such as attaching event listeners, propagating events through the DOM tree, and executing associated JavaScript code.
Step 7: Implementing History and Navigation

Manage the browser's history and enable navigation between different web pages.
Handle back and forward actions, updating the displayed webpage accordingly.
Remember, building a full browser engine is an extensive project that involves many intricate details and considerations. Therefore, it might be helpful to refer to resources like documentation, open-source browser projects, or books that delve into the topic more deeply.

**THIS IS NOT A BROWSER ENGINE ChatGPT!!!** Also since we noted we weren't using libraries for this we can ignore the suggestions provided.