<script>
  import "../app.css"
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';

  let isSearching = $state(false);
  let searchTerm = $state('');

  let showModal = $state(false);
  let modalMessage = $state('');

  const API_KEY = import.meta.env.VITE_FMP_API_KEY; 

  const popularStocks = [
    { symbol: 'AAPL', name: 'Apple Inc.' },
    { symbol: 'GOOGL', name: 'Alphabet Inc.' },
    { symbol: 'MSFT', name: 'Microsoft Corp.' },
    { symbol: 'TSLA', name: 'Tesla, Inc.' },
    { symbol: 'AMZN', name: 'Amazon.com, Inc.' },
  ];

  function closeModal() {
  showModal = false;
  isSearching = false;
}

async function search() {
  if (!searchTerm.trim()) return;
  isSearching = true;

  try {
    await invoke('validate_symbol', {
      symbol: searchTerm.trim(),
      apiKey: API_KEY
    });
    
    isSearching = false; 
    goto(`/stock/${searchTerm.trim().toUpperCase()}`);

  } catch (error) {
    modalMessage = 'Esa acción no existe. Intenta nuevamente con una existente.';
    showModal = true;
  }
}
</script>

{#if showModal}
  <div class="modal-backdrop" on:click={closeModal}>
    <div class="modal-content" on:click|stopPropagation>
      <h3>Error de Búsqueda</h3>
      <p>{modalMessage}</p>
      <button on:click={closeModal}>Cerrar</button>
    </div>
  </div>
{/if}

<div class="home-container">
  <div class="content">
    <h1 class="brand-title">BolsaBit 📈</h1>
    <p class="subtitle">Tu asistente de análisis de acciones</p>
    
    <form class="search-form" on:submit|preventDefault={search}>
      <input
        type="text"
        placeholder="Buscar un símbolo de acción (ej. AAPL)"
        bind:value={searchTerm}
        disabled={isSearching}
      />
      <button type="submit" disabled={isSearching}>
        {#if isSearching} Buscando... {:else} Buscar {/if}
      </button>
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
  .modal-backdrop {
    position: fixed; top: 0; left: 0; width: 100%; height: 100%;
    background-color: rgba(0, 0, 0, 0.6);
    display: flex; justify-content: center; align-items: center; z-index: 100;
  }
  .modal-content {
    background-color: #2a2a2a; padding: 2rem; border-radius: 8px;
    text-align: center; border: 1px solid #444;
    max-width: 400px;
    width: 90%;
  }
  .modal-content h3 { margin-top: 0; color: #ff3e00; }
  .modal-content button {
    margin-top: 1rem; background-color: #007acc; color: white; border: none;
    padding: 0.5rem 1.5rem; border-radius: 6px; cursor: pointer;
  }

  .home-container {
    display: flex; justify-content: center; align-items: center;
    text-align: center; height: 100vh; padding: 2rem;
  }
  .content { max-width: 600px; width: 100%; }
  .brand-title { font-size: 4rem; font-weight: 700; color: #007acc; margin: 0; }
  .subtitle { font-size: 1.25rem; color: #888; margin-bottom: 3rem; }
  .search-form { display: flex; margin-bottom: 4rem; }
  .search-form input {
    flex-grow: 1; padding: 1rem; font-size: 1rem; border: 1px solid #333;
    background-color: #222; color: #e0e0e0; border-radius: 8px 0 0 8px;
  }
  .search-form button {
    padding: 1rem 1.5rem; font-size: 1rem; border: none; background-color: #007acc;
    color: white; cursor: pointer; border-radius: 0 8px 8px 0; transition: background-color 0.2s;
  }
  .search-form button:disabled { background-color: #555; cursor: not-allowed; }
  .search-form button:hover:not(:disabled) { background-color: #005f9e; }
  .popular-stocks h2 {
    text-align: left; color: #aaa; font-size: 1rem; text-transform: uppercase;
    letter-spacing: 1px; border-bottom: 1px solid #333; padding-bottom: 0.5rem;
  }
  .stock-list { display: flex; flex-direction: column; gap: 0.5rem; }
  .stock-item {
    display: flex; justify-content: space-between; padding: 1rem; background-color: #222;
    border-radius: 6px; text-decoration: none; color: #e0e0e0; transition: background-color 0.2s;
  }
  .stock-item:hover { background-color: #333; }
  .stock-item .symbol { font-weight: bold; }
</style>