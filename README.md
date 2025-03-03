# Todo Backend API

This is the backend for the Solana Todo application, built using Actix-Web.

## Setup Instructions

1. Install Rust and Cargo
2. Clone the repository:
   \`\`\`sh
   git clone https://github.com/Ezhil2k/todo_backend_api.git
   cd todo_backend_api
   \`\`\`
3. Install dependencies:
   \`\`\`sh
   cargo build
   \`\`\`
4. Run the server:
   \`\`\`sh
   cargo run
   \`\`\`

## API Endpoints

- **GET** \`/api/todos\` - List all todos
- **POST** \`/api/todos\` - Create a new todo
- **PUT** \`/api/todos/:id\` - Update a todo
- **DELETE** \`/api/todos/:id\` - Delete a todo


