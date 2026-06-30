export const EXAMPLE_CSV_HEADER = "Utterance ID,Human transcript,ASR transcript\n"
export const EXAMPLE_CSV_BODY =
    "Participant01-laughing,L AE F IH N,B R AA P R IH NG\n" +
    "Participant01-house,EY HH AW S,HH AW S\n" +
    "Participant01-comb,K OW M,K OW M\n" +
    "Participant01-toothbrush,T UW TH B R AH SH,T UW B R AH SH\n";


export const EXAMPLE_FILE = new File([EXAMPLE_CSV_HEADER, EXAMPLE_CSV_BODY], "example.csv")