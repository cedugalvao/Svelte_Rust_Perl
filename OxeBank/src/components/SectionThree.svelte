<script>
	import CardTwitter from './CardTwitter.svelte';
	import { register } from 'swiper/element/bundle';
	register();
</script>

<div class="flex flex-col w-full justify-center items-center">
	<div class=" max-w-[1216px] py-[134px] w-full">
		<h2 class="text-neutral-300 text-[40px] font-bold leading-[44.80px]">
			Prático, fácil, moderno
		</h2>
		<h2
			class="leading-[44.80px] pt-3 flex items-center gap-3 text-neutral-700 text-[40px] font-bold"
		>
			Você resolve tudo sem estresse <span><img src="/assets/emogi.svg" alt="" /></span>
		</h2>
		<swiper-container slides-per-view="3.2" space-between="30" class="mySwiper pt-10" loop="false">
			<swiper-slide
				><CardTwitter
					user={'@vg_garcia'}
					text={'Valeu @OxeBank! Consegui o cartão e finalmente paguei o agiota! Agora posso viver em paz'}
				/></swiper-slide
			>
			<swiper-slide
				><CardTwitter
					user={'@viniz1nDoGrau'}
					text={'Eu uso o @OxeBank pra render o meu dinheirinho enquanto durmo, foco é viver de renda passiva'}
				/></swiper-slide
			>
			<swiper-slide
				><CardTwitter
					user={'@iLoveEsquisitas'}
					text={'A interface o app @OxeBank é muito intuitiva, não tem como errar, até eu que sou lerdo consegui usar'}
				/></swiper-slide
			>
			<swiper-slide
				><CardTwitter
					user={'@iBotaNoTigre'}
					text={'O @OxeBank mudou minha vida, agora consigo apostar no tigrinho com o cartão'}
				/></swiper-slide
			>
			<swiper-slide
				><CardTwitter
					user={'@ApenasEuMesmo'}
					text={'O @OxeBank é o melhor banco do mundo, não tem como competir com ele'}
				/></swiper-slide
			>
		</swiper-container>
	</div>
</div>
