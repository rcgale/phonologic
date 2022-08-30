export default {
    props: ["delta"],
    methods: {
        value(value) {
            switch (value) {
                case -1:
                    return `–`;
                case +1:
                    return `+`;
                case -0.5:
                    return `–+`;
                case +0.5:
                    return `+–`;
                default:
                    return `${value}`;
            }
        },
        cost(delta) {
            let name = (delta.left || delta.right).replace(/[+\-0]/, '')
            return `Cost for [${name}]: ${delta.cost}`
        }
    },
    template: `
        <span :title="cost(delta)">
            <span v-if="delta.left && delta.right">
                {{delta.left}} &rarr; {{delta.right}} 
            </span>
            <span v-if="delta.left && !delta.right">
                {{delta.left}}
            </span>
            <span v-if="!delta.left && delta.right">
                {{delta.right}}
            </span>
        </span>`

}