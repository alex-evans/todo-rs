# todo-rs
Rust Learning Small Project To Do App


1. As a user, I want to add a new task to my to-do list.
    - **Acceptance Criteria:**
        - [ ] The application should prompt me for task details (such as title and description).
        - [ ] The task should be stored persistently (e.g., in a file).
        - [ ] I should receive feedback confirming that the task has been added successfully.

2. As a user, I want to view my existing tasks in the to-do list.
    - **Acceptance Criteria:**
        - [ ] The application should display a list of all tasks.
        - [ ] Each task should show its title, description, and status (e.g., completed or pending).

3. As a user, I want to mark a task as completed.
    - **Acceptance Criteria:**
        - [ ] I should be able to select a task from the list.
        - [ ] The application should update the task’s status to “completed.”

4. As a user, I want to edit an existing task.
    - **Acceptance Criteria:**
        - [ ] I should be able to choose a task to edit.
        - [ ] The application should allow me to modify the task’s title and description.
        - [ ] The updated task details should be saved.

5. As a user, I want to delete a task from the list.
    - **Acceptance Criteria:**
        - [ ] I should be able to select a task for deletion.
        - [ ] The application should remove the task from the list.

6. As a user, I want error handling for invalid inputs.
    - **Acceptance Criteria:**
        - [ ] The application should handle cases where the user provides incorrect input (e.g., non-numeric task IDs).
        - [ ] Clear error messages should be displayed to guide the user.

7. As a user, I want my tasks to persist across sessions.
    - **Acceptance Criteria:**
        - The application should save tasks to a file.
        - When I restart the app, my tasks should still be available.