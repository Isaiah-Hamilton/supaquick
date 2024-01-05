mod ui;

use ui::{input, list, logo, option};

fn main() {
    logo();
    let import_options = ["Automatic", "Manual"];
    let frameworks = [
        "Next.js",
        "React",
        "NuxtJS",
        "RedwoodJS",
        "Flutter",
        "Android Kotlin",
        "SvelteKit",
        "SolidJS",
        "Vue",
        "refine",
    ];

    let name = input("What is your project named: ", "my-app", "my-app");
    let framework = list("Choose a framework:", &frameworks);
    let framework_template = list("Choose a template:", &frameworks);
    let import_option = option(
        "Import Supabase project automatically or manually: ",
        &import_options,
    );

    if import_option == import_options[0] {
        automatic_import();
    } else {
        let project_url = input("Project url: ", "", "");
        let project_anon_key = input("What is your Project anon key: ", "", "");
    }
}

fn automatic_import() {
    let organizations = ["supabase", "Isaiah-Hamilton"];
    let options = ["New", "Existing"];
    let existing_projects = ["test", "test2"];
    let regions = ["north america", "south america"];

    let access_token = input("access token: ", "", "");
    let organization = list("Choose an organization:", &organizations);
    let new_project = option(
        "Create new Supabase project or use existing project: ",
        &options,
    );

    if new_project == options[0] {
        let project_name = input("What is your project named: ", "", "");
        let database_password = input("What is your database password: ", "", "");
        let project_region = list("Where is your project location: ", &regions);
    } else {
        let project = list("Choose an existing porject:", &existing_projects);
    }
}
