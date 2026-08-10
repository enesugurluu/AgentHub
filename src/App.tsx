import { useEffect } from 'react'

import { AgentSidebar } from '@/components/AgentSidebar'
import { InspectorPanel } from '@/components/InspectorPanel'
import { OfficeFloor } from '@/components/OfficeFloor'
import { TerminalTabs } from '@/components/TerminalTabs'
import { TopBar } from '@/components/TopBar'
import { useAgentStore } from '@/store/agents'

function App() {
  const fetchAgents = useAgentStore((s) => s.fetchAgents)

  useEffect(() => {
    void fetchAgents()
  }, [fetchAgents])

  return (
    <div className="flex h-full flex-col bg-background text-foreground">
      <TopBar />
      <main className="grid min-h-0 flex-1 grid-cols-[240px_1fr_260px]">
        <AgentSidebar />
        <OfficeFloor />
        <InspectorPanel />
      </main>
      <TerminalTabs />
    </div>
  )
}

export default App
