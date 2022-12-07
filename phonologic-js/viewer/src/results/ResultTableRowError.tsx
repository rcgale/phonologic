// @ts-ignore
import {defineComponent} from "vue";

export default defineComponent({
    props: {
        analysisException: Object,
    },
    template: `
      <tr v-if="analysisException" class="analysis-error">
          <td class="column-utterance-id">
            <button>{{ analysisException.id }}</button>
          </td>
          <td class="column-transcript">
            {{analysisException.message}}
          </td>
          <td class="column-error-metric">
            &mdash;
          </td>
          <td class="column-error-counts">
            &mdash;
          </td>
          <td class="column-error-metric">
            &mdash;
          </td>
          <td class="column-error-counts">
            &mdash;
          </td>
      </tr>`
})