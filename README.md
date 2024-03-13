
![image](https://github.com/protocol-Weaver/Rust_Inventory/assets/98874867/1b731667-ae30-4486-a6ab-c3f7a97af6c4)

# Rust_Inventory
Inventory Handling System is a console-based application that allows users to manage their inventories of products using Rust. The application features:

- A secure authentication system that requires users to enter their username and password before accessing the inventory data.
- A vector-based data structure that stores the inventory information, such as product name, quantity, price, and category.
- A user-friendly interface that displays the inventory data in a table format on the console, and allows users to perform various operations, such as adding, deleting, editing, and searching products.
- A report generation function that creates a formatted and printable inventory report on the console, and saves it as a text file in the local directory.

The project aims to provide a simple and efficient way of handling inventories using Rust, a fast and reliable programming language. The project demonstrates the use of Rust’s features, such as ownership, borrowing, error handling, and generics.

Our vision for the Rust Inventory Handling System is to revolutionize inventory management, prioritizing utmost security. We aim to redefine how inventories are handled by implementing robust security measures, ensuring the confidentiality and integrity of crucial business data. By providing a secure and user-friendly platform, our project strives to empower businesses of all sizes, fostering trust and reliability in digital inventory management. We envision a future where companies can operate confidently, knowing their inventories are not only efficiently managed but also fortified against potential threats. The impact of our project lies in creating a new standard for secure, streamlined inventory handling.

**Software Development Plan: Rust Inventory Handling System**

**Step 1: Project Setup and Requirements Analysis**
- Define the project scope, objectives, and functionalities.
- Identify user roles, such as admin and standard users.
- Specify security measures and authentication protocols.

**Step 2: Editing Inventory Functionality**
- Implement functions for product editing, addition, and deletion via Rust Vectors.
- Integrate authentication functions for secure user access.

**Step 3: Report Generation and Storage**
- Design functions to generate comprehensive inventory reports.
- Implement features to save reports in a file for future reference.

**Step 4: Front-End Development**
- Create a user-friendly console interface for seamless navigation.
- Design interfaces for viewing, editing, and managing inventory.

**Step 5: Testing and Debugging**
- Conduct rigorous testing of smart contract functions and front-end interfaces.
- Address any bugs, glitches, or security vulnerabilities identified during testing.

**Step 6: Deployment**
- Deploy the Rust Inventory Handling System on the chosen platform.
- Monitor system performance and address any post-deployment issues promptly.

This development plan ensures a systematic approach to building the Rust Inventory Handling System, focusing on the core smart contract functions, security, user interfaces, and thorough testing before deployment.

# Personal Story 

AbdulRehman, a passionate BsCs student, embarks on a journey to enhance his Rust skills by creating an engaging Inventory Handling System. Driven by a desire for hands-on learning, he embraces the project as both a skill-building opportunity and a source of enjoyment. In crafting this innovative solution, AbdulRehman not only refines his technical prowess but also discovers the joy of turning a practical challenge into a fulfilling endeavor.


# Rust Inventory Handling System

## Overview

Welcome to the Rust Inventory Handling System! This project, developed by AbdulRehman, is designed to streamline inventory management through the power of Rust. Manage your products, generate insightful reports, and ensure the security of your data with this intuitive console-based system.

## Installation

Follow these steps to set up the Rust Inventory Handling System on your local machine:

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/AbdulRehman/Rust_Inventory
   cd Rust_Inventory
   ```

2. **Install Rust:**
   Ensure that Rust is installed on your system. If not, follow the instructions at [Rust Installation](https://www.rust-lang.org/tools/install).

3. **Build the Project:**
   ```bash
   cargo build
   ```

4. **Run the Project:**
   ```bash
   cargo run
   ```

5. **Explore and Enjoy:**
   Navigate through the console interface to manage your inventory efficiently. Have fun exploring the features and functionalities!

### NOTE
In order to add admins,passwords within the access control, you need to follow the example below : 
```cpp
managers.insert("alice".to_string(), Manager{username: "alice".to_string(), password: "1234".to_string()});
```

In the Code developed, Default admin is "alice" and password is "1234"


## Contribution

Contributions are welcome! If you find any issues or have ideas for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute the code as per the terms of the license.

Happy inventory management with Rust! 🚀






