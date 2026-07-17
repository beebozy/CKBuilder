  CKB Learning Report — Week 10
  
  Introduction

  This week focused on improving the browser readiness of the
  Dilithium wallet stack and strengthening the connection between
  the SDK and the user interface. The main goal was to make the SDK
  safer and more practical for frontend use by removing
  Node-specific assumptions and making key functionality compatible
  with browser environments. In parallel, I refined parts of the
  UI so it better reflects real wallet behavior and prepares the
  project for deeper interaction flows.

  Overall, the work this week was about frontend integration 
  readiness. Rather than only building isolated components, I
  focused on ensuring that the SDK, WASM layer, and UI can work
  together in a way that supports an actual browser-based wallet
  experience.

  Topics Covered
  
  1. Making the SDK Browser-Safe

  A major focus this week was adapting the TypeScript SDK so it can
  be used more reliably in browser-based environments.

  Key Learnings

  - SDK code that works in Node.js does not automatically work in
  the browser, especially when it depends on Node-specific modules
  such as filesystem or crypto APIs.
  - Browser compatibility requires careful use of Web APIs such as
  crypto.subtle and browser-friendly asset loading strategies.
  - Supporting both Node and browser environments means thinking
  clearly about runtime detection and fallback behavior.

  Understanding Gained

  This work deepened my understanding of cross-environment
  JavaScript/TypeScript development. I saw more clearly that
  cryptographic and packaging logic must be designed with the
  target runtime in mind. A wallet SDK is not only about
  implementing features correctly — it also needs to load safely
  and predictably in the environment where users will actually
  interact with it. This week helped me understand how browser
  support is not a small add-on, but an important architectural
  concern.

  2. Improving WASM and Asset Loading for Frontend Use
  
  Another important area of work was ensuring that the WASM module
  used by the SDK can be loaded correctly in browser-oriented
  workflows.

  Key Learnings

  - WASM integration in frontend projects depends not just on the
  module itself, but also on how build tools and package exports
  expose the compiled asset.
  - A package intended for frontend use must clearly define how its
  WASM files and compiled outputs are made available.
  - Runtime logic should distinguish between Node-based file access
  and browser-based fetch behavior to avoid failures during UI
  integration.

  Understanding Gained

  This part of the work gave me a more practical understanding of
  the “bridge layer” between low-level cryptographic functionality
  and actual application usage. Even if the underlying WASM logic
  is correct, poor asset packaging or loading behavior can prevent
  the wallet from functioning in the browser. I learned that
  integration work often depends on solving small but critical
  packaging and runtime issues that sit between core logic and user
  experience.


  3. Refining the Wallet UI for Real Integration
  
  This week also included small but meaningful UI refinement work
  to better prepare the wallet interface for real usage.

  Key Learnings

  - UI polish matters because it helps the interface reflect actual
  wallet state more clearly instead of relying on placeholder
  behavior.
  - Even a small change in display logic can improve how users
  interpret whether wallet data is available or not.
  - Frontend development becomes more meaningful when it is tied
  closely to how the SDK and wallet state will behave in real
  interaction flows.

  Understanding Gained

  This helped me think more carefully about state presentation in
  wallet design. A wallet UI should not only display data; it
  should also clearly communicate when data is unavailable,
  uninitialized, or waiting to be generated. That distinction
  becomes especially important when moving from scaffolded screens
  toward a more realistic interface. This week reinforced the idea
  that usability grows through both major integrations and small
  interface decisions.


  4. Connecting SDK Packaging with Product Development
  
  This week also showed how package configuration, browser
  compatibility, and UI readiness all contribute directly to
  product development.

  Key Learnings

  - Package metadata and exports are part of application design
  because they determine how other parts of the project can consume
  the SDK.
  - Browser-safe SDK design makes it easier to move from technical
  proof-of-concept to a usable wallet application.
  - Product progress often depends on infrastructure-level cleanup,
  not just visible features.

  Understanding Gained

  I gained a stronger appreciation for how “invisible” engineering
  work supports visible product growth. Updating exports, browser
  behavior, and WASM loading may not look as dramatic as adding a
  new wallet feature, but those improvements are what allow the
  interface and SDK to function together properly. This week made
  it clear that packaging and compatibility work are essential
  parts of building a complete blockchain application.

  Practical Development Work

  Browser-Safe SDK Development

  Activities Completed

  - Updated SDK functionality to remove Node-only assumptions in
  browser-facing flows.
  - Replaced Node-based hashing behavior with browser-compatible
  Web Crypto for script identifier generation.
  - Improved WASM loading logic so the SDK can distinguish between
  Node and browser environments more safely.
  - Updated SDK package configuration and exports to better support
  frontend integration and bundled WASM usage.
  - Regenerated SDK distribution artifacts to reflect the latest
  browser-safe changes.

  Status

  ┌───────────────────────────────────────┬───────────┐
  │               Component               │  Status   │
  ├───────────────────────────────────────┼───────────┤
  │ Browser-safe hashing update           │ Completed │
  ├───────────────────────────────────────┼───────────┤
  │ Runtime environment handling          │ Completed │
  ├───────────────────────────────────────┼───────────┤
  │ WASM loading improvement              │ Completed │
  ├───────────────────────────────────────┼───────────┤
  │ SDK package export refinement         │ Completed │
  └───────────────────────────────────────┴───────────┘


  UI Development
  
  Activities Completed

  - Refined the wallet UI so address display behavior is cleaner
  and more aligned with real wallet state.
  - Continued preparing the interface for live SDK-backed wallet
  interactions rather than placeholder-only flows.
  - Improved the project’s readiness for browser-based wallet
  integration by aligning UI behavior with SDK changes.

  Status

  ┌────────────────────────────┬───────────┐
  │            Task            │  Status   │
  ├────────────────────────────┼───────────┤
  │ UI refinement              │ Completed │
  ├────────────────────────────┼───────────┤
  │ Address display cleanup    │ Completed │
  ├────────────────────────────┼───────────┤
  │ SDK/UI integration prep    │ Completed │
  └────────────────────────────┴───────────┘

  ---
  Next Steps
  
  - Continue integrating the browser-safe SDK directly into the
  wallet UI for real key generation and signing flows.
  - Test the WASM-backed SDK behavior more thoroughly in frontend
  runtime conditions.
  - Expand the UI from basic structure and display refinements into
  more interactive wallet actions.
  - Verify that browser compatibility improvements remain stable
  across packaging and build workflows.
  - Continue moving the project from a CLI-oriented workflow toward
  a complete browser-based wallet experience.
