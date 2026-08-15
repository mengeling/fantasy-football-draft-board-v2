<script>
    import '../app.css';
</script>

<style>
    @media (max-width: 700px) {
        .footer {
            font-size: 0.75rem;
            margin: 0.5rem 0;
        }
    }
</style>

<div class="app">
    <slot />
    <footer>
        <p class="footer">Made by 
            <a href="https://www.linkedin.com/in/mengeling/" target="_blank">Michael Engeling</a>
        </p>
    </footer>
</div>
