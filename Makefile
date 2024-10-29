# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Run all formatting, linting, and testing tasks
all: format lint test run

# Extract iris data from url
extract: 
	cargo run extract

# Load data to database
transform:
	cargo run transform

# insert a new database entry
insert:
	cargo run query "INSERT INTO iris (sepal_length, sepal_width, petal_length, petal_width, species) VALUES (5.1, 3.5, 1.4, 0.2, 'new_species')"

# Read last entry from the database
read:
	cargo run query "SELECT * FROM iris ORDER BY id DESC LIMIT 1"
# Update first database entry
update:
	cargo run query "UPDATE iris SET sepal_length = 6.0 WHERE id = 1"
# Delete last database entry
delete:
	cargo run query "DELETE FROM iris WHERE id = (SELECT max(id) FROM iris)"