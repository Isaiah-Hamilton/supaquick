mod ui;

use ui::{input, list, logo, option};

fn main() {
    logo();
    let options = ["Yes", "No"];
    let option = option("Do you want to use tailwindcss: ", &options);
    let frameworks = [
        "Next.js",
        "React",
        // "NuxtJS",
        // "RedwoodJS",
        // "Flutter",
        // "Android Kotlin",
        "SvelteKit",
        "SolidJS",
        "Vue",
        // "refine",
    ];

    // Get project name
    let name = input("What is your project named: ", "my-app", "my-app");
    // Get framework
    let framework = list("Choose a framework:", &frameworks);
    // Get framework template
    let framework_template = list("Choose a template:", &frameworks);

    // let access_token = input("access token: ", "none");

    // TODO: Pick a framework template

    // TODO: Pick an organization

    // TODO: use existing supabase project or create new project
    // // TODO: if new then pick a supabase project name
    // // TODO: Pick a database password (auto gen???)
    // // TODO: Pick database location
    // // TODO: Pick a plan
    // // TODO: Get annon key from user
    // // TODO: Get service_role secret from user (might not need)

    println!("option: {}", option);
    println!("name: {}", name);
    println!("framework: {}", framework);
    println!("framework template: {}", framework_template);
}
