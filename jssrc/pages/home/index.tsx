import { useStore } from '@/store/context'
import { observer } from 'mobx-react-lite'
import React, { useState } from 'react'
import init, { encrypt_to_base64, decrypt_from_base64 } from '../../../pkg/wasm_demo'
import Wasm from '../../../pkg/wasm_demo_bg.wasm'
import { Link } from 'react-router-dom'
import './index.less'

const HomePage = observer(() => {
  const [getResult, setGetResult] = useState('')
  const store = useStore()

  const testWasm = async () => {
    await init(Wasm)
    // console.log(greet())

    const origin = 'hello world'
    const encoded_str = encrypt_to_base64(origin)
    console.log(encoded_str) // ${encoded_str}
    const decoded_str = decrypt_from_base64(encoded_str)
    console.log(decoded_str === origin)
  }

  return (
    <div className="Home">
      <button onClick={testWasm}>call wasm</button>
    </div>
  )
})

export default HomePage
