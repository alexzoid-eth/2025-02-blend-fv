Error running certoraMutate for user alexzoid-eth with args: ['--collect_mode', '--collect_file', 'blend-contracts-v2/backstop/collection/isolation_pool__collect.json', '--dump_csv', 'blend-contracts-v2/backstop/coverage/isolation_pool_.csv', '--dump_link', 'blend-contracts-v2/backstop/coverage/isolation_pool_.txt']
But, there was no error output from certoraMutate

Possible cause: this mutation job had no rules complete (all timed out / incomplete):  https://prover.certora.com/output/7749274/b4776e4f67a44fa6b0049741213306cd/?anonymousKey=eba3fabe8b3ae5df4a8fb8b371d3d2e2457ff69a
Tried deleting that job entry in isolation_pool__collect.json but that didn't prevent the error
This job also had all rules time out (but job state is not "Problem"): https://prover.certora.com/output/7749274/755e9dc96cab4c5c9976183e0dd7fbe7/?anonymousKey=b9ae546580d56145c80417c1e56b548e65b6d2f4
Deleted that one from the json also ... but this also didn't prevent the error
Since none of the other mutation jobs had a rule fail (no catches), I am renaming isolation_pool__collect.json to EXCLUDE it entirely
