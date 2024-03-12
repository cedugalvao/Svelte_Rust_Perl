<script>
	import Button from '../Button.svelte';
	import { Router, navigate } from 'svelte-navigator';

	function scrollToAbout() {
	  const aboutSection = document.getElementById('about-section');
	  if (aboutSection) {
		aboutSection.scrollIntoView({ behavior: 'smooth', block: 'start' });
	  }
	}

	function scrollToAjuda() {
	  const footerSection = document.getElementById('footer-section');
	  if (footerSection) {
		footerSection.scrollIntoView({ behavior: 'smooth', block: 'start' });
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
				<button on:click="{scrollToAbout}" class="text-white">Quem somos</button>
			</li>
			<li>
				<!-- <a href="/" class="text-white">Ajuda</a> -->
				<button on:click="{scrollToAjuda}" class="text-white">Ajuda</button>
			</li>
			<li>
				<Button textButton={'Encerrar sessão'} classList={''} rota={'/'} />
			</li>
		</ul>
	</div>
</div>
