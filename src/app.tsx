import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

function App() {
  const [message, setMessage] = useState('')

  useEffect(() => {
    const init = async () => {
      try {
        const result = await invoke('init_launcher')
        console.log('✅ Инициализация:', result)
        setMessage(String(result))
      } catch (e) {
        console.error('❌ Ошибка инициализации:', e)
        setMessage('Ошибка инициализации')
      }
    }
    init()
  }, [])

  const play = async () => {
    console.log('🟢 Нажата кнопка "Играть"')
    try {
      const result = await invoke('launch_minecraft')
      console.log('🟢 Результат:', result)
      setMessage(String(result))
    } catch (error) {
      console.error('🔴 Ошибка:', error)
      setMessage('Ошибка: ' + String(error))
      alert('Не удалось запустить Minecraft')
    }
  }

  return (
    <div className="launcher">
      <div className="header">
        <div className="logo">
          <span className="logo-icon">⚡</span>
          AirDLC
        </div>
        <div className="version-badge">v1.0.0</div>
      </div>

      <div className="hero">
        <h1>Готов к <span className="highlight">победе</span>?</h1>
        <p className="subtitle">Запускай Minecraft с максимальным контролем и скоростью</p>
      </div>

      <div className="play-section">
        <button className="play-btn" onClick={play}>
          <span className="play-icon">▶</span> Играть
        </button>
        <p className="status">{message}</p>
      </div>

      <div className="footer">
        <span>© 2026 AirDLC. Все права защищены.</span>
        <div className="footer-links">
          <a href="#">Discord</a>
          <a href="#">Поддержка</a>
        </div>
      </div>
    </div>
  )
}

export default App