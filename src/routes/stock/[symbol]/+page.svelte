<script>
  import { page } from '$app/stores';

  const stockSymbol = $page.params.symbol.toUpperCase();
  let stockData = {};
  let derivativeData = {};

  async function fetchStockData() {
    console.log(`Buscando datos para el símbolo de la URL: ${stockSymbol}`);
  }
</script>

<div class="app-container">
  <header class="controls">
    <h1 class="title">Análisis de {stockSymbol}</h1>
    <a href="/" class="back-button">← Volver</a>
  </header>

  <main class="main-content">
    <div class="chart-container">
      <div class="chart-placeholder">
        <p>El gráfico de precios de {stockSymbol} aparecerá aquí.</p>
      </div>
    </div>

    <aside class="info-panel">
      <h2>Análisis de Tendencia</h2>
      <div class="info-card">
        <h3>Tasa de Cambio (Derivada)</h3>
        <p class="data-value positive">+1.25%/día</p>
        <span> Indica una tendencia de crecimiento fuerte. </span>
      </div>
       <div class="info-card">
        <h3>Precio Actual</h3>
        <p class="data-value">$195.45</p>
      </div>
    </aside>
  </main>
</div>

<style>
  .back-button {
    background-color: #444;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    text-decoration: none;
    transition: background-color 0.2s;
  }
  .back-button:hover {
    background-color: #555;
  }

  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    background-color: #1e1e1e;
    color: #e0e0e0;
    overflow: hidden;
  }

  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background-color: #2a2a2a;
    border-bottom: 1px solid #444;
  }
  
  .title {
    font-size: 1.5rem;
    margin: 0;
  }

  .main-content {
    display: flex;
    flex: 1;
    padding: 2rem;
    gap: 2rem;
  }

  .chart-container {
    flex: 3; 
    background-color: #2a2a2a;
    border-radius: 8px;
    padding: 1.5rem;
  }

  .chart-placeholder {
    display: grid;
    place-content: center;
    height: 100%;
    border: 2px dashed #444;
    border-radius: 6px;
    color: #777;
  }

  .info-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .info-card {
    background-color: #2a2a2a;
    border-radius: 8px;
    padding: 1.5rem;
  }

  .info-card h3 {
    margin-top: 0;
    font-size: 1rem;
    color: #aaa;
  }

  .data-value {
    font-size: 2rem;
    font-weight: bold;
    margin: 0.5rem 0;
  }

  .data-value.positive {
    color: #4caf50;
  }

  .data-value.negative {
    color: #f44336;
  }
</style>
