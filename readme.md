# CLI Calender

A simple calender app written in **Rust**, using **Ratatui**.

The application is modeled on my understanding of the MVC architecture. 

### Usage Instructions
Navigate with left and right arrow keys.  
Scroll with up and down arrow keys.  
Press 1-7 to add tasks, and SHIFT+1-7 to clear tasks.  
Press Delete to exit application.  
Press Escape while adding tasks to cancel addition.

### Running locally

1. Clone the project.  
2. Following dependencies, as mentioned in the cargo.toml file, are required:  

        [package]
        name = "calender"
        version = "0.1.0"
        edition = "2021"

        # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

        [dependencies]
        crossterm = "0.27.0"
        ratatui = "0.23.0"
        chrono = "0"
        serde_json = "1"

3. Run the project with `cargo run`



![Screenshot of Application](image.png)