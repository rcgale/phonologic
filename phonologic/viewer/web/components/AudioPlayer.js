export default {
    props: ["utteranceId"],
    methods: {
        src(utteranceId) {
            return `/audio/${utteranceId}.wav`;
        },
    },
    mounted: function () {
        this.$watch('utteranceId', () => {
            this.$refs.player.load()
        })
    },
    template: `
        <audio ref="player" controls v-if="utteranceId">
            <source :src="src(utteranceId)" type="audio/wav" controls>
            Your browser does not support the audio element.
        </audio>`
}