<script>
	import Button from '../Button.svelte';
	import { Router, navigate } from 'svelte-navigator';
</script>

<div class="bg-[#F7AD19] w-full h-[400px] flex flex-col items-center justify-center">
	<div class="max-w-[1216px] w-full">
		<div>
			<h1 class="text-[80px] font-semibold text-white text-left">Bem vindo!</h1>
			<h3 class="font-semibold text-white text-[21px]">
				Ter você como cliente é o nosso maior prazer!
			</h3>
			<!-- <Button
                textButton={'Abra sua conta digital'}
                classList={'bg-white !text-black mt-6'}
                rota={'/login'}
            /> -->
		</div>

	</div>
</div>
