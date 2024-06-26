# todo-rs
Rust Learning Small Project To Do App


1. As a user, I want to add a new task to my to-do list.
    - **Acceptance Criteria:**
        - [x] The application should prompt me for task details (such as title and description).
        - [x] The task should be stored persistently (e.g., in a file).
        - [x] I should receive feedback confirming that the task has been added successfully.

        ![TODO-US1](./images/ToDo-US1.png)

2. As a user, I want to view my existing tasks in the to-do list.
    - **Acceptance Criteria:**
        - [x] The application should display a list of all tasks.
        - [x] Each task should show its title, description, and status (e.g., completed or pending).

        ![TODO-US2](./images/ToDo-US2.png)

3. As a user, I want to mark a task as completed.
    - **Acceptance Criteria:**
        - [x] I should be able to select a task from the list.
        - [x] The application should update the task’s status to “completed.”

4. As a user, I want to edit an existing task.
    - **Acceptance Criteria:**
        - [x] I should be able to choose a task to edit.
        - [x] The application should allow me to modify the task’s title and description.
        - [x] The updated task details should be saved.

5. As a user, I want to delete a task from the list.
    - **Acceptance Criteria:**
        - [x] I should be able to select a task for deletion.
        - [x] The application should remove the task from the list.

6. As a user, I want error handling for invalid inputs.
    - **Acceptance Criteria:**
        - [x] The application should handle cases where the user provides incorrect input (e.g., non-numeric task IDs).
        - [x] Clear error messages should be displayed to guide the user.

7. As a user, I want my tasks to persist across sessions.
    - **Acceptance Criteria:**
        - [x] The application should save tasks to a file.
        - [x] When I restart the app, my tasks should still be available.