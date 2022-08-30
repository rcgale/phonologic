import Features from "./Features.js";
import TranscriptFormatted from "./TranscriptFormatted.js";

export default {
    props: ["steps", "detailHoverIndex", "alphabet"],
    components: { Features, TranscriptFormatted, },
    methods: {
        stepFeatureCost(step) {
            return `${this.costFormatted(step.cost)} / 24`
        },
        costFormatted(cost) {
            let rounded = Math.round(100 * cost, 2) / 100
            return `${rounded}`
        },

    },
    template: `
        <div id="detail" class="main-pane">
            <table class="feature-steps">
                <thead>
                    <th>Action</th>
                    <th>Cost</th>
                    <th>From</th>
                    <th>To</th>
                    <th>Features</th>
                </thead>
                <tbody>
                    <tr v-for="step, n in steps"
                        @mouseover="detailHoverIndex = n"
                        @mouseleave="detailHoverIndex = null"
                        :class="['action-' + step.action.toLowerCase()]"
                    >
                    <td>{{step.action}}</td>
                    <td>{{stepFeatureCost(step)}}</td>
                    <td>
                        <span v-if="step.expected">
                            <TranscriptFormatted :transcript="[step.expected]" :alphabet="alphabet" />
                        </span>
                    </td>
                    <td>
                        <span v-if="step.actual">
                            <TranscriptFormatted :transcript="[step.actual]" :alphabet="alphabet" />
                        </span>
                    </td>
                    <td>
                        <ul v-if="step.deltas.length" class="feature-collection with-brackets">
                            <li v-for="delta in step.deltas">
                                <Features :delta="delta" />
                            </li>
                        </ul>
                    </td>
                </tr>

                </tbody>
            </table>
        </div>`
}