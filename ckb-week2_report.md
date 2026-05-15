CKB Learning Report — Week 2

Introduction
This week focused on building practical decentralized applications (dApps) on Nervos CKB. The goal was to move from foundational concepts into hands-on development, working with real examples such as token transfer, on-chain data storage, and DOB (Digital Object) creation. I also explored serialization in CKB, with emphasis on Molecule schema language and Rust integration.

Topics Covered

1. Simple Transfer dApp
Worked on a basic dApp that demonstrates how value is transferred between cells in CKB using scripts.

Key Learnings
- Understanding transaction structure in CKB dApps
- How inputs and outputs represent state transitions
- Role of lock scripts in authorizing transfers
- Basic interaction between frontend logic and CKB cells

Resource
https://github.com/beebozy/CKBuilder/tree/main/docs.nervos.org/examples/dApp/simple-transfer

Screenshot:
![Simple Transfer Screenshot](./images/transfer.png)

---

2. Store Data on Cell dApp
Explored how arbitrary data can be stored inside CKB cells.

Key Learnings
- Cells can store custom data beyond simple value transfers
- How data is embedded into output cells
- Relationship between type scripts and stored state
- On-chain data persistence model in CKB

Resource
https://github.com/beebozy/CKBuilder/tree/main/docs.nervos.org/examples/dApp/store-data-on-cell

Screenshot:

![Spore Data Screenshot](./images/storeData.png)
---

3. Create DOB (Digital Object) dApp
Worked on creating Digital Objects (DOBs) as on-chain programmable assets.

Key Learnings
- Concept of Digital Objects in CKB ecosystem
- Structuring DOB metadata on-chain
- Using scripts to define object behavior
- Relationship between identity and state representation in CKB

Resource
https://github.com/beebozy/CKBuilder/tree/main/docs.nervos.org/examples/dApp/create-dob

Screenshot:

![Dob Screenshot](./images/dob.png)
---

4. Serialization in CKB (Molecule)
Studied how data is serialized and deserialized in CKB using Molecule.

Key Learnings
- Molecule is a schema-based serialization language used in CKB
- Ensures deterministic binary encoding for blockchain consistency
- Defines strict data structures for contracts and transactions
- Prevents ambiguity in cross-platform data interpretation
- Used heavily in CKB script communication and state encoding

Resource
https://docs.nervos.org/docs/serialization/serialization-molecule-in-ckb

---

5. JsLibraries and CKB Development
Explored Js ecosystem features used in building CKB scripts and dApps.

Key Learnings
- Understood the use of Js library in writing script 
- Learned how transactions are constructed in the frontend, including defining inputs, outputs, and witness data using JavaScript objects.
- Understood how dApp state updates are represented through CKB cells and how JavaScript logic maps user actions to on-chain state changes.
---

6. Molecule Schema Language
Studied Molecule schema definition and its role in CKB development.

Key Learnings
- Schema defines data structure before compilation
- Ensures compatibility across CKB components
- Used to define inputs, outputs, witnesses, and custom types

Resource
https://docs.nervos.org/docs/serialization/serialization-molecule-in-ckb

---

Practical Development Work
dApp Implementation Practice
Worked through multiple example repositories to understand full-stack CKB dApp development.

Activities Completed
- Cloned and explored simple-transfer dApp
- Cloned and explored store-data-on-cell dApp
- Cloned and explored create-dob dApp
- Understood frontend-backend interaction in CKB dApps
- Tested transaction flows in simple-transfer dApp
- Practiced storing structured data on-chain
- Experimented with DOB creation workflow


Repositories
- Simple Transfer dApp:
  https://github.com/beebozy/CKBuilder/tree/main/docs.nervos.org/examples/dApp/simple-transfer

- Store Data on Cell dApp:
  https://github.com/beebozy/CKBuilder/tree/main/docs.nervos.org/examples/dApp/store-data-on-cell

- Create DOB dApp:
  https://github.com/beebozy/CKBuilder/tree/main/docs.nervos.org/examples/dApp/create-dob

---

Challenges Encountered
- Understanding Molecule schema structure and generation flow

- Learning how DOB structure maps to cell data

---

Key Takeaways
By the end of Week 2, I gained:

- Practical understanding of how CKB dApps are built
- Stronger grasp of Cell-based state management
- Understanding of Molecule serialization system
- Exposure to DOB (Digital Object) design patterns
