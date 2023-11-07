![](https://jobs.mindtheproduct.com/wp-content/uploads/job-manager-uploads/company_logo/2021/04/holoplot_logo_logo-purple-dark-1.png)

## Software Engineer – Coding Assignment
AudioObjectManager is an interface to manage AudioObjects in 3D space. It implements standard CRUD (Create, Retrieve, Update, Delete) features:
* **Create**: AudioObjectManager.add(position: Position)
* **Retrieve**: AudioObjectManager.get(id: usize)
* **Update**: AudioObjectManager.move_object(to_position: Position)
* **Delete**: AudioObjectManager.remove(id: usize)

In addition, all mutating actions support undo/redo features:
* **Undo**: AudioObjectManager.undo()
* **Redo**: AudioObjectManager.redo()

## How to install & Run
Running the project using standard cargo module. You need to have Rust programming language installed and cargo too.

    cargo run
