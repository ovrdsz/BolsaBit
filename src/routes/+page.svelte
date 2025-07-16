<script>
  import { goto } from '$app/navigation';

  let searchTerm = '';

  const popularStocks = [
    { symbol: 'AAPL', name: 'Apple Inc.' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.' },
    { symbol: 'MSFT', name: 'Microsoft Corp.' },
    { symbol: 'TSLA', name: 'Tesla, Inc.' },
    { symbol: 'AMZN', name: 'Amazon.com, Inc.' },
  ];

  function search() {
    if (searchTerm.trim()) {
      goto(`/stock/${searchTerm.toUpperCase()}`);
    }
  }
</script>

<div class="home-container">
  <div class="content">
    <h1 class="brand-title">BolsaBit 📈</h1>
    <p class="subtitle">Tu asistente de análisis de acciones (que usa derivadas 🙂)</p>

    <form class="search-form" on:submit|preventDefault={search}>
      <input
        type="text"
        placeholder="Buscar un símbolo de acción (ej. AAPL)"
        bind:value={searchTerm}
      />
      <button type="submit">Buscar</button>
    </form>

    <div class="popular-stocks">
      <h2>Acciones Populares</h2>
      <div class="stock-list">
        {#each popularStocks as stock}
          <a href="/stock/{stock.symbol}" class="stock-item">
            <span class="symbol">{stock.symbol}</span>
            <span class="name">{stock.name}</span>
          </a>
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    background-color: #121212;
    color: #e0e0e0;
    overflow: hidden;
  }

  .home-container {
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
    height: 100vh;
    padding: 2rem;
  }

  .content {
    max-width: 600px;
    width: 100%;
  }

  .brand-title {
    font-size: 4rem;
    font-weight: 700;
    color: #007acc;
    margin: 0;
  }

  .subtitle {
    font-size: 1.25rem;
    color: #888;
    margin-bottom: 3rem;
  }

  .search-form {
    display: flex;
    margin-bottom: 4rem;
  }

  .search-form input {
    flex-grow: 1;
    padding: 1rem;
    font-size: 1rem;
    border: 1px solid #333;
    background-color: #222;
    color: #e0e0e0;
    border-radius: 8px 0 0 8px;
  }
  
  .search-form button {
    padding: 1rem 1.5rem;
    font-size: 1rem;
    border: none;
    background-color: #007acc;
    color: white;
    cursor: pointer;
    border-radius: 0 8px 8px 0;
    transition: background-color 0.2s;
  }

  .search-form button:hover {
    background-color: #005f9e;
  }
  
  .popular-stocks h2 {
    text-align: left;
    color: #aaa;
    font-size: 1rem;
    text-transform: uppercase;
    letter-spacing: 1px;
    border-bottom: 1px solid #333;
    padding-bottom: 0.5rem;
  }

  .stock-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .stock-item {
    display: flex;
    justify-content: space-between;
    padding: 1rem;
    background-color: #222;
    border-radius: 6px;
    text-decoration: none;
    color: #e0e0e0;
    transition: background-color 0.2s;
  }

  .stock-item:hover {
    background-color: #333;
  }
  
  .stock-item .symbol {
    font-weight: bold;
  }
</style>