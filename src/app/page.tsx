'use client'

import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react'

export default function Home() {
  const [envValue, setEnvValue] = useState('')

  useEffect(() => {
    invoke<string>('get_env', { name: 'WEATHER_API_SECRET' })
      .then((result) => setEnvValue(result))
      .catch(console.error)

    const aoba = async () => {
      const res = await invoke<string>('get_env', {
        name: 'WEATHER_API_SECRET',
      })
      console.log(res)
    }
    aoba()
  }, [])

  return (
    <main className="text-white">
      <h1>hello there</h1>
      <p>{envValue}</p>
    </main>
  )
}
