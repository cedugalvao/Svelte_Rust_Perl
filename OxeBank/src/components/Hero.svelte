<script>
	import Button from './Button.svelte';
</script>

<section class="h-[675px] bg-hero bg-cover w-full justify-center flex">
	<div class="pt-[87px] max-w-[1216px] justify-between flex w-full">
		<div class="max-w-[560px]">
			<p class="text-[#429EBD] font-semibold text-2xl flex">
				Que 2024 seja radiante e feliz! <span class="pl-4">
					<img src="/assets/iconFesta.svg" alt="Icone de festa" />
				</span>
			</p>
			<h1 class="text-white text-[54px] font-semibold leading-[65px] mb-[46px] pt-[32px]">
				Inicie o ano com a conta digital <span class="text-[#F7AD19]">perfeita</span> para
				<span class="text-[#F7AD19]">seu dinheiro</span>
			</h1>
			<div class="flex gap-[24px]">
				<Button
					classList={'!bg-[#F7AD19] !text-[#033650] text-base'}
					textButton={'Acessar conta digital'}
					rota={'/login'}
				/>
			</div>
			<div class="w-full flex justify-between mt-[64px]">
				<div class="w-full max-w-[148px] flex justify-between">
					<div class="w-[48px]">
						<img src="/assets/elipseHero.svg" alt="hero elipse" class="absolute" />
						<img
							src="/assets/cardIconHero.svg"
							alt="hero elipse"
							class="relative top-[15px] left-[12px]"
						/>
					</div>
					<p class="text-white text-base ml-8">Cartão sem anuidade</p>
				</div>
				<div class="w-full max-w-[148px] flex justify-between">
					<div class="w-[48px]">
						<img src="/assets/elipseHero.svg" alt="hero elipse" class="absolute" />
						<img
							src="/assets/zeroIconHero.svg"
							alt="hero elipse"
							class="relative top-[12px] left-[16px]"
						/>
					</div>
					<p class="text-white text-base ml-8">Conta digital 100% grátis</p>
				</div>
				<div class="w-full max-w-[155px] flex justify-between">
					<div class="w-[48px]">
						<img src="/assets/elipseHero.svg" alt="hero elipse" class="absolute" />
						<img
							src="/assets/moneyIconHero.svg"
							alt="hero elipse"
							class="relative top-[14px] left-[18px]"
						/>
					</div>
					<p class="text-white text-base ml-8">Seu dinheiro rendendo mais</p>
				</div>
			</div>
		</div>
		<div>
			<h1 class=" text-[#3687A2] text-[48px] max-w-[500px] font-semibold w-full absolute">
				Banco 100% digital
			</h1>
			<div class="relative">
				<img src="/assets/circleHero.svg" alt="hero phone" class="mt-[90px]" />
				<img
					src="/assets/cardBackHero.svg"
					alt="hero phone"
					class="absolute bottom-0 right-[37px]"
				/>
				<img
					src="/assets/cardFrontHero.svg"
					alt="hero phone"
					class="absolute top-[29px] left-[-88px]"
				/>
			</div>
		</div>
	</div>
</section>
