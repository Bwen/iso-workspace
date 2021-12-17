<svelte:head>
    <title>RISO About Site</title>
    <meta name="description" content="A little outline of how the site was built and with what" />
</svelte:head>

<style lang="css">
</style>
<script lang="ts">
    import { Heading, Link } from 'svelte-bwen/components';
    import { faVuejs, faGithub, faDiscord } from '@fortawesome/free-brands-svg-icons';
    import { faLanguage } from '@fortawesome/free-solid-svg-icons';
    import { faGlobe } from '@fortawesome/pro-light-svg-icons';
    import { faCoins } from '@fortawesome/pro-duotone-svg-icons';
</script>

<div class="page">
    <Heading tag="h1" text="The Author" />
    <p>
        I'm a backend developer and I am a pretty private person... So, that's all you'll get about "The Author". 🤣
    </p>

    <Heading tag="h2" text="The Search for a frontend framework" />
    <p>
        When I sent out to make this demo site for my <Link href="https://github.com/Bwen/iso-workspace" icon={faGithub}>RISO Rust library</Link>
        my first reflex was to pick <Link href="https://vuejs.org" icon={faVuejs}>VueJS</Link>, because
        I already knew a little bit of it. And I was never really of a fan of AngularJS or ReactJS. I'm a strong
        believer of segregation and those frameworks make Javascript return HTML, that is just 🤮 to me.
        While <Link href="https://vuejs.org" icon={faVuejs}>VueJS</Link> as it neatly segregated in its components,
        it was really a no brainer.
    </p>
    <p>
        Then I searched for a component library and ended up picking
        <Link href="https://primefaces.org/primevue/showcase">PrimeVue</Link>. After spending time learning their components,
        they were well documented and easily integrated, I quickly arrived to something viable.
    </p>
    <img src="static/images/vuejs-old-site.png" />
    <p>
        Although it was functional and not to shabby I was still not satisfied. The components were bulky,
        did more than I really needed and some things I couldn't make them do. Sure, I could of tweaked it more
        and add little transition here and there. But I felt it was time to really search for the something
        more minimalist. So off I went on a quest to revamp my frontend skills!
    </p>

    <p>
        My research brought me to <Link href="https://kit.svelte.dev/">SvelteKit</Link>. When I first started learning it, it seemed very VueJS like.
        But it is so much more simplified! I could just create folders and <i>.svelte</i> files with no <i>route.js</i> to modify.
        It feels like the good old days again. 😍 The environment uses <Link href="https://vitejs.dev/">Vite</Link> to package & do dev hot reload,
        no more webpack! Although this site is pretty static, <Link href="https://kit.svelte.dev/">SvelteKit</Link> is SSR first which I didnt really
        need for this project but I am looking forward to using its full potential in future projects. 👍
    </p>
    <p>
        This time I decided not to look for a component library, and decided to do my own components library. I know what you are thinking:
        <i>Is he crazy? This will take forever!</i> Keep in mind that we are not talking about a 100 components library here,
        just small efficient components that I can bend to my will. So it was not so bad, and I learned a lot from doing so.
    </p>
    <p>
        Not picking a components library implies not having any base CSS that comes with it... 😱
        Ohh... My goodness. I spent quite some time searching for a CSS library, I kept seeing TailWindCSS on the
        <Link href="https://discordapp.com/channels/457912077277855764/506988048375087114" icon={faDiscord}>Svelte Discord</Link>
        and I though I would try this so called "Semantic" CSS framework. After trying it, I can
        clearly say <b>this is not for me, and goes against most of what I believe in</b>. And I will leave it at that.
    </p>
    <p>
        I came across other CSS frameworks such as Bootstrap, BulbaCSS, Materialize CSS, and eventually came across
        <Link href="https://github.com/csstools/sanitize.css" icon={faGithub}>Sanitize CSS</Link>! This was perfect,
        it only fixed cross browser bugs but did not add any look &amp; feel. And I though I would do the theming myself...
        OMG... Color wheel, Analogous, Monochromatic, Triad, Complementary... 😵 What kind of can of worms did I just open!
    </p>

    <Heading tag="h2" text="An Overwhelming feeling" />
    <p>
        While developing it all, I had this overwhelming feeling that it was too much and I should of just picked
        a known component library. Creating pages &amp; components I had this nagging reminder about how it all fits
        together and it kept slowing me down. So I came up with rules for myself:
    </p>
    <ol>
        <li>Components CSS is only for placement (<i>icon float left, position, etc...</i>)</li>
        <li>Theming is done in external CSS files (<i>not in pages or components</i>)</li>
    </ol>
    <p>
        With these two simple rules, it allowed me to focus primarily on component functionality. And eventually
        get to the theming part which is a complete different mindset in my opinion. Of course you might retouch
        your components to add a class or markup to make it easier to style, but not that much I found.
    </p>
    <p>
        Overall, I am pretty pleased with the result. Although the theming part definitely need to be redone, if you
        have any skills in it feel free to contact me. Might even be ready to pay for it. 🤔
    </p>
</div>
