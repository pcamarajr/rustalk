<script lang="ts">
  import '../app.css';

  let name = 'RUSTALK';
</script>

<div class="container">
  <h1>Welcome to {name}</h1>
  <p class="description">AI-Developed VoIP Desktop Application</p>
  <div class="status">
    <span class="badge">Phase 1: Infrastructure Setup</span>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 2rem;
  }

  h1 {
    font-size: 3rem;
    font-weight: 700;
    margin: 0;
    color: #333;
  }

  .description {
    font-size: 1.25rem;
    color: #666;
    margin: 1rem 0 2rem;
  }

  .status {
    margin-top: 2rem;
  }

  .badge {
    background: #4f46e5;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
  }
</style>
