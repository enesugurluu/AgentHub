import './App.css'
import { PtyTerminal } from './components/PtyTerminal'

function App() {
  return (
    <main style={{ padding: 24, maxWidth: 1100, margin: '0 auto' }}>
      <h1 style={{ marginBottom: 12 }}>agentHub</h1>
      <PtyTerminal />
    </main>
  )
}

export default App
