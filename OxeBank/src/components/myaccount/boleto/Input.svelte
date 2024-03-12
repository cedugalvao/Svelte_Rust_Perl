<script>
	import Button from '../../Button.svelte';
    import ButtonB from '../../ButtonB.svelte';
    var valorBoleto = '1348,44';
</script>

<div class="bg-white w-full h-[242px] flex flex-col items-center mt-24 mb-24">
	<div class="max-w-[1216px] w-full flex justify-center">
		<form class="form-boleto flex flex-col w-full max-w-[400px] !border-opacity-0">
            <img src="/assets/barcodeCenter.png" alt="" class="w-[400px] mb-8">
            <input type="number" placeholder="Digite o código de barras" class="w-full border border-gray-500 rounded-md p-2 input-boleto mb-6">
            <div class="w-full flex justify-between">
                <Button textButton='Retornar' rota='/myaccount' classList='!text-white !font-light !px-8 !bg-[#053F5C]'  />
                <ButtonB textButton='Prosseguir' classList='!text-black !px-8 !bg-[#F7AD19]'  />
            </div>
        </form>

        <!-- SE ESSE BOLETO EXISTIR, DEVE APARECER ESSE BLOCO ABAIXO -->

        <div class=" confirma-boleto flex-col w-full max-w-[400px] !border-opacity-0 hidden">
            <h2 class="text-center text-[22px]">
                O valor total desse boleto é de<br> R$ {valorBoleto}
            </h2>
            
            <div class="w-full flex justify-between mt-8">
                <Button textButton='Cancelar' rota='/myaccount' classList='!text-white !font-light !px-8 !bg-[#053F5C]'  />
                <ButtonB textButton='Efetuar pagamento' classList='!text-black !px-8 !bg-[#F7AD19]'  />
            </div>
        </div>
	</div>
</div>
