---
  CKB Learning Report — Week 9

  Introduction

  This week focused on strengthening the project at two important layers:
  the on-chain contract and the user-facing interface. On the contract
  side, I worked on correcting a vulnerability related to witness
  handling, making the lock script safer and more aligned with the
  intended verification flow. On the application side, I scaffolded the UI
  for the Dilithium wallet, creating the foundation for moving from a
  CLI-focused workflow toward a more accessible interface. Altogether, the
  week was about improving both correctness and usability — tightening
  the core security logic while beginning the transition toward a fuller
  product experience.

  Topics Covered

  1. Correcting the Contract Vulnerability

  A major focus this week was fixing a vulnerability in the contract
  related to witness processing and validation.

  Key Learnings
  - Witness handling is one of the most security-sensitive parts of a lock
  script because it is the bridge between transaction data and signature
  verification.
  - A small weakness in how witness data is parsed or validated can
  undermine otherwise-correct cryptographic verification logic.
  - Fixing the contract required thinking carefully about how the pubkey
  and signature are extracted, checked, and matched against the expected
  lock arguments before verification proceeds.
  
  Understanding Gained

  This work reinforced how important defensive validation is in smart
  contract development. It is not enough for the contract to simply verify
  a signature — it must also ensure that every input leading into that
  verification step is correctly structured and trustworthy. This
  correction deepened my understanding of secure witness parsing and the
  importance of making contract logic fail safely when data is malformed
  or inconsistent.

  2. Scaffolding the UI

  The second major area of work this week was building the initial UI
  structure for the project.

  Key Learnings
  - Moving from a CLI-based workflow to a UI requires translating
  low-level wallet actions into flows that are clear and usable for an end
  user.
  - UI scaffolding is not just about layout; it also helps define how core
  wallet features such as key generation, address display, balance
  checks, and transfers will eventually be presented and connected.
  - Establishing the UI structure early makes it easier to plan future
  integration with the SDK and contract logic in a clean, modular way.
  
  Understanding Gained

  This part of the work helped me think about the project from a product
  perspective rather than only a protocol or contract perspective. A
  working backend or CLI proves functionality, but a UI begins to shape
  how a real user would experience the wallet. It highlighted the
  difference between “the system works” and “the system is usable,” which
  is an important distinction in practical blockchain application
  development.

  3. Connecting Security Work with Product Development

  This week also showed how contract-level correctness and frontend
  development are closely related, even though they happen at different
  layers of the stack.

  Key Learnings
  - Security fixes at the contract level directly affect the reliability
  of everything built on top of them, including wallet flows exposed in
  the UI.
  - UI work benefits from having stable and trustworthy backend/contract
  behavior, since it reduces uncertainty when integrating wallet actions
  later.
  - Building the product in layers — contract correctness first, then
  interface structure — creates a stronger foundation than trying to do
  both without first stabilizing the underlying logic.
  
  Understanding Gained

  I gained a better appreciation for sequencing in development. Fixing the
  witness-related contract issue before pushing further into UI
  integration was the right step, because it ensures that future
  user-facing features will sit on top of a safer and more reliable base.
  This week made the relationship between protocol security and
  application design much more concrete.

  Practical Development Work

  Dilithium Lock Contract

  Activities Completed
  - Identified and corrected a vulnerability related to witness handling
  in the contract. 
  - Improved the safety and correctness of the verification flow by
  tightening how witness data is interpreted before signature
  verification.
  - Confirmed that the contract logic now better reflects the intended
  lock behavior and security expectations.
  
  Status

  ┌───────────────────────────────┬───────────┐
  │           Component           │  Status   │
  ├───────────────────────────────┼───────────┤
  │ Witness handling correction   │ Completed │
  ├───────────────────────────────┼───────────┤
  │ Contract security improvement │ Completed │
  ├───────────────────────────────┼───────────┤
  │ Verification flow alignment   │ Completed │
  └───────────────────────────────┴───────────┘

  UI Development

  Activities Completed
  - Scaffolded the initial UI structure for the wallet project.
  - Began establishing the layout and flow for exposing wallet
  functionality through a graphical interface.
  - Created a foundation for future integration between the UI, SDK, and
  contract-backed wallet actions.
  
  Status

  ┌────────────────────────────┬─────────────┐
  │            Task            │   Status    │
  ├────────────────────────────┼─────────────┤
  │ UI scaffolding             │  
  ├────────────────────────────┼─────────────┤
  │ Wallet interface structure │ 
  ├────────────────────────────┼─────────────┤
  │ SDK/UI integration         │  
  └────────────────────────────┴─────────────┘

  Next Steps

  - Connect the UI scaffold to the wallet’s actual functionality such as
  key generation, address display, balance checks, and transfer flow.
  - Continue testing the contract changes to ensure the witness-handling
  correction holds under expected transaction scenarios.
  - Refine the UI so it moves from a structural prototype toward an
  interactive wallet interface.
  - Continue building toward a more complete integration between the
  contract, SDK, and frontend.

