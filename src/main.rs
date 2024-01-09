mod ui;
mod utils;

use utils::{create_project, get_project_name};

fn main() {
    ui::logo();

    let project_name = get_project_name();

    let import_options = ["Automatic", "Manual"];
    let frameworks = [
        "Nextjs",
        // "React",
        // "NuxtJS",
        // "RedwoodJS",
        // "Flutter",
        // "Android Kotlin",
        // "SvelteKit",
        // "SolidJS",
        // "Vue",
        // "refine",
    ];
    let templates = ["app", "app-tw", "default", "default-tw"];

    let framework = ui::list("Choose a framework:", &frameworks);
    let template = ui::list("Choose a template:", &templates);
    let import_option = ui::option(
        "Import Supabase project automatically or manually: ",
        &import_options,
    );

    if import_option == import_options[0] {
        automatic_import();
    } else {
        let project_url = ui::input("Project url: ", "", "");
        let project_anon_key = ui::input("What is your Project anon key: ", "", "");
    }

    let src = "./templates/".to_owned() + &framework + "/" + &template;
    create_project(src, project_name);
}

fn automatic_import() {
    let organizations = ["supabase", "Isaiah-Hamilton"];
    let options = ["New", "Existing"];
    let existing_projects = ["test", "test2"];
    let regions = ["north america", "south america"];

    let access_token = ui::input("access token: ", "", "");
    let organization = ui::list("Choose an organization:", &organizations);
    let new_project = ui::option(
        "Create new Supabase project or use existing project: ",
        &options,
    );

    if new_project == options[0] {
        let project_name = ui::input("What is your project named: ", "", "");
        let database_password = ui::input("What is your database password: ", "", "");
        let project_region = ui::list("Where is your project location: ", &regions);
    } else {
        let project = ui::list("Choose an existing porject:", &existing_projects);
    }
}
