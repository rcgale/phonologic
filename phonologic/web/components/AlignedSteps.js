import TranscriptFormatted from "./TranscriptFormatted.js";

export default {
    props: ["steps", "alphabet", "detailHoverIndex"],
    components: {TranscriptFormatted},
    template: `
        <div class="transcript-steps-wrapper">
          <div class="transcript-steps" :style="{gridTemplateColumns: 'repeat('+steps.length+', auto)'}">
            <div class="expected">
                <span v-for="step, n in steps"
                      class="step-expected"
                      :class="{ highlight: detailHoverIndex == n, 'step-error': step.cost > 0}">
                    <TranscriptFormatted :transcript="step.expected" :alphabet="alphabet" />
                </span>
            </div>
            <div class="actual">
                <span v-for="step, n in steps"
                      class="step-actual"
                      :class="{ highlight: detailHoverIndex == n, 'step-error': step.cost > 0}">
                        <TranscriptFormatted :transcript="step.actual" :alphabet="alphabet" />
                </span>
            </div>
          </div>
        </div>`
};