<script>
  import { page } from '$app/stores';4
  import { onMount, onDestroy, tick } from 'svelte';
  import Chart from 'chart.js/auto';

  const API_KEY = import.meta.env.VITE_FMP_API_KEY; 
  
  const stockSymbol = $page.params.symbol.toUpperCase();
  let loading = $state(true);
  let errorMessage = $state('');
  
  let canvasElement;
  let chartInstance;

  let currentPrice = $state(0);
  let rateOfChange = $state(0);
  let trendDirection = $state('neutral');
  let activeRange = $state('1Y');

  let chartData = {
    labels: [],
    datasets: [{
        label: `Precio de Cierre (${stockSymbol})`,
        data: [],
        borderColor: '#007acc',
        backgroundColor: 'rgba(0, 122, 204, 0.1)',
        tension: 0.1,
        fill: true,
      },],
  };
  const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
  };
  
  function createChart() {
    if (chartInstance) chartInstance.destroy();
    const ctx = canvasElement.getContext('2d');
    chartInstance = new Chart(ctx, {
      type: 'line', data: chartData, options: chartOptions
    });
  }
  
  onMount(() => {
    fetchStockData(activeRange);
  });
  
  onDestroy(() => {
    if (chartInstance) chartInstance.destroy();
  });

  function setTimeRange(range) {
    activeRange = range;
    fetchStockData(range);
  }

  async function fetchStockData(range) {
    loading = true;
    errorMessage = '';
    
    let url;
    const baseUrl = `https://financialmodelingprep.com/api/v3/historical-price-full/${stockSymbol}?apikey=${API_KEY}`;

    if (range === 'ALL') {
      url = baseUrl;
    } else {
      const today = new Date();
      const yearsToGoBack = parseInt(range.replace('Y', ''));
      const fromDate = new Date(new Date().setFullYear(today.getFullYear() - yearsToGoBack));
      
      const toDateStr = today.toISOString().split('T')[0];
      const fromDateStr = fromDate.toISOString().split('T')[0];
      
      url = `${baseUrl}&from=${fromDateStr}&to=${toDateStr}`;
    }

    try {
      if (API_KEY === 'miau') {
          throw new Error('Por favor, introduce tu clave de API de FMP.');
      }
      
      const response = await fetch(url);
      if (!response.ok) throw new Error(`Error de red al cargar el gráfico.`);
      
      const data = await response.json();
      const timeSeries = data.historical;
      if (!timeSeries || timeSeries.length === 0) {
        throw new Error('No se pudieron cargar los datos para el gráfico.');
      }

      const labels = timeSeries.map(d => d.date).reverse();
      const prices = timeSeries.map(d => d.close).reverse();

      currentPrice = prices[prices.length - 1];
      if (prices.length >= 2) {
        const previousPrice = prices[prices.length - 2];
        const change = ((currentPrice - previousPrice) / previousPrice) * 100;
        rateOfChange = change;
        if (change > 0.1) trendDirection = 'positive';
        else if (change < -0.1) trendDirection = 'negative';
        else trendDirection = 'neutral';
      }

      loading = false;
      chartData.labels = labels;
      chartData.datasets[0].data = prices;
      await tick(); 
      createChart();

    } catch (error) {
      errorMessage = error.message;
      loading = false;
    }
  }
</script>

<div class="app-container">
  <header class="controls">
    <h1 class="title">Análisis de {stockSymbol}</h1>
    <a href="/" class="back-button">← Volver</a>
  </header>

  <main class="main-content">
    <div class="chart-container">
      {#if loading}
        <div class="placeholder">Cargando datos...</div>
      {:else if errorMessage}
        <div class="placeholder error">{errorMessage}</div>
      {:else}
        <div class="chart-wrapper">
          <canvas bind:this={canvasElement}></canvas>
        </div>
      {/if}
    </div>

    <aside class="info-panel">
      <h2>Análisis de Tendencia</h2>
      
      <div class="info-card">
        <h3>Tasa de Cambio (Aplicación de derivadas)</h3>
        <p class="data-value" class:positive={trendDirection === 'positive'} class:negative={trendDirection === 'negative'}>
          {rateOfChange.toFixed(2)}%
        </p>
        {#if trendDirection === 'positive'}
          <span>Indica una tendencia de crecimiento.</span>
        {:else if trendDirection === 'negative'}
          <span>Indica una tendencia de caída.</span>
        {:else}
          <span>Indica una tendencia estable.</span>
        {/if}
      </div>

      <div class="info-card">
        <h3>Precio Actual</h3>
        <p class="data-value">${currentPrice ? currentPrice.toFixed(2) : '0.00'}</p>
      </div>

      <div class="info-card">
        <h3>Período</h3>
        <div class="time-range-selector">
          <button on:click={() => setTimeRange('1Y')} class:active={activeRange === '1Y'}>1A</button>
          <button on:click={() => setTimeRange('2Y')} class:active={activeRange === '2Y'}>2A</button>
          <button on:click={() => setTimeRange('5Y')} class:active={activeRange === '5Y'}>5A</button>
          <button on:click={() => setTimeRange('ALL')} class:active={activeRange === 'ALL'}>Todo</button>
        </div>
      </div>
    </aside>
  </main>
</div>

<style>
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
  .main-content {
    display: flex;
    flex: 1;
    padding: 2rem;
    gap: 2rem;
    overflow: hidden; 
  }
  .chart-container {
    flex: 3;
    background-color: #2a2a2a;
    border-radius: 8px;
    padding: 1.5rem;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .chart-wrapper {
    position: relative;
    height: 100%;
    width: 100%;
  }
  .placeholder {
    display: grid;
    place-content: center;
    height: 100%;
    color: #777;
  }
  .placeholder.error {
    color: #f44336;
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
  .time-range-selector {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }
  .time-range-selector button {
    background-color: transparent;
    border: 1px solid transparent;
    color: #aaa;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }
  .time-range-selector button:hover {
    color: #fff;
  }
  .time-range-selector button.active {
    background-color: #007acc;
    color: #fff;
  }
</style>