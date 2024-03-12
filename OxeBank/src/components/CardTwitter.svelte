<script>
    export let user;
    export let text;
</script>


<div class="bg-[#F1F0F5] max-w-[384px] rounded-[9px]">
	<div class="px-8 py-9">
        <div class="flex justify-between pb-5">
            <h3 class="text-zinc-600 text-lg font-semibold leading-tight ">{user}</h3>
            <img src="/assets/twitterLogo.svg" alt="" />
        </div>
        <div class="leading-[27px] text-lg text-gray-600">
            <p>{text}</p>
        </div>
    </div>
</div>
