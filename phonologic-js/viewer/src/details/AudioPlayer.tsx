// @ts-ignore
import {defineComponent} from "vue";

export default defineComponent({
    props: ["utteranceId"],
    methods: {
        src(utteranceId: string) {
            return `/audio/${utteranceId}.wav`;
        },
    },
    mounted: function () {
        // @ts-ignore
        this.$watch('utteranceId', () => {
            // @ts-ignore
            this.$refs.player.load()
        })
    },
    template: `
        <audio ref="player" controls v-if="utteranceId">
            <source :src="src(utteranceId)" type="audio/wav" controls>
            Your browser does not support the audio element.
        </audio>`
})