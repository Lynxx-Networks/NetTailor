# Developing

First of all, thanks for considering contributing on NetTailor! NetTailor is a rust project frontend that uses the Yew framework to build a wasm application. On the backend we use fastapi to call data needed from the database of the users choice.

This article outlines how to get NetTailor running in a development environment, and outlines the basics of the architecture.

- [Setting up the Development Environment](#setting-up-the-dev-environment)
  - [Prerequisites](#prerequisites)
  - [Running the App](#running-the-project)
  - [Notes](#notes)
- [Git Strategy](#git-strategy)
  - [Flow](#git-flow)
  - [Branches](#git-branch-naming)
  - [PR Guidelines](#pr-guidelines)
- [Resources for Beginners](#resources-for-beginners)

## Setting up the Dev Environment

### Prerequisites

You will need either the latest version of **[Rust](https://www.rust-lang.org/tools/install)**. You'll also need the latest version of **[Trunk](https://trunkrs.dev/)** to build and serve the application. Finally **[Git](https://git-scm.com/downloads)** to easily fetch the code, and push any changes. If you plan on running or deploying the container, you'll also need **[Docker](https://docs.docker.com/get-docker/)**. 

With all this installed you also need the compilation target for wasm

```
rustup target add wasm32-unknown-unknown
```

### Running the Project

1. Get Code: `git clone https://github.com/Lynxx-Networks/NetTailor.git`
2. Navigate into the directory: `cd web`
4. Start dev server: `trunk serve --features server_build`

NetTailor should now be being served on <http://localhost:8080/>. Hot reload is enabled, so making changes to any of the files and saving will trigger them to be rebuilt and the page refreshed.

Now, what you've just done is run the web frontend of the project. The backend is not running currently, meaning, unless you connect to an external NetTailor server you will not be able to sign in. To sign in you need the backend running. To run the backend you can either just pull the NetTailor docker image form docker hub or you can build it yourself. You can build it with this command after navigating to the base repo directory: 

```
cd NetTailor/
sudo docker build -t madeofpendletonwool/NetTailor:latest
```

Then once built you can run the container with **[Docker Compose](https://github.com/Lynxx-Networks/NetTailor/tree/main/deployment/docker/compose-files)**. Once you have the backend running you can connect to the server and sign in with your development frontend. Go to the development server url: http://localhost:8080 by default. Click the connect to different server button:

![Connect Different Server Button](../../static/img/diff-serv.png)

And then enter the server url in the server name: http://localhost:8040 by default with the compose file linked above. You're now signed into a dev NetTailor server using a local backend!

### Notes

- You'll notice above we're using the server_build feature. This is because when compiling without that feature it builds the project for the client version which builds on Tauri. 

---

## Git Strategy

### Git Flow

Like most Git repos, we are following the [Github Flow](https://guides.github.com/introduction/flow) standard.

1. Create a fork
2. Make any changes you want 🧑‍💻
3. Add, commit and push your changes to your branch/ fork
4. Head over to GitHub and create a Pull Request
5. Hit submit
6. Follow up with any reviews on your code
7. Merge 🎉

Please limit PRs to one feature/bug fix per PR. This will make it less complicated to merge your code if there's a problem with one thing but not the other.

### Git Branch Naming

The format of your branch name should be something similar to: `[TYPE]/[PR] [TITLE]`
For example, `FEATURE/Awesome feature` or `FIX/login server error`

### PR Guidelines

Once you've made your changes, and pushed them to your fork or branch, you're ready to open a pull request!

For a pull request to be merged, it must:

- Must be backwards compatible
- The build and tests (run by GH actions) must pass
- There must not be any merge conflicts

When you submit your PR, include some required info. Including:

- A brief description of your changes
- The issue or discussion number (if applicable)
- For UI related updates include a screenshot
- If any dependencies were added, explain why it was needed, state the cost associated, and confirm it does not introduce any security issues
- Finally hit submit!

---

## Resources for Beginners

New to Rust Web Development or Python Backend? Glad you're here! The following articles should point you in the right direction for getting up to speed with the technologies used in this project:

- [Open Source for Beginners](https://opensource.guide/how-to-contribute/)
- [Tutorial for Yew](https://yew.rs/docs/tutorial)
- [FastAPI Walkthrough](https://fastapi.tiangolo.com/tutorial/first-steps/)
- [Complete beginners guide to Docker](https://docker-curriculum.com/)
- [Docker Classroom - Interactive Tutorials](https://training.play-with-docker.com/)
- [Git cheat sheet](http://git-cheatsheet.com/)

As well as Rust, Yew, Git and Docker- you'll also need an IDE (e.g. [VS Code](https://code.visualstudio.com/) or [Vim](https://www.vim.org/)) and a terminal (Windows users may find [WSL](https://docs.microsoft.com/en-us/windows/wsl/) more convenient).

---


