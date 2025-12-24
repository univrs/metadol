# Run biomimicry implementation  DONE
#npx claude-flow@alpha swarm "read claude-flow-biomimicry.yaml --workflow biomimicry-full"

# Run Parser dol 2
npx claude-flow@alpha swarm "read claude-flow-parser-dol2.yaml --workflow parser-full"

# Run SEX system implementation
npx claude-flow@alpha swarm "read claude-flow-sex-system.yaml --workflow sex-full"



# Run specific sub-workflows
npx claude-flow@alpha swarm "read claude-flow-biomimicry.yaml --workflow mycelium-only"

npx claude-flow@alpha swarm "read claude-flow-sex-system.yaml --workflow parsing-only"
