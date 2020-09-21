# TODO: set proper dependency?

build:
	docker build -t todo-app .

run:
	docker run -p 8080:8080 todo-app

migration:
	diesel migration run --database-url "./todo.db"