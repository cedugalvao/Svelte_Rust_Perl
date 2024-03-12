<script>
	import Button from './Button.svelte';
	import { Router, navigate } from 'svelte-navigator';

	function scrollToProdutos() {
	  const produtosSection = document.getElementById('produtos-section');
	  if (produtosSection) {
		produtosSection.scrollIntoView({ behavior: 'smooth', block: 'start' });
	  }
	}

	function scrollToAjuda() {
	  const ajudaSection = document.getElementById('footer-section');
	  if (ajudaSection) {
		ajudaSection.scrollIntoView({ behavior: 'smooth', block: 'start' });
	  }
	}

</script>

<div
	class="w-full sticky top-0 z-[9999] h-[88px] justify-center flex bg-hero bg-cover border-b-gray-50 border-b border-opacity-30"
>
	<div class="max-w-[1216px] flex justify-between w-full">
		<a href="/" class="h-full flex">
			<img src="/assets/logoWhite.svg" alt="logo arkademy"  class="w-[180px]" />
		</a>
		<ul class="flex gap-[40px] items-center">
			<li>
				<!-- <a href="/" class="text-white">Produtos</a> -->
				<button on:click="{scrollToProdutos}" class="text-white">Produtos</button>
			</li>
			<li>
				<a href="/quem-somos" class="text-white">Quem somos</a>
			</li>
			<li>
				<!-- <a href="/" class="text-white">Ajuda</a> -->
				<button on:click="{scrollToAjuda}" class="text-white">Ajuda</button>
			</li>
			<li>
				<Button textButton={'Acessar conta digital'} classList={''} rota={'/login'} />
			</li>
		</ul>
	</div>
</div>
