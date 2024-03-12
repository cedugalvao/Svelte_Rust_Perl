<script>
	import Button from '../Button.svelte';
	import Services from './Services.svelte';
</script>

<div class="w-full bg-white flex justify-center">
	<div class="max-w-[1216px] w-full relative">
		<div
			class="container w-full bg-white text-left border rounded-lg shadow-lg absolute top-[-60px]"
		>
			<div class="px-[140px]">
				<h1 class="text-[48px] font-bold pt-8">Olá, Ghedyvan</h1>
				<p class="text-[22px] font-semibold pb-8">Esse é o status da sua conta hoje</p>
				<div class="flex justify-between">
					<div class="flex flex-col pb-10">
						<div class="flex flex-col justify-center items-left">
							<h2 class="text-[24px] font-semibold pb-1">Saldo</h2>
							<p class="text-[22px]">R$ 51.214,34</p>
						</div>
						<div class="flex flex-col justify-center items-left mt-6">
							<h2 class="text-[24px] font-semibold pb-1">Fatura atual</h2>
							<p class="text-[22px]">R$ 3.412,18</p>
						</div>
						<div class="flex flex-col justify-center items-left mt-6">
							<h2 class="text-[24px] font-semibold pb-1">Limite disponível</h2>
							<p class="text-[22px]">R$ 38.412,18</p>
						</div>
					</div>
					<div class="grid grid-cols-2 gap-x-20">
						<Services rota="/myaccount/boleto" nome="Boleto" imagem="/assets/myaccount/boleto.svg" />
						<Services rota="/myaccount/pix" nome="Pix" imagem="/assets/myaccount/moneySend.svg" />
						<Services
							rota="/myaccount/transferir"
							nome="Transferir"
							imagem="/assets/myaccount/moneySend2.svg"
						/>
						<Services rota="/myaccount/fatura" nome="Fatura" imagem="/assets/myaccount/cartao.svg" />
					</div>
				</div>
			</div>
		</div>
		<div class="pt-[470px] pb-[100px] text-justify">
			<h1 id="about-section" class="text-[48px] font-bold">
				Sobre o <span class="text-laranja">OxeBank</span>
			</h1>
			<p class="text-[22px] font-semibold pt-8">
				O OxeBank é uma plataforma digital que oferece serviços financeiros de forma simples e sem
				burocracia.
			</p>
			<p class="text-[16px] text-black pt-8">
				Em 2023 o OxeBank nasceu com um propósito: permitir o acesso das pessoas ao mundo digital.
				De lá para cá, trabalhamos duro e de forma genuína para tornar a experiência dos nossos
				clientes cada vez mais simples e intuitiva.
			</p>
			<p class="text-[16px] text-black pt-3">
				Hoje, somos uma plataforma digital completa, com serviços financeiros e não financeiros, que
				oferece uma experiência única e personalizada para cada cliente.
			</p>
			<p class="text-[16px] text-black pt-3">
				Além disso, em nosso app e site, disponibilizamos os produtos e serviços mais adequados para
				a sua realidade, planos e necessidades. Sem taxas, zero burocracia e sem letras miúdas.
				Prezamos pela total transparência com nossos clientes!
			</p>
		</div>
	</div>
</div>
