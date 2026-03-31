use std::fmt;

// --- ドメインモデル ---

#[derive(Debug, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: Status,
}

impl Task {
    fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            status: Status::Todo,
        }
    }
}

// --- 管理ロジック ---

struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

#[derive(Debug, PartialEq)]
enum TaskError {
    NotFound(u32),
    EmptyTitle,
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::NotFound(id) => write!(f, "Task with ID {} not found.", id),
            TaskError::EmptyTitle => write!(f, "Task title cannot be empty."),
        }
    }
}

impl TaskList {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: &str) -> Result<u32, TaskError> {
        if title.trim().is_empty() {
            return Err(TaskError::EmptyTitle);
        }
        let id = self.next_id;
        let task = Task::new(id, title);
        self.tasks.push(task);
        self.next_id += 1;
        Ok(id)
    }

    fn complete_task(&mut self, id: u32) -> Result<(), TaskError> {
        let task = self.tasks.iter_mut().find(|t| t.id == id);
        match task {
            Some(t) => {
                t.status = Status::Done;
                Ok(())
            }
            None => Err(TaskError::NotFound(id)),
        }
    }

    fn list_tasks(&self) {
        println!("\n--- Current Task List ---");
        for task in &self.tasks {
            let marker = match task.status {
                Status::Todo => "[ ]",
                Status::InProgress => "[-]",
                Status::Done => "[x]",
            };
            println!("{} ID: {}: {}", marker, task.id, task.title);
        }
        println!("--------------------------\n");
    }
}

// --- メイン関数 ---

fn main() {
    let mut my_list = TaskList::new();

    println!("Adding tasks...");
    let id1 = my_list.add_task("GitHub Actionsの設定を確認する").unwrap();
    let id2 = my_list.add_task("Rustの単体テストを実行する").unwrap();

    my_list.list_tasks();

    println!("Completing task {}...", id1);
    my_list.complete_task(id1).expect("Failed to complete task");

    my_list.list_tasks();

    println!("All operations completed successfully.");
}

// --- ユニットテスト ---
// GitHub Actionsの 'cargo test' ステップでここが実行されます。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut list = TaskList::new();
        let id = list.add_task("Test task").unwrap();
        assert_eq!(id, 1);
        assert_eq!(list.tasks.len(), 1);
        assert_eq!(list.tasks[0].title, "Test task");
        assert_eq!(list.tasks[0].status, Status::Todo);
    }

    #[test]
    fn test_add_empty_task_fails() {
        let mut list = TaskList::new();
        let result = list.add_task("   ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TaskError::EmptyTitle);
    }

    #[test]
    fn test_complete_task() {
        let mut list = TaskList::new();
        let id = list.add_task("Finish CI setup").unwrap();
        let res = list.complete_task(id);
        assert!(res.is_ok());
        assert_eq!(list.tasks[0].status, Status::Done);
    }

    #[test]
    fn test_complete_non_existent_task() {
        let mut list = TaskList::new();
        let res = list.complete_task(999);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), TaskError::NotFound(999));
    }

    #[test]
    fn test_task_id_increments() {
        let mut list = TaskList::new();
        let id1 = list.add_task("First").unwrap();
        let id2 = list.add_task("Second").unwrap();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
}

