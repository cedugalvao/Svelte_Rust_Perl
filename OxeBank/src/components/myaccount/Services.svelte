<script>

	export let nome = '';
    export let imagem = '';
    export let rota = '';

</script>
<a href="{rota}" class="services">
  <div class="flex bg-[#F7AD19] h-[110px] w-[160px] rounded-2xl flex-col justify-between text-white p-4">
    <div class="services__icon">
        <img src={imagem} alt="Icone serviço" class="max-w-[30px]" />
      </div>
      <div class="services__text">
        <h3>{nome}</h3>
      </div>
  </div>
</a>